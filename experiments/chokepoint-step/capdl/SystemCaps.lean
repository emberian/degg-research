/-
# Capability distribution of a Microkit image, as data, with decidable authority predicates

A `Spec` is the CapDL specification the Microkit tool hands seL4's CapDL initialiser: every
kernel object, and every capability in its slots. The image is constructed from exactly this
specification (the initialiser is the trusted step; see Named residuals). It is transliterated
from `microkit --capdl-json` by `capdl_to_lean.py`, with no semantic reduction. Everything about
*who can reach what* is computed here:

* `vspaceFrames` — the frames a thread can touch through its address space: walk from the
  TCB's VSpace root through page tables to frame caps.
* `cspaceCaps` — the capabilities a thread can invoke: walk from the TCB's CSpace root
  through CNodes.
* `touches` — the thread maps one of the given frames (e.g. those overlapping a physical window).
* `holdsIrq` — the thread can invoke an IRQ's handler.
* `canWaitOn` — the thread holds a notification cap with the Read right (seL4_Wait).

The statements about `chokepoint.system` are in `ChokepointCapTheorems.lean`.
-/
namespace CapDL

inductive Cap where
  | frame (obj : Nat) (write : Bool)
  | pageTable (obj : Nat)
  | cnode (obj : Nat)
  | irqHandler (obj : Nat)
  | notification (obj : Nat) (read write : Bool)
  | endpoint (obj : Nat) (grant : Bool)
  | tcb (obj : Nat)
  | other (obj : Nat)
  deriving DecidableEq, Repr

inductive Kind where
  | frame (paddr : Option Nat) (sizeBits : Nat)
  | pageTable | cnode | tcb | irq | ntfn | ep | other
  deriving DecidableEq, Repr

structure Obj where
  name : String
  kind : Kind
  caps : List Cap
  deriving Repr

/-- Objects (index = object id) and the IRQ table `(irq number, handler object)`. -/
structure Spec where
  objs : List Obj
  irqs : List (Nat × Nat)

/-- Every object with its id. Each predicate below makes one pass over this list and then works
on small filtered lists (the kernel evaluates these by `decide +kernel`). -/
def indexed (s : Spec) : List (Nat × Obj) := (List.range s.objs.length).zip s.objs

/-- The containers a walk passes through: page tables and CNodes. -/
def containers (s : Spec) : List (Nat × Obj) :=
  (indexed s).filter fun (_, o) => o.kind = .pageTable ∨ o.kind = .cnode

def lookupIn (cs : List (Nat × Obj)) (i : Nat) : Option Obj :=
  (cs.find? (·.1 = i)).map (·.2)

/-- Frames reachable from a page table, through page tables. -/
def ptFrames (cs : List (Nat × Obj)) : Nat → Nat → List (Nat × Bool)
  | 0, _ => []
  | fuel + 1, pt =>
    match lookupIn cs pt with
    | some o =>
      if o.kind = .pageTable then
        o.caps.flatMap fun
          | .frame f w => [(f, w)]
          | .pageTable p => ptFrames cs fuel p
          | _ => []
      else []
    | none => []

/-- Capabilities reachable from a CNode, through CNodes. -/
def cnodeCaps (cs : List (Nat × Obj)) : Nat → Nat → List Cap
  | 0, _ => []
  | fuel + 1, cn =>
    match lookupIn cs cn with
    | some o =>
      if o.kind = .cnode then
        o.caps.flatMap fun
          | .cnode c => .cnode c :: cnodeCaps cs fuel c
          | c => [c]
      else []
    | none => []

/-- The threads, with their objects. -/
def tcbs (s : Spec) : List (Nat × Obj) := (indexed s).filter fun (_, o) => o.kind = .tcb

def vspaceFrames (cs : List (Nat × Obj)) (t : Obj) : List (Nat × Bool) :=
  t.caps.flatMap fun
    | .pageTable p => ptFrames cs 8 p
    | _ => []

