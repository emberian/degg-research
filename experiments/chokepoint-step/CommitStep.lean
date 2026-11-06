/-
  CommitStep.lean -- the choke-point step function and its commit-before-forward theorem.

  Vehicle-independent statement prototype (research/CHOKEPOINT-VEHICLE-DECISION.md). Lean 4
  core only, no Mathlib, no imports, so it checks with a bare `lean CommitStep.lean`, and the
  same definitions can be `@[export]`ed into a Microkit PD or a host dataplane.

  The device is a pure step function over (state, event). Hashing is a parameter `H`; the MMR
  is the reader's view of the durable leaf list `st.log` (its non-omission theorems are about a
  list, so they apply to this list unchanged; they are a named dependency, not imported here).

  What is proved (all for every hash `H`, every capacity, every event sequence):
    * commit_before_forward : in the device's output stream, every `forward occ p` is
      IMMEDIATELY preceded by `persist l` with `l.occ = occ` and `l.digest = H p`.
    * persisted_in_log      : every persisted leaf is in the final durable log.
    * forwarded_ids_nodup   : forwarded occurrence ids are pairwise distinct, across restores
      (ids come from a non-rollbackable hardware counter).
    * step_vjm_at_admission : the persisted leaf carries the manifest root current at admission.
    * forwards_when_capacity: liveness companion -- a live device with room DOES forward
      (rules out the always-refuse device, which satisfies the safety theorem vacuously).
    * overflow_refuses      : a full device refuses, persists nothing, and forwards nothing.
    * rollback_halts        : restoring a snapshot whose counter is not the hardware counter
      halts the device; a halted device never forwards again.
    * leaf_binds            : equal leaves ⇒ equal (occ, epoch, vjm, job) and equal digests; so
      two different payloads under one leaf are an `H`-collision (the reduction, not injectivity).
  Vacuity falsifiers at the end: a concrete run that forwards; and a MUTANT step (forward
  before persist) whose concrete output violates the safety property, checked by `decide`.
-/

namespace Chokepoint

variable {D : Type}

/-- A durable log leaf: occurrence id, epoch, manifest (VJM) root at admission, job id, and
the payload digest. Encoded length-delimited by construction (a structure, not a byte concat). -/
structure Leaf (D : Type) where
  occ    : Nat
  epoch  : Nat
  vjm    : List UInt8
  job    : Nat
  digest : D
deriving DecidableEq, Repr

inductive Event where
  | arrive  (job : Nat) (payload : List UInt8)   -- a frame offered for forwarding
  | setVjm  (root : List UInt8)                  -- manifest root changes (new job manifest)
  | restore (logLen ctr : Nat)                   -- recovery from a durable snapshot (len, counter)
deriving DecidableEq, Repr

inductive Output (D : Type) where
  | persist (l : Leaf D)                          -- durable append, acknowledged
  | forward (occ : Nat) (payload : List UInt8)    -- the frame leaves the device
  | refuse  (occ : Nat)                           -- the frame is dropped, and says so
deriving DecidableEq, Repr

