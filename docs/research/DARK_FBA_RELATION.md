# Specialized Dark Frequent-Batch Relation

Status: PROPOSED research relation with a VERIFIED offline semantic toy. No Dark
backend, proof, network, settlement adapter, or deployment exists.

Relation identifier: `dark-fba/n4-k4-q15/v0`.

## 1. Purpose and exact claim boundary

PROPOSED: This relation studies one fixed-capacity, fixed-grid, uniform-price
frequent batch because its semantics can be stated using bounded admission,
additive aggregation, a canonical argmax, exact allocation, and conservation.
It does not study a continuous order book, arbitrary encrypted programs,
leverage, liquidation, cross-margin, external AMM calls, or live custody.

VERIFIED: `experiments/dark-fba` contains a dependency-free offline Rust oracle
for the relation's clearing and allocation arithmetic. The one process executing
that oracle receives every order and computes every owner output. It is Clear or
`ShieldedSingleExecutor`, depending on who is allowed to inspect the process. It
is never Dark.

REJECTED: An opaque input-root field, fixed padding, exact arithmetic, or a
`DarkTarget` enum value does not provide confidentiality. The toy refuses
`DarkTarget` execution.

## 2. Frozen semantic parameters

The v0 module freezes:

| Field | Value | Visibility |
|---|---:|---|
| Owners | 4 | Public |
| Padded slots | 4 | Public |
| Price ticks | `[1, 2, 3, 4]` quote atoms per base atom | Public |
| Quantity domain | 1 through 15 for occupied slots | Private in Dark target |
| Empty-slot encoding | `Empty`, with no latent fields | Private occupancy in Dark target |
| Price objective | Maximize matched volume | Public rule |
| Price tie | Lowest tick index | Public rule |
| Allocation | Exact pro-rata over every eligible order on each side | Public rule |
| Allocation residual tie | Earliest canonical admitted slot | Public rule |
| Fees | Zero | Public rule |
| No-trade | Volume zero plus a distinct no-trade tick | Public output |

Changing any row creates a new relation version. In particular, a different
tick grid, marginal-price-only allocation, random residual policy, nonzero fee,
or larger capacity is not v0.

## 3. Public-input boundary

The relation's frozen public inputs are:

- relation identifier and version;
- batch identifier and market identifier;
- cutoff time in a named external time domain;
- the fixed dimensions and price ticks above;
- one 32-byte accepted-input root `R`;
- one admission-log finality statement;
- one root-to-slots binding statement;
- one non-equivocation statement;
- one availability-by-cutoff statement; and
- requested execution mode.

PROPOSED: A real backend must replace each statement with a typed proof,
certificate, or consensus fact. The offline toy represents the four boundary
statements as booleans supplied by its named Shielded executor. They are not
proofs. Its `R` is merely preserved into the result; the toy neither hashes the
slots nor verifies a cryptographic opening.

The public result is exactly:

- relation, batch, market, and `R`;
- selected tick or the no-trade tag;
- aggregate matched volume; and
- success or one typed public refusal class.

Curves, occupancy, orders, allocation vector, owner deltas, reservations, and
nullifiers are not public outputs in the Dark target.

## 4. Private witness and admission

Each occupied slot contains:

- batch and market binding;
- owner in `0..4`;
- side in `{Buy, Sell}`;
- limit tick in `0..4`;
- quantity from 1 through 15;
- reserved amount in the spending asset;
- nonzero batch-scoped nullifier;
- arrival time at or before cutoff;
- authorization statement;
- eligibility statement;
- exact inclusion statement for `R`; and
- custody-binding statement for the reserved amount.

An empty slot contributes zero to every curve and has no hidden order fields.
Every occupied slot must pass every admission rule or the batch relation
refuses. Buy reservation must cover `quantity * price[limit]`; sell reservation
must cover `quantity`. The external admission relation must additionally prove
that reservations refer to distinct or otherwise non-double-counted custody;
that obligation enters the witness as the custody-binding statement, rule 17
of the frozen order below. Both offline oracles carry it as a boolean and check
the numeric lower bound separately; neither can bind either one to a ledger.

Nullifiers must be nonzero and pairwise distinct within the batch. A malformed,
late, unauthorized, ineligible, unavailable, unbound, or duplicate-nullifier
batch is not reinterpreted as no trade.

### 4.1 Frozen admission-check order

Status: PROPOSED-now-frozen-for-v0. Changing this order, or the tier boundaries
below, creates a new relation version exactly as a changed row of section 2
does.

