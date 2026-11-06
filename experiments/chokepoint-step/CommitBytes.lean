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
-/
import Compiler.Blake3
import CommitStep

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
