# The Settlement Relation

Status: PROPOSED specification of settlement as a separate relation, with a
VERIFIED deterministic offline model at exactly the bounds recorded in
section 11. No chain, token standard, signature, cryptographic security,
economic, or liveness property is claimed here, and none is measured. The
implementation is
[`experiments/settlement-model`](../../experiments/settlement-model/).

**Private computation alone does not make settlement Dark.** That sentence is
this packet's assignment, and the packet's sharpest content is a measurement
rather than an argument: on a transparent per-claim transfer surface, the
public settlement transcript alone mechanically reconstructs every settled
position's owner, side, exact fill, and exact reservation — everything the
computation's frozen leakage table hid. Section 8 holds the measurement.

Model identifier: `degg-settlement/v0`.

## 1. Why this packet exists

[`../../CLAUDE_HANDOFF.md`](../../CLAUDE_HANDOFF.md) section 6 P1.7 asks for
authorized custody, nullifiers, retries, conservation, and public/private
transfer leakage "as a separate relation". The architecture already commits
to that separation: the pipeline of [`../ARCHITECTURE.md`](../ARCHITECTURE.md)
section 2 ends at "local outputs + public receipt + separately specified
settlement", and its section 5 names what settlement must verify — the
accepted input root, the relation identifier, the exact output root,
authorization and nullifiers, asset conservation, limit satisfaction, the fee
schedule, replay resistance, phase transition, and custody availability.

Three landed packets each stop one step short of it, and say so:

- [`DARK_FBA_RELATION.md`](DARK_FBA_RELATION.md) section 10: "Settlement is a
  different relation. Ordinary public token transfers reveal the
  account/amount graph and make the composed system pre-trade dark at most."
  The oracle computes synthetic deltas and moves no asset.
- [`INCLUSION_AVAILABILITY.md`](INCLUSION_AVAILABILITY.md) section 7: "the
  settled path releases reservations to a relation that does not exist in
  this repository." Its reserve ledger resolves each nullifier exactly once,
  to an amount with no asset, no owner, and no transfer.
- [`SHIELDED_BASELINE.md`](SHIELDED_BASELINE.md) section 9 places settlement
  as the last rung of its promotion path, and its delivery commitment is the
  object that makes an owner's settlement claim checkable at all.

This packet specifies the relation those three point at, implements it as a
deterministic offline model over their actual published objects, and measures
what each transfer surface then discloses.

## 2. The objects and the join

PROPOSED as the relation; VERIFIED as the behaviour of the model at the
bounds of section 11.

Settlement consumes only published or owner-held objects. The executor is
never consulted:

| Input | Produced by | What settlement uses it for |
|---|---|---|
| `ShieldedReceipt` | shielded baseline | the relation identity (module digest), the accepted-input root (cutoff binding), the outcome digest, the delivery root, the typed status |
| `CutoffRoot` | inclusion lane | the committed admitted set the receipt must be bound to |
| `Phase` of the relying party's own `BatchMachine` | inclusion lane | the settled-versus-aborted gate; `Settled` must name this receipt's outcome |
| `InclusionReceipt` for one position | inclusion lane | the record — nullifier, submitter, rank — is committed under the root |
| `DeliveryOpening` for the same position | shielded baseline | the committed local effect at that position under the receipt's delivery root |
| custody entry under the record's nullifier | this relation | the reserved owner, asset, and amount the effect must agree with |

A **settlement authorization** is the verified conjunction of all six, plus
exact re-derivation: the relation recomputes the deltas and released amounts
from the side, the fill, and the public price, computes the reservation the
effect implies, and requires custody to hold exactly that amount, in the
side's spending asset, for the effect's owner. The published numbers are
checked, never trusted. The authorization check order is frozen and pinned by
tests, for the reason `DARK_FBA_RELATION.md` section 4.1 froze the admission
order: a typed refusal is a public observable.

Against `ARCHITECTURE.md` section 5's checklist:

