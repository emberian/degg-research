/-
  CommitBytes.lean -- the concrete BLAKE3 device: leaf byte encoding with an injectivity
  theorem, the collision reduction, the summary-state refinement, and the `@[export]` wire.

  * BLAKE3 is minidregg's Lean BLAKE3 (`Compiler/Blake3.lean`), imported from the minidregg
    source tree, not copied. Its identity with BLAKE3 is minidregg's KAT (`Compiler/Blake3Kat`);
    one vector is re-checked below.
  * `enc_inj` : the leaf encoding (8-byte LE fields, length-prefixed manifest root, then the
    digest) is injective on leaves whose numeric fields fit in 64 bits.
  * `leaf_collision` : two distinct payloads whose leaves encode to the same bytes are a
    BLAKE3 collision. That is the reduction Codex round 2 asked for, in place of "injectivity".
  * `stepS_refines` : the device the export runs keeps only a summary of the log (its length);
    the summary step commutes with the projection of the full `step`, outputs included.
  * `@[export dregg_chokepoint_arrive]` : a String -> String C-ABI entry, the
    `dregg_deleg_admit` pattern. The wire codec itself is pinned by `#guard`s, not by a theorem
    (named residual, as in DelegAdmit).
  * The Ariadne device (`AriadneStep.lean`), section 6: a byte codec for its state, events,
    packages and outputs, with round-trip theorems for every value whose numbers fit 64 bits;
    `stepBytes_complete` and `stepBytes_sound` (the byte step is exactly `Ariadne.step
    Blake3.hash` on canonical encodings and refuses everything else); and
    `@[export dregg_chokepoint_ariadne_step]`, the String -> String entry for the HOL4 bridge.
-/
import Compiler.Blake3
import CommitStep
import AriadneStep

namespace Chokepoint.Bytes

open Chokepoint
open Minidregg.Compiler

/-! ## 1. Fixed-width little-endian encoding and its injectivity -/

