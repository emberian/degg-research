# Paired-payoff corpus: bundling invariance

Status: PROPOSED position, VERIFIED as an offline deterministic measurement of
this crate after the commands below pass. Synthetic corpus over a toy model. It
exhibits a classification arbitrage structurally and asserts nothing about any
real rule's text, any real instrument, or any real venue.

This standalone crate turns one position into a computation. The definitions
research memo on Question 8 of the joint definitions notice argues that
classification should be invariant under bundling and unbundling: a set of
cash-or-nothing claims covering every outcome of one reference variable, without
overlap, is a portfolio of such claims on that variable; the complete set is
economically identical to the collateral it was issued against; and the
conversions run both ways as ordinary operation. A criterion that answers
differently for the bundle and for its parts therefore contains a classification
arbitrage exercisable for free.

The memo names the experiment that would strengthen it. This is that experiment,
built exhaustively rather than as four worked pairs.

Memo:
[definitions-q8-event-contracts-and-options-on-securities.md](../../docs/regulatory/research-memos/definitions-q8-event-contracts-and-options-on-securities.md).

## What is modelled

- A **payoff object**: an exhaustive, non-overlapping partition of one reference
  variable into cells, a nonnegative integer payoff vector over those cells, and
  three economic facts a criterion may read (what the settlement amount is a
  function of, whether the maximum payout is prefunded, whether the claim is
  transferable). Nothing else about the object exists.
- The two **costless operations**: bundle (hold the parts together; add the
  payoffs cell by cell) and split (move part of a position elsewhere), together
  with deposit and recombination, which are the only ways a claim is created or
  destroyed.
- A **classification criterion**: any decidable function from a payoff object's
  economic facts to a category label, together with its own **aggregation
  story** — its account of how the labels of the parts bear on the label of the
  whole.

## The test

A criterion is **bundling-invariant** when, for every payoff object and every
decomposition of it into parts carrying the same economic facts, unanimous parts
fix the whole's label. That is the strict reading. Each criterion may also
declare extra labels it considers acceptable for a whole whose parts are
unanimous, and the sweep counts violations under both readings, so no result
here rests on an uncharitable reading of a candidate.

Every violation is emitted as a witness: the object, its decomposition, the two
labels, the labels the criterion's story permitted, and the zero-cost path
between the two sides. The witnesses are minimized by cell count, then bundle
size, then vector order.

## Bounds

Full enumeration inside these bounds, and no claim outside them:

| Bound | Value |
|---|---|
| Cells per partition | 2 through 5 |
| Payout per cell | 0 through 3 |
| Payoff vectors | 1,360 |
| Fact profiles | 12 (three reference kinds, prefunded or not, transferable or not) |
| Payoff objects | 16,320 |
| Decompositions checked per criterion | 56,936 vector-level, 683,232 counting fact profiles |

Decomposition families: every unordered binary split of every vector, and the
elementary unbundling of every vector into its individual one-cell claims.

## What it found

| Criterion | Reads | Strict witnesses | Witnesses under its own story | Verdict |
|---|---|---|---|---|
| `binary-payout` | payoff | 49,032 | 49,032 | arbitrage |
| `bounded-payout` (ceiling 2) | payoff | 180,696 | 180,696 | arbitrage |
| `distinct-values` | payoff | 159,120 | 113,472 | arbitrage |
| `complete-set` | payoff | 27,420 | 0 | arbitrage under the strict reading only |
| `support-nonempty` | payoff | 0 | 0 | invariant |
| `prefunding-exclusion` | facts | 0 | 0 | invariant |
| `reference-and-settlement` (control) | facts | 0 | 0 | invariant |
| `constant-label` (degenerate control) | nothing | 0 | 0 | invariant, and separates nothing |

The smallest witness in the whole corpus is the memo's own example, found rather
than chosen. Over a two-cell partition, `[1,0]` and `[0,1]` are each
cash-or-nothing, so `binary-payout` calls each an event contract; held together
they are `[1,1]`, which pays the same amount in both states and is therefore not
cash-or-nothing, so the same criterion calls the pair swap-like. Splitting a
position in two mints no claim, burns none, and moves no collateral: the claims
outstanding are `[1,1]` before and after. The holder chooses the label at no
cost, and can also recombine `[1,1]` into its collateral before resolution.

Three results are findings rather than design choices, and are pinned by tests
that say so:

