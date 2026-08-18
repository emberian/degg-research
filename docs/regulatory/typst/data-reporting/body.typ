#import "../shared/template.typ": key_point, note_ref

= Executive summary

*SOURCED:* This comment addresses Questions 3, 8, and 19 of the joint request. Those
questions concern blockchain reporting, public identity and strategy leakage,
and machine-readable reporting logic.#note_ref(1)

*PROPOSED:* For Clear and regulator-observable Shielded systems, the central
recommendation is to define three different records rather than forcing one
public ledger event to serve every purpose:

1. a *public transparency record* containing fields justified by price discovery
   and public market transparency;
2. an *exact confidential regulatory record* containing the owner-linked order,
   funding, execution, position, and settlement lifecycle needed for supervision,
   examination, and enforcement; and
3. a *machine-verifiable validation package* containing versioned schemas,
   deterministic transition rules, conformance vectors, provenance bindings,
   and proof requirements.

#key_point("Direct answers", [
  *Question 3:* Report normalized economic lifecycle events, not transaction
  hashes alone, and add chain-specific provenance and finality fields.  \
  *Question 8:* Treat public dissemination as a purpose-limited leakage policy;
  keep the confidential regulatory record exact even when public fields are
  capped, delayed, bucketed, or aggregated.  \
  *Question 19:* Publish governed, versioned, machine-readable specifications
  with test vectors and historical replay. Do not let one opaque implementation
  silently become the law.
])

*INFERRED:* Blockchain records can strengthen provenance. They do not automatically reveal
beneficial ownership, product semantics, rejected or canceled instructions,
offchain modifications, allocation, or the meaning of a fork, upgrade, or
application-level dispute. Conversely, publishing every regulatory field on a
public ledger can expose a participant's identity, inventory, hedge, execution
policy, and relationships even when names are omitted.

= Question 3: report economic events, not ledger writes

== A normalized lifecycle

*PROPOSED:* A reporting adapter should distinguish at least the following states when they
are economically relevant under the applicable reporting framework:

- policy or order creation;
- signature and authorization;
- funding or collateral lock;
- receipt, acceptance, rejection, modification, and cancellation;
- batch open and close, admission sequence, accepted-set inclusion or exclusion,
  duplicate or nullifier disposition, and data-availability status;
- match, execution, and allocation;
- candidate, disputed, pending, unsupported, or expired resolution;
- finality or resolution certification;
- early exit, compression, unilateral close, unwind, cancellation, or replacement;
- settlement, delivery, transfer, maturity termination, or correction;
- proof or verifier result and its exact statement version; and
- replacement or reversal caused by ledger reorganization or an application
  dispute process.

*INFERRED:* These states are not interchangeable. A transaction may fail before state
change. A submitted commitment may remain pending. Execution may precede final
settlement. A ledger observation may later be superseded. Mapping every
transaction hash directly to "trade" creates false positives and omits essential
lifecycle facts.

*PROPOSED:* The reporting unit should therefore be a normalized economic event with an exact
link to the underlying source event. Where several ledger instructions jointly
implement one reportable event, the adapter should bind the complete source set.
Where one ledger transaction contains several reportable events, the adapter
should identify them separately.

== Chain-specific provenance

*PROPOSED:* Onchain reporting should add fields necessary to interpret the economic event,
not replace ordinary economic and counterparty fields.

#table(
  columns: (1.75in, 1fr),
  table.header([*Field*], [*Purpose*]),
  [Network, chain, and deployment identifier], [Disambiguate ledgers, forks, environments, and redeployments.],
  [Block or slot height and block hash], [Anchor the observed state to a particular history.],
  [Transaction and intra-block index], [Establish ordering at the ledger layer.],
  [Instruction, log, or event index], [Identify the exact transition inside a transaction.],
  [Program address, executable hash, upgrade authority, proxy or program-data address, configuration version, and activation height], [Bind the event to executable logic and its mutability history rather than a mutable name.],
  [Interface and schema version], [Interpret encoded fields and reject incompatible decoders.],
  [Product or rule identifier and terms digest], [Bind the event to its economic semantics.],
  [Accepted-input or batch root], [Bind an aggregate execution to the set admitted for processing.],
  [Prior and resulting state commitments], [Support continuity and correction history.],
  [Finality status and finality observation time], [Separate observed, confirmed, economically final, and protocol-final states.],
  [Reorganization or replacement reference], [Link superseded events without silent deletion.],
  [Fee, payer, and fee-class fields], [Separate infrastructure and execution economics from product value.],
  [Confidential account-to-party linkage], [Preserve ownership continuity for regulators without public identification.],
)

Guidance should specify how to report ledger time, local receipt time, ordering,
economic execution time, and finality time when they differ. It should also
specify corrections for probabilistic finality, chain reorganization, protocol
upgrade, cross-chain execution, and application-level disputes.

== Commitments, proofs, and retained evidence