| Check | Object here | Status |
|---|---|---|
| accepted input root | receipt's `CutoffBinding` against the observed `CutoffRoot` | modelled |
| relation/program identifier | the module digest inside the receipt binding | modelled |
| exact output root | outcome digest and delivery root inside the receipt binding, and the phase's result digest | modelled |
| authorization | the six-object conjunction above | modelled |
| nullifiers | admission nullifier keys custody; a derived settlement nullifier is spent once per position (section 4) | modelled |
| asset conservation | exact split of every reservation plus a pre-funded pool (section 6) | modelled and measured |
| limit satisfaction | **not re-checked**; only fill-versus-volume and delta consistency are; the limit lives in the hidden witness | named gap, inherited (section 7) |
| fee schedule | zero, frozen; any fee is a new relation version | modelled trivially |
| replay resistance | idempotent execution by settlement nullifier (section 5) | modelled |
| phase transition | `Settled` and terminal aborts gate the two lanes exclusively | modelled |
| custody availability | escrow held by the relation from submission, before any clearing | modelled; a real chain binding is absent |

## 3. Custody: reserved, obligated, settled, refunded

The upstream reserve ledger has three dispositions — outstanding, refunded,
settled. This relation refines them to four states per admission nullifier:

```text
Reserved --observe settled receipt--> Obligated --execute--> Settled
   |                                      |
   | terminal abort                       X  (no transition)
   v                                      |
Refunded <--------------------------------+
```

- **Reserved.** Escrowed at submission: the owner, the spending asset (quote
  for a buy, base for a sell), and the exact reservation, keyed by the
  admission nullifier, held in one pool. Refundable under a terminal abort.
- **Obligated.** The relying party observed a well-formed settled receipt
  whose outcome digest its own machine's `Settled` phase names. Every
  reserved entry obligates to that receipt binding at once. Obligation is
  the state upstream cannot express: the refund lane is now foreclosed —
  `Obligated -> Refunded` is a refused transition — but no instruction has
  executed yet.
- **Settled.** Exactly one settlement instruction executed. The reservation
  splits exactly: the consumed part stays in the pool to fund the
  counterparty legs, the released remainder returns to the owner, and the
  acquired leg of the trade is paid out of the pool.
- **Refunded.** Returned in full under a terminal abort, exactly once, with
  the phase gate deciding *when* and the upstream entitlement objects
  (inclusion receipts, repudiated-root statements) deciding *who*. The model
  drives its refund lane beside the upstream `claim_refund` and the two agree
  on every amount.

A padding position has no custody entry at all: it escrows nothing, settles
nothing, refunds nothing, and its settlement claim is refused as
`PaddingPositionSettlesNothing` before custody is consulted.

INFERRED, and stated because it is a two-line proof with a real consequence:
because *every* occupied position's full worst-case reservation is already in
the pool before clearing, and full settlement pays out per asset exactly the
per-asset totals deposited, any *subset* of honest settlements pays out at
most what the pool holds. Pre-funding is what makes per-position, any-order
settlement solvent without coordination. Section 11 records the exhaustive
check over all 24 execution orders and every prefix of each.

## 4. Nullifier discipline

Two nullifier domains, deliberately separated:

- The **admission nullifier** is the record's batch-scoped nullifier from
  `DARK_FBA_RELATION.md` section 4. It keys escrow, exactly as the upstream
  reserve ledger keys refunds, so the amount owed is fixed at submission and
  is not a function of which history a holder publishes.
- The **settlement nullifier** is derived:

```text
settlement_nullifier = tagged("degg/settlement-model/v0/settlement-nullifier",
                              delivery_domain || admission_nullifier || seq)
```

where `delivery_domain` already binds the cutoff root and the module digest.
One settlement per delivery position, by construction: replay within the
batch hits the spent set; replay across batches, markets, or relation
versions moves the delivery domain and with it the nullifier; and the
derivation is domain-separated from every other tagged object in the
composition. The settlement nullifier never equals the admission nullifier,
and the model asserts pairwise distinctness across positions.

