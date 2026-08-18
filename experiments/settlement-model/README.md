# Settlement Relation Model

Status: VERIFIED implementation of a deterministic offline model, at exactly
the bounds its tests state. PROPOSED as the settlement relation joined to
`dark-fba/n4-k4-q15/v0` through the shielded-baseline composition. No chain,
token, signature, cryptographic, economic, or liveness property is claimed,
and none is measured here.

`ARCHITECTURE.md` §5 says computation and settlement are separate relations
joined by exact identifiers, and `DARK_FBA_RELATION.md` §10 says a private
computation followed by ordinary public transfers is pre-trade dark at most.
This crate models that separate relation — authorized custody, settlement
nullifiers, idempotent retries, exact conservation — and then *measures* the
second sentence: on a transparent per-claim surface the public settlement
transcript alone reconstructs every settled position's owner, side, exact
fill, and exact reservation.

Model identifier: `degg-settlement/v0`. The specification is
[`docs/research/SETTLEMENT_RELATION.md`](../../docs/research/SETTLEMENT_RELATION.md).

## Layout

- `src/custody.rs` — the custody ledger: `Reserved -> Obligated -> Settled |
  Refunded` per admission nullifier, a pre-funded two-asset pool, checked
  payouts, exact conservation.
- `src/authorize.rs` — settlement authorization: receipt, inclusion receipt,
  delivery opening, and custody consistency, with every delta re-derived from
  side, fill, and public price, in a frozen check order.
- `src/relation.rs` — the settlement book: obligation on observing one
  settled receipt, execution idempotent by settlement nullifier, and the
  terminal-abort refund gate.
- `src/surface.rs` — four transfer surfaces (per-claim public, netted public,
  named Shielded agent, refusing Dark target) and the reconstruction
  measurement.
- `src/harness.rs` — drivers shared by the tests and the corpus renderer.
- `src/transcript.rs` — the byte-stable corpus renderer.
- `vectors/v1.txt` — the corpus.

## Running

```sh
cargo test --offline --locked
cargo clippy --offline --locked --all-targets -- -D warnings
cargo fmt --check
cargo run --quiet --offline --locked --bin degg-settlement-vectors \
  | cmp - vectors/v1.txt
```

56 tests: 9 custody, 22 authorization, 8 settlement, 6 refund, 3 residual
inheritance, 6 surface, 2 corpus. Zero third-party dependencies; the only
dependencies are path dependencies on `experiments/inclusion-availability`,
`experiments/relation-ir`, and `experiments/shielded-baseline`, none of which
this crate modifies.

Corpus SHA-256:
`f090d751d78c217c5a1405a17375e1864da75d2adfc70fc4e0aa89bca62547c0`.
