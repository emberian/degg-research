# Guarded Holes and Partial Economic Computation

Status: technical dossier plus regulatory research questions. Not legal advice.

## Technical object

A weak guarded hole is a late-filled position whose complete transition shape is
fixed before the late value arrives.

In the inspected Dregg/Minidregg work, eager data can include:

- logical value type and codec;
- turn and pre-state root;
- authority demand;
- state footprint;
- guard commitment;
- effect commitment;
- deadline;
- continuation;
- nullifier/replay domain.

Late advice contributes only a value of the already-selected type. Admission is
total and fail-closed. It distinguishes:

- accepted;
- semantically rejected;
- pending;
- expired;
- unsupported;
- backend unavailable.

A successful fill is bound to the exact eager shape, and durable consumption is
one-shot.

## Weak versus strong

Weak:

- economic/effect shape is eager;
- only a witness or value is late;
- conservation and authority are already bounded;
- compatible with a proof obligation or asynchronous service.

Strong:

- later input may choose an undetermined economic delta, authority shape, or
  conservation consequence;
- effectively permits a future party to define the transaction;
- deliberately inexpressible in the inspected Dregg design.

This distinction is important technically and legally. Eager shape makes risk
and authority analyzable, but does not by itself determine instrument
classification.

## Why “partial computation” may still be economically complete

A program can be incomplete as a computation while already creating a complete
economic arrangement. At time t0 the parties may have fixed:

- consideration;
- collateral;
- eligible future states;
- payout mapping;
- expiry and fallback;
- transferability;
- the predicate or oracle;
- who may submit the witness.

Only the world-state witness remains unknown. Economically, that can resemble a
contingent claim even though operationally it is a hole awaiting a value.

Conversely, many guarded holes are not market instruments:

- a delayed document attachment;
- a service callback;
- a proof of completed work;
- a capability handoff with no speculative payoff;
- a deterministic computation continuation with no transferable funded right.

The correct classification question is not “is a guarded hole an event
contract?” in the abstract. It is “which factual configurations of a guarded
hole create the rights and economic exposure of an event-contingent contract?”

## Factual matrix for each use

Record:

| Field | Question |
|---|---|
| Parties | Who creates, holds, fills, verifies, and benefits? |
| Consideration | What is paid or locked, by whom, and when? |
| Reference | What future fact, event, value, or computation matters? |
| Right | What can the holder demand if the guard is satisfied? |
| Obligation | Who owes value or performance? |
| Payout | Fixed, binary, categorical, linear, path-dependent, or service-only? |
| Risk | Can either party gain or lose from the contingency? |
| Transfer | Can the hole/right be assigned, traded, bundled, or collateralized? |
| Term | Expiry, repair, ambiguity, and non-fill outcome? |
| Discretion | Can anyone change terms or choose among outcomes? |
| Venue | Is there multilateral order interaction or matching? |
| Custody | Who holds collateral or controls settlement? |
| Fees | Who earns from creation, execution, fill, or settlement? |
| Operation | Who deploys, upgrades, curates, routes, or promotes? |

## Questions for the CFTC

1. Which economic facts distinguish an ordinary guarded software promise from
   an event-contingent contract within the Commission's jurisdiction?
2. Does fixing the entire payout/effect shape before the future witness arrives
   affect classification, or only risk and compliance analysis?
3. How should a protocol be analyzed when a partial computation represents a
   set of possible terminal results, but every reachable result is fully
   prefunded and mechanically settled?
4. Is transferability necessary, important, or irrelevant to the classification
   of the underlying arrangement?
5. How should non-tradable bilateral service escrows be distinguished from
   fungible or multilateral claims over the same predicate?
6. When a general-purpose document/proof calculus can express both mundane
   callbacks and economically contingent fills, should analysis attach to each
   application/term set rather than the base protocol?
7. Which actor is the operator when the base calculus is published as software,
   an application freezes terms, permissionless parties fill/settle, and no
   party can alter the result?
8. What pre-deployment process should a developer use to obtain clarity for a
   bounded, testable factual matrix without asking the Commission to classify an
   entire general-purpose programming model?

## Research position

PROPOSED: The most useful submission does not argue that formal soundness
creates a legal exemption. It argues that eager shape, full prefunding, exact
settlement, no debt, no liquidation, and fail-closed ambiguity are technically
meaningful risk-reducing facts—and asks the Commission how those facts map to
classification and registration.

The protocol's generality cuts both ways. It demonstrates that contingent
computation is a foundational software primitive, not synonymous with gambling.
It also means an economically event-contingent application cannot evade analysis
by calling its payout a fill.

