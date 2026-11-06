# Choke-point step function (2026-11-05)

`CommitStep.lean`: the in-path receipt device as a pure Lean step function, with
`commit_before_forward`, `persisted_in_log`, `forwarded_ids_nodup`, the liveness companion
`forwards_when_capacity`, `overflow_refuses`, `rollback_halts`, `halted_never_forwards`, and two
vacuity falsifiers (`concrete_run`, `mutant_violates`).

Check: `~/.elan/toolchains/leanprover--lean4---v4.30.0/bin/lean CommitStep.lean`. It is Lean
core only, takes about 1.2 s, and emits no errors or warnings.

Why Lean rather than the Pancake orb: `research/CHOKEPOINT-VEHICLE-DECISION.md`.