Section 8 makes the refusal class a public typed output. A witness that
violates several admission rules therefore publishes a choice, and nothing
above fixed that choice: two implementations conforming to sections 1 through
11 could refuse the same batch and name different classes. Section 13 records
that happening 11,587 times across sixteen class pairs. This subsection closes
the gap by fixing which rule is reported.

The order frozen here is the order `experiments/dark-fba` already implements.
That oracle is the anchor artifact: the published corpus
`experiments/dark-fba/vectors/v1.txt` is rendered from it, section 11's
validation record is a record of it, and its outputs are the only checked-in
expected outputs for the relation. The corpus pins the class of each of its
five single-fault refusal witnesses, and so the rule identities; adopting the
same artifact's priority supplies the one missing coordinate without editing an
anchor byte, and the corpus is byte-identical after this freeze. Adopting any
other total order would have been equally sound as semantics and would have
required rewriting the anchor implementation instead. The choice is one of
stability, not of merit: no admissible batch's verdict, tick, volume,
allocation, or owner output depends on it. It exists so that the public failure
output is a function of the witness rather than of the implementation.

Tier 1, batch-level, evaluated first and in this order:

| # | Rule | Class |
|---:|---|---|
| 1 | Requested execution mode is executable; `DarkTarget` is refused before any witness is inspected | `dark-target-unavailable` |
| 2 | The admission log is final | `admission-log-not-final` |
| 3 | The witness slots open exactly to `R` | `root-binding-absent` |
| 4 | No conflicting finalized root exists | `root-equivocation` |
| 5 | Every admitted payload is available by the declared threshold | `payload-unavailable` |

Tier 2, per occupied slot. Slots are visited in ascending slot index, and the
first failing rule of the first failing slot decides the class. An empty slot
is skipped and can fail nothing.

| # | Rule | Class |
|---:|---|---|
| 6 | Slot binds the batch identifier | `batch-binding-mismatch` |
| 7 | Slot binds the market identifier | `market-binding-mismatch` |
| 8 | Owner is in `0..4` | `owner-out-of-domain` |
| 9 | Quantity is in `1..=15` | `quantity-out-of-domain` |
| 10 | Limit tick is in `0..4` | `limit-out-of-domain` |
| 11 | Nullifier is nonzero | `nullifier-zero` |
| 12 | Nullifier differs from every strictly earlier occupied slot's | `nullifier-repeated` |
| 13 | Arrival is at or before cutoff | `late-arrival` |
| 14 | Authorization statement is present | `unauthorized` |
| 15 | Eligibility statement is present | `ineligible` |
| 16 | Exact-inclusion statement for `R` is present | `inclusion-absent` |
| 17 | Custody-binding statement is present: the reservation refers to distinct, non-double-counted custody | `custody-binding-absent` |
| 18 | Reservation covers the worst-case obligation: `quantity * price[limit]` for a buy, `quantity` for a sell | `reservation-insufficient` |

Tier 3, after every tier-1 and tier-2 rule has passed: clearing, allocation,
and settlement may still refuse for accumulator overflow or a failed internal
invariant. No tier-3 class can preempt an admission class.

Consequences a conforming implementation must respect:

- Slot order dominates rule order. A violation at slot `i` is reported in
  preference to any violation at slot `j > i`, whatever the two rule numbers
  are. In particular, batch-scoped nullifier uniqueness is rule 12 inside the
  per-slot sequence, not a separate later phase.
- Rule 12 looks only at strictly earlier occupied slots, so the slot it names
  is the later repeat and the first-use index it carries is the earliest prior
  use.
- Rules 9 and 10 precede rule 18, which indexes the tick grid and multiplies by
  quantity. A conforming implementation must not evaluate rule 18 on an
  out-of-domain limit or quantity.
- The order is total: exactly one class is reachable for any witness, and a
  conforming backend publishes that class and no other.
- Only the class is public. The slot index, the first-use index, and the
  required/supplied amounts that an implementation may carry alongside a class
  are diagnostics, and section 8 forbids publishing secret-bearing diagnostics
  in the Dark target.
- The class spelling is not frozen. The two implementations of section 13 use
  different spellings that map one to one; what is frozen is each rule's
  identity and its position. The tags above are that shared vocabulary. The
  anchor implementation spells rule 17 `ReservationNotBound`.