def cspaceCaps (cs : List (Nat × Obj)) (t : Obj) : List Cap :=
  t.caps.flatMap fun
    | .cnode c => cnodeCaps cs 8 c
    | _ => []

/-- The frames covering part of physical `[lo, hi)`. -/
def windowFrames (s : Spec) (lo hi : Nat) : List Nat :=
  ((indexed s).filter fun (_, o) =>
    match o.kind with
    | .frame (some p) sb => decide (p < hi ∧ lo < p + 2 ^ sb)
    | _ => false).map (·.1)

/-- The frames of a Microkit memory region (named `frame_mr_<region>_<n>` by the tool). -/
def regionFrames (s : Spec) (pfx : String) : List Nat :=
  ((indexed s).filter fun (_, o) => o.name.startsWith pfx).map (·.1)

def touches (cs : List (Nat × Obj)) (fs : List Nat) (needWrite : Bool) (t : Obj) : Bool :=
  (vspaceFrames cs t).any fun (f, w) => (!needWrite || w) && fs.contains f

def handlerOf (s : Spec) (n : Nat) : List Nat :=
  (s.irqs.filter (·.1 = n)).map (·.2)

def holdsIrq (cs : List (Nat × Obj)) (hs : List Nat) (t : Obj) : Bool :=
  (cspaceCaps cs t).any fun
    | .irqHandler h => hs.contains h
    | _ => false

/-- The notifications IRQ `n` signals (the caps in its handler objects). -/
def irqNotifications (s : Spec) (n : Nat) : List Nat :=
  ((indexed s).filter fun (i, _) => (handlerOf s n).contains i).flatMap fun (_, o) =>
    o.caps.filterMap fun
      | .notification o _ _ => some o
      | _ => none

/-- `t` can seL4_Wait on notification `o`: a Read-right cap in its CSpace or bound to it. -/
def canWaitOn (cs : List (Nat × Obj)) (t : Obj) (o : Nat) : Bool :=
  ((cspaceCaps cs t) ++ t.caps).any fun
    | .notification o' r _ => o' = o && r
    | _ => false

def holdsEndpoint (cs : List (Nat × Obj)) (e : Nat) (t : Obj) : Bool :=
  (cspaceCaps cs t).any fun
    | .endpoint e' _ => e' = e
    | _ => false

def idOf (s : Spec) (n : String) : Option Nat :=
  ((indexed s).find? fun (_, o) => o.name = n).map (·.1)

/-- **The choke-point capability property.** Of all threads in the image:
1. only `commit` maps any frame overlapping the NIC's MMIO window, and it does;
2. only `commit` can invoke the NIC IRQ's handler, and it can;
3. only `persist` maps the durable log, and it maps it writable;
4. only `commit` and `persist` hold a cap to persist's endpoint (the PPC). -/
def chokepointCaps (s : Spec) (nicLo nicHi irq : Nat) : Bool :=
  let cs := containers s
  let ts := tcbs s
  let nic := windowFrames s nicLo nicHi
  let log := regionFrames s "frame_mr_durable_log_"
  let hs := handlerOf s irq
  match idOf s "tcb_commit", idOf s "tcb_persist", idOf s "ep_persist" with
  | some c, some p, some e =>
    !nic.isEmpty && !log.isEmpty && !hs.isEmpty &&
    ts.any (fun (i, t) => i = c && touches cs nic false t && holdsIrq cs hs t) &&
    ts.all (fun (i, t) => !touches cs nic false t || i = c) &&
    ts.all (fun (i, t) => !holdsIrq cs hs t || i = c) &&
    ts.any (fun (i, t) => i = p && touches cs log true t) &&
    ts.all (fun (i, t) => !touches cs log false t || i = p) &&
    ts.all (fun (i, t) => !holdsEndpoint cs e t || i = c || i = p)
  | _, _, _ => false

end CapDL