*PROPOSED:* Where records are encrypted or represented by commitments, every
reportable event should bind to the accepted report payload, the schema used to
interpret it, and the authorized correction history. Separate controls must
address completeness, inclusion and exclusion, censorship, custody, retention,
and availability. Proofs can establish useful facts,
including authorized signer, valid schema, collateral sufficiency, deterministic
matching, position-limit compliance, conservation, consistency between public
and confidential projections, and absence of duplicate consumption.

A proof is not automatically the evidence needed for examination. It proves only
the proposition encoded by its statement, relative to named committed inputs,
rule and program versions, ledger state, and external facts supplied to the
relation, under the stated assumptions. Unless
the Commissions determine that a proof is itself the required datum, the system
should preserve the committed source evidence needed to reconstruct the event.
The regulatory record should also identify the proof-system and statement
version, verifier implementation or hash, setup parameters where applicable,
public-input digest, verification result and time, correction linkage, and the
retention location and access policy for supporting evidence. An authorization
proof does not establish current legal authority without key and revocation
governance; a collateral proof does not establish custody or lack of encumbrance
unless those facts are inside the relation and bound to the actual settlement.

= Question 8: public transparency without involuntary strategy publication

== Identity and policy leakage

*INFERRED:* Removing a name does not necessarily remove identity or strategy leakage. A
combination of public wallet, timestamp, exact size, price, product, collateral
movement, bridge transfer, and settlement destination can re-identify a trader.
Repeated behavior can reveal entry policy, inventory limits, hedge timing,
liquidation avoidance, or an algorithm's response kernel. Sparse or bespoke
markets heighten this risk because an otherwise anonymous event can be unique.

*PROPOSED:* Public dissemination should be defined as a frozen, purpose-limited leakage
function. Each public field should have a stated transparency objective and be
tested against re-identification and strategy-inference risk. Questions should
include:

- whether exact notional must be public immediately or whether caps and buckets
  preserve useful transparency for unusually large or sparse trades;
- whether precise sub-second timing is necessary or whether a short aggregation
  window reduces linkage without harming price discovery;
- which wallet addresses, transaction hashes, and settlement links help public
  understanding and which primarily expose identity;
- whether related executions may be aggregated without concealing manipulation;
- how a correction, cancellation, or reorganization is linked without producing
  a misleading tape; and
- how activity level, participant count, product bespoke-ness, and observable
  settlement graphs affect the leakage analysis.

Any cap, delay, bucket, or aggregation policy should be explicit, deterministic,
versioned, and empirically reviewed. The exact confidential record should remain
timely and complete.

Official dissemination is only one leakage surface. A public mempool, fee payer,
funding graph, failed instruction, relayer path, transaction trace, public token
account, or settlement graph can reveal information before or beyond the
regulated tape. Evaluation should measure both the dissemination policy and the
independently observable base-chain and application-layer leakage.

== Separate outputs by purpose

#table(
  columns: (1.15in, 1.7in, 1fr),
  table.header([*Data class*], [*Public transparency record*], [*Confidential regulatory record*]),
  [Product], [Stable identifier and public terms], [Full terms, classification, source, rule, and version history],
  [Execution], [Time under stated precision; price and volume under stated policy], [Exact receipt, ordering, execution, allocation, and correction times],
  [Participant], [No direct owner mapping unless independently required], [Verified owner, controller, account, and relevant identifier linkage],
  [Ledger], [Network and finality facts needed to interpret execution], [Exact accounts, instructions, funding sources, and settlement graph],
  [Order lifecycle], [Fields justified by transparency policy], [Receipt, modification, cancellation, rejection, match, allocation, and disposition],
  [Strategy-sensitive state], [Minimized under the declared leakage policy], [Exact protected state where required for supervision],
  [Proof], [Integrity or public-consistency proof where useful], [Proof plus committed evidence, schema, and opening or validation material],
)

= Privacy modes must not be conflated

*PROPOSED (local terminology):* This research uses three precise terms:

- *Clear*: the specified state and computation are public.
- *Shielded*: a named executor, committee, or auditor may learn private inputs.
- *Dark*: no actor learns anything beyond the frozen leakage function and its own
  authorized local output, within an explicit corruption model.

An end-to-end claim under this taxonomy must also state the topology, assumptions,
ingress, intermediate-state, proof-production, settlement, timing, and
availability leakage. A due-process threshold opening path is regulator-observable Shielded, not Dark.
An operator-readable encrypted database is also Shielded. Unpadded ingress,
timing, participant count, public account-and-amount settlement, or a plaintext
proof producer can also make a purportedly Dark design Shielded. The distinction matters
because each architecture provides different evidence, incident response,
availability, and lawful-access properties.

The three privacy modes include two materially different Shielded architectures:
operator-readable Shielded and threshold-disclosure Shielded. Clear and these
regulator-observable Shielded patterns can be evaluated against applicable confidential
reporting objectives by inspecting their data and governance paths. A true Dark
architecture presents a narrower research question: whether fixed encrypted
compliance queries and bounded leakage can satisfy each applicable reporting,
correction, examination, and enforcement objective without a general opening
capability. That should be tested obligation by obligation rather than answered
by the word "private."

= Question 19: governed machine-readable rules

