/-
  GrayCounter.lean -- the non-volatile counter word of AriadneStep.lean.

  Ariadne (§4.2) stores its monotonic counter as a Gray code so that each increment is a single bit
  flip, which a non-volatile cell performs atomically: a power loss leaves either the old word or the
  new one, never a torn mixture (a binary word changes several bits per increment, 011 → 100, and a
  crash between cell writes can leave 010 or 000: `/Users/ember/dev/veil-pilot/VeilPilot/
  CounterWord.lean` finds that run). Reflected Gray code here; Ariadne's balanced Gray code evens out
  cell wear and keeps the one-flip property, which is all the device's proof uses.

  Proved for every counter value: gray_one_flip, grayEnc_injective. Lean core only.
-/

namespace Chokepoint.Gray

def grayEnc (v : Nat) : Nat := v ^^^ (v >>> 1)

theorem testBit_grayEnc (v i : Nat) : (grayEnc v).testBit i = (v.testBit i ^^ v.testBit (i + 1)) := by
  simp [grayEnc, Nat.testBit_xor, Nat.testBit_shiftRight, Nat.add_comm]

theorem testBit_two_mul_succ (k j : Nat) : (2 * k).testBit (j + 1) = k.testBit j := by
  rw [Nat.testBit_succ]; congr 1; omega
theorem testBit_two_mul_add_one_succ (k j : Nat) : (2 * k + 1).testBit (j + 1) = k.testBit j := by
  rw [Nat.testBit_succ]; congr 1; omega

theorem grayEnc_even_succ (m : Nat) : grayEnc (2 * m + 1) = grayEnc (2 * m) ^^^ 2 ^ 0 := by
  refine Nat.eq_of_testBit_eq fun i => ?_
  rw [Nat.testBit_xor, testBit_grayEnc, testBit_grayEnc, Nat.testBit_two_pow]
  cases i with
  | zero => simp [testBit_two_mul_add_one_succ, testBit_two_mul_succ]
  | succ j => simp [testBit_two_mul_add_one_succ, testBit_two_mul_succ]

theorem grayEnc_odd_succ (m : Nat) : ∃ k, grayEnc (2 * m + 2) = grayEnc (2 * m + 1) ^^^ 2 ^ k := by
  induction m using Nat.strongRecOn with
  | _ m ih =>
  have step : ∃ k, grayEnc (m + 1) = grayEnc m ^^^ 2 ^ k := by
    rcases Nat.mod_two_eq_zero_or_one m with h | h
    · have hm : m = 2 * (m / 2) := by omega
      exact ⟨0, by rw [hm]; exact grayEnc_even_succ (m / 2)⟩
    · have hm : m = 2 * (m / 2) + 1 := by omega
      obtain ⟨k, hk⟩ := ih (m / 2) (by omega)
      exact ⟨k, by rw [hm]; exact hk⟩
  obtain ⟨k, hk⟩ := step
  refine ⟨k + 1, Nat.eq_of_testBit_eq fun i => ?_⟩
  have e : 2 * m + 2 = 2 * (m + 1) := by omega
  rw [e, Nat.testBit_xor, testBit_grayEnc, testBit_grayEnc, Nat.testBit_two_pow]
  cases i with
  | zero =>
    have b0 := congrArg (fun x => Nat.testBit x 0) hk
    simp only [testBit_grayEnc, Nat.testBit_xor, Nat.testBit_two_pow] at b0
    simp [testBit_two_mul_add_one_succ, testBit_two_mul_succ]
    rcases Nat.mod_two_eq_zero_or_one m with h | h <;> simp [h] <;> omega
  | succ j =>
    have bj := congrArg (fun x => Nat.testBit x j) hk
    simp only [testBit_grayEnc, Nat.testBit_xor, Nat.testBit_two_pow] at bj
    simp [testBit_two_mul_add_one_succ, testBit_two_mul_succ]
    rw [bj]; cases m.testBit j <;> cases m.testBit (j + 1) <;> cases decide (k = j) <;> rfl

/-- **gray_one_flip.** Advancing the counter by one changes exactly one bit of its stored word, so
an increment is a single cell write: a power loss leaves the old value or the new one. -/
theorem gray_one_flip (n : Nat) : ∃ k, grayEnc (n + 1) = grayEnc n ^^^ 2 ^ k := by
  rcases Nat.mod_two_eq_zero_or_one n with h | h
  · have hn : n = 2 * (n / 2) := by omega
    exact ⟨0, by rw [hn]; exact grayEnc_even_succ (n / 2)⟩
  · have hn : n + 1 = 2 * (n / 2) + 2 := by omega
    have hn' : n = 2 * (n / 2) + 1 := by omega
    obtain ⟨k, hk⟩ := grayEnc_odd_succ (n / 2)
    exact ⟨k, by rw [hn, hk, ← hn']⟩

/-- The encoding is injective, so a reader decodes the word it finds to one counter value. -/
theorem grayEnc_injective {a b : Nat} (h : grayEnc a = grayEnc b) : a = b := by
  have top : ∀ i, a + b ≤ i → a.testBit i = b.testBit i := by
    intro i hi
    have ha : a < 2 ^ i := Nat.lt_of_lt_of_le (Nat.lt_two_pow_self) (Nat.pow_le_pow_right (by decide) (by omega))
    have hb : b < 2 ^ i := Nat.lt_of_lt_of_le (Nat.lt_two_pow_self) (Nat.pow_le_pow_right (by decide) (by omega))
    rw [Nat.testBit_lt_two_pow ha, Nat.testBit_lt_two_pow hb]
  have down : ∀ d i, i + d = a + b → a.testBit i = b.testBit i := by
    intro d
    induction d with
    | zero => intro i hi; exact top i (by omega)
    | succ d ih =>
      intro i hi
      have hi1 := ih (i + 1) (by omega)
      have gi := congrArg (fun x => Nat.testBit x i) h
      simp only [testBit_grayEnc] at gi
      rw [hi1] at gi
      cases ha : a.testBit i <;> cases hb : b.testBit i <;> cases b.testBit (i + 1) <;> simp_all
  refine Nat.eq_of_testBit_eq fun i => ?_
  by_cases hi : i ≤ a + b
  · exact down (a + b - i) i (by omega)
  · exact top i (by omega)

end Chokepoint.Gray