One record occupies one position in this composition, so "one settlement per
position" and "one settlement per admission nullifier" coincide here. The
discipline is stated over positions because the delivery commitment is
per-position, and because a *second* receipt for the same cutoff — the case
where the two could diverge — is an outcome equivocation: the settlement book
pins the first observed receipt binding, refuses a conflicting one, and the
adjudicating object is the dispute verdict of `SHIELDED_BASELINE.md`, not
this relation.

## 5. Retry semantics: idempotent by nullifier

A settlement instruction is a pure function of published objects, so
resubmission is the normal case, not an anomaly: a crashed settlement adapter
re-derives the same authorization and submits again.

- If the settlement nullifier is unspent, the instruction executes and the
  payout moves, exactly once.
- If it is spent by the *same* execution digest, the answer is
  `AlreadyExecuted`, an `Ok` variant carrying the original digest. Nothing
  moves; balances, pool, and claim transcript are byte-identical. Retry until
  one of the two answers arrives, and the funds moved exactly once either
  way.
- If it is spent by a *different* execution digest, `ConflictingExecution`
  refuses. VERIFIED as defence in depth rather than as a reachable state:
  under one observed receipt binding the execution digest is a function of
  that binding, the position, and the committed effect, and the delivery root
  inside the binding commits the effect at each position — two digests for
  one nullifier require a hash collision. The class exists so a refactor
  cannot turn that impossibility into a silent overwrite, exactly as the
  upstream lane keeps `ReceiptDefect::SequenceMismatch`.

This is a different axis from the upstream `compute-timeout` retry, which
reruns the *computation* against the same cutoff root. Settlement retries
rerun only the *transfer*, against the same receipt; there is no path from a
settlement retry to a recomputation, and no retry budget, because an
idempotent instruction needs none.

## 6. Conservation

Frozen: fees are zero; every amount is an exact `u64`; there is no floating
point anywhere in the crate. Changing the fee schedule is a new relation
version.

Per position, the reservation splits exactly, with nothing rounded and
nothing retained:

```text
buy:  reserved = released_quote + fill * price     consumed = fill * price
sell: reserved = released_base  + fill             consumed = fill
```

and the acquired leg is `fill` base for a buy, `fill * price` quote for a
sell. The authorization *recomputes* this split and refuses a claim whose
custody entry does not hold exactly the implied reservation.

Per batch, per asset: total deposited equals pool plus total credited, at
every intermediate state — the invariant the ledger exposes as a predicate
and the tests assert after every operation. On the fully settled balanced
book the pool drains to exactly zero: 21 quote and 8 base in, 21 quote and
8 base out, redistributed by the trade. On every terminal abort the refund
lane drains it to zero as well, once per nullifier, in agreement with the
upstream ledger's amounts. A zero-fill occupied position settles through the
settlement lane — full reservation released, state `Settled`, not `Refunded`
— keeping the two lanes semantically distinct even when the amounts coincide.

VERIFIED at the bounds of section 11: solvency and conservation hold in
every one of the 24 settlement orders at every prefix, and the pool ends
empty in all of them.

## 7. What settlement does not check, measured

`SHIELDED_BASELINE.md` section 6.2 measured the executor's freedom in the
published result: 377 of 1,125 alternative runs, including all 33 well-formed
public results, pass every check the composed system has. Settlement
inherits that freedom undiminished, and this packet measures the inheritance
rather than hiding it. Three demonstrations, each a test:

1. **A wrong-but-consistent result settles and conserves.** The executor
   substitutes one owner's limit inside the same escrowed budget; the public
   result moves from tick 2, volume 5 to tick 0, volume 4; every position
   authorizes, every position settles, the pool drains to exactly zero.
   Settlement conserved value around a result the honest book never
   produced. Only a verifiable statement about the evaluation — rung 5 of
   the shielded baseline's promotion path — closes this; custody cannot.