*PROPOSED:* The Commissions should publish machine-readable reporting structures, but treat
them as governed specifications rather than an opaque executable or substitute
for controlling legal text.

== Minimum package

1. *Normative semantic model.* Define economic events, lifecycle states, field
   meanings, valid transitions, and correction history independently of a chain
   or vendor.
2. *Versioned schemas.* Specify types, exact units, integer bounds, enumerations,
   optionality, identities, references, and cross-field constraints.
3. *Deterministic validation logic.* Provide small, reviewable predicates for
   syntax and semantic consistency.
4. *Conformance vectors.* Publish valid and invalid records, boundary integers,
   lifecycle sequences, reorganizations, batching, ambiguous timestamps, and
   corrections.
5. *Reference implementations.* Provide reproducible, non-normative code in more
   than one implementation where feasible.
6. *Differential and property testing.* Require implementations to agree across
   randomized and adversarial corpora, with exact arithmetic for prices,
   quantities, fees, and conservation-sensitive values.
7. *Version and effective-date binding.* Bind each report to the exact schema and
   rule version and retain historical versions for replay.
8. *Public change control.* Publish proposals, rationale, migration rules,
   examples, and interpretation procedures.
9. *Failure taxonomy.* Standardize rejected, pending, unsupported, expired,
   corrected, reorganized, and backend-unavailable states.
10. *Human-readable parity.* Map each rule provision to its data elements,
    validators, examples, and controlling text.

== Guarded updates and candidate states

*VERIFIED (local research object):* A guarded update can bind a later report to an eager report identifier, actor,
field, predicate version, and permitted transition. It can fail closed when a
late value violates the schema or a cross-field constraint. Candidate-result
logic can preserve pending or alternative provenance-bearing states rather than
inventing finality.

*INFERRED:* These are useful reporting design patterns, not a proposal that the Commissions
adopt a particular research calculus. Their value is narrower: they make state
transitions, authority, failure, and ambiguity explicit. A finality or correction
certificate should be required before one candidate becomes the final regulatory
state, and rejected or superseded transitions should remain auditable without
being counted as trades.

= Governance and pilot design

*PROPOSED:* A machine-readable regime should be tested against at least these failures:

- code and controlling text diverge;
- new schemas are applied retroactively;
- validators reject valid records or accept incomplete ones;
- implementations disagree about units, time, optionality, or correction;
- public fields enable unexpected re-identification;
- private-data keys are lost, compromised, or used beyond authorization;
- a fixed encrypted compliance query proves too narrow for a later investigation;
- proof assumptions, circuits, or setup parameters change;
- chain finality or reorganization semantics are misunderstood; and
- historical-version maintenance disproportionately burdens smaller entities.

*PROPOSED:* Before mandating a machine-readable structure, the Commissions should conduct a
public conformance pilot involving reporting entities, repositories, market-data
users, privacy researchers, cryptographers, and smaller implementers. The pilot
should publish acceptance and rejection rates, correction latency, disagreement
cases, implementation cost, performance, and privacy measurements.

= Requested actions

*PROPOSED:* I respectfully request that the Commissions:

1. define normalized lifecycle events for onchain transactions;
2. require chain provenance and explicit finality or correction semantics where
   necessary to interpret those events;
3. separate public, confidential regulatory, and machine-verifiable records;
4. treat public dissemination as a documented leakage policy and measure its
   identity and strategy effects;
5. recognize the difference between regulator-observable Shielded systems and
   systems whose declared Dark leakage has no general opening path;
6. state when a proof can satisfy a reporting element and when retained source
   evidence remains required; and
7. publish versioned schemas, validators, test vectors, and change-control rules
   with human-readable parity.

= Limits of this comment

*VERIFIED (document scope):* This comment does not argue for reduced regulatory access merely because a
transaction is onchain. It does not claim that a transaction hash is a complete
report, that zero knowledge eliminates recordkeeping, or that a presently
available Dark architecture satisfies existing reporting obligations. It does
not ask the Commissions to adopt a particular proof system, blockchain, or local
research formalism.

= Conclusion

*INFERRED:* Onchain provenance, confidential regulatory data, public transparency, and
machine-verifiable rules are compatible only when their boundaries are explicit.
"Onchain" should not mean "already reported." "Public" should not mean "publish
every strategy-revealing field." "Encrypted" should not mean "available to no
lawful process." "Machine-readable" should not mean "an opaque program silently
becomes law."

*PROPOSED:* The Commissions should standardize normalized lifecycle events and
chain provenance, preserve an exact confidential regulatory record for Clear and
regulator-observable Shielded systems, minimize unnecessary identity and strategy
leakage while preserving the transparency required for price discovery,
surveillance, and market integrity, and publish governed machine-readable
specifications with conformance vectors. A true Dark system should be evaluated
separately against each obligation rather than assumed to possess a complete
confidential record that its leakage contract intentionally does not expose.

#block(breakable: false)[
  #v(18pt)
  Respectfully submitted,

  [FULL NAME]  \
  [AFFILIATION, IF ANY]  \
  [DATE]
]
