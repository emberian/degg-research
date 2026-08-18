# Relation IR: Frozen Types and a Clear Lowering

Status: PROPOSED compiler-boundary experiment. VERIFIED only as the offline
differential recorded below, at exactly the stated bounds. No privacy property
of any kind is claimed, measured, or approximated here: the only lowering is
Clear, and under it one process sees every input and every output.

This workspace makes the typed Dark Relation IR of
[`docs/DARK_RELATION_IR.md`](../../docs/DARK_RELATION_IR.md) real for exactly
one relation, `dark-fba/n4-k4-q15/v0`, frozen by
[`docs/research/DARK_FBA_RELATION.md`](../../docs/research/DARK_FBA_RELATION.md):

- `ir/` — `degg-relation-ir`, dependency-free. The relation as *data*: a
  `RelationModule` value freezing input/output ports with visibility
  annotations (`Public` / `PrivateToOwner` / `Executor`), the numeric
  parameters, the admission predicates and their check priority, the clearing
  and allocation rules, the refusal-class vocabulary of specification section
  4.1, and the receipt shapes. Every IR object has canonical `degg-cbe/v1`
  bytes and a SHA-256 digest; the module digest is the relation identity.
  `lower()` compiles the module to a Clear evaluator that *interprets* the
  module's data; lowering to a Shielded committee or a Dark target refuses.
- `differ/` — `degg-relation-ir-differ`. Runs the IR Clear lowering, the
  reference toy in [`../dark-fba`](../dark-fba), and the independent oracle in
  [`../dark-fba-independent`](../dark-fba-independent) on every batch of the
  enumerated domains below and compares complete outputs.
- [`goldens/v1.txt`](goldens/v1.txt) — golden SHA-256 digests over canonical
  bytes for the frozen module, its admission policy, and named fixture
  batches, outcomes, owner outputs, and receipts. Compared byte-for-byte by a
  unit test.

## Running

```sh
cargo test --offline --locked --manifest-path experiments/relation-ir/Cargo.toml
cargo clippy --offline --locked --all-targets \
  --manifest-path experiments/relation-ir/Cargo.toml -- -D warnings
cargo fmt --check --manifest-path experiments/relation-ir/Cargo.toml
cargo run --offline --locked --release \
  --manifest-path experiments/relation-ir/Cargo.toml \
  --bin relation-ir-differ -- all
cargo run --offline --locked \
  --manifest-path experiments/relation-ir/Cargo.toml \
  --bin relation-ir-goldens
```

`all` may be replaced by `a`, `b`, `c`, or `fixtures`. The full differential
takes a few seconds single-threaded in release mode.

## The check-priority freeze, as data

The 2026-08-18 two-oracle differential proved the prose specification
underdetermined which public refusal class a multi-fault witness receives;
specification section 4.1 now freezes the reference oracle's observed order.
Here that order is a frozen *field*: `frozen_v0_check_order()` inside the
module value. The evaluator interprets whatever order the module carries, so a
module carrying the rejected pre-freeze alternative order is a different value
with a different digest and publicly different refusal classes on the
differential's minimal witnesses. Both facts are pinned by unit tests and by
`goldens/v1.txt`.

## Differential domains and result

For every enumerated batch the harness compares, against both oracles:
accept versus refuse, the refusal class (with slot and first-use diagnostics
against the toy, and the section 4.1 class tag against the independent
oracle), clearing tick or no-trade tag, public volume, the per-slot fill
vector, and every owner-local output.

| Domain | Content | Cases |
|---|---|---:|
| A | Every book over quantity ceiling 3: four padded slots, each vacant or a (side, tick, quantity) triple, `25^4`; owner `i` at slot `i`, exact reservations | 390,625 |
| B | Every book over quantity ceiling 1 (`9^4`) crossed with all `4^4` owner assignments, plus the same books crossed with four reservation-surplus patterns | 1,705,860 |
| C | Six base books crossed with every subset of size at most two drawn from 82 perturbation applications (6 batch-level, 19 per-slot at 4 slots) | 20,424 |
| fixtures | Named fixtures, including the two minimal check-priority witnesses | 7 |
| | Total | 2,116,916 |

VERIFIED on 2026-08-18, at exactly these bounds: zero divergences of any kind
against either oracle. These domains are deliberately smaller than the earlier
300M-case oracle-versus-oracle differential; they test the *lowering's
fidelity* to two already-cross-validated implementations, not the relation
semantics from scratch. Quantity extremes in domain C stay within `u32` so the
independent oracle's width represents every enumerated witness exactly.

## Visibility honesty

Visibility annotations are types, not mechanisms. `LoweringTarget::DarkTarget`
and `LoweringTarget::ShieldedCommittee` refuse with typed errors, exactly as
the oracles refuse `DarkTarget` execution; a batch requesting `DarkTarget` is
refused by the Clear evaluator before any witness is inspected. The Clear
evaluator's widening is declared in `CLEAR_VISIBILITY_DISCLOSURE`. Receipts
are plain data over canonical bytes: nothing signs them, nothing verifies
them, and a digest over low-entropy private data is trivially brute-forceable,
so they hide nothing and prove nothing.

## Provenance

Original work in this repository, informed by both existing oracles (both were
read; no independence or clean-room status is claimed for this crate), by
`relations/CLEARING_V0.md`, and by the two specification documents above. The
SHA-256 implementation is written freshly from FIPS 180-4 and self-checked
against the published test vectors. No source, fixture, or vendor code was
transplanted from any sibling repository.

- Validation toolchain: `rustc 1.98.0-nightly (91fe22da8 2026-06-21)`,
  `cargo 1.98.0-nightly (a595d0da2 2026-06-20)`.
- Validation date: 2026-08-18.