1. **A payoff-shape criterion can be invariant.** `support-nonempty` reads only
   whether the object pays in some state, and survives. The outcomes a bundle
   pays in are exactly the union of the outcomes its parts pay in, so a criterion
   whose label classes are closed under that union is invariant.
2. **Invariance does not make a criterion a good one.** `prefunding-exclusion`
   is invariant, because prefunding is a fact the parts and the bundle share.
   The memo's objection to it is Position 6, not arbitrage. `constant-label` is
   invariant and separates nothing at all.
3. **A criterion can survive by declining to answer.** `complete-set` has 27,420
   strict witnesses and none under its own story — but the story it needs
   permits three different labels (option, portfolio, collateral) for the same
   unanimous parts. The corpus records that as `story_permits_alternatives`.

## The table the memo asked for

The memo proposed taking four payoff shapes a market participant would recognize
and expressing each one twice, with the exact integer cashflow vector printed on
both sides. The `[paired_payoffs]` section of `vectors/v1.txt` is that table, with
the complete set added as a fifth row and each candidate criterion's answer
printed for both expressions:

| Shape | As one bespoke instrument | As a portfolio over the basis | Criteria giving the two expressions different answers |
|---|---|---|---|
| Digital | `[0,1,0,0,0]` | 1 claim on cell 2 | none |
| Range | `[0,1,1,0,0]` | 1 claim on cell 2 + 1 on cell 3 | `complete-set` |
| Capped directional | `[0,1,2,3,3]` | 1 on cell 2 + 2 on cell 3 + 3 on cell 4 + 3 on cell 5 | `binary-payout`, `bounded-payout`, `distinct-values`, `complete-set` |
| Tail | `[0,0,0,0,3]` | 3 claims on cell 5 | `bounded-payout` |
| Complete set | `[1,1,1,1,1]` | 1 claim on every cell | `binary-payout`, `complete-set` |

Every row pays the same integer amount in every state on both sides; that is
checked for all 1,360 vectors in the corpus, not only these five.

## The census

The candidates are eight tests somebody might write down. The census asks the
wider question where enumeration can answer it: of **every** two-label criterion
that reads only which outcomes an object pays in, how many are invariant?

| Cells | Such criteria | Invariant | Invariant and separating something |
|---|---|---|---|
| 2 | 16 | 12 | 10 |
| 3 | 256 | 52 | 50 |
| 4 | 65,536 | 300 | 298 |

At five cells there are 2^32 such criteria and this experiment says nothing about
them. The counts are cross-checked against a direct sweep over payoff vectors and
decompositions at two and three cells, with no appeal to the union argument.

## Running it

```sh
cargo fmt --manifest-path experiments/bundling-invariance/Cargo.toml --check
cargo clippy --manifest-path experiments/bundling-invariance/Cargo.toml \
  --all-targets -- -D warnings
cargo test --manifest-path experiments/bundling-invariance/Cargo.toml
cargo run --quiet --manifest-path experiments/bundling-invariance/Cargo.toml \
  --bin degg-bundling-vectors
```

The checked-in `vectors/v1.txt` is the transcript of the whole experiment and is
compared byte for byte by `tests/vectors.rs`. It is synthetic test data, not a
cryptographic artifact or an external market record. SHA-256 at the commit that
introduced it:

```
d1ca9fb9105daf89208a88fa6e33827ac451a9485dd0f3efc5ff85f8b58deae7  vectors/v1.txt
```

Regenerate the digest with `shasum -a 256 vectors/v1.txt`.

## Claim boundary

VERIFIED, at exactly the bounds above: the payoff arithmetic, the conservation
identity of the collateral ledger, the label each criterion assigns to each
object, the exhaustiveness of the sweep, the witness counts, the minimized
witnesses, and the census counts. Those are deterministic offline measurements of
this crate and nothing else. Tested, not formally verified.

PROPOSED: that bundling invariance is a test a classification criterion should
have to survive.

Deliberately absent, each named rather than approximated: no price, probability,
discount rate, or fee, so nothing here measures how profitable an arbitrage would
be, only that its cost is zero; no time, so nothing distinguishes a decomposition
performed before resolution from one performed after; no market data and no
network; and no legal content — the category labels are the criteria's own
vocabulary, not statutory categories, and no label here is a legal conclusion. A
criterion recorded as invariant is invariant on this corpus, not as a theorem.

This crate has no dependencies. It is first-party material under
AGPL-3.0-or-later; see [../../LICENSING.md](../../LICENSING.md).