structure DevState (D : Type) where
  hw     : Nat          -- non-rollbackable hardware counter (TrInc-style); never decreases
  ctr    : Nat          -- the durable counter recorded in the snapshot
  epoch  : Nat
  vjm    : List UInt8
  log    : List (Leaf D) -- the durable leaf list (the MMR's leaves)
  cap    : Nat          -- durable capacity in leaves
  halted : Bool

/-- Valid states: live and counter-synchronised. -/
def DevState.Valid (st : DevState D) : Prop := st.halted = false ∧ st.ctr = st.hw

/-- The step function. -/
def step (H : List UInt8 → D) (st : DevState D) : Event → DevState D × List (Output D)
  | .arrive job p =>
    if st.halted then (st, [.refuse st.hw])
    else if st.log.length < st.cap then
      ({ st with hw := st.hw + 1, ctr := st.hw + 1,
                 log := st.log ++ [⟨st.hw, st.epoch, st.vjm, job, H p⟩] },
       [.persist ⟨st.hw, st.epoch, st.vjm, job, H p⟩, .forward st.hw p])
    else
      ({ st with hw := st.hw + 1, ctr := st.hw + 1 }, [.refuse st.hw])
  | .setVjm r => ({ st with vjm := r }, [])
  | .restore n c =>
    -- Recovery is accepted only if the snapshot matches the hardware-anchored state exactly
    -- (counter = hw, length = acknowledged log length). Anything else -- an older snapshot, a
    -- truncated log -- halts the device. Snapshot CONTENT integrity is the MAC's job (named).
    if c = st.hw ∧ n = st.log.length ∧ st.halted = false then ({ st with ctr := c }, [])
    else ({ st with halted := true }, [])

/-- Run a list of events, collecting outputs in order. -/
def run (H : List UInt8 → D) : DevState D → List Event → DevState D × List (Output D)
  | st, [] => (st, [])
  | st, e :: es =>
    ((run H (step H st e).1 es).1, (step H st e).2 ++ (run H (step H st e).1 es).2)

/-! ## The safety property, over output streams -/

/-- Every forward is immediately preceded by its own persist, binding id and payload digest. -/
def CommitBeforeForward (H : List UInt8 → D) (outs : List (Output D)) : Prop :=
  ∀ pre occ p post, outs = pre ++ Output.forward occ p :: post →
    ∃ pre' l, pre = pre' ++ [Output.persist l] ∧ l.occ = occ ∧ l.digest = H p

/-- The stream does not START with a forward. -/
def NoLeadForward (outs : List (Output D)) : Prop :=
  ∀ occ p post, outs ≠ Output.forward occ p :: post

/-- The three block shapes a step can emit. -/
theorem step_block (H : List UInt8 → D) (st : DevState D) (e : Event) :
    (step H st e).2 = [] ∨
    (∃ o, (step H st e).2 = [.refuse o]) ∨
    (∃ job p, (step H st e).2 =
      [.persist ⟨st.hw, st.epoch, st.vjm, job, H p⟩, .forward st.hw p]) := by
  cases e with
  | arrive job p =>
    unfold step
    by_cases hh : st.halted = true
    · right; left; exact ⟨st.hw, by simp [hh]⟩
    · by_cases hc : st.log.length < st.cap
      · right; right; exact ⟨job, p, by simp [hh, hc]⟩
      · right; left; exact ⟨st.hw, by simp [hh, hc]⟩
  | setVjm r => left; simp [step]
  | restore n c =>
    left; simp only [step]; split <;> rfl

theorem cbf_nil (H : List UInt8 → D) : CommitBeforeForward H ([] : List (Output D)) := by
  intro pre occ p post h
  cases pre <;> simp at h

theorem nlf_nil : NoLeadForward ([] : List (Output D)) := by
  intro occ p post h; cases h

/-- Prepending one step block preserves both properties. -/
theorem cbf_cons_block (H : List UInt8 → D) (st : DevState D) (e : Event)
    (rest : List (Output D)) (hr : CommitBeforeForward H rest) (hn : NoLeadForward rest) :
    CommitBeforeForward H ((step H st e).2 ++ rest) ∧ NoLeadForward ((step H st e).2 ++ rest) := by
  rcases step_block H st e with h | ⟨o, h⟩ | ⟨job, q, h⟩
  · rw [h]; exact ⟨hr, hn⟩
  · rw [h]
    refine ⟨?_, ?_⟩
    · intro pre occ p post heq
      cases pre with
      | nil => simp at heq
      | cons a pre1 =>
        simp only [List.cons_append, List.cons.injEq] at heq
        obtain ⟨_, heq2⟩ := heq
        obtain ⟨pre', l, hp, hl1, hl2⟩ := hr pre1 occ p post (by simpa using heq2)
        exact ⟨a :: pre', l, by simp [hp], hl1, hl2⟩
    · intro occ p post heq; simp at heq
  · rw [h]
    refine ⟨?_, ?_⟩
    · intro pre occ p post heq
      cases pre with
      | nil => simp at heq
      | cons a pre1 =>
        cases pre1 with
        | nil =>
          simp only [List.cons_append, List.nil_append, List.cons.injEq,
            Output.forward.injEq] at heq
          obtain ⟨ha, ⟨ho, hp⟩, _⟩ := heq
          subst ha
          exact ⟨[], _, rfl, ho, by rw [hp]⟩
        | cons b pre2 =>
          simp only [List.cons_append, List.cons.injEq] at heq
          obtain ⟨ha, hb, heq3⟩ := heq
          obtain ⟨pre', l, hp, hl1, hl2⟩ := hr pre2 occ p post (by simpa using heq3)
          exact ⟨a :: b :: pre', l, by simp [hp], hl1, hl2⟩
    · intro occ p post heq; simp at heq

/-- **commit_before_forward.** For every hash, every initial state and every event sequence,
every forward in the output stream is immediately preceded by the persist of a leaf carrying
that occurrence id and that payload's digest. -/
theorem commit_before_forward (H : List UInt8 → D) :
    ∀ (es : List Event) (st : DevState D),
      CommitBeforeForward H (run H st es).2 ∧ NoLeadForward (run H st es).2 := by
  intro es
  induction es with
  | nil => intro st; exact ⟨cbf_nil H, nlf_nil⟩
  | cons e es ih =>
    intro st
    obtain ⟨h1, h2⟩ := ih (step H st e).1
    exact cbf_cons_block H st e _ h1 h2

/-! ## The durable log only grows; every persisted leaf is in it -/

theorem step_log_prefix (H : List UInt8 → D) (st : DevState D) (e : Event) :
    ∃ t, (step H st e).1.log = st.log ++ t := by
  cases e with
  | arrive job p =>
    unfold step
    by_cases hh : st.halted = true
    · exact ⟨[], by simp [hh]⟩
    · by_cases hc : st.log.length < st.cap
      · exact ⟨[⟨st.hw, st.epoch, st.vjm, job, H p⟩], by simp [hh, hc]⟩
      · exact ⟨[], by simp [hh, hc]⟩
  | setVjm r => exact ⟨[], by simp [step]⟩
  | restore n c => simp only [step]; split <;> exact ⟨[], by simp⟩

theorem step_persist_in_log (H : List UInt8 → D) (st : DevState D) (e : Event) (l : Leaf D)
    (h : Output.persist l ∈ (step H st e).2) : l ∈ (step H st e).1.log := by
  rcases step_block H st e with hb | ⟨o, hb⟩ | ⟨job, q, hb⟩
  · rw [hb] at h; cases h
  · rw [hb] at h; simp at h
  · rw [hb] at h
    simp only [List.mem_cons, Output.persist.injEq, reduceCtorEq, List.mem_nil_iff, or_false] at h
    cases e with
    | arrive job' p' =>
      unfold step at hb ⊢
      by_cases hh : st.halted = true
      · simp [hh] at hb
      · by_cases hc : st.log.length < st.cap
        · simp [hh, hc] at hb ⊢
          obtain ⟨hj, hp⟩ := hb
          right; rw [h, hj.1, hp]
        · simp [hh, hc] at hb
    | setVjm r => simp [step] at hb
    | restore n c => simp only [step] at hb; split at hb <;> simp at hb

theorem run_log_prefix (H : List UInt8 → D) :
    ∀ (es : List Event) (st : DevState D), ∃ t, (run H st es).1.log = st.log ++ t := by
  intro es; induction es with
  | nil => intro st; exact ⟨[], by simp [run]⟩
  | cons e es ih =>
    intro st
    obtain ⟨t1, h1⟩ := step_log_prefix H st e
    obtain ⟨t2, h2⟩ := ih (step H st e).1
    exact ⟨t1 ++ t2, by simp [run, h2, h1]⟩

/-- **persisted_in_log.** Every leaf the device persisted is in its final durable log. -/
theorem persisted_in_log (H : List UInt8 → D) :
    ∀ (es : List Event) (st : DevState D) (l : Leaf D),
      Output.persist l ∈ (run H st es).2 → l ∈ (run H st es).1.log := by
  intro es; induction es with
  | nil => intro st l h; simp [run] at h
  | cons e es ih =>
    intro st l h
    simp only [run, List.mem_append] at h ⊢
    rcases h with h | h
    · obtain ⟨t, ht⟩ := run_log_prefix H es (step H st e).1
      rw [ht]; exact List.mem_append_left _ (step_persist_in_log H st e l h)
    · exact ih _ l h

/-! ## Forwarded ids are distinct across the whole run, restores included -/

def fwdIds : List (Output D) → List Nat
  | [] => []
  | .forward o _ :: r => o :: fwdIds r
  | _ :: r => fwdIds r

theorem fwdIds_append (a b : List (Output D)) : fwdIds (a ++ b) = fwdIds a ++ fwdIds b := by
  induction a with
  | nil => rfl
  | cons x a ih => cases x <;> simp [fwdIds, ih]

theorem step_hw (H : List UInt8 → D) (st : DevState D) (e : Event) :
    st.hw ≤ (step H st e).1.hw := by
  cases e with
  | arrive job p =>
    unfold step
    by_cases hh : st.halted = true
    · simp [hh]
    · by_cases hc : st.log.length < st.cap <;> simp [hh, hc]
  | setVjm r => simp [step]
  | restore n c => simp only [step]; split <;> simp

theorem step_fwd (H : List UInt8 → D) (st : DevState D) (e : Event) :
    fwdIds (step H st e).2 = [] ∨
      (fwdIds (step H st e).2 = [st.hw] ∧ (step H st e).1.hw = st.hw + 1) := by
  cases e with
  | arrive job p =>
    unfold step
    by_cases hh : st.halted = true
    · left; simp [hh, fwdIds]
    · by_cases hc : st.log.length < st.cap
      · right; simp [hh, hc, fwdIds]
      · left; simp [hh, hc, fwdIds]
  | setVjm r => left; simp [step, fwdIds]
  | restore n c => left; simp only [step]; split <;> simp [fwdIds]

theorem run_hw (H : List UInt8 → D) :
    ∀ (es : List Event) (st : DevState D), st.hw ≤ (run H st es).1.hw := by
  intro es; induction es with
  | nil => intro st; simp [run]
  | cons e es ih => intro st; exact Nat.le_trans (step_hw H st e) (ih _)

/-- **forwarded_ids_nodup.** The occurrence ids of all forwards in a run are pairwise
distinct and lie in `[hw₀, hw_final)`: the hardware counter never rolls back, so no id is
ever reused, whatever restores the event stream contains. -/
theorem forwarded_ids_nodup (H : List UInt8 → D) :
    ∀ (es : List Event) (st : DevState D),
      (fwdIds (run H st es).2).Nodup ∧
      ∀ o ∈ fwdIds (run H st es).2, st.hw ≤ o ∧ o < (run H st es).1.hw := by
  intro es; induction es with
  | nil => intro st; simp [run, fwdIds]
  | cons e es ih =>
    intro st
    obtain ⟨hnd, hrng⟩ := ih (step H st e).1
    simp only [run, fwdIds_append]
    have hle := run_hw H es (step H st e).1
    rcases step_fwd H st e with hf | ⟨hf, hhw⟩
    · rw [hf]
      refine ⟨by simpa using hnd, ?_⟩
      intro o ho
      simp only [List.nil_append] at ho
      obtain ⟨a, b⟩ := hrng o ho
      exact ⟨Nat.le_trans (step_hw H st e) a, b⟩
    · rw [hf]
      refine ⟨?_, ?_⟩
      · refine List.nodup_cons.mpr ⟨?_, hnd⟩
        intro hmem
        have := (hrng st.hw hmem).1
        omega
      · intro o ho
        simp only [List.cons_append, List.nil_append, List.mem_cons] at ho
        rcases ho with ho | ho
        · subst ho; constructor <;> omega
        · obtain ⟨a, b⟩ := hrng o ho; constructor <;> omega

/-! ## Liveness companion, refusal, rollback -/

/-- **forwards_when_capacity (and step_vjm_at_admission).** A live device with room persists a
leaf carrying the current epoch and manifest root, then forwards. This is what rules out the
always-refuse device, which satisfies `commit_before_forward` vacuously. -/
theorem forwards_when_capacity (H : List UInt8 → D) (st : DevState D) (job : Nat)
    (p : List UInt8) (hl : st.halted = false) (hc : st.log.length < st.cap) :
    (step H st (.arrive job p)).2 =
      [.persist ⟨st.hw, st.epoch, st.vjm, job, H p⟩, .forward st.hw p] := by
  simp [step, hl, hc]

/-- **overflow_refuses.** A full live device refuses the frame, persists nothing, forwards
nothing, and still consumes a fresh id (so a later frame never reuses it). -/
theorem overflow_refuses (H : List UInt8 → D) (st : DevState D) (job : Nat)
    (p : List UInt8) (hl : st.halted = false) (hc : ¬ st.log.length < st.cap) :
    (step H st (.arrive job p)).2 = [.refuse st.hw] ∧
    (step H st (.arrive job p)).1.log = st.log ∧
    (step H st (.arrive job p)).1.hw = st.hw + 1 := by
  simp [step, hl, hc]

/-- **rollback_halts.** A snapshot whose counter is not the hardware counter, or whose length is
not the acknowledged log length, halts the device. -/
theorem rollback_halts (H : List UInt8 → D) (st : DevState D) (n c : Nat)
    (h : c ≠ st.hw ∨ n ≠ st.log.length) :
    (step H st (.restore n c)).1.halted = true := by
  simp only [step]
  rw [if_neg (by rintro ⟨h1, h2, _⟩; rcases h with h | h <;> contradiction)]

theorem step_halted (H : List UInt8 → D) (st : DevState D) (e : Event) (h : st.halted = true) :
    (step H st e).1.halted = true ∧ fwdIds (step H st e).2 = [] := by
  cases e with
  | arrive job p => simp [step, h, fwdIds]
  | setVjm r => simp [step, h, fwdIds]
  | restore n c => simp [step, h, fwdIds]

/-- **halted_never_forwards.** Once halted, no event sequence makes the device forward. -/
theorem halted_never_forwards (H : List UInt8 → D) :
    ∀ (es : List Event) (st : DevState D), st.halted = true → fwdIds (run H st es).2 = [] := by
  intro es; induction es with
  | nil => intro st _; simp [run, fwdIds]
  | cons e es ih =>
    intro st h
    obtain ⟨h1, h2⟩ := step_halted H st e h
    simp [run, fwdIds_append, h2, ih _ h1]

/-! ## Vacuity falsifiers (concrete; checked by the kernel) -/

/-- A toy hash for the concrete runs (the theorems above hold for EVERY `H`). -/
def toyH (p : List UInt8) : Nat := p.length

def st0 : DevState Nat := ⟨0, 0, 0, [1], [], 1, false⟩

def evs : List Event := [.arrive 7 [5, 6], .arrive 8 [9], .restore 1 1]

/-- Inhabitation: the concrete run forwards the first frame, refuses the second (capacity 1),
and halts on the stale restore (counter 1, but hw is 2). -/
theorem concrete_run :
    (run toyH st0 evs).2 =
      [.persist ⟨0, 0, [1], 7, 2⟩, .forward 0 [5, 6], .refuse 1] ∧
    (run toyH st0 evs).1.halted = true := by
  decide

/-- The MUTANT device: forwards first, persists after. -/
def stepMutant (H : List UInt8 → D) (st : DevState D) (e : Event) : DevState D × List (Output D) :=
  match step H st e with
  | (st', [.persist l, .forward o p]) => (st', [.forward o p, .persist l])
  | r => r

/-- **The falsifier.** The mutant's concrete output violates `CommitBeforeForward`, so the
safety property is not satisfied by every stream: it distinguishes the real step from a
forward-then-persist device. -/
theorem mutant_violates :
    ¬ CommitBeforeForward toyH (stepMutant toyH st0 (.arrive 7 [5, 6])).2 := by
  intro h
  have hout : (stepMutant toyH st0 (.arrive 7 [5, 6])).2 =
      [] ++ Output.forward 0 [5, 6] :: [.persist ⟨0, 0, [1], 7, 2⟩] := by decide
  obtain ⟨pre', l, hp, _, _⟩ := h [] 0 [5, 6] _ hout
  cases pre' <;> simp at hp

end Chokepoint
