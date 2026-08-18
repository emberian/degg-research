# Dark FBA Independent Oracle and Differential Harness

Status: PROPOSED relation experiment. VERIFIED only as the differential run
recorded below, at exactly the bounds stated. No privacy property of any kind is
claimed, measured, or approximated here. Both oracles are Clear-mode semantics:
one process sees every order and computes every owner output.

This workspace holds a second, independent implementation of the relation
`dark-fba/n4-k4-q15/v0` and a harness that compares it against the existing
offline toy in [`../dark-fba`](../dark-fba) over exhaustively enumerated domains.

- `oracle/` — `degg-batch-oracle`, written from `relations/CLEARING_V0.md` and
  `docs/research/DARK_FBA_RELATION.md` alone. Dependency-free, exact integers,
  deterministic, no floats, `unsafe_code = "forbid"`.
- `differ/` — `degg-batch-differ`, the differential harness. It depends on both
  oracles by path and never modifies either.
- `vectors/v1-independent-reproduction.txt` — the published golden corpus as
  regenerated from this oracle.

The independence protocol, including the digests of the implementation at the
moment its own tests passed and before any of the other implementation was read,
is in [INDEPENDENCE.md](INDEPENDENCE.md).

## Running

```sh
cargo test --offline --locked --manifest-path experiments/dark-fba-independent/Cargo.toml
cargo clippy --offline --locked --all-targets \
  --manifest-path experiments/dark-fba-independent/Cargo.toml -- -D warnings
cargo fmt --check --manifest-path experiments/dark-fba-independent/Cargo.toml \
  -p degg-batch-oracle -p degg-batch-differ
cargo run --release --offline --locked \
  --manifest-path experiments/dark-fba-independent/Cargo.toml \
  --bin degg-batch-differ -- all
```

The full differential takes about seven seconds of wall time on twelve cores.
`all` may be replaced by `a`, `b`, `c`, or `vectors` to run one part.

## What the harness compares

For every enumerated batch it runs both oracles and compares their complete
outputs, not a summary: accept versus refuse, refusal class, clearing tick or
no-trade tag, public volume, the per-slot allocation vector, and every
owner-local output (bought, sold, signed base and quote deltas, released base
and quote reservations). The existing toy exposes no public accessor for its
fill vector, so the harness reassembles it from the owner-local `order_fills`,
which is the same information an owner would receive.

Every divergence is a finding. Neither implementation is edited to make a
divergence disappear; the two known refusal-class disagreements are pinned by
tests that assert the disagreement still exists.

## Domains

| Domain | Content | Cases |
|---|---|---:|
| A | Every book over the complete frozen order domain: four slots, each vacant or a (side, tick, quantity) triple with quantity `1..=15`; `121^4` | 214,358,881 |
| B | Every book over quantity `1..=2` (`17^4`) crossed with all `4^4` owner assignments and four reservation-surplus patterns | 85,525,504 |
| C | Six base books crossed with every subset of size at most three drawn from 78 admission perturbations | 474,948 |

Domain A fixes owner `i` to slot `i` and reserves exactly the required amount,
so it isolates clearing and allocation. Domain B varies exactly what A holds
fixed. Domain C is the refusal surface, and its multi-perturbation subsets are
what expose disagreement about which rule wins when a witness violates several
at once.

One admission statement is out of scope for the comparison: the existing toy's
`ToyAdmissionWitness::reservation_bound`, the custody-binding obligation that
`DARK_FBA_RELATION.md` §4 places on the external admission relation. The
independent oracle does not model it, the harness always supplies it as present,
and this asymmetry is reported rather than patched.

## Result

Domains A and B: complete agreement on all 299,884,385 batches.

Domain C: 474,948 batches, identical accept/refuse verdict on every one. Of the
386,125 batches both oracles refused, 374,538 carried corresponding refusal
classes and 11,587 did not, falling into 16 class pairs. All 11,587 come from two check-priority choices
that the specification does not fix, and in every one of them a third
independent rule enumerator confirms that both reported rules really are
violated by the witness. Details and minimal witnesses are in the dated
addendum to `docs/research/DARK_FBA_RELATION.md`.

Vector reproduction: the published `../dark-fba/vectors/v1.txt` is regenerated
byte-for-byte from this oracle, SHA-256
`9a00d7393d00b5cca1e1b980a468a48cb7c21053fac8ae9e15abe2ba7fc9a767`, once the
refusal spellings are passed through the declared vocabulary map. Without the
map the five refusal lines differ in spelling only; all settled lines, and so
every number in the corpus, match unaided.

## Provenance

Original work in this repository, dependency-free. The oracle was written from
the two specification documents; the other implementation's source was read only
after the oracle's own test suite passed, in order to build the adapter. Because
that source was subsequently read, the harness and this README do not claim
clean-room status for anything written after that point; the oracle's own
digests at the boundary are recorded in `INDEPENDENCE.md`. No source, fixture,
or serialization format was transplanted from any sibling repository; the vector
layout was transcribed from the published corpus file itself.

- Validation toolchain: `rustc 1.98.0-nightly (91fe22da8 2026-06-21)`,
  `cargo 1.98.0-nightly (a595d0da2 2026-06-20)`.
- Validation date: 2026-08-18.