VERIFIED on 2026-08-18 that both implementations of section 13 realize this
order: the independent oracle's own suite asserts that a slot violating any two
per-slot rules is refused with the lower-numbered one, over all 78 pairs, and
that an earlier slot's violation preempts a later slot's; the differential's
domain C compares the two implementations over 551,784 batches carrying up to
three simultaneous admission faults and finds no class disagreement. Neither
fact is a proof that the order is realizable inside a Dark backend, which does
not exist.

## 5. Inclusion and availability

PROPOSED: Before cutoff, admission produces an append-only ordered log and an
inclusion receipt for every accepted slot. At cutoff, an external finality rule
freezes `R`. The residual allocation rank is the slot's position in that frozen
log; a later builder cannot reorder equal remainders.

A valid batch requires all four properties:

1. the admission log is final;
2. the witness slots open exactly to `R`;
3. no conflicting finalized root exists for the same relation/batch/market; and
4. every admitted encrypted payload is recoverable by the declared availability
   threshold before computation begins.

If any property is absent, the relation returns a typed retryable or terminal
abort according to the external phase state. Funds remain reserved or are
released by a separately specified timeout relation. The clearing relation
never selects a convenient subset and never converts missing data into an empty
slot.

The offline toy exercises refusal booleans only. It contains no bulletin board,
data-availability code, timeout clock, recovery, or refund implementation.

## 6. Clearing relation

For tick `k`:

```text
Demand[k] = sum(q_i for admitted buys with limit_i >= k)
Supply[k] = sum(q_i for admitted sells with limit_i <= k)
Volume[k] = min(Demand[k], Supply[k])
```

The accumulator width must represent `4 * 15 = 60` exactly. Let `Vmax` be the
maximum `Volume[k]`. If `Vmax = 0`, the valid result is no trade. Otherwise the
selected tick is the smallest `k` whose volume equals `Vmax`.

INFERRED: Fixed padding and a strict `>` update while scanning ticks in ascending
order are sufficient to implement the ties-low rule without a secret-dependent
sort. This is a semantic observation, not a backend performance result.

## 7. Exact allocation and local outputs

At the selected tick, allocation runs independently on eligible buys and
eligible sells. For side total `T` and target `Vmax`:

```text
base_i = floor(q_i * Vmax / T)
rem_i  = (q_i * Vmax) mod T
```

Residual atoms go to the largest remainders, one per order, with slot-index ties
low. Each side sums to exactly `Vmax`. No residual atom becomes protocol dust or
treasury revenue.

Each owner-local output contains:

- bought and sold base atoms;
- signed base and quote deltas at the selected public price;
- released base reservation;
- released quote reservation; and
- the owner's order fills.

PROPOSED: A Dark backend must reveal each local output only to its owner and
prove that all local commitments join the public result and conservation
statement. The toy retains every local output in one Rust object and therefore
provides no access-control or local-output privacy claim.

## 8. Frozen leakage declaration

The Dark target leakage is:

| Category | Frozen declaration |
|---|---|
| Market identity | Public |
| Relation/version and parameters | Public |
| Batch cadence and cutoff | Public |
| Participant count | Hidden within padded capacity 4 |
| Occupancy | Hidden |
| Arrival timing | Epoch-only; exact time hidden |
| Owner identity | Hidden behind admission credential/commitment |
| Side | Hidden |
| Limit | Hidden |
| Quantity | Hidden |
| Reservation and position | Hidden |
| Cancellation | Not supported in v0 |
| Input root and availability/finality receipts | Public |
| Clearing tick/no-trade | Public |
| Aggregate volume | Public |
| Individual fill and deltas | Owner-local only |
| Settlement graph | Outside v0; cannot be claimed hidden |
| Failure | Public typed class, with secret-bearing diagnostics forbidden |
| Regulatory access | None in this mode |

Ciphertext sizes, proof sizes, phase count, and network traffic must be fixed or
added to this table before a backend can claim Dark. The toy has no traffic or
ciphertexts and makes no noninterference claim.

## 9. Frozen corruption model for the Dark target

PROPOSED target profile `static-active-1-of-4`:

- four independently administered computation/custody parties;
- static active corruption of at most one party per key epoch;
- arbitrary public observers, relayers, builders, and proof verifiers;
- arbitrarily malicious order submitters, who know their own inputs and local
  outputs;
- authenticated confidential party channels and erasures as explicit
  assumptions;
- correctness against one corrupt party, with invalid results unable to settle;
- privacy against the same adversary up to the frozen leakage above;
- progress with any three available honest-or-protocol-conforming parties; and
- public typed abort if fewer than three can complete.

