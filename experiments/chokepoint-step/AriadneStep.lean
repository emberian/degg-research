/-
  AriadneStep.lean -- the choke-point device with Ariadne state continuity.

  Lean 4 core only, no imports: checks with a bare `lean`, like CommitStep.lean. The step is a
  first-order function over structures and lists (no closures in the state), so it can be exported
  like `CommitBytes.dregg_chokepoint_arrive` and carried across the HOL4 bridge.

  Why: CommitStep's `restore` accepts a snapshot whose counter equals the hardware counter. With an
  asynchronous host, crashes and a store-replaying adversary that rule both bricks the device after
  an honest crash and, with out-of-order persists, lets a replayed store hide a forwarded frame
  (Veil pilot, `/Users/ember/dev/veil-pilot/VeilPilot/ChokepointAsync.lean`). Ariadne (Strackx and
  Piessens, USENIX Security 2016, §4.1 Listing 2; `/Users/ember/paperbin/strackx2016-ariadne-
  state-continuity.txt`) fixes both with one monotonic counter `C` and packages on untrusted disk:

    store:    write a package (C + 1, state), then increment C;
    retrieve: accept only a package with counter C; then twice
              (write a package (C + 1, recovered state), increment C) before resuming.

  The counter lives in non-volatile memory as a Gray code, so an increment is one bit flip
  (`GrayCounter.gray_one_flip`). Here every `step` case is one atomic action: one package write,
  one increment, or one forward; a crash may come between any two.

  What is proved (every hash `H`, every run):
    * non_omission           : against an adversary that keeps every package the device wrote,
                               hands back any of them (or none) on every restore, crashes the device
                               anywhere and advances the counter at will, a running device holds
                               every entry that has left it.
    * restore_accepts_only_current : resumption only from a package with the counter's value.
    * honest_never_halts     : with an honest disk and no counter tampering, the device never
                               fail-stops, however crashes fall (no brick).
    * honest_recovers        : from any crash, the honest restore and four atomic steps resume it
                               (one uninterrupted sequence; progress under further crashes needs
                               crashes to stop, as in Ariadne §5).
    * occ_binds              : an occurrence id is never carried by two different entries, so a
                               receiver that drops repeats by id is exact. Repeats do happen:
                               recovery re-queues the committed state (at-least-once delivery;
                               CommitStep's `forwarded_ids_nodup` does not survive crashes).
  Falsifiers, checked by `decide`: the naive repair "accept one behind" (`tolerateOne_omits`) and
  Ariadne's Appendix A.3 strawman, one increment in retrieve (`singleInc_omits`), each end with a
  running device missing a forwarded entry; the same event lists against `step` do not.

  Model boundary (named, not proved; Codex astra review 2026-11-06):
    * the MAC: the adversary cannot forge packages, so restores carry only packages in `made`;
    * writes are synchronous and atomic: a `write` output completes before the next step (the host
      acknowledges), and a torn write fails its MAC, which is the `restore none` case (fail-stop,
      so it costs liveness, not safety);
    * the counter: `GrayCounter` proves the arithmetic (one bit per increment, injective); that a
      single-bit NV write is atomic under power loss is the hardware's promise, and the finite
      width (overflow) is not modelled;
    * `non_omission` is about membership: it does not by itself exclude a fork that keeps every
      forwarded entry. Ariadne's "skipping unprocessed input" is reachable here: an entry admitted
      but never committed can resurface through retrieve (Codex's trace) and is then persisted
      before it is forwarded; arrivals outside `run` are dropped silently (no refusal output);
    * Ariadne's `purge_state` appears only as the initial package (`world0`), not as a reset action;
    * one device instance (Ariadne §6: no second instance racing on the counter).
-/

namespace Chokepoint.Ariadne

variable {D : Type} [DecidableEq D]

/-- A log entry: occurrence id (the counter value that commits it), payload digest, payload. -/
structure Entry (D : Type) where
  occ     : Nat
  digest  : D
  payload : List UInt8
deriving DecidableEq, Repr

/-- A package on untrusted disk: counter value and the device state it carries. (The MAC is the
device's; the model's adversary can present only packages the device wrote.) -/
structure Pkg (D : Type) where
  ctr : Nat
  log : List (Entry D)
deriving DecidableEq, Repr

inductive Mode where
  | run | commit | down | rec1 | rec1i | rec2 | rec2i
deriving DecidableEq, Repr

structure Dev (D : Type) where
  hw       : Nat              -- the non-volatile counter (stored Gray-coded; see `GrayCounter.lean`)
  pc       : Mode
  halted   : Bool
  log      : List (Entry D)   -- RAM: the committed state
  inflight : List (Entry D)   -- RAM: the admission being committed ([] or one entry)
  queue    : List (Entry D)   -- RAM: committed entries still to forward
deriving DecidableEq, Repr

inductive Event (D : Type) where
  | arrive  (payload : List UInt8)
  | tick                            -- the next atomic step of a commit or a recovery
  | forward
  | crash
  | restore (p : Option (Pkg D))    -- the host hands back a package (none: it has nothing)
deriving DecidableEq, Repr

inductive Output (D : Type) where
  | write   (p : Pkg D)             -- store this package on disk
  | forward (e : Entry D)           -- the frame leaves the device
deriving DecidableEq, Repr

/-- The device step (Ariadne, Strackx and Piessens 2016, Listing 2). Every case is one atomic
action: one package write, or one counter increment, or one forward. -/
def step (H : List UInt8 → D) (d : Dev D) (ev : Event D) : Dev D × List (Output D) :=
  if d.halted then (d, []) else
  match ev with
  | .arrive p =>
    if d.pc = .run then
      let x : Entry D := ⟨d.hw + 1, H p, p⟩
      ({ d with pc := .commit, inflight := [x] }, [.write ⟨d.hw + 1, d.log ++ [x]⟩])
    else (d, [])
  | .tick =>
    match d.pc with
    | .commit => ({ d with hw := d.hw + 1, log := d.log ++ d.inflight,
                           queue := d.queue ++ d.inflight, inflight := [], pc := .run }, [])
    | .rec1   => ({ d with pc := .rec1i }, [.write ⟨d.hw + 1, d.log⟩])
    | .rec1i  => ({ d with hw := d.hw + 1, pc := .rec2 }, [])
    | .rec2   => ({ d with pc := .rec2i }, [.write ⟨d.hw + 1, d.log⟩])
    | .rec2i  => ({ d with hw := d.hw + 1, pc := .run, queue := d.log }, [])
    | _       => (d, [])
  | .forward =>
    if d.pc = .run then
      match d.queue with
      | x :: rest => ({ d with queue := rest.filter (· != x) }, [.forward x])
      | []        => (d, [])
    else (d, [])
  | .crash =>
    if d.pc = .down then (d, [])
    else ({ d with pc := .down, log := [], inflight := [], queue := [] }, [])
  | .restore (some q) =>
    if d.pc = .down then
      if q.ctr = d.hw then ({ d with log := q.log, pc := .rec1 }, [])
      else ({ d with halted := true }, [])
    else (d, [])
  | .restore none =>
    if d.pc = .down then ({ d with halted := true }, []) else (d, [])

def writesOf : List (Output D) → List (Pkg D)
  | [] => []
  | .write p :: os => p :: writesOf os
  | .forward _ :: os => writesOf os

def forwardsOf : List (Output D) → List (Entry D)
  | [] => []
  | .write _ :: os => forwardsOf os
  | .forward e :: os => e :: forwardsOf os

/-! ## The world: the device, every package it ever wrote, and what has left -/

structure World (D : Type) where
  dev  : Dev D
  made : List (Pkg D)            -- every package written (the adversary keeps them all)
  fwd  : List (Entry D × Nat)    -- ghost: forwarded entries, with the counter when they left

inductive WEvent (D : Type) where
  | dev  (e : Event D)
  | bump                          -- the adversary advances the counter (denial of service)

def wstep (H : List UInt8 → D) (w : World D) : WEvent D → World D
  | .dev e =>
    let r := step H w.dev e
    { dev := r.1, made := w.made ++ writesOf r.2,
      fwd := w.fwd ++ (forwardsOf r.2).map (fun x => (x, w.dev.hw)) }
  | .bump => { w with dev := { w.dev with hw := w.dev.hw + 1 } }

/-- The adversary may hand back any package the device wrote (the MAC stops forgeries), or none. -/
def Admissible (w : World D) : WEvent D → Prop
  | .dev (.restore (some q)) => q ∈ w.made
  | _ => True

/-- Ariadne's `purge_state`: the empty state stored under counter 0. -/
def world0 : World D :=
  { dev := ⟨0, .run, false, [], [], []⟩, made := [⟨0, []⟩], fwd := [] }

inductive Reach (H : List UInt8 → D) : World D → Prop where
  | init : Reach H world0
  | step {w : World D} {e : WEvent D} : Reach H w → Admissible w e → Reach H (wstep H w e)

/-! ## The inductive invariant -/

def Sub (l m : List (Entry D)) : Prop := ∀ x, x ∈ l → x ∈ m

structure Inv (w : World D) : Prop where
  ahead      : ∀ q ∈ w.made, q.ctr ≤ w.dev.hw + 1
  noneAhead  : (w.dev.pc = .run ∨ w.dev.pc = .rec2) → ∀ q ∈ w.made, q.ctr ≤ w.dev.hw
  freshRun   : w.dev.pc = .run → ∀ q ∈ w.made, w.dev.hw ≤ q.ctr → Sub w.dev.log q.log
  freshCommit : w.dev.pc = .commit → ∀ q ∈ w.made, w.dev.hw + 1 ≤ q.ctr →
                  Sub (w.dev.log ++ w.dev.inflight) q.log
  freshRec2  : w.dev.pc = .rec2i → ∀ q ∈ w.made, w.dev.hw + 1 ≤ q.ctr → Sub w.dev.log q.log
  fwdStamp   : ∀ p ∈ w.fwd, p.2 ≤ w.dev.hw
  fwdLater   : ∀ p ∈ w.fwd, ∀ q ∈ w.made, p.2 ≤ q.ctr → p.1 ∈ q.log
  fwdRam     : w.dev.pc ≠ .down → ∀ p ∈ w.fwd, p.1 ∈ w.dev.log
  queueLog   : Sub w.dev.queue w.dev.log
  idleQueue  : (w.dev.pc = .down ∨ w.dev.pc = .rec1 ∨ w.dev.pc = .rec1i ∨ w.dev.pc = .rec2 ∨
                w.dev.pc = .rec2i) → w.dev.queue = []
  inflightOk : w.dev.pc ≠ .commit → w.dev.inflight = []

omit [DecidableEq D] in
theorem inv_init : Inv (world0 : World D) := by
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_⟩ <;>
    simp [world0, Sub]


theorem inv_bump {w : World D} (h : Inv w) : Inv (wstep H w .bump) := by
  rcases w with ⟨⟨hw, pc, halted, log, inflight, queue⟩, made, fwd⟩
  rcases h with ⟨a1, a2, a3, a5, a6, k1, k2, k3, q1, q2, q3⟩
  rw [show wstep H _ _ = (⟨⟨hw + 1, pc, halted, log, inflight, queue⟩, made, fwd⟩ : World D) by simp [wstep]]
  dsimp only at *
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_, k2, k3, q1, q2, q3⟩
  · intro q hq; dsimp only at *; have := a1 q hq; omega
  · intro hp q hq; dsimp only at *; have := a2 hp q hq; omega
  · intro hp q hq hle; dsimp only at *; exact a3 hp q hq (by omega)
  · intro hp q hq hle; dsimp only at *; have := a1 q hq; omega
  · intro hp q hq hle; dsimp only at *; have := a1 q hq; omega
  · intro p hp; dsimp only at *; have := k1 p hp; omega

theorem inv_halted {w : World D} (h : Inv w) (hh : w.dev.halted = true) (e : Event D) :
    wstep H w (.dev e) = w := by
  rcases w with ⟨d, made, fwd⟩
  simp only at hh
  simp [wstep, step, hh, writesOf, forwardsOf]

omit [DecidableEq D] in
/-- A step that changes only the device's halted flag. -/
theorem inv_set_halted {hw : Nat} {pc : Mode} {b : Bool} {log inflight queue : List (Entry D)}
    {made : List (Pkg D)} {fwd : List (Entry D × Nat)} {b' : Bool}
    (h : Inv ⟨⟨hw, pc, b, log, inflight, queue⟩, made, fwd⟩) :
    Inv ⟨⟨hw, pc, b', log, inflight, queue⟩, made, fwd⟩ := by
  rcases h with ⟨a1, a2, a3, a5, a6, k1, k2, k3, q1, q2, q3⟩
  exact ⟨a1, a2, a3, a5, a6, k1, k2, k3, q1, q2, q3⟩

theorem inv_dev {w : World D} (h : Inv w) (ha : Admissible w (.dev e)) :
    Inv (wstep H w (.dev e)) := by
  rcases w with ⟨⟨hw, pc, halted, log, inflight, queue⟩, made, fwd⟩
  cases halted
  case true => rw [inv_halted h rfl]; exact h
  have h' := h
  rcases h with ⟨a1, a2, a3, a5, a6, k1, k2, k3, q1, q2, q3⟩
  dsimp only at a1 a2 a3 a5 a6 k1 k2 k3 q1 q2 q3
  cases e with
  | arrive p =>
    cases pc
    case run =>
      rw [show wstep H _ _ = (⟨⟨hw, .commit, false, log, [⟨hw + 1, H p, p⟩], queue⟩,
        made ++ [⟨hw + 1, log ++ [⟨hw + 1, H p, p⟩]⟩], fwd⟩ : World D) by simp [wstep, step, writesOf, forwardsOf]]
      refine ⟨?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_, q1, ?_, ?_⟩ <;> dsimp only
      · intro q hq; simp at hq; rcases hq with hq | rfl
        · have := a2 (.inl rfl) q hq; omega
        · simp
      · intro hp; simp at hp
      · intro hp; simp at hp
      · intro _ q hq hle; simp at hq; rcases hq with hq | rfl
        · have := a2 (.inl rfl) q hq; omega
        · intro x hx; exact hx
      · intro hp; simp at hp
      · exact k1
      · intro pr hpr q hq hle; simp at hq; rcases hq with hq | rfl
        · exact k2 pr hpr q hq hle
        · exact List.mem_append_left _ (k3 (by simp) pr hpr)
      · intro _; exact k3 (by simp)
      · intro hp; simp at hp
      · intro hp; simp at hp
    all_goals (simp [wstep, step, writesOf, forwardsOf]; first | exact h' | simp_all)
  | tick =>
    cases pc
    case commit =>
      rw [show wstep H _ _ = (⟨⟨hw + 1, .run, false, log ++ inflight, [], queue ++ inflight⟩, made, fwd⟩ : World D) by simp [wstep, step, writesOf, forwardsOf]]
      refine ⟨?_, ?_, ?_, ?_, ?_, ?_, k2, ?_, ?_, ?_, ?_⟩ <;> dsimp only
      · intro q hq; have := a1 q hq; omega
      · intro _ q hq; exact a1 q hq
      · intro _ q hq hle; exact a5 rfl q hq hle
      · intro hp; simp at hp
      · intro hp; simp at hp
      · intro pr hpr; have := k1 pr hpr; omega
      · intro _ pr hpr; exact List.mem_append_left _ (k3 (by simp) pr hpr)
      · intro x hx; simp at hx; rcases hx with hx | hx
        · exact List.mem_append_left _ (q1 x hx)
        · exact List.mem_append_right _ hx
      · intro hp; simp at hp
      · intro _; rfl
    case rec1 =>
      rw [show wstep H _ _ = (⟨⟨hw, .rec1i, false, log, inflight, queue⟩, made ++ [⟨hw + 1, log⟩], fwd⟩ : World D) by simp [wstep, step, writesOf, forwardsOf]]
      refine ⟨?_, ?_, ?_, ?_, ?_, k1, ?_, ?_, q1, ?_, ?_⟩ <;> dsimp only
      · intro q hq; simp at hq; rcases hq with hq | rfl
        · exact a1 q hq
        · simp
      · intro hp; simp at hp
      · intro hp; simp at hp
      · intro hp; simp at hp
      · intro hp; simp at hp
      · intro pr hpr q hq hle; simp at hq; rcases hq with hq | rfl
        · exact k2 pr hpr q hq hle
        · exact k3 (by simp) pr hpr
      · intro _; exact k3 (by simp)
      · intro _; exact q2 (by simp)
      · intro _; exact q3 (by simp)
    case rec1i =>
      rw [show wstep H _ _ = (⟨⟨hw + 1, .rec2, false, log, inflight, queue⟩, made, fwd⟩ : World D) by simp [wstep, step, writesOf, forwardsOf]]
      refine ⟨?_, ?_, ?_, ?_, ?_, ?_, k2, ?_, q1, ?_, ?_⟩ <;> dsimp only
      · intro q hq; have := a1 q hq; omega
      · intro _ q hq; exact a1 q hq
      · intro hp; simp at hp
      · intro hp; simp at hp
      · intro hp; simp at hp
      · intro pr hpr; have := k1 pr hpr; omega
      · intro _; exact k3 (by simp)
      · intro _; exact q2 (by simp)
      · intro _; exact q3 (by simp)
    case rec2 =>
      rw [show wstep H _ _ = (⟨⟨hw, .rec2i, false, log, inflight, queue⟩, made ++ [⟨hw + 1, log⟩], fwd⟩ : World D) by simp [wstep, step, writesOf, forwardsOf]]
      refine ⟨?_, ?_, ?_, ?_, ?_, k1, ?_, ?_, q1, ?_, ?_⟩ <;> dsimp only
      · intro q hq; simp at hq; rcases hq with hq | rfl
        · have := a2 (.inr rfl) q hq; omega
        · simp
      · intro hp; simp at hp
      · intro hp; simp at hp
      · intro hp; simp at hp
      · intro _ q hq hle; simp at hq; rcases hq with hq | rfl
        · have := a2 (.inr rfl) q hq; omega
        · intro x hx; exact hx
      · intro pr hpr q hq hle; simp at hq; rcases hq with hq | rfl
        · exact k2 pr hpr q hq hle
        · exact k3 (by simp) pr hpr
      · intro _; exact k3 (by simp)
      · intro _; exact q2 (by simp)
      · intro _; exact q3 (by simp)
    case rec2i =>
      rw [show wstep H _ _ = (⟨⟨hw + 1, .run, false, log, inflight, log⟩, made, fwd⟩ : World D) by simp [wstep, step, writesOf, forwardsOf]]
      refine ⟨?_, ?_, ?_, ?_, ?_, ?_, k2, ?_, ?_, ?_, ?_⟩ <;> dsimp only
      · intro q hq; have := a1 q hq; omega
      · intro _ q hq; exact a1 q hq
      · intro _ q hq hle; exact a6 rfl q hq hle
      · intro hp; simp at hp
      · intro hp; simp at hp
      · intro pr hpr; have := k1 pr hpr; omega
      · intro _; exact k3 (by simp)
      · intro x hx; exact hx
      · intro hp; simp at hp
      · intro _; exact q3 (by simp)
    all_goals (simp [wstep, step, writesOf, forwardsOf]; exact h')
  | forward =>
    cases pc
    case run =>
      cases queue with
      | nil => simp [wstep, step, writesOf, forwardsOf]; exact h'
      | cons x rest =>
        rw [show wstep H _ _ = (⟨⟨hw, .run, false, log, inflight, rest.filter (· != x)⟩, made,
          fwd ++ [(x, hw)]⟩ : World D) by simp [wstep, step, writesOf, forwardsOf]]
        have hx : x ∈ log := q1 x (by simp)
        refine ⟨a1, a2, a3, a5, a6, ?_, ?_, ?_, ?_, ?_, q3⟩ <;> dsimp only
        · intro pr hpr; simp at hpr; rcases hpr with hpr | rfl
          · exact k1 pr hpr
          · simp
        · intro pr hpr q hq hle; simp at hpr; rcases hpr with hpr | rfl
          · exact k2 pr hpr q hq hle
          · exact a3 rfl q hq hle x hx
        · intro _ pr hpr; simp at hpr; rcases hpr with hpr | rfl
          · exact k3 (by simp) pr hpr
          · exact hx
        · intro y hy; exact q1 y (by simp at hy; simp [hy.1])
        · intro hp; simp at hp
    all_goals (simp [wstep, step, writesOf, forwardsOf]; first | exact h' | simp_all)
  | crash =>
    cases pc
    case down => simp [wstep, step, writesOf, forwardsOf]; exact h'
    all_goals
      rw [show wstep H _ _ = (⟨⟨hw, .down, false, [], [], []⟩, made, fwd⟩ : World D) by
        simp [wstep, step, writesOf, forwardsOf]]
      refine ⟨a1, ?_, ?_, ?_, ?_, k1, k2, ?_, ?_, ?_, ?_⟩ <;> simp [Sub]
  | restore p =>
    cases p with
    | none =>
      cases pc
      case down =>
        simp [wstep, step, writesOf, forwardsOf]
        exact inv_set_halted h'
      all_goals (simp [wstep, step, writesOf, forwardsOf]; first | exact h' | simp_all)
    | some q =>
      have hq : q ∈ made := ha
      cases pc
      case down =>
        by_cases hc : q.ctr = hw
        · rw [show wstep H _ _ = (⟨⟨hw, .rec1, false, q.log, inflight, queue⟩, made, fwd⟩ : World D) by
            simp [wstep, step, writesOf, forwardsOf, hc]]
          have hq0 : queue = [] := q2 (.inl rfl)
          refine ⟨a1, ?_, ?_, ?_, ?_, k1, k2, ?_, ?_, ?_, ?_⟩ <;> dsimp only
          · intro hp; simp at hp
          · intro hp; simp at hp
          · intro hp; simp at hp
          · intro hp; simp at hp
          · intro _ pr hpr; exact k2 pr hpr q hq (by rw [hc]; exact k1 pr hpr)
          · intro x hx; rw [hq0] at hx; simp at hx
          · intro _; exact hq0
          · intro _; exact q3 (by simp)
        · rw [show wstep H _ _ = (⟨⟨hw, .down, true, log, inflight, queue⟩, made, fwd⟩ : World D) by
            simp [wstep, step, writesOf, forwardsOf, hc]]
          exact inv_set_halted h'
      all_goals (simp [wstep, step, writesOf, forwardsOf]; first | exact h' | simp_all)


theorem inv_step {w : World D} {e : WEvent D} (h : Inv w) (ha : Admissible w e) :
    Inv (wstep H w e) := by
  cases e with
  | dev e => exact inv_dev h ha
  | bump => exact inv_bump h

theorem reach_inv {w : World D} (h : Reach H w) : Inv w := by
  induction h with
  | init => exact inv_init
  | step _ ha ih => exact inv_step ih ha

/-- **non_omission.** Against an adversary that keeps every package the device ever wrote, hands
back any of them (or none) on every restore, crashes the device between any two atomic steps, and
advances the counter at will: whenever the device is running, every entry that has left it is in
its committed state. -/
theorem non_omission {w : World D} (h : Reach H w) (hrun : w.dev.pc = .run) :
    ∀ p ∈ w.fwd, p.1 ∈ w.dev.log :=
  (reach_inv h).fwdRam (by rw [hrun]; decide)

/-- **restore_accepts_only_current.** The device resumes only from a package carrying the counter's
current value; anything else halts it. -/
theorem restore_accepts_only_current (d : Dev D) (q : Pkg D) (hd : d.halted = false)
    (hpc : d.pc = .down) :
    (step H d (.restore (some q))).1.pc = .rec1 ↔ q.ctr = d.hw := by
  by_cases hc : q.ctr = d.hw <;> simp [step, hd, hpc, hc]

/-! ## Honest operation: no brick, and recovery completes -/

/-- What an honest disk returns for counter value `c`: the latest package written under it. -/
def lookup (w : World D) : Option (Pkg D) := w.made.reverse.find? (fun q => q.ctr == w.dev.hw)

/-- Honest: no counter tampering, and every restore gets what the disk holds under the counter. -/
def HonestAdm (w : World D) : WEvent D → Prop
  | .bump => False
  | .dev (.restore p) => p = lookup w
  | _ => True

inductive HReach (H : List UInt8 → D) : World D → Prop where
  | init : HReach H world0
  | step {w : World D} {e : WEvent D} : HReach H w → HonestAdm w e → HReach H (wstep H w e)

structure HInv (w : World D) : Prop where
  current : ∃ q ∈ w.made, q.ctr = w.dev.hw
  next    : (w.dev.pc = .commit ∨ w.dev.pc = .rec1i ∨ w.dev.pc = .rec2i) →
              ∃ q ∈ w.made, q.ctr = w.dev.hw + 1
  live    : w.dev.halted = false

omit [DecidableEq D] in
theorem lookup_current {w : World D} (h : ∃ q ∈ w.made, q.ctr = w.dev.hw) :
    ∃ q, lookup w = some q ∧ q.ctr = w.dev.hw := by
  unfold lookup
  cases hf : w.made.reverse.find? (fun q => q.ctr == w.dev.hw) with
  | none =>
    obtain ⟨q, hq, hc⟩ := h
    have := List.find?_eq_none.mp hf q (List.mem_reverse.mpr hq)
    simp [hc] at this
  | some q =>
    have := List.find?_some hf
    exact ⟨q, rfl, by simpa using this⟩

omit [DecidableEq D] in
theorem hinv_init : HInv (world0 : World D) :=
  ⟨⟨⟨0, []⟩, by simp [world0], rfl⟩, by simp [world0], rfl⟩

theorem hinv_step {w : World D} {e : WEvent D} (h : HInv w) (ha : HonestAdm w e) :
    HInv (wstep H w e) := by
  rcases w with ⟨⟨hw, pc, halted, log, inflight, queue⟩, made, fwd⟩
  have h0 := h
  rcases h with ⟨⟨c, hc, hcc⟩, hn, hl⟩
  dsimp only at hc hcc hn hl
  subst hl
  have cur : ∀ m : List (Pkg D), ∃ q ∈ made ++ m, q.ctr = hw := fun m => ⟨c, by simp [hc], hcc⟩
  cases e with
  | bump => exact absurd ha id
  | dev e =>
    cases e with
    | arrive p =>
      cases pc
      case run =>
        rw [show wstep H _ _ = (⟨⟨hw, .commit, false, log, [⟨hw + 1, H p, p⟩], queue⟩,
          made ++ [⟨hw + 1, log ++ [⟨hw + 1, H p, p⟩]⟩], fwd⟩ : World D) by
            simp [wstep, step, writesOf, forwardsOf]]
        exact ⟨cur _, fun _ => ⟨_, List.mem_append_right _ (List.mem_singleton_self _), rfl⟩, rfl⟩
      all_goals (simp [wstep, step, writesOf, forwardsOf]; exact h0)
    | tick =>
      cases pc
      case commit =>
        rw [show wstep H _ _ = (⟨⟨hw + 1, .run, false, log ++ inflight, [], queue ++ inflight⟩,
          made, fwd⟩ : World D) by simp [wstep, step, writesOf, forwardsOf]]
        exact ⟨hn (.inl rfl), by simp, rfl⟩
      case rec1 =>
        rw [show wstep H _ _ = (⟨⟨hw, .rec1i, false, log, inflight, queue⟩, made ++ [⟨hw + 1, log⟩],
          fwd⟩ : World D) by simp [wstep, step, writesOf, forwardsOf]]
        exact ⟨cur _, fun _ => ⟨_, List.mem_append_right _ (List.mem_singleton_self _), rfl⟩, rfl⟩
      case rec1i =>
        rw [show wstep H _ _ = (⟨⟨hw + 1, .rec2, false, log, inflight, queue⟩, made, fwd⟩ : World D) by
          simp [wstep, step, writesOf, forwardsOf]]
        exact ⟨hn (.inr (.inl rfl)), by simp, rfl⟩
      case rec2 =>
        rw [show wstep H _ _ = (⟨⟨hw, .rec2i, false, log, inflight, queue⟩, made ++ [⟨hw + 1, log⟩],
          fwd⟩ : World D) by simp [wstep, step, writesOf, forwardsOf]]
        exact ⟨cur _, fun _ => ⟨_, List.mem_append_right _ (List.mem_singleton_self _), rfl⟩, rfl⟩
      case rec2i =>
        rw [show wstep H _ _ = (⟨⟨hw + 1, .run, false, log, inflight, log⟩, made, fwd⟩ : World D) by
          simp [wstep, step, writesOf, forwardsOf]]
        exact ⟨hn (.inr (.inr rfl)), by simp, rfl⟩
      all_goals (simp [wstep, step, writesOf, forwardsOf]; exact h0)
    | forward =>
      cases pc
      case run =>
        cases queue with
        | nil => simp [wstep, step, writesOf, forwardsOf]; exact h0
        | cons x rest =>
          rw [show wstep H _ _ = (⟨⟨hw, .run, false, log, inflight, rest.filter (· != x)⟩, made,
            fwd ++ [(x, hw)]⟩ : World D) by simp [wstep, step, writesOf, forwardsOf]]
          exact ⟨⟨c, hc, hcc⟩, by simp, rfl⟩
      all_goals (simp [wstep, step, writesOf, forwardsOf]; exact h0)
    | crash =>
      cases pc
      case down => simp [wstep, step, writesOf, forwardsOf]; exact h0
      all_goals
        rw [show wstep H _ _ = (⟨⟨hw, .down, false, [], [], []⟩, made, fwd⟩ : World D) by
          simp [wstep, step, writesOf, forwardsOf]]
        exact ⟨⟨c, hc, hcc⟩, by simp, rfl⟩
    | restore p =>
      have hp : p = lookup ⟨⟨hw, pc, false, log, inflight, queue⟩, made, fwd⟩ := ha
      obtain ⟨q, hlq, hqc⟩ := lookup_current (w := ⟨⟨hw, pc, false, log, inflight, queue⟩, made, fwd⟩)
        ⟨c, hc, hcc⟩
      rw [hp, hlq]
      cases pc
      case down =>
        rw [show wstep H _ _ = (⟨⟨hw, .rec1, false, q.log, inflight, queue⟩, made, fwd⟩ : World D) by
          simp at hqc; simp [wstep, step, writesOf, forwardsOf, hqc]]
        exact ⟨⟨c, hc, hcc⟩, by simp, rfl⟩
      all_goals (simp [wstep, step, writesOf, forwardsOf]; exact h0)

theorem hreach_hinv {w : World D} (h : HReach H w) : HInv w := by
  induction h with
  | init => exact hinv_init
  | step _ ha ih => exact hinv_step ih ha

/-- **honest_never_halts (no brick).** With an honest disk and no counter tampering, the device
never fail-stops, however the crashes fall. -/
theorem honest_never_halts {w : World D} (h : HReach H w) : w.dev.halted = false :=
  (hreach_hinv h).live

/-- **honest_recovers.** From any crash, the honest restore and four atomic steps put the device
back into normal operation (this is where Ariadne's two increments go). -/
theorem honest_recovers {w : World D} (h : HReach H w) (hd : w.dev.pc = .down) :
    let w1 := wstep H w (.dev (.restore (lookup w)))
    let w5 := wstep H (wstep H (wstep H (wstep H w1 (.dev .tick)) (.dev .tick)) (.dev .tick)) (.dev .tick)
    w5.dev.pc = .run ∧ w5.dev.halted = false := by
  have hi := hreach_hinv h
  obtain ⟨q, hlq, hqc⟩ := lookup_current hi.current
  rcases w with ⟨⟨hw, pc, halted, log, inflight, queue⟩, made, fwd⟩
  dsimp only at hd hqc hlq
  have hl : halted = false := hi.live
  subst hd hl
  simp [hlq, wstep, step, writesOf, forwardsOf, hqc]

/-! ## Occurrence ids bind entries

Recovery re-queues the whole committed state, so an entry can leave twice (at-least-once: a crash
between a forward and anything durable cannot tell whether it left). The receiver drops repeats by
occurrence id. That is sound because an occurrence id is never given to two different entries:
an entry gets `occ = C + 1` only when admitted in `run` at counter `C`, and the device never runs at
counter `C` again after leaving it (commit or crash both move the counter past `C + 1` before the
next `run`). -/

def AllIn (l : List (Entry D)) (made : List (Pkg D)) : Prop := ∀ x ∈ l, ∃ q ∈ made, x ∈ q.log

structure IdInv (w : World D) : Prop where
  binds    : ∀ q₁ ∈ w.made, ∀ q₂ ∈ w.made, ∀ x ∈ q₁.log, ∀ y ∈ q₂.log, x.occ = y.occ → x = y
  bound    : ∀ q ∈ w.made, ∀ x ∈ q.log, x.occ ≤ w.dev.hw + 1
  boundRun : w.dev.pc = .run → ∀ q ∈ w.made, ∀ x ∈ q.log, x.occ ≤ w.dev.hw
  logIn    : AllIn w.dev.log w.made
  queueIn  : AllIn w.dev.queue w.made
  inflightIn : AllIn w.dev.inflight w.made
  fwdIn    : ∀ p ∈ w.fwd, ∃ q ∈ w.made, p.1 ∈ q.log

omit [DecidableEq D] in
theorem idinv_init : IdInv (world0 : World D) := by
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_, ?_⟩ <;> simp [world0, AllIn]

omit [DecidableEq D] in
theorem allIn_append {l : List (Entry D)} {made : List (Pkg D)} (m : List (Pkg D)) (h : AllIn l made) :
    AllIn l (made ++ m) := by
  intro x hx; obtain ⟨q, hq, hxq⟩ := h x hx; exact ⟨q, by simp [hq], hxq⟩

theorem idinv_step {w : World D} {e : WEvent D} (hI : Inv w) (h : IdInv w) (ha : Admissible w e) :
    IdInv (wstep H w e) := by
  rcases w with ⟨⟨hw, pc, halted, log, inflight, queue⟩, made, fwd⟩
  have h' := h
  rcases h with ⟨u1, u2, u3, u4, u5, u7, u6⟩
  dsimp only at u1 u2 u3 u4 u5 u6 u7
  cases e with
  | bump =>
    rw [show wstep H _ _ = (⟨⟨hw + 1, pc, halted, log, inflight, queue⟩, made, fwd⟩ : World D) by simp [wstep]]
    refine ⟨u1, ?_, ?_, u4, u5, u7, u6⟩ <;> dsimp only
    · intro q hq x hx; have := u2 q hq x hx; omega
    · intro hp q hq x hx; have := u3 hp q hq x hx; omega
  | dev e =>
  cases halted
  case true => rw [inv_halted hI rfl]; exact h'
  cases e with
  | arrive p =>
    cases pc
    case run =>
      rw [show wstep H _ _ = (⟨⟨hw, .commit, false, log, [⟨hw + 1, H p, p⟩], queue⟩,
        made ++ [⟨hw + 1, log ++ [⟨hw + 1, H p, p⟩]⟩], fwd⟩ : World D) by
          simp [wstep, step, writesOf, forwardsOf]]
      have old : ∀ y, (∃ q ∈ made, y ∈ q.log) → y.occ ≤ hw := by
        rintro y ⟨q, hq, hy⟩; exact u3 rfl q hq y hy
      have inMade : ∀ y ∈ log ++ [(⟨hw + 1, H p, p⟩ : Entry D)], y = ⟨hw + 1, H p, p⟩ ∨ ∃ q ∈ made, y ∈ q.log := by
        intro y hy; simp at hy; rcases hy with hy | rfl
        · exact .inr (u4 y hy)
        · exact .inl rfl
      refine ⟨?_, ?_, ?_, allIn_append _ u4, allIn_append _ u5, ?_, ?_⟩ <;> dsimp only
      · intro q₁ hq₁ q₂ hq₂ x hx y hy hxy
        simp at hq₁ hq₂
        have cx : x = ⟨hw + 1, H p, p⟩ ∨ ∃ q ∈ made, x ∈ q.log := by
          rcases hq₁ with hq₁ | rfl
          · exact .inr ⟨q₁, hq₁, hx⟩
          · exact inMade x hx
        have cy : y = ⟨hw + 1, H p, p⟩ ∨ ∃ q ∈ made, y ∈ q.log := by
          rcases hq₂ with hq₂ | rfl
          · exact .inr ⟨q₂, hq₂, hy⟩
          · exact inMade y hy
        rcases cx with rfl | ⟨r, hr, hxr⟩ <;> rcases cy with rfl | ⟨s, hs, hys⟩
        · rfl
        · have := old y ⟨s, hs, hys⟩; simp at hxy; omega
        · have := old x ⟨r, hr, hxr⟩; simp at hxy; omega
        · exact u1 r hr s hs x hxr y hys hxy
      · intro q hq x hx; simp at hq; rcases hq with hq | rfl
        · have := u3 rfl q hq x hx; omega
        · rcases inMade x hx with rfl | hm
          · simp
          · have := old x hm; omega
      · intro hp; simp at hp
      · intro y hy; simp at hy; subst hy; exact ⟨_, List.mem_append_right _ (List.mem_singleton_self _), by simp⟩
      · intro pr hpr; obtain ⟨q, hq, h1⟩ := u6 pr hpr; exact ⟨q, by simp [hq], h1⟩
    all_goals (simp [wstep, step, writesOf, forwardsOf]; first | exact h' | simp_all)
  | tick =>
    cases pc
    case commit =>
      rw [show wstep H _ _ = (⟨⟨hw + 1, .run, false, log ++ inflight, [], queue ++ inflight⟩,
        made, fwd⟩ : World D) by simp [wstep, step, writesOf, forwardsOf]]
      refine ⟨u1, ?_, ?_, ?_, ?_, ?_, u6⟩ <;> dsimp only
      · intro q hq x hx; have := u2 q hq x hx; omega
      · intro _ q hq x hx; exact u2 q hq x hx
      · intro x hx; simp at hx; rcases hx with hx | hx
        · exact u4 x hx
        · exact u7 x hx
      · intro x hx; simp at hx; rcases hx with hx | hx
        · exact u5 x hx
        · exact u7 x hx
      · intro x hx; simp at hx
    case rec1 =>
      rw [show wstep H _ _ = (⟨⟨hw, .rec1i, false, log, inflight, queue⟩, made ++ [⟨hw + 1, log⟩],
        fwd⟩ : World D) by simp [wstep, step, writesOf, forwardsOf]]
      refine ⟨?_, ?_, ?_, allIn_append _ u4, allIn_append _ u5, allIn_append _ u7, ?_⟩ <;> dsimp only
      · intro q₁ hq₁ q₂ hq₂ x hx y hy hxy
        simp at hq₁ hq₂
        have cx : ∃ q ∈ made, x ∈ q.log := by
          rcases hq₁ with hq₁ | rfl
          · exact ⟨q₁, hq₁, hx⟩
          · exact u4 x hx
        have cy : ∃ q ∈ made, y ∈ q.log := by
          rcases hq₂ with hq₂ | rfl
          · exact ⟨q₂, hq₂, hy⟩
          · exact u4 y hy
        obtain ⟨r, hr, hxr⟩ := cx; obtain ⟨t, ht, hyt⟩ := cy
        exact u1 r hr t ht x hxr y hyt hxy
      · intro q hq x hx; simp at hq; rcases hq with hq | rfl
        · exact u2 q hq x hx
        · obtain ⟨r, hr, hxr⟩ := u4 x hx; exact u2 r hr x hxr
      · intro hp; simp at hp
      · intro pr hpr; obtain ⟨q, hq, h1⟩ := u6 pr hpr; exact ⟨q, by simp [hq], h1⟩
    case rec1i =>
      rw [show wstep H _ _ = (⟨⟨hw + 1, .rec2, false, log, inflight, queue⟩, made, fwd⟩ : World D) by
        simp [wstep, step, writesOf, forwardsOf]]
      refine ⟨u1, ?_, ?_, u4, u5, u7, u6⟩ <;> dsimp only
      · intro q hq x hx; have := u2 q hq x hx; omega
      · intro hp; simp at hp
    case rec2 =>
      rw [show wstep H _ _ = (⟨⟨hw, .rec2i, false, log, inflight, queue⟩, made ++ [⟨hw + 1, log⟩],
        fwd⟩ : World D) by simp [wstep, step, writesOf, forwardsOf]]
      refine ⟨?_, ?_, ?_, allIn_append _ u4, allIn_append _ u5, allIn_append _ u7, ?_⟩ <;> dsimp only
      · intro q₁ hq₁ q₂ hq₂ x hx y hy hxy
        simp at hq₁ hq₂
        have cx : ∃ q ∈ made, x ∈ q.log := by
          rcases hq₁ with hq₁ | rfl
          · exact ⟨q₁, hq₁, hx⟩
          · exact u4 x hx
        have cy : ∃ q ∈ made, y ∈ q.log := by
          rcases hq₂ with hq₂ | rfl
          · exact ⟨q₂, hq₂, hy⟩
          · exact u4 y hy
        obtain ⟨r, hr, hxr⟩ := cx; obtain ⟨t, ht, hyt⟩ := cy
        exact u1 r hr t ht x hxr y hyt hxy
      · intro q hq x hx; simp at hq; rcases hq with hq | rfl
        · exact u2 q hq x hx
        · obtain ⟨r, hr, hxr⟩ := u4 x hx; exact u2 r hr x hxr
      · intro hp; simp at hp
      · intro pr hpr; obtain ⟨q, hq, h1⟩ := u6 pr hpr; exact ⟨q, by simp [hq], h1⟩
    case rec2i =>
      rw [show wstep H _ _ = (⟨⟨hw + 1, .run, false, log, inflight, log⟩, made, fwd⟩ : World D) by
        simp [wstep, step, writesOf, forwardsOf]]
      refine ⟨u1, ?_, ?_, u4, u4, u7, u6⟩ <;> dsimp only
      · intro q hq x hx; have := u2 q hq x hx; omega
      · intro _ q hq x hx; exact u2 q hq x hx
    all_goals (simp [wstep, step, writesOf, forwardsOf]; exact h')
  | forward =>
    cases pc
    case run =>
      cases queue with
      | nil => simp [wstep, step, writesOf, forwardsOf]; exact h'
      | cons x rest =>
        rw [show wstep H _ _ = (⟨⟨hw, .run, false, log, inflight, rest.filter (· != x)⟩, made,
          fwd ++ [(x, hw)]⟩ : World D) by simp [wstep, step, writesOf, forwardsOf]]
        refine ⟨u1, u2, u3, u4, ?_, u7, ?_⟩ <;> dsimp only
        · intro y hy; simp at hy; exact u5 y (by simp [hy.1])
        · intro pr hpr; simp at hpr; rcases hpr with hpr | rfl
          · exact u6 pr hpr
          · exact u5 x (by simp)
    all_goals (simp [wstep, step, writesOf, forwardsOf]; first | exact h' | simp_all)
  | crash =>
    cases pc
    case down => simp [wstep, step, writesOf, forwardsOf]; exact h'
    all_goals
      rw [show wstep H _ _ = (⟨⟨hw, .down, false, [], [], []⟩, made, fwd⟩ : World D) by
        simp [wstep, step, writesOf, forwardsOf]]
      refine ⟨u1, u2, ?_, ?_, ?_, ?_, u6⟩ <;> simp [AllIn]
  | restore p =>
    cases p with
    | none =>
      cases pc
      case down =>
        rw [show wstep H _ _ = (⟨⟨hw, .down, true, log, inflight, queue⟩, made, fwd⟩ : World D) by
          simp [wstep, step, writesOf, forwardsOf]]
        exact ⟨u1, u2, u3, u4, u5, u7, u6⟩
      all_goals (simp [wstep, step, writesOf, forwardsOf]; first | exact h' | simp_all)
    | some q =>
      have hq : q ∈ made := ha
      cases pc
      case down =>
        by_cases hc : q.ctr = hw
        · rw [show wstep H _ _ = (⟨⟨hw, .rec1, false, q.log, inflight, queue⟩, made, fwd⟩ : World D) by
            simp [wstep, step, writesOf, forwardsOf, hc]]
          refine ⟨u1, u2, ?_, ?_, u5, u7, u6⟩ <;> dsimp only
          · intro hp; simp at hp
          · intro x hx; exact ⟨q, hq, hx⟩
        · rw [show wstep H _ _ = (⟨⟨hw, .down, true, log, inflight, queue⟩, made, fwd⟩ : World D) by
            simp [wstep, step, writesOf, forwardsOf, hc]]
          exact ⟨u1, u2, u3, u4, u5, u7, u6⟩
      all_goals (simp [wstep, step, writesOf, forwardsOf]; first | exact h' | simp_all)

theorem reach_idinv {w : World D} (h : Reach H w) : IdInv w := by
  induction h with
  | init => exact idinv_init
  | step hr ha ih => exact idinv_step (reach_inv hr) ih ha

/-- **occ_binds.** No occurrence id is ever carried by two different entries, in any package the
device wrote; in particular, two forwards with one id forward the same entry, so a receiver that
drops repeats by id loses nothing and keeps nothing twice. -/
theorem occ_binds {w : World D} (h : Reach H w) :
    ∀ p₁ ∈ w.fwd, ∀ p₂ ∈ w.fwd, p₁.1.occ = p₂.1.occ → p₁.1 = p₂.1 := by
  have hi := reach_idinv h
  intro p₁ h₁ p₂ h₂ he
  obtain ⟨q₁, hq₁, hx⟩ := hi.fwdIn p₁ h₁
  obtain ⟨q₂, hq₂, hy⟩ := hi.fwdIn p₂ h₂
  exact hi.binds q₁ hq₁ q₂ hq₂ _ hx _ hy he

/-! ## The refuted variants, as concrete runs (kernel-checked by `decide`)

Each variant changes one case of `step`. The attacks hand back only packages the device wrote
(checked: every restore is admissible), and end with the device running while an entry that has
left is missing from its state. -/

/-- The naive repair: restore also accepts a package one behind the counter. -/
def stepTolerateOne (H : List UInt8 → D) (d : Dev D) : Event D → Dev D × List (Output D)
  | .restore (some q) =>
    if d.halted = false ∧ d.pc = .down ∧ q.ctr + 1 = d.hw then ({ d with log := q.log, pc := .rec1 }, [])
    else step H d (.restore (some q))
  | e => step H d e

/-- Ariadne's Appendix A.3 strawman: one increment in retrieve instead of two. -/
def stepSingleInc (H : List UInt8 → D) (d : Dev D) : Event D → Dev D × List (Output D)
  | .tick =>
    if d.halted = false ∧ d.pc = .rec1i then ({ d with hw := d.hw + 1, pc := .run, queue := d.log }, [])
    else step H d .tick
  | e => step H d e

def wstepWith (st : Dev D → Event D → Dev D × List (Output D)) (w : World D) (e : Event D) : World D :=
  let r := st w.dev e
  { dev := r.1, made := w.made ++ writesOf r.2,
    fwd := w.fwd ++ (forwardsOf r.2).map (fun x => (x, w.dev.hw)) }

/-- Run events, checking each restore hands back a package already written. -/
def runChecked (st : Dev D → Event D → Dev D × List (Output D)) : World D → List (Event D) → Option (World D)
  | w, [] => some w
  | w, e :: es =>
    match e with
    | .restore (some q) => if w.made.contains q then runChecked st (wstepWith st w e) es else none
    | _ => runChecked st (wstepWith st w e) es

def toyH (p : List UInt8) : Nat := p.length

/-- The device is running and some entry that left is missing from its state. -/
def omits (w : World Nat) : Bool := w.dev.pc == .run && w.fwd.any (fun p => !(w.dev.log.contains p.1))

def e1 : Entry Nat := ⟨1, 1, [7]⟩

/-- Commit and forward one frame, crash, hand back the initial package (one behind). -/
def attackTolerateOne : List (Event Nat) :=
  [.arrive [7], .tick, .forward, .crash, .restore (some ⟨0, []⟩), .tick, .tick, .tick, .tick]

theorem tolerateOne_omits :
    (runChecked (stepTolerateOne toyH) world0 attackTolerateOne).map omits = some true := by decide

def eg : Entry Nat := ⟨1, 1, [7]⟩
def eh : Entry Nat := ⟨2, 1, [8]⟩

/-- Two packages end up under one counter value with different contents; the device resumes from
one, forwards from it, crashes, and resumes from the other. -/
def attackSingleInc : List (Event Nat) :=
  [.arrive [7], .crash, .restore (some ⟨0, []⟩), .tick, .tick,       -- P1 = (1,[g]); resumed at 1
   .arrive [8], .crash, .restore (some ⟨1, [eg]⟩), .tick, .tick,     -- P3 = (2,[h]); resume P1
   .forward, .crash, .restore (some ⟨2, [eh]⟩), .tick, .tick]        -- g leaves; resume P3

theorem singleInc_omits :
    (runChecked (stepSingleInc toyH) world0 attackSingleInc).map omits = some true := by decide

/-- Controls. The first attack against Ariadne's `step` is admissible and ends fail-stopped, not
omitting (the one-behind package is refused). The second is not even admissible against `step`:
its third restore hands back a package `(2, [h])` that Ariadne never writes. -/
theorem ariadne_same_attacks_safe :
    (runChecked (step toyH) world0 attackTolerateOne).map omits = some false ∧
    (runChecked (step toyH) world0 attackSingleInc) = none := by decide

end Chokepoint.Ariadne
