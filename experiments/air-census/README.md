# AIR authorship census (2026-11-05)

Read-only syntactic census of where AIR constraints are authored in breadstuffs, against the
Lean-authored AIR law (minidregg design laws; PROGRAM C9).

Run: `./air_census.sh /Users/ember/dev/breadstuffs` (output committed as `RESULTS-2026-11-05.txt`).

These are syntax counts: places to look, not proven violations. A spot check of
`circuit-prove/src/solvency_leaf_adapter.rs:173` confirms one real violation, a descriptor whose
constraints are built by hand in Rust. Reading: `research/governance-foundations-critique.md` W8.