This profile does not protect against two computation parties colluding, adaptive
corruption across an epoch without secure erasure, endpoint compromise, owner
inference from public settlement, coercion, or leakage logically implied by a
participant's own order and output. Those are outside the claim, not silently
solved.

No implementation of this corruption model exists here. A single-process MPC,
one complete-witness proof producer, threshold decryption unconstrained to
relation-authorized outputs, or an operator-held recovery key compiles only to
Shielded.

## 10. Correctness, liveness, and settlement obligations

A future backend must establish:

- accepted slots bind to `R` and the frozen admission log;
- aggregate curves and selected tick satisfy the exact relation;
- every fill is eligible, bounded, and follows the residual rank;
- buy and sell totals both equal public volume;
- quote and base deltas conserve with zero fees;
- every local output commitment is bound to the same result;
- no invalid output can settle;
- inclusion, equivocation, withholding, timeout, retry, and abort have distinct
  receipts; and
- a builder or computation party cannot learn a partial result and silently
  substitute, censor, or restart the batch on more favorable inputs.

Settlement is a different relation. Ordinary public token transfers reveal the
account/amount graph and make the composed system pre-trade dark at most. Fully
Dark settlement requires a separately specified shielded-note or confidential
asset relation. The offline toy computes synthetic deltas and moves no asset.

## 11. Offline validation and falsifiers

VERIFIED on 2026-08-18 after narrow local execution:

- deterministic vector rendering is checked byte-for-byte;
- one balanced crossing exercises pro-rata residual ties;
- one price tie selects the lowest tick;
- one no-cross book returns no trade rather than refusal;
- late, unavailable, duplicate-nullifier, under-reserved, and Dark-mode cases
  refuse distinctly;
- all one-buy/one-sell combinations over four limits and quantities `1..15`
  preserve fill bounds, limit satisfaction, and conservation; and
- swapping equal-remainder orders preserves public price/volume but changes the
  residual recipient, demonstrating why canonical inclusion rank is economic.

The verification command and current vector bytes live under
`experiments/dark-fba`. A failed test, a differing independent implementation,
an allocation manipulation, or a backend that widens leakage falsifies any
stronger claim. No benchmark, formal theorem, cryptographic security proof, or
independent audit is claimed.

## 12. Provenance

The implementation is original work in this repository, informed by
`relations/CLEARING_V0.md` and public call-auction/apportionment mathematics. No
source or fixtures were transplanted from sibling repositories. Because the
author previously inspected related implementations, this document does not
claim clean-room status. See
[`PROVENANCE.md`](../../experiments/dark-fba/PROVENANCE.md).

## 13. Addendum, 2026-08-18: independent implementation and differential run

This section records a second implementation of the relation and an exhaustive
differential comparison against `experiments/dark-fba`. It adds no privacy
claim, and it does not weaken or restate any claim above. Both oracles are Clear
or `ShieldedSingleExecutor` semantics; nothing here is evidence about a Dark
backend, which still does not exist.

### 13.1 What was built

`experiments/dark-fba-independent/oracle` is a second, dependency-free Rust
oracle for `dark-fba/n4-k4-q15/v0`, written from `relations/CLEARING_V0.md` and
sections 1 through 11 of this document alone. Its own 35-test suite covering
admission refusals, curve construction, maximum-volume and ties-low selection,
largest-remainder exactness, residual rank, and conservation passed before any
line of the existing implementation's source was read. The file digests at that
boundary are recorded in
`experiments/dark-fba-independent/INDEPENDENCE.md`. The existing implementation
was then read once, in order to write the adapter, and neither implementation
was modified: the recorded digests were re-verified after the differential run
and are unchanged, so the comparison ran against the pre-read artifact exactly.

`experiments/dark-fba-independent/differ` enumerates finite domains of batches,
runs both oracles on every batch, and compares complete outputs: accept versus
refuse, refusal class, clearing tick or no-trade tag, public volume, the
per-slot allocation vector, and every owner-local output.

### 13.2 Differential domain and case counts

| Domain | Content | Cases |
|---|---|---:|
| A | Every book over the complete frozen order domain: four padded slots, each vacant or a (side, tick, quantity) triple with quantity `1..=15`; `121^4` | 214,358,881 |
| B | Every book over quantity `1..=2` (`17^4`) crossed with all `4^4` owner assignments and four reservation-surplus patterns | 85,525,504 |
| C | Six base books crossed with every subset of size at most three drawn from 78 admission perturbations | 474,948 |
| | Total | 300,359,333 |

