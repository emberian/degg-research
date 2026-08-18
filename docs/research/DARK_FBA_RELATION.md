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
- eligibility statement; and
- exact inclusion statement for `R`.

An empty slot contributes zero to every curve and has no hidden order fields.
Every occupied slot must pass every admission rule or the batch relation
refuses. Buy reservation must cover `quantity * price[limit]`; sell reservation
must cover `quantity`. The external admission relation must additionally prove
that reservations refer to distinct or otherwise non-double-counted custody.
The toy checks the numeric lower bound but cannot bind it to a ledger.

Nullifiers must be nonzero and pairwise distinct within the batch. A malformed,
late, unauthorized, ineligible, unavailable, unbound, or duplicate-nullifier
batch is not reinterpreted as no trade.

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