/-- Eight little-endian bytes of a 64-bit word. -/
def le8 (x : BitVec 64) : List UInt8 :=
  (List.range 8).map (fun i => UInt8.ofBitVec (x.extractLsb' (8 * i) 8))

theorem le8_length (x : BitVec 64) : (le8 x).length = 8 := by simp [le8]

theorem le8_inj {x y : BitVec 64} (h : le8 x = le8 y) : x = y := by
  apply BitVec.eq_of_getLsbD_eq
  intro j hj
  have hi : (le8 x)[j / 8]? = (le8 y)[j / 8]? := by rw [h]
  have hlt : j / 8 < 8 := by omega
  simp only [le8, List.getElem?_map, List.getElem?_range hlt, Option.map_some,
    Option.some.injEq] at hi
  have hb := congrArg (fun u : UInt8 => u.toBitVec.getLsbD (j % 8)) hi
  simp only [BitVec.getLsbD_extractLsb'] at hb
  have hm : j % 8 < 8 := Nat.mod_lt _ (by omega)
  have hj' : 8 * (j / 8) + j % 8 = j := by omega
  simpa [hm, hj'] using hb

/-- `le8` of a bounded natural number. -/
def le8n (n : Nat) : List UInt8 := le8 (BitVec.ofNat 64 n)

theorem le8n_inj {a b : Nat} (ha : a < 2 ^ 64) (hb : b < 2 ^ 64) (h : le8n a = le8n b) : a = b := by
  have := congrArg BitVec.toNat (le8_inj h)
  simpa [BitVec.toNat_ofNat, Nat.mod_eq_of_lt ha, Nat.mod_eq_of_lt hb] using this

/-! ## 2. The leaf encoding -/

abbrev Digest := List UInt8

/-- Leaf bytes: occ ‖ epoch ‖ len(vjm) ‖ vjm ‖ job ‖ digest. The only variable-length interior
field is length-prefixed; the digest is the tail. -/
def enc (l : Leaf Digest) : List UInt8 :=
  le8n l.occ ++ (le8n l.epoch ++ (le8n l.vjm.length ++ (l.vjm ++ (le8n l.job ++ l.digest))))

/-- Leaves whose numeric fields fit the 64-bit encoding. -/
def Bounded (l : Leaf Digest) : Prop :=
  l.occ < 2 ^ 64 ∧ l.epoch < 2 ^ 64 ∧ l.vjm.length < 2 ^ 64 ∧ l.job < 2 ^ 64

/-- **enc_inj.** The leaf encoding is injective on bounded leaves. -/
theorem enc_inj {a b : Leaf Digest} (ha : Bounded a) (hb : Bounded b) (h : enc a = enc b) :
    a = b := by
  obtain ⟨ho, he, hv, hj⟩ := ha
  obtain ⟨ho', he', hv', hj'⟩ := hb
  unfold enc at h
  obtain ⟨h1, h⟩ := List.append_inj h (by simp [le8n, le8_length])
  obtain ⟨h2, h⟩ := List.append_inj h (by simp [le8n, le8_length])
  obtain ⟨h3, h⟩ := List.append_inj h (by simp [le8n, le8_length])
  have hlen : a.vjm.length = b.vjm.length := le8n_inj hv hv' h3
  obtain ⟨h4, h⟩ := List.append_inj h hlen
  obtain ⟨h5, h6⟩ := List.append_inj h (by simp [le8n, le8_length])
  cases a; cases b
  simp only [Leaf.mk.injEq]
  exact ⟨le8n_inj ho ho' h1, le8n_inj he he' h2, h4, le8n_inj hj hj' h5, h6⟩

/-- **leaf_collision.** If two leaves carry BLAKE3 digests of two DIFFERENT payloads and encode
to the same bytes, those payloads are a BLAKE3 collision. Equivalently: under BLAKE3 collision
resistance, a leaf's bytes determine the payload it committed. -/
theorem leaf_collision {a b : Leaf Digest} {p q : List UInt8} (ha : Bounded a) (hb : Bounded b)
    (hpa : a.digest = Blake3.hash p) (hqb : b.digest = Blake3.hash q) (h : enc a = enc b)
    (hne : p ≠ q) : Blake3.hash p = Blake3.hash q ∧ p ≠ q := by
  have := enc_inj ha hb h
  subst this
  exact ⟨by rw [← hpa, ← hqb], hne⟩

/-! ## 3. The summary state the export runs, and its refinement theorem -/

/-- What the device must hold across calls: everything but the log CONTENT (the log lives in
the persist PD / durable store; the device needs only its length for the capacity check). -/
structure Summary where
  hw     : Nat
  ctr    : Nat
  epoch  : Nat
  vjm    : List UInt8
  logLen : Nat
  cap    : Nat
  halted : Bool
deriving DecidableEq, Repr

def proj (st : DevState Digest) : Summary :=
  ⟨st.hw, st.ctr, st.epoch, st.vjm, st.log.length, st.cap, st.halted⟩

/-- The summary step: `step` with the log replaced by its length. -/
def stepS (H : List UInt8 → Digest) (s : Summary) : Event → Summary × List (Output Digest)
  | .arrive job p =>
    if s.halted then (s, [.refuse s.hw])
    else if s.logLen < s.cap then
      ({ s with hw := s.hw + 1, ctr := s.hw + 1, logLen := s.logLen + 1 },
       [.persist ⟨s.hw, s.epoch, s.vjm, job, H p⟩, .forward s.hw p])
    else ({ s with hw := s.hw + 1, ctr := s.hw + 1 }, [.refuse s.hw])
  | .setVjm r => ({ s with vjm := r }, [])
  | .restore n c =>
    if c = s.hw ∧ n = s.logLen ∧ s.halted = false then ({ s with ctr := c }, [])
    else ({ s with halted := true }, [])

/-- **stepS_refines.** The summary step computes exactly the projection of the full step:
same outputs, same projected successor. So every theorem of `CommitStep` about `step`'s
outputs holds of what the export emits. -/
theorem stepS_refines (H : List UInt8 → Digest) (st : DevState Digest) (e : Event) :
    stepS H (proj st) e = (proj (step H st e).1, (step H st e).2) := by
  cases e with
  | arrive job p =>
    by_cases hh : st.halted = true
    · simp [stepS, step, proj, hh]
    · by_cases hc : st.log.length < st.cap
      · simp [stepS, step, proj, hh, hc]
      · simp [stepS, step, proj, hh, hc]
  | setVjm r => simp [stepS, step, proj]
  | restore n c =>
    by_cases hcond : c = st.hw ∧ n = st.log.length ∧ st.halted = false
    · simp [stepS, step, proj, hcond]
    · simp [stepS, step, proj, hcond]

/-! ## 4. The wire and the `@[export]` -/

def hexDigit (c : Char) : Option Nat :=
  if '0' ≤ c ∧ c ≤ '9' then some (c.toNat - '0'.toNat)
  else if 'a' ≤ c ∧ c ≤ 'f' then some (c.toNat - 'a'.toNat + 10)
  else none

def hexDecode : List Char → Option (List UInt8)
  | [] => some []
  | a :: b :: rest =>
    match hexDigit a, hexDigit b, hexDecode rest with
    | some x, some y, some r => some (UInt8.ofNat (16 * x + y) :: r)
    | _, _, _ => none
  | [_] => none

def hexEncode (bs : List UInt8) : String :=
  String.join (bs.map fun b =>
    let d := "0123456789abcdef".toList
    String.ofList [d.getD (b.toNat / 16) '0', d.getD (b.toNat % 16) '0'])

/-- Wire in: `hw ctr epoch logLen cap halted(0|1) job vjmhex payloadhex` (an empty hex field is
written `-`). Wire out: `""` malformed (no verdict, fail-closed); `R <hw'> <logLen'>` refused;
`F <hw'> <logLen'> <occ> <leafhex>` = persist `leafhex` durably, THEN forward the payload. -/
def parseHex (t : String) : Option (List UInt8) :=
  if t = "-" then some [] else hexDecode t.toList

def arriveWire (s : String) : String :=
  match (s.splitOn " ").filter (· ≠ "") with
  | [hw, ctr, ep, ll, cap, hl, job, vjm, pay] =>
    match hw.toNat?, ctr.toNat?, ep.toNat?, ll.toNat?, cap.toNat?, hl.toNat?, job.toNat?,
          parseHex vjm, parseHex pay with
    | some hw, some ctr, some ep, some ll, some cap, some hl, some job, some vjm, some pay =>
      if hl > 1 then "" else
      let s0 : Summary := ⟨hw, ctr, ep, vjm, ll, cap, hl = 1⟩
      match stepS Blake3.hash s0 (.arrive job pay) with
      | (s1, [.persist l, .forward o _]) =>
        s!"F {s1.hw} {s1.logLen} {o} {hexEncode (enc l)}"
      | (s1, [.refuse _]) => s!"R {s1.hw} {s1.logLen}"
      | _ => ""
    | _, _, _, _, _, _, _, _, _ => ""
  | _ => ""

/-- **`@[export dregg_chokepoint_arrive]`** -- the C-ABI entry a Microkit PD or the host
dataplane calls per offered frame. The decision and the leaf bytes are computed HERE. -/
@[export dregg_chokepoint_arrive]
def arriveFFI (s : String) : String := arriveWire s

/-! ## 5. Self-tests (`#guard` fails the check on regression) -/

-- BLAKE3("") official vector, re-checked against the imported implementation.
#guard Blake3.toHex (Blake3.hash []) ==
  "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
-- Hex round trip on a sample.
#guard hexDecode (hexEncode [0, 1, 171, 255]).toList == some [0, 1, 171, 255]
-- Live device with room: forwards, id 5, log grows to 1, leaf = 8+8+8+2+8+32 = 66 bytes.
#guard (arriveWire "5 5 1 0 4 0 9 abcd 68656c6c6f").startsWith "F 6 1 5 "
#guard (((arriveWire "5 5 1 0 4 0 9 abcd 68656c6c6f").splitOn " ").getD 4 "").length == 132
-- Full device: refuses, burns the id, log unchanged.
#guard arriveWire "5 5 1 4 4 0 9 abcd 68656c6c6f" == "R 6 4"
-- Halted device: refuses, nothing changes.
#guard arriveWire "5 5 1 0 4 1 9 abcd 68656c6c6f" == "R 5 0"
-- Fail-closed on malformation.
#guard arriveWire "5 5 1 0 4 0 9 abcd 68656c6c6" == ""      -- odd hex
#guard arriveWire "5 5 1 0 4 2 9 abcd 68656c6c6f" == ""     -- halted flag not 0/1
#guard arriveWire "5 5 1 0 4 0 9 abcd" == ""                -- short wire
#guard arriveWire "" == ""
-- The leaf's digest field IS BLAKE3 of the payload (tail 32 bytes of the leaf).
#guard (((arriveWire "5 5 1 0 4 0 9 - 68656c6c6f").splitOn " ").getD 4 "").endsWith
  (Blake3.toHex (Blake3.hash "hello".toUTF8.toList))

end Chokepoint.Bytes

/-! # 6. The Ariadne device on bytes -/

namespace Chokepoint.AriadneBytes

open Chokepoint.Ariadne
open Minidregg.Compiler

/-! ## Fixed-width words -/

/-- `k` little-endian base-256 digits of `n`. -/
def digits : Nat → Nat → List UInt8
  | 0, _ => []
  | k + 1, n => UInt8.ofNat (n % 256) :: digits k (n / 256)

def undigits : List UInt8 → Nat
  | [] => 0
  | b :: bs => b.toNat + 256 * undigits bs

theorem digits_length (k n : Nat) : (digits k n).length = k := by
  induction k generalizing n with
  | zero => rfl
  | succ k ih => simp [digits, ih]

theorem undigits_digits (k n : Nat) : undigits (digits k n) = n % 256 ^ k := by
  induction k generalizing n with
  | zero => simp [digits, undigits, Nat.mod_one]
  | succ k ih =>
    simp only [digits, undigits, ih]
    rw [Nat.pow_succ', Nat.mod_mul]
    simp

/-- A 64-bit word, little-endian. -/
def w64 (n : Nat) : List UInt8 := digits 8 n

def r64 (bs : List UInt8) : Option (Nat × List UInt8) :=
  if bs.length < 8 then none else some (undigits (bs.take 8), bs.drop 8)

theorem r64_w64 {n : Nat} (h : n < 2 ^ 64) (r : List UInt8) : r64 (w64 n ++ r) = some (n, r) := by
  have hl := digits_length 8 n
  have h256 : (256 : Nat) ^ 8 = 2 ^ 64 := by decide
  simp [r64, w64, hl, undigits_digits, h256, Nat.mod_eq_of_lt h]

/-! ## Byte strings and lists -/

/-- A length-prefixed byte string. -/
def wB (bs : List UInt8) : List UInt8 := w64 bs.length ++ bs

def rB (bs : List UInt8) : Option (List UInt8 × List UInt8) := do
  let (n, r) ← r64 bs
  if r.length < n then none else some (r.take n, r.drop n)

theorem rB_wB {bs : List UInt8} (h : bs.length < 2 ^ 64) (r : List UInt8) :
    rB (wB bs ++ r) = some (bs, r) := by
  simp [rB, wB, List.append_assoc, r64_w64 h]

/-- A count-prefixed list. -/
def wL {α : Type} (f : α → List UInt8) (xs : List α) : List UInt8 :=
  w64 xs.length ++ (xs.map f).flatten

def rN {α : Type} (g : List UInt8 → Option (α × List UInt8)) :
    Nat → List UInt8 → Option (List α × List UInt8)
  | 0, bs => some ([], bs)
  | n + 1, bs => do
    let (x, r) ← g bs
    let (xs, r') ← rN g n r
    some (x :: xs, r')

def rL {α : Type} (g : List UInt8 → Option (α × List UInt8)) (bs : List UInt8) :
    Option (List α × List UInt8) := do
  let (n, r) ← r64 bs
  rN g n r

theorem rN_flatten {α : Type} {f : α → List UInt8} {g : List UInt8 → Option (α × List UInt8)}
    {xs : List α} (hg : ∀ x ∈ xs, ∀ r, g (f x ++ r) = some (x, r)) (r : List UInt8) :
    rN g xs.length ((xs.map f).flatten ++ r) = some (xs, r) := by
  induction xs with
  | nil => rfl
  | cons x xs ih =>
    have hx := hg x (by simp)
    have ih' := ih (fun y hy => hg y (by simp [hy]))
    simp [rN, List.append_assoc, hx, ih']

theorem rL_wL {α : Type} {f : α → List UInt8} {g : List UInt8 → Option (α × List UInt8)}
    {xs : List α} (hlen : xs.length < 2 ^ 64) (hg : ∀ x ∈ xs, ∀ r, g (f x ++ r) = some (x, r))
    (r : List UInt8) : rL g (wL f xs ++ r) = some (xs, r) := by
  simp [rL, wL, List.append_assoc, r64_w64 hlen, rN_flatten hg]

/-! ## Entries, packages, device state, events, outputs -/

abbrev Digest := List UInt8

def wE (e : Entry Digest) : List UInt8 := w64 e.occ ++ wB e.digest ++ wB e.payload

def rE (bs : List UInt8) : Option (Entry Digest × List UInt8) := do
  let (o, r) ← r64 bs
  let (d, r) ← rB r
  let (p, r) ← rB r
  some (⟨o, d, p⟩, r)

/-- Every number in the entry fits a 64-bit word. -/
def FitsE (e : Entry Digest) : Prop :=
  e.occ < 2 ^ 64 ∧ e.digest.length < 2 ^ 64 ∧ e.payload.length < 2 ^ 64

def FitsL (l : List (Entry Digest)) : Prop := l.length < 2 ^ 64 ∧ ∀ e ∈ l, FitsE e

theorem rE_wE {e : Entry Digest} (h : FitsE e) (r : List UInt8) : rE (wE e ++ r) = some (e, r) := by
  obtain ⟨h1, h2, h3⟩ := h
  simp [rE, wE, List.append_assoc, r64_w64 h1, rB_wB h2, rB_wB h3]

theorem rL_wE {l : List (Entry Digest)} (h : FitsL l) (r : List UInt8) :
    rL rE (wL wE l ++ r) = some (l, r) :=
  rL_wL h.1 (fun e he r => rE_wE (h.2 e he) r) r

/-- A package as the host stores it (the MAC over these bytes is the host side's). -/
def wP (q : Pkg Digest) : List UInt8 := w64 q.ctr ++ wL wE q.log

def rP (bs : List UInt8) : Option (Pkg Digest × List UInt8) := do
  let (c, r) ← r64 bs
  let (l, r) ← rL rE r
  some (⟨c, l⟩, r)

def FitsP (q : Pkg Digest) : Prop := q.ctr < 2 ^ 64 ∧ FitsL q.log

theorem rP_wP {q : Pkg Digest} (h : FitsP q) (r : List UInt8) : rP (wP q ++ r) = some (q, r) := by
  simp [rP, wP, List.append_assoc, r64_w64 h.1, rL_wE h.2]

def modeTag : Mode → UInt8
  | .run => 0 | .commit => 1 | .down => 2 | .rec1 => 3 | .rec1i => 4 | .rec2 => 5 | .rec2i => 6

def modeOfTag : UInt8 → Option Mode
  | 0 => some .run | 1 => some .commit | 2 => some .down | 3 => some .rec1 | 4 => some .rec1i
  | 5 => some .rec2 | 6 => some .rec2i | _ => none

theorem modeOfTag_modeTag (m : Mode) : modeOfTag (modeTag m) = some m := by cases m <;> rfl

def wD (d : Dev Digest) : List UInt8 :=
  w64 d.hw ++ [modeTag d.pc, if d.halted then 1 else 0] ++ wL wE d.log ++ wL wE d.inflight ++
    wL wE d.queue

def rD (bs : List UInt8) : Option (Dev Digest × List UInt8) := do
  let (hw, r) ← r64 bs
  match r with
  | t :: h :: r =>
    let pc ← modeOfTag t
    let halted ← (if h = 0 then some false else if h = 1 then some true else none)
    let (log, r) ← rL rE r
    let (inflight, r) ← rL rE r
    let (queue, r) ← rL rE r
    some (⟨hw, pc, halted, log, inflight, queue⟩, r)
  | _ => none

def FitsD (d : Dev Digest) : Prop :=
  d.hw < 2 ^ 64 ∧ FitsL d.log ∧ FitsL d.inflight ∧ FitsL d.queue

theorem rD_wD {d : Dev Digest} (h : FitsD d) (r : List UInt8) : rD (wD d ++ r) = some (d, r) := by
  obtain ⟨h1, h2, h3, h4⟩ := h
  cases d with
  | mk hw pc halted log inflight queue =>
    cases halted <;>
      simp [rD, wD, List.append_assoc, r64_w64 h1, modeOfTag_modeTag, rL_wE h2, rL_wE h3, rL_wE h4]

def wEv : Ariadne.Event Digest → List UInt8
  | .arrive p => 0 :: wB p
  | .tick => [1]
  | .forward => [2]
  | .crash => [3]
  | .restore none => [4]
  | .restore (some q) => 5 :: wP q

def rEv : List UInt8 → Option (Ariadne.Event Digest × List UInt8)
  | 0 :: r => do let (p, r) ← rB r; some (.arrive p, r)
  | 1 :: r => some (.tick, r)
  | 2 :: r => some (.forward, r)
  | 3 :: r => some (.crash, r)
  | 4 :: r => some (.restore none, r)
  | 5 :: r => do let (q, r) ← rP r; some (.restore (some q), r)
  | _ => none

def FitsEv : Ariadne.Event Digest → Prop
  | .arrive p => p.length < 2 ^ 64
  | .restore (some q) => FitsP q
  | _ => True

theorem rEv_wEv {ev : Ariadne.Event Digest} (h : FitsEv ev) (r : List UInt8) :
    rEv (wEv ev ++ r) = some (ev, r) := by
  cases ev with
  | arrive p => simp [rEv, wEv, rB_wB (show p.length < 2 ^ 64 from h)]
  | tick => rfl
  | forward => rfl
  | crash => rfl
  | restore q =>
    cases q with
    | none => rfl
    | some q => simp [rEv, wEv, rP_wP (show FitsP q from h)]

def wO : Ariadne.Output Digest → List UInt8
  | .write q => 0 :: wP q
  | .forward e => 1 :: wE e

def rO : List UInt8 → Option (Ariadne.Output Digest × List UInt8)
  | 0 :: r => do let (q, r) ← rP r; some (.write q, r)
  | 1 :: r => do let (e, r) ← rE r; some (.forward e, r)
  | _ => none

/-! ## The byte step -/

/-- The device step on bytes: `wD d ++ wEv ev` in, `wD d' ++ wL wO outs` out, where
`(d', outs) = Ariadne.step Blake3.hash d ev`. Anything else, including a non-canonical encoding or
trailing bytes, is refused (`none`). -/
def stepBytes (bs : List UInt8) : Option (List UInt8) :=
  match rD bs with
  | some (d, r) =>
    match rEv r with
    | some (ev, []) =>
      if wD d ++ wEv ev = bs then
        let o := Ariadne.step Blake3.hash d ev
        some (wD o.1 ++ wL wO o.2)
      else none
    | _ => none
  | none => none

/-- **stepBytes_complete.** On the encoding of any device state and event whose numbers fit
64-bit words, the byte step returns the encoding of `step Blake3.hash`'s result. -/
theorem stepBytes_complete {d : Dev Digest} {ev : Ariadne.Event Digest} (hd : FitsD d) (he : FitsEv ev) :
    stepBytes (wD d ++ wEv ev) =
      some (wD (Ariadne.step Blake3.hash d ev).1 ++ wL wO (Ariadne.step Blake3.hash d ev).2) := by
  have h2 := rEv_wEv he []
  rw [List.append_nil] at h2
  simp [stepBytes, rD_wD hd, h2]

/-- **stepBytes_sound.** Whatever the byte step accepts is the canonical encoding of a device
state and an event, and its answer is the encoding of `step Blake3.hash` on them. So every theorem
of `AriadneStep` about `step` holds of what the export computes. -/
theorem stepBytes_sound {bs out : List UInt8} (h : stepBytes bs = some out) :
    ∃ d ev, bs = wD d ++ wEv ev ∧
      out = wD (Ariadne.step Blake3.hash d ev).1 ++ wL wO (Ariadne.step Blake3.hash d ev).2 := by
  cases hD : rD bs with
  | none => simp [stepBytes, hD] at h
  | some p =>
    obtain ⟨d, r⟩ := p
    cases hE : rEv r with
    | none => simp [stepBytes, hD, hE] at h
    | some p2 =>
      obtain ⟨ev, r'⟩ := p2
      cases r' with
      | cons _ _ => simp [stepBytes, hD, hE] at h
      | nil =>
        by_cases hc : wD d ++ wEv ev = bs
        · simp [stepBytes, hD, hE, hc] at h
          exact ⟨d, ev, hc.symm, h.symm⟩
        · simp [stepBytes, hD, hE, hc] at h

/-! ## The wire and the `@[export]` -/

open Chokepoint.Bytes (hexEncode parseHex)

/-- Wire in: hex of `wD d ++ wEv ev`. Wire out: hex of `wD d' ++ wL wO outs`; `""` refused. The
host must carry out the outputs in order: a `write` completes (durably) before the next call. -/
def ariadneWire (s : String) : String :=
  match parseHex s with
  | some bs => match stepBytes bs with
    | some out => hexEncode out
    | none => ""
  | none => ""

/-- **`@[export dregg_chokepoint_ariadne_step]`** -- the Ariadne device step for the HOL4 bridge
and the host dataplane, BLAKE3 digests computed here. -/
@[export dregg_chokepoint_ariadne_step]
def ariadneFFI (s : String) : String := ariadneWire s

/-! ## Self-tests through the String wire (`#guard`) -/

/-- One call through the wire, decoding the answer as the host would. -/
def viaWire (d : Dev Digest) (ev : Ariadne.Event Digest) : Option (Dev Digest × List (Ariadne.Output Digest)) := do
  let bs ← parseHex (ariadneWire (hexEncode (wD d ++ wEv ev)))
  let (d', r) ← rD bs
  let (outs, r) ← rL rO r
  if r = [] then some (d', outs) else none

def dev0 : Dev Digest := ⟨0, .run, false, [], [], []⟩
def hello : List UInt8 := "hello".toUTF8.toList

/-- Arrive, commit, forward, crash, restore from the package the arrival wrote, retrieve. -/
def honestRun : Option (Dev Digest × Pkg Digest × List (Ariadne.Output Digest)) := do
  let (d, o1) ← viaWire dev0 (.arrive hello)
  let p1 ← match o1 with | [.write q] => some q | _ => none
  let (d, _) ← viaWire d .tick
  let (d, o3) ← viaWire d .forward
  let (d, _) ← viaWire d .crash
  let (d, _) ← viaWire d (.restore (some p1))
  let (d, _) ← viaWire d .tick
  let (d, _) ← viaWire d .tick
  let (d, _) ← viaWire d .tick
  let (d, _) ← viaWire d .tick
  some (d, p1, o3)

-- The leaf's digest is BLAKE3 of the payload; the package carries counter 1 and the new entry.
#guard (honestRun.map fun (_, p1, _) =>
  p1.ctr == 1 && p1.log == [⟨1, Blake3.hash hello, hello⟩]) == some true
#guard (honestRun.map fun (_, p1, _) => (p1.log.map (·.digest)) ==
  [Blake3.hash hello]) == some true
-- The committed entry left, and after crash and retrieve the device runs at counter 3 with it.
#guard (honestRun.map fun (d, p1, o3) =>
  o3 == [.forward ⟨1, Blake3.hash hello, hello⟩] && d.pc == .run && d.hw == 3 &&
    d.halted == false && d.log == p1.log && d.queue == p1.log) == some true
-- A stale package (the initial one, counter 0) after the commit: fail-stop.
#guard ((do
    let (d, _) ← viaWire dev0 (.arrive hello)
    let (d, _) ← viaWire d .tick
    let (d, _) ← viaWire d .crash
    let (d, _) ← viaWire d (.restore (some ⟨0, []⟩))
    some d).map (fun d : Dev Digest => d.halted)) == some true
-- Refusals: malformed hex, trailing byte, unknown mode tag, halted flag not 0/1, empty.
#guard ariadneWire (hexEncode (wD dev0 ++ wEv .tick) ++ "0") == ""
#guard ariadneWire (hexEncode (wD dev0 ++ wEv .tick ++ [0])) == ""
#guard ariadneWire (hexEncode (w64 0 ++ [7, 0] ++ wL wE [] ++ wL wE [] ++ wL wE [] ++ [1])) == ""
#guard ariadneWire (hexEncode (w64 0 ++ [0, 2] ++ wL wE [] ++ wL wE [] ++ wL wE [] ++ [1])) == ""
#guard ariadneWire "" == ""
-- A canonical call answers.
#guard ariadneWire (hexEncode (wD dev0 ++ wEv .tick)) == hexEncode (wD dev0 ++ wL wO [])

/-- info: 'Chokepoint.AriadneBytes.stepBytes_sound' depends on axioms: [propext, Quot.sound] -/
#guard_msgs in #print axioms stepBytes_sound
/-- info: 'Chokepoint.AriadneBytes.stepBytes_complete' depends on axioms: [propext, Classical.choice, Quot.sound] -/
#guard_msgs in #print axioms stepBytes_complete

end Chokepoint.AriadneBytes