Domain A is exhaustive over the whole frozen order domain of section 2, so the
clearing and allocation semantics are compared on every book the relation
admits, not on a sample. Domain A holds owner `i` at slot `i` and reserves
exactly the required amount; domain B varies exactly those two dimensions.
Domain C is the refusal surface, and its multi-perturbation subsets are what
expose disagreement about which rule wins when one witness violates several.

### 13.3 Result

VERIFIED on 2026-08-18, at exactly these bounds: over 300,359,333 enumerated
batches the two independent implementations agree on every accept-versus-refuse
verdict, and on every settled batch they agree on the clearing tick or no-trade
tag, the public volume, the complete per-slot allocation vector, and every
owner-local output. Domains A and B produced zero divergences of any kind.

Domain C produced 11,587 divergences, all of one kind: both oracles refuse the
same batch but name different refusal classes. They fall into sixteen class
pairs, and all sixteen are consequences of two check-priority choices that
neither this document nor `CLEARING_V0.md` fixes:

1. Order of the two domain checks. The independent oracle tests the limit tick
   before the quantity; the existing toy tests the quantity before the limit.
   Minimal witness, one slot, everything else empty and canonical:
   `slot0 = buy(owner 0, limit 4, quantity 0, reserved 0, nullifier 1)`. The
   independent oracle answers `limit-out-of-domain`; the existing toy answers
   `order[0]:QuantityOutOfRange`. 4,311 cases.

2. Position of the nullifier rules. The existing toy tests each slot's zero and
   duplicate nullifier before that slot's arrival, authorization, eligibility,
   inclusion, and reservation rules, and returns at the first failing slot. The
   independent oracle applies every per-slot rule to every slot first and tests
   batch-scoped nullifier uniqueness afterwards. Minimal witness:
   `slot0 = buy(owner 0, limit 0, quantity 1, reserved 1, nullifier 1)`,
   `slot1 = buy(owner 0, limit 0, quantity 1, reserved 0, nullifier 1)`. The
   independent oracle answers `reservation-insufficient`; the existing toy
   answers `order[1]:DuplicateNullifier { first_slot: 0 }`. 7,276 cases across
   the remaining fifteen pairs.

A third, deliberately naive rule enumerator in the harness checks every
admission rule independently rather than at first failure. In all 11,587
divergences it confirms that both reported classes correspond to rules the
witness genuinely violates: neither oracle ever names a rule that is not
broken. The disagreement is therefore about priority among simultaneously
violated rules, not about the admission predicate.

This is a real gap in the specification, not a defect in either
implementation. Section 4 says a malformed batch is refused and must not be
reinterpreted as no trade, but it does not say which class a batch violating
several rules must report. A backend that publishes a typed refusal class makes
that class an observable, and section 8 already requires failure to be a public
typed class; two conforming implementations can therefore disagree publicly.
Freezing a check order, or declaring the reported class underdetermined, was
open work for v0 when this run was recorded. It is now closed: section 4.1
freezes the order, and section 13.6 records the conformance and the re-run.

The sixteen pairs, with counts, are:

```text
batch-binding-mismatch   vs nullifier-repeated       216
inclusion-absent         vs nullifier-repeated       494
inclusion-absent         vs nullifier-zero           591
ineligible               vs nullifier-repeated       517
ineligible               vs nullifier-zero           606
late-arrival             vs nullifier-repeated       563
late-arrival             vs nullifier-zero           636
limit-out-of-domain      vs nullifier-repeated       369
limit-out-of-domain      vs quantity-out-of-domain  4311
market-binding-mismatch  vs nullifier-repeated       209
nullifier-zero           vs nullifier-repeated       111
owner-out-of-domain      vs nullifier-repeated       397
quantity-out-of-domain   vs nullifier-repeated       501
reservation-insufficient vs nullifier-repeated       905
unauthorized             vs nullifier-repeated       540
unauthorized             vs nullifier-zero           621
```

### 13.4 Golden vector reproduction

The published corpus `experiments/dark-fba/vectors/v1.txt` was regenerated from
the independent oracle. Every settled line, and therefore every number in the
corpus, reproduces byte-for-byte unaided. The five refusal lines carry each
implementation's own class spelling; passing the independent oracle's classes
through a declared one-to-one vocabulary map reproduces the whole file
byte-for-byte, SHA-256
`9a00d7393d00b5cca1e1b980a468a48cb7c21053fac8ae9e15abe2ba7fc9a767`. The
reproduction is checked in at
`experiments/dark-fba-independent/vectors/v1-independent-reproduction.txt`.

