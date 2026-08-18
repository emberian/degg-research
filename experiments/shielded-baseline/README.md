# Shielded Single-Executor Baseline

Status: VERIFIED implementation of a deterministic offline model, at exactly
the bounds its tests state. PROPOSED as rung 2 of the honest prototype ladder
in [`docs/VERDICTS.md`](../../docs/VERDICTS.md) V3.

**The named executor sees everything.** It holds the sealing key material,
opens every admitted order payload, learns every owner's side, limit, quantity,
reservation, and nullifier, computes every owner-local output, and could
publish all of it at any moment. That is what
[`docs/PRIVACY_MODES.md`](../../docs/PRIVACY_MODES.md) means by Shielded. No
privacy, confidentiality, noninterference, or leakage property is claimed,
measured, or approximated anywhere in this crate.

What the packet studies is the other half: given that confidentiality is
*assumed* against the executor, exactly how much of correctness and inclusion
stops being a matter of trust, and exactly how much does not.

Model identifier: `degg-shielded-baseline/v0`.

## What is composed

This crate reimplements nothing the two landed packets already implement. It is
a path dependency on both:

- [`../inclusion-availability`](../inclusion-availability) — the admission log,
  the Merkle mountain range, padded cutoff sealing, inclusion receipts, root
  equivocation verdicts, the typed abort machine, and the reserve ledger. Its
  `Mmr`, `AdmissionLog`, `BatchMachine`, `ReserveLedger`, and hash are used
  directly.
- [`../relation-ir`](../relation-ir) — the relation as data
  (`dark_fba_n4_k4_q15_v0`), the canonical `degg-cbe/v1` encoding, the frozen
  admission-check order of `DARK_FBA_RELATION.md` section 4.1, and the Clear
  evaluator that interprets the module. Its Shielded lowering still refuses,
  correctly and untouched: this crate adds no lowering target, and a Shielded
  run is the module's Clear evaluator executed by one named process.

## Layout

- `src/roles.rs` — the four roles, and the seven questions `PRIVACY_MODES.md`
  section 2 requires a Shielded claim to answer, answered with absences where
  the model has them.
- `src/seal.rs` — the modelled seal, the fixed 64-byte order wire shape, and
  the `compile_fail` doctests that make a public-role read of an order field a
  compile error.
- `src/submit.rs` — plaintext to sealed payload, admission request, escrow.
- `src/executor.rs` — assembly from the committed log positions, the derived
  boundary and per-slot statements, evaluation, publication, and the three
  named adversary handles.
- `src/receipt.rs` — the computation receipt and the per-position delivery
  commitment, with the public verifier's complete check list.
- `src/owner.rs` — the owner check battery and what it cannot do.
- `src/dispute.rs` — omission and outcome-equivocation verdicts.
- `src/scenario.rs` — the end-to-end session.
- `src/differential.rs` — the Shielded run against the Clear lowering.
- `src/transcript.rs`, `vectors/v1.txt` — the byte-stable corpus.

## Running

```sh
cargo test --offline --locked \
  --manifest-path experiments/shielded-baseline/Cargo.toml
cargo clippy --offline --locked --all-targets \
  --manifest-path experiments/shielded-baseline/Cargo.toml -- -D warnings
cargo fmt --check \
  --manifest-path experiments/shielded-baseline/Cargo.toml
cargo run --quiet --offline --locked \
  --manifest-path experiments/shielded-baseline/Cargo.toml \
  --bin degg-shielded-vectors \
  | cmp - experiments/shielded-baseline/vectors/v1.txt
cargo run --quiet --offline --locked --release \
  --manifest-path experiments/shielded-baseline/Cargo.toml \
  --bin degg-shielded-differ
```

The default suite takes about twenty seconds, almost all of it in the
differential's domain A. Domain B is the `degg-shielded-differ` binary and
takes about half a minute in release.

## Test inventory

| Suite | Tests | Covers |
|---|---:|---|
| `tests/honest_run.rs` | 5 | end to end settlement, padded positions, no-trade, a reservation the log cannot see, the envelope-versus-payload arrival split |
| `tests/detection.rs` | 13 | omission with a transferable verdict, the bindings that refuse a substitution, the two substitutions that evade every check, and the pro-rata check's soundness over the whole bounded domain |
| `tests/owner_findings.rs` | 6 | a witness for every evidence-level finding |
| `tests/equivocation.rs` | 6 | four outcome-conflict classes, every rejection class, honest agreement refused |
| `tests/abort.rs` | 6 | crash/timeout/retry/exhaustion, withheld payload, withheld root, unbound result, refund conservation, composition gap C-1 |
| `tests/visibility.rs` | 7 | the public wire and commitment material carry no order field, key separation, canonical plaintext encoding, declared public ports |
| `tests/residual_trust.rs` | 1 | 1,125 alternative published runs enumerated; 377 admissible; all 33 well-formed public results admissible |
| `tests/differential.rs` | 1 | domain A, 6,561 books, zero divergences |
| `tests/vectors.rs` | 2 | the corpus reproduces byte for byte |
| doctests | 4 | three `compile_fail` role-boundary proofs and one executor-opens-it example |
| Total | 51 | |

## Differential result

VERIFIED on 2026-08-18, at exactly these bounds: the Shielded run and the Clear
lowering agree on the assembled slot vector, the public outcome, the per-slot
fills, every owner-local output, and the refusal class, over

| Domain | Content | Cases | Divergences |
|---|---|---:|---:|
| A | four slots, each vacant or a (side, tick, quantity `= 1`) triple; `9^4` | 6,561 | 0 |
| B | the same with quantity in `1..=2`; `17^4` | 83,521 | 0 |
| | Total | 90,082 | 0 |

Both sides share the evaluator by construction, so this is evidence about the
composed assembly path — sealing, admission, padded sealing, position
assignment, payload opening, commitment matching, padding recognition, and the
derived statements — and not about the relation's semantics. Those were
compared elsewhere, over 2,116,916 and 300,436,169 cases respectively.

## Byte identity

`vectors/v1.txt` SHA-256:
`627e30fcff2a8696d29a649be923bfa84571892433a27f8c5fab34deb8f2b0e9`.

Validation toolchain: `rustc 1.98.0-nightly (91fe22da8 2026-06-21)`,
`cargo 1.98.0-nightly (a595d0da2 2026-06-20)`. Validation date: 2026-08-18.

## Provenance

Original work in this repository, with no third-party dependencies. SHA-256,
the tagged hash, and the Merkle mountain range are consumed from
`degg-inclusion-availability` by path dependency rather than reimplemented; the
relation module, its canonical encoding, and its Clear evaluator are consumed
from `degg-relation-ir` the same way. The role-capability discipline — a value
constructible only through one path, with a `compile_fail` doctest showing that
the wrong value cannot be substituted — is an idea observed in Dragon's
Clutch's `clutch-accumulator::WindowResult` and freshly implemented here for
different types; no code, fixture, constant, or serialization format crossed
between the repositories. Because the author has read related implementations
in sibling repositories, this README does not claim clean-room status.

The full model, its composition map, what remains executor-trusted, and its
falsifier ledger are in
[`docs/research/SHIELDED_BASELINE.md`](../../docs/research/SHIELDED_BASELINE.md).
