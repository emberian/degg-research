# Confidential Energy Dispatch Relation

Status: VERIFIED deterministic offline **Clear** model at its exact bounded
domain; PROPOSED specialized relation and Dark leakage target. No encryption,
FHE, vFHE, MPC, proof, network, custody, local-output channel, or deployment is
implemented.

Relation identifier:
`confidential-energy-dispatch/p3-t3-b2-q4/v0`.

This crate freezes a tiny three-provider, three-period, two-bus integer
dispatch and pay-as-cost settlement. Provider cost segments, capacity, minimum
output, ramp bounds, forced availability, bus, and output-recipient binding are
private relation inputs. Zonal demand, system reserve, line limits, instance,
and coarse epoch are public. Exactly three provider slots are committed,
including canonical padding.

Minimum output plus an implicit off/on decision makes the relation nonconvex.
The Clear oracle therefore performs bounded exhaustive trajectory search. The
verifier checks bindings and feasibility, then performs the same search again.
Its `OptimalityCertificate` is a byte-stable search transcript, not a succinct
proof. Tests include a feasible, conserving, correctly recommitted but more
expensive schedule; direct recomputation rejects it.

The full semantic, leakage, operation, backend, and claim boundary is in
[`CONFIDENTIAL_ENERGY_DISPATCH_RELATION.md`](../../docs/research/CONFIDENTIAL_ENERGY_DISPATCH_RELATION.md).

## Reproduce

```sh
cargo test --offline --locked
cargo clippy --offline --locked --all-targets -- -D warnings
cargo fmt --check
RUSTDOCFLAGS="-D warnings" cargo doc --offline --locked --no-deps
cargo run --quiet --offline --locked --bin degg-energy-dispatch-vectors \
  | cmp - vectors/v1.txt
```

The crate has no third-party dependencies. Twenty-five tests cover the hash,
relation, adversarial verifier, and corpus surfaces. Its checked-in corpus pins one
settled result, exact commitments, private oracle output, search counts, and the
176-byte success/refusal public-frame size.

Corpus SHA-256:
`426b28168e4fdaef25d1fc6da33f4b18eb09122f7552dbaeeb848f95a8bb3925`.