### 13.5 Boundaries of this addendum

- No privacy, noninterference, leakage, or confidentiality property is claimed,
  tested, or implied. This is Clear-mode semantic falsification only.
- Agreement between two implementations is not a proof. It is a falsification
  attempt that failed on the stated domains; no formal theorem, mechanized
  refinement, cryptographic argument, or independent audit is claimed.
- The domains bound the claim exactly. Domain A is exhaustive over the frozen
  order domain, but domain B restricts quantities to `1..=2`, and domain C
  restricts admission faults to at most three simultaneous perturbations of six
  base books.
- One admission statement is untested by the differential: the existing toy's
  `reservation_bound` witness, which stands for the custody-binding obligation
  of section 4. The independent oracle does not model it and the harness always
  supplies it as present. That the independent oracle omits a statement this
  document requires is itself a finding about the independent oracle. Closed
  after this run; see section 13.6.
- The vector layout was transcribed from the published corpus file, since a
  serialization shape is a rendering convention rather than a semantic property
  of the relation.

### 13.6 Addendum, later on 2026-08-18: the refusal-class gap is closed

Sections 13.1 through 13.5 record the state before this subsection and are not
restated, weakened, or corrected by it. This subsection records what was done
about the gap they found.

**Spec.** Section 4.1 freezes the admission-check order for v0, adopting the
order the anchor implementation already realizes, for the reasons stated there.
`experiments/dark-fba` is unmodified and its golden corpus is byte-identical,
still SHA-256
`9a00d7393d00b5cca1e1b980a468a48cb7c21053fac8ae9e15abe2ba7fc9a767`. A backend
that publishes a different class for a multiply-invalid book is now
non-conforming rather than merely different.

**Conformance.** The independent oracle was edited to the frozen order: the
quantity domain now precedes the limit domain (rules 9 and 10), and the two
nullifier rules moved out of a separate batch-scoped pass into the per-slot
sequence at rules 11 and 12, so slot order dominates rule order. The edit is
confined to that oracle's admission module and witness struct; no line of its
curve construction, tick selection, apportionment, or settlement changed.

**Custody-binding statement.** The statement 13.5 recorded as untested is now
modelled by both oracles and covered by the differential. The independent
oracle carries it as rule 17, the adapter maps it to the anchor's
`ToyAdmissionWitness::reservation_bound` instead of always supplying `true`,
and domain C's per-slot perturbation catalogue gained a `custody-unbound`
perturbation, applied at each of the four slots. The action catalogue therefore
grows from 78 to 82 and domain C from 474,948 batches to 551,784. No admission statement of section 4 is now outside
the comparison.

**Re-run.** VERIFIED on 2026-08-18, at exactly these bounds:

| Domain | Cases | Divergences |
|---|---:|---:|
| A | 214,358,881 | 0 |
| B | 85,525,504 | 0 |
| C | 551,784 | 0 |
| Total | 300,436,169 | 0 |

Refusal class included: of domain C's 551,784 batches the two oracles settled
the same 103,743 and refused the same 448,041, naming corresponding classes on
every one. The published corpus still reproduces byte-for-byte through the
declared vocabulary map, at the digest above.

**What the zero is worth.** The 11,587 divergences of 13.3 were found between
implementations written independently, and that finding stands exactly as
recorded. The zero above is weaker in kind: the independent oracle now
implements a section derived from the anchor's own behavior, so agreement on
refusal class is a conformance check, not independent corroboration. Domains A
and B are not affected by that caveat, because the clearing, selection,
allocation, and settlement code on both sides is unchanged from the artifacts
those runs compared. The independence claim of 13.1 continues to describe only
the pre-conformance oracle; the post-conformance digests are recorded
separately in
[`INDEPENDENCE.md`](../../experiments/dark-fba-independent/INDEPENDENCE.md).

**Tests.** The two harness tests that asserted the divergence still existed now
assert its absence and cite section 4.1, and domain C's assertion is zero
divergences in every class rather than a nonzero refusal-class count. The
independent oracle's own suite gained order tests: each per-slot edit violates
exactly its own rule, a slot violating any two of the thirteen per-slot rules
is refused with the lower-numbered one over all 78 pairs, an earlier slot's
violation preempts a later slot's, mode and boundary rules preempt every
per-slot rule, and the eighteen admission classes carry the rule numbers 1
through 18 exactly once each.
