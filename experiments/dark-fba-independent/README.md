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
  deterministic, no floats, `unsafe_code = "forbid"`. Its admission module was
  later conformed to the frozen check order of §4.1; see `INDEPENDENCE.md`.
- `differ/` — `degg-batch-differ`, the differential harness. It depends on both
  oracles by path and never modifies either.
- `vectors/v1-independent-reproduction.txt` — the published golden corpus as
  regenerated from this oracle.

The independence protocol, including the digests of the implementation at the
moment its own tests passed and before any of the other implementation was read,
and the digests after the later conformance edit, is in
[INDEPENDENCE.md](INDEPENDENCE.md).

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

The full differential takes about eight seconds of wall time on twelve cores.
`all` may be replaced by `a`, `b`, `c`, or `vectors` to run one part.

## What the harness compares

For every enumerated batch it runs both oracles and compares their complete
outputs, not a summary: accept versus refuse, refusal class, clearing tick or
no-trade tag, public volume, the per-slot allocation vector, and every
owner-local output (bought, sold, signed base and quote deltas, released base
and quote reservations). The existing toy exposes no public accessor for its
fill vector, so the harness reassembles it from the owner-local `order_fills`,
which is the same information an owner would receive.

Every divergence is a finding. The refusal-class disagreements this harness
found — sixteen class pairs from two check-priority choices — were recorded
first and closed afterwards in the specification rather than quietly edited
away: `DARK_FBA_RELATION.md` section 4.1 now freezes a numbered admission-check
order for v0, adopting the anchor implementation's order for the stability
reason stated there, and this oracle was then conformed to the frozen text. The
tests that once asserted the disagreements persisted now assert their absence
and cite that section. The anchor implementation under `../dark-fba` was not
modified at any point, and its golden corpus is byte-identical.

## Domains

| Domain | Content | Cases |
|---|---|---:|
| A | Every book over the complete frozen order domain: four slots, each vacant or a (side, tick, quantity) triple with quantity `1..=15`; `121^4` | 214,358,881 |
| B | Every book over quantity `1..=2` (`17^4`) crossed with all `4^4` owner assignments and four reservation-surplus patterns | 85,525,504 |
| C | Six base books crossed with every subset of size at most three drawn from 82 admission perturbations | 551,784 |

Domain A fixes owner `i` to slot `i` and reserves exactly the required amount,
so it isolates clearing and allocation. Domain B varies exactly what A holds
fixed. Domain C is the refusal surface, and its multi-perturbation subsets are
what decide which rule wins when a witness violates several at once, so it is
the check that both oracles realize the frozen order of §4.1.

No admission statement is out of scope. The custody-binding obligation that
`DARK_FBA_RELATION.md` §4 places on the external admission relation — the
existing toy's `ToyAdmissionWitness::reservation_bound` — was outside the
comparison until 2026-08-18, because this oracle did not model it and the
harness supplied it as present. It is now this oracle's rule 17
(`Order::custody_bound`), the adapter maps it across, and domain C perturbs it,
which is what took the perturbation catalogue from 78 actions to 82 and domain
C from 474,948 batches to 551,784.

## Result

VERIFIED on 2026-08-18, after conformance to the frozen check order, at exactly
these bounds: 300,436,169 enumerated batches, zero divergences of any kind.
Domain A 214,358,881, domain B 85,525,504, domain C 551,784. In domain C both
oracles settled the same 103,743 batches and refused the same 448,041, naming
corresponding refusal classes on every one.

Before that conformance, the same harness reported 11,587 refusal-class
divergences in domain C over 16 class pairs — both oracles refusing the same
batch and naming different classes — from two check-priority choices the
specification did not then fix. In every one of them a third, deliberately
naive rule enumerator (it shares parameter constants and `required_reservation`
with this oracle, so it is independent of the toy but not of this crate)
confirmed that both reported rules really were violated by the witness, so the
disagreement was about priority and never about the admission predicate. That
finding, its sixteen pairs with counts, its minimal witnesses, and the closure
are recorded in the dated addendum to `docs/research/DARK_FBA_RELATION.md`,
§§13.3 and 13.6.

The re-run's zero is weaker in kind than the pre-conformance comparison: this
oracle now implements a specification section derived from the other
implementation's behavior, so refusal-class agreement is a conformance check
rather than independent corroboration. Domains A and B carry no such caveat —
every line of curve construction, tick selection, apportionment, and settlement
is unchanged from the digested pre-read artifact, as `INDEPENDENCE.md` records
file by file.

Vector reproduction: the published `../dark-fba/vectors/v1.txt` is regenerated
byte-for-byte from this oracle, SHA-256
`9a00d7393d00b5cca1e1b980a468a48cb7c21053fac8ae9e15abe2ba7fc9a767`, once the
refusal spellings are passed through the declared vocabulary map. Without the
map the five refusal lines differ in spelling only; all settled lines, and so
every number in the corpus, match unaided.

## Provenance

Original work in this repository, dependency-free. The oracle was written from
the two specification documents; the other implementation's source was read only
after the oracle's own test suite passed, in order to build the adapter. The
conformance edit of 2026-08-18 followed that reading and is bounded in
`INDEPENDENCE.md`; it changed this oracle's admission module only. Because
that source was subsequently read, the harness and this README do not claim
clean-room status for anything written after that point; the oracle's own
digests at the boundary are recorded in `INDEPENDENCE.md`. No source, fixture,
or serialization format was transplanted from any sibling repository; the vector
layout was transcribed from the published corpus file itself.

- Validation toolchain: `rustc 1.98.0-nightly (91fe22da8 2026-06-21)`,
  `cargo 1.98.0-nightly (a595d0da2 2026-06-20)`.
- Validation date: 2026-08-18.
