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

Named residuals:
- the wire codec (guarded by tests, not proved);
- the host must execute the outputs in order, persisting before forwarding (`F ... leafhex`
  means: persist `leafhex` durably, then forward);
- snapshot content integrity (a MAC assumption);
- delivery.