2. **A batch-imbalanced forgery is caught only as terminal insolvency, and
   lands on the wrong party.** The executor forges one delivery entry (a
   fill of 3 instead of 2, internally consistent with its own escrow).
   Per-position checking cannot see the batch total; the forged claim
   settles. The batch now owes more quote than the pool holds, and whoever
   claims last — in the test, an honest owner, after the forger claimed
   first — is refused with `PoolInsolvent`. Conservation still holds:
   nothing was created; the loss is a stranded honest obligation. Detection
   without attribution is the exact shape of the upstream equivocation
   findings, now in funds.
3. **An omitted position is stranded with a verdict but no funds path.** The
   executor omits one committed position; the other three settle against the
   reduced result; the omitted owner's claim is refused as
   `NoLocalOutputAtPosition`, its refund is refused because the phase is
   `Settled`, and its reservation stays `Obligated` forever — exactly the
   pool's residue. The owner holds the transferable `OmissionProof` verdict,
   and this relation has **no adjudication rule that turns a verdict into
   funds**. That absence is recorded in section 10 rather than papered over.

The division of labour, stated plainly: settlement verifies *binding*
(everything names the same receipt, root, module, and phase), *arithmetic
consistency* (the effect is what side, fill, and price imply), and *custody*
(the implied reservation is exactly what was escrowed, spent exactly once).
It does not and cannot verify *correctness* of the clearing, the allocation,
or the limit satisfaction of hidden witnesses. "No invalid output can
settle" from `DARK_FBA_RELATION.md` section 10 therefore remains **not met**
in this composition, now with a settlement-layer measurement of exactly how.

## 8. Transfer-surface leakage

The model executes identical settlements and projects them onto four
surfaces. What follows is mechanical content of the projections, measured by
tests against ground truth the surfaces were never shown.

### 8.1 Per-claim public transfers

Every deposit, claim, and refund is a public account-and-amount event, as on
a transparent chain. VERIFIED, from the projection alone plus the public
price and the frozen grid:

- **Deposits leak the side before the batch runs.** A two-asset custody
  means a buy deposits quote and a sell deposits base; the deposit rows
  alone map every participant to its side, pre-trade. The computation's
  "side: hidden" row is falsified at the settlement layer before any
  clearing happens.
- **Claims leak occupancy and rank.** Padding positions never claim, so the
  claimed-position set is the exact occupancy the padded cutoff root
  concealed; each claim names its `seq`, the residual-allocation rank that
  [`INCLUSION_AVAILABILITY.md`](INCLUSION_AVAILABILITY.md) section 9.1
  already flagged as the price of publishing a receipt.
- **Amounts reconstruct the book.** `reconstruct` recovers every settled
  position's owner, side, and exact fill; every sell's reservation, which
  upper-bounds its quantity and *equals* it under exact worst-case
  reservation; and for every buy the exact budget with a small candidate set
  of (limit, quantity) pairs containing the truth. On the corpus book the
  buys' candidate sets have two and three elements.
- **Refunds leak without a trade.** An aborted batch never cleared, and the
  refund rows still publish every admitted order's spending asset — the side
  — and its full reservation.

### 8.2 Netted public transfers

One net flow per account. VERIFIED: the flows are exactly the owners' signed
base and quote deltas — the very numbers the computation's leakage table
marks "owner-local only" — and the account list is exact participation,
including the zero-fill owner at flow zero. Netting hides the per-position
breakdown and the ranks; it publishes the economic outcome per identity.
On a full-refund abort the netted flows are all zero, so this surface leaks
participation but not budgets there.

### 8.3 Named settlement agent

A named agent executes custody; the public sees conservation totals only.
This is Shielded, by definition and by declaration: the agent sees every
row, and the surface's honesty is that it says so. The projection type
itself carries no per-account or per-position data.

### 8.4 The Dark settlement target

Refuses, with `DarkSettlementAbsent`, exactly as the oracles refuse
`DarkTarget` execution. A Dark settlement surface needs at least: hiding
commitments to values and recipients, claims unlinkable to deposits and to
positions, value-conservation proofs replacing the visible pool arithmetic,
and a uniform (capacity-shaped or note-based) claim pattern that does not
reproduce the occupancy channel of section 8.1. None of that exists in this
repository, and no object in this model approximates any of it.

