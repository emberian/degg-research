# Inclusion and Availability Model

Status: VERIFIED implementation of a deterministic offline model, at exactly the
bounds its tests state. PROPOSED as the mechanism behind the accepted-input
root of `dark-fba/n4-k4-q15/v0`. No privacy, cryptographic security, network,
availability, or economic property is claimed, and none is measured here.

`docs/research/DARK_FBA_RELATION.md` §5 requires four things of a valid batch:
a final admission log, witness slots that open exactly to the accepted-input
root `R`, no conflicting finalized root, and every admitted payload recoverable
before computation. Its offline toy represents all four as booleans supplied by
its own executor, and preserves `R` verbatim without ever computing it. This
crate replaces the booleans with objects a third party can check, and records
precisely where an object is still not available.

Model identifier: `degg-inclusion-availability/v0`.

## Layout

- `src/hash.rs` — SHA-256 written from FIPS 180-4 plus a tagged hash, so the
  crate has zero dependencies. Checked against published vectors.
- `src/mmr.rs` — an append-only Merkle mountain range: leaf and node hashing,
  peak bagging, a root that binds the domain and the exact leaf count, node
  inclusion proofs whose verifier *derives* the position, and prefix-consistency
  proofs.
- `src/log.rs` — the admission log: frozen domain, frozen admission check order,
  per-admission acknowledgements carrying a running root, padded and unpadded
  cutoff sealing, and standalone receipt verification.
- `src/equivocation.rs` — root statements, four conflict classes, and a
  content-addressed verdict object.
- `src/lifecycle.rs` — the typed abort machine and the reserve ledger:
  withholding, timeout, retry, result binding, public relation refusal, and
  exact refund conservation.
- `src/transcript.rs` — the byte-stable corpus renderer.
- `vectors/v1.txt` — the corpus.

## Running

```sh
cargo test --offline --locked \
  --manifest-path experiments/inclusion-availability/Cargo.toml
cargo clippy --offline --locked --all-targets \
  --manifest-path experiments/inclusion-availability/Cargo.toml -- -D warnings
cargo fmt --check \
  --manifest-path experiments/inclusion-availability/Cargo.toml
cargo run --quiet --offline --locked \
  --manifest-path experiments/inclusion-availability/Cargo.toml \
  --bin degg-inclusion-vectors \
  | cmp - experiments/inclusion-availability/vectors/v1.txt
```

## Test inventory

| Suite | Tests | Covers |
|---|---:|---|
| `tests/hash.rs` | 5 | SHA-256 against published vectors, streaming at every split, tag separation |
| `tests/mmr.rs` | 29 | peak shape for every count to 256, exhaustive leaf proofs to 33 leaves, exhaustive consistency to 17 leaves, append-only node stability, a 14-case tamper battery |
| `tests/admission.rs` | 23 | frozen check order under simultaneous faults, every refusal class reachable, cutoff root binding, padded sealing, reserved padding nullifiers |
| `tests/inclusion.rs` | 20 | receipts verified from a root alone, wrong root, wrong index, sibling and side tamper, record tamper, post-cutoff append, padded sealing, uniform receipt shape |
| `tests/equivocation.rs` | 19 | four conflict classes accepted, every rejection class, honest single-root operation exhaustively refused |
| `tests/lifecycle.rs` | 33 | the abort matrix, every abort reachable, terminal phases immovable, refund conservation on every path including a public relation refusal, every lifecycle and refund error class |
| `tests/vectors.rs` | 2 | the corpus reproduces byte for byte |
| Total | 131 | |

## Byte identity

`vectors/v1.txt` SHA-256:
`e99afccc5fcb66a9458d2fda6eb29e38328f7a71ae4edd9991170bf177e8cf9a`.

Validation toolchain: `rustc 1.98.0-nightly (91fe22da8 2026-06-21)`,
`cargo 1.98.0-nightly (a595d0da2 2026-06-20)`. Validation date: 2026-08-18.

## Provenance

Original work in this repository, dependency-free. The Merkle mountain range,
the tagged-hash construction, and SHA-256 are public mathematics, freshly
implemented here from their public descriptions; no source, fixture, constant
table beyond the published SHA-256 constants, or serialization format was
transplanted from any sibling repository. The design questions it answers come
from `docs/research/DARK_FBA_RELATION.md`,
`docs/research/DARK_RELATION_THREAT_MODEL.md`, and `docs/ARCHITECTURE.md` §6.
Because the author has read related implementations in sibling repositories,
this README does not claim clean-room status.

The full model, its named non-goals, its composition with the batch relation,
and its falsifier ledger are in
[`docs/research/INCLUSION_AVAILABILITY.md`](../../docs/research/INCLUSION_AVAILABILITY.md).
