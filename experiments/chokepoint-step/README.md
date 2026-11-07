# Choke-point step function (2026-11-05)

Two files, checked with bare Lean 4.33.1 (no lake, no Mathlib) by `./check.sh`, about 3 s:

- `CommitStep.lean`: the device as a pure step function over any hash. Theorems:
  `commit_before_forward`, `persisted_in_log`, `forwarded_ids_nodup`,
  `forwards_when_capacity`, `overflow_refuses`, `rollback_halts`, `halted_never_forwards`.
  Vacuity falsifiers: `concrete_run` and `mutant_violates`.
- `CommitBytes.lean`: the concrete BLAKE3 device. It contains:
  - the leaf byte encoding and `enc_inj` (injective on 64-bit-bounded leaves);
  - `leaf_collision`, the BLAKE3 collision reduction;
  - the summary state the export keeps (log length, not content) and `stepS_refines` (the
    summary step equals the projection of `step`, outputs included);
  - `@[export dregg_chokepoint_arrive] : String → String`, whose wire codec is pinned by
    `#guard`s, as in `dregg_deleg_admit`.

BLAKE3 is minidregg's Lean implementation (`/Users/ember/dev/minidregg/Compiler/Blake3.lean`),
compiled from its source by `check.sh` into a private olean directory. It is not copied. One
official vector is re-checked here (BLAKE3("")).

The axioms used are at most `propext` and `Quot.sound`. `lean -c` emits C with the
`dregg_chokepoint_arrive` symbol.

Why Lean rather than the Pancake orb: `research/CHOKEPOINT-VEHICLE-DECISION.md`.

- `GrayCounter.lean`: the counter word. `gray_one_flip` (an increment flips exactly one bit) and
  `grayEnc_injective`, for every counter value.
- `AriadneStep.lean`: the device with Ariadne state continuity (Strackx and Piessens 2016): packages
  `(C + 1, state)` written before each increment, restore only from counter `C`, two
  write-and-increment rounds before resuming. Theorems for every hash and every run: `non_omission`
  (against an adversary holding every package the device wrote, crashing it anywhere, advancing the
  counter), `restore_accepts_only_current`, `honest_never_halts`, `honest_recovers`, `occ_binds` (one id never names two entries; recovery
  re-forwards, so receivers drop repeats by id). Falsifiers by
  `decide`: `tolerateOne_omits` (accept one behind) and `singleInc_omits` (one increment in
  retrieve). Its model is the Veil pilot `/Users/ember/dev/veil-pilot/VeilPilot/ChokepointAriadne.lean`,
  and `/Users/ember/dev/veil-pilot/VeilPilot/AriadneRefines.lean` proves `step` a forward simulation
  of that model (`refines`), with the one-increment variant as a control that does not simulate.
- `CommitBytes.lean` section 6: the Ariadne device on bytes. A codec for its state, events, packages
  and outputs (round trips proved for every value whose numbers fit 64 bits), `stepBytes_complete`
  and `stepBytes_sound` (the byte step is exactly `Ariadne.step Blake3.hash` on canonical encodings
  and refuses anything else), and `@[export dregg_chokepoint_ariadne_step] : String → String`, hex
  in and out, for the HOL4 bridge. `#guard`s run an arrive, commit, forward, crash, restore and
  retrieve sequence through the String wire and check the BLAKE3 digests. `check.sh` emits C and
  checks both export symbols are in it.

Named residuals:
- the hex layer of both wires (guarded by tests, not proved; the Ariadne byte codec under it is proved);
- the host must execute the outputs in order, persisting before forwarding (`F ... leafhex`
  means: persist `leafhex` durably, then forward);
- snapshot content integrity (a MAC assumption);
- delivery.