### 8.5 The composed leakage verdict

Using the [`../PRIVACY_MODES.md`](../PRIVACY_MODES.md) template for the
settlement graph row: per-claim public is `public`; netted public is
`public` at owner granularity; the named agent is `shielded`; `hidden` has
no implementation. INFERRED, restated as the packet's conclusion: a
hypothetically perfect Dark computation composed with either public surface
is pre-trade dark at most — and with a two-asset public custody not even
fully that, because deposits disclose sides before the cutoff. Account
identity is a separate axis: replacing account labels with pseudonyms
relabels the reconstruction's owner column without shrinking it, and
external correlation of pseudonyms is exactly the settlement-graph threat
`DARK_RELATION_THREAT_MODEL.md` lists.

REJECTED as claims, per `PRIVACY_MODES.md` section 6: "settled on-chain
means done" (an obligated position can strand), "netted means private"
(section 8.2 is the owner-delta vector), "the pool is an exchange balance"
(it is a modelled map), and any use of "Dark" for a surface in this packet.

## 9. Falsifier ledger

Against the threat cases of
[`DARK_RELATION_THREAT_MODEL.md`](DARK_RELATION_THREAT_MODEL.md):

| Falsifier | Disposition |
|---|---|
| Public settlement reconstructs the participant graph | **Measured, and confirmed as designed-in for public surfaces.** Section 8.1 is that falsifier executed deliberately: the per-claim surface yields owner, side, fill, reservation, occupancy, and rank. Any composed claim of end-to-end darkness over such a surface is falsified by construction. |
| Local-fill delivery leaks through a public notification | **Partially inherited.** Local outputs stay sealed, but a settlement *claim* is a public act that discloses the claimer's position and amounts on public surfaces; claiming is the notification. |
| Retries or abort timing reveal whether a batch crossed | **Open, and widened.** Claim epochs, refund epochs, and the settled-versus-refunded shape of the transcript are public here and unanalysed as a timing channel. |
| An unavailable payload becomes an empty slot | **Unchanged upstream**; settlement adds no subset path — an unsettleable position refuses, it never shrinks the batch. |
| The audit plane silently decrypts | Out of scope: this model has no audit plane; the named-agent surface names its viewer instead. |

Against `DARK_FBA_RELATION.md` section 10's obligations, the rows this packet
touches:

- "quote and base deltas conserve with zero fees" — **met and measured** at
  the settlement layer, per position and per batch, in every order.
- "no invalid output can settle" — **not met**, measured; section 7.
- "inclusion, equivocation, withholding, timeout, retry, and abort have
  distinct receipts" — extended: settlement adds typed authorization
  defects, idempotent-retry answers, custody errors, observe errors, refund
  refusals, and surface refusals, every one named by a test.

## 10. Open, and named

1. **The pre-admission window.** Escrow is recorded at submission, before
   admission; an escrow whose submission the log refuses has no release rule
   here (the model escrows only admitted submissions). A real relation needs
   either escrow-after-admission or a non-inclusion refund path, and
   non-inclusion proofs are an object the inclusion lane does not currently
   export.
2. **Omission adjudication.** A settled batch with an omitted position
   strands that reservation as `Obligated`: verdict, no funds. The missing
   object is an adjudication rule consuming the content-addressed
   `OmissionProof` verdict — which immediately meets the attribution gap,
   since a verdict without attribution names a contradiction, not a payer.
3. **Attribution, inherited verbatim.** Nothing signs. Insolvency lands on
   the last claimant, not on the forger; a slashing or make-whole rule has
   no one to charge. This is the same first-open-item as both upstream
   packets, now with funds attached.
4. **The equivocation refund path is not re-driven here.** The model treats
   `equivocation` as a terminal refundable abort and relies on the upstream
   lane's verified once-per-nullifier sweep; no second holder history is
   constructed in this crate's tests.
5. **Timing channels.** Claim order, claim epochs, and retry patterns are
   public and unanalysed; correlating them with owner identity is untouched.
6. **The deposit side channel.** Closing "deposits leak the side" needs
   single-collateral custody, both-asset padding deposits, or shielded
   custody — each a relation change with its own leakage table, none
   modelled.
7. **Multi-batch custody, netting sets, and cross-batch nullifier scope.**
   This model is one batch; reusing custody across batches re-opens every
   linkage question section 8 measured, plus nullifier-scope questions the
   admission relation's batch-scoped uniqueness does not answer.
8. **Chain binding.** The pool is a map. Binding custody availability to a
   real ledger — the Solana adapter `ARCHITECTURE.md` section 5 anticipates
   — is implementation work owned elsewhere (Dragon's Clutch for transparent
   protocol code), consuming this specification rather than extending this
   model.

## 11. Reproduction and bounds

From the repository root:

```sh
cargo test --offline --locked \
  --manifest-path experiments/settlement-model/Cargo.toml
cargo clippy --offline --locked --all-targets \
  --manifest-path experiments/settlement-model/Cargo.toml -- -D warnings
cargo fmt --check \
  --manifest-path experiments/settlement-model/Cargo.toml
cargo run --quiet --offline --locked \
  --manifest-path experiments/settlement-model/Cargo.toml \
  --bin degg-settlement-vectors \
  | cmp - experiments/settlement-model/vectors/v1.txt
```

VERIFIED on 2026-08-18: 56 tests pass — 9 custody, 22 authorization, 8
settlement, 6 refund, 3 residual inheritance, 6 surface, 2 corpus. Clippy
with `-D warnings` and `cargo fmt --check` are clean. Zero third-party
dependencies; the only dependencies are unmodified path dependencies on the
three landed experiments, whose own suites and corpora are untouched.

Bounds of the VERIFIED label, stated exactly:

- the custody state machine is checked by direct unit witnesses for every
  transition and every refusal class, including no-partial-application on an
  insolvent payout;
- the frozen authorization order is checked by one witness per defect class
  plus two explicit priority pairs (refusal-before-phase,
  padding-before-opening); it is not checked by exhaustive pair enumeration
  as `dark-fba-independent` does for admission;
- solvency and conservation are exhaustive over all 24 settlement orders of
  the four-position corpus book, at every prefix, on the honest run only;
- the residual-trust inheritance is three constructed adversarial runs
  (substitution, single-entry forgery, omission), not an enumeration of the
  executor's full freedom — the enumeration lives upstream in
  `SHIELDED_BASELINE.md` section 6.2;
- the reconstruction measurement is over the corpus book with exact
  worst-case reservations and one position per account; the multi-position
  join and slack reservations are untested;
- everything else — larger books, multi-batch custody, adversarial claim
  schedules, real assets, any cryptographic property — is covered by no
  statement in this document.

Corpus byte identity: `experiments/settlement-model/vectors/v1.txt` SHA-256
`f090d751d78c217c5a1405a17375e1864da75d2adfc70fc4e0aa89bca62547c0`.

Validation toolchain: `rustc 1.98.0-nightly (91fe22da8 2026-06-21)`,
`cargo 1.98.0-nightly (a595d0da2 2026-06-20)`.

## 12. Provenance

Original work in this repository. The custody split, settlement-nullifier
derivation, execution-digest construction, surface projections, and
reconstruction are specified and implemented here; the tagged hash, cutoff
root, inclusion receipts, abort machine, reserve ledger, relation module,
receipts, and delivery commitments are consumed from the three sibling
experiments by path dependency and none was modified. No code, fixture,
constant, or serialization format crossed from any other repository. Because
the author has read related implementations in sibling repositories —
including Minidregg's sealed-receipt and public-settlement separation and
Dragon's Clutch's transparent protocol code — this document does not claim
clean-room status.
