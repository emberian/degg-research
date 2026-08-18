#import "../shared/template.typ": key_point, note_ref

= Executive summary

The Commissions ask how swap and security-based swap reporting should treat
blockchain-native execution, what public dissemination should reveal when the
underlying ledger is already public, and whether reporting logic should be
published in machine-readable form.#note_ref(1) I write in response to
Questions 3, 8, and 19, and take no position on the other questions.

I am a software and formal-methods researcher. I study staged programmable
commitments: programs whose records come into existence at different moments,
in different places, for different audiences. This comment places one concrete
design on a public ledger and asks, at each milestone of its life, three
reporting questions: which records exist, who can already read them, and what
a machine could check about them.

The design is a fully collateralized conditional-asset market over an
objectively verifiable onchain price band. A depositor locks collateral into a
segregated pool that belongs to one market only and receives one claim for
each cell of an exhaustive, disjoint partition of the possible outcomes.
Claims trade through a batch venue with a stated close. A deterministic
observation program, frozen when the market is created, later identifies the
realized cell, and settlement pays that cell's claims from the pool. I have
built an offline research prototype of the core accounting for this design. It
is not a deployed system, a product, or an offer, and nothing in this comment
asks either Commission to approve it. I use it here only as a lens: it makes
the reporting questions concrete.

Walking that market from creation to settlement shows the same fact at every
milestone: three different consumers need three different records, and no
single ledger event serves all of them. For Clear and regulator-observable
systems, define three records rather than forcing one public ledger event to
serve every purpose:

1. a *public transparency record* containing the fields justified by price
   discovery and public market integrity;
2. an *exact confidential regulatory record* containing the owner-linked
   order, funding, execution, position, and settlement lifecycle needed for
   supervision, examination, and enforcement; and
3. a *machine-verifiable validation package* containing versioned schemas,
   deterministic transition rules, conformance vectors, and provenance
   bindings.

#key_point("Direct answers", [
  *Question 3:* Report normalized economic lifecycle events, not transaction
  hashes alone, and add chain-specific provenance and finality fields.  \
  *Question 8:* Treat public dissemination as a purpose-limited leakage
  policy; keep the confidential regulatory record exact and timely even when
  public fields are capped, delayed, bucketed, or aggregated.  \
  *Question 19:* Publish governed, versioned, machine-readable specifications
  with test vectors and historical replay. Do not let one opaque
  implementation silently become the law.
])

Blockchain records strengthen provenance. They do not automatically reveal
beneficial ownership, product semantics, rejected or canceled instructions,
offchain modifications, allocation, or the meaning of a fork, upgrade, or
application-level dispute. In the other direction, publishing every regulatory
field on a public ledger can expose a participant's identity, inventory,
hedge, execution policy, and relationships even when names are omitted. Both
failures come from the same mistake: treating one ledger event as if it were
every record at once.

= The worked example, viewed as records

At each milestone, ask which records exist, which are public bytes, and which
exist only if somebody is required to keep them.

*Publication.* The market template is created on the ledger: the partition
rule, the version of the frozen observation program, the batch policy, the
settlement terms. The public bytes are a creation transaction, a program
address, an executable hash, and a digest of the terms. Everything about this
milestone is public, and none of it is reportable as a transaction: there are
no parties, no funding, and no exposure. A machine-readable rule can already
validate something real --- that the terms digest matches the published terms,
that the schema version is one it recognizes --- but a reporting adapter that
mapped this ledger write to "trade" would invent a trade with no participants.
Ledger writes precede reportable events.

*Funding.* A wallet deposits one unit of collateral into the market's
segregated pool and receives a complete set: one claim for each of the five
bands. The public bytes are precise --- pool address, wallet address, amount,
time. They are not an identification: nothing on the ledger says who controls
the wallet. And they are not a contingent position: a complete set plus the
right to recombine it into its collateral is fully hedged, interchangeable
with the deposit itself. Contingent exposure arises only later, when the
depositor sells some claims and keeps others --- and that sale may occur in a
venue whose individual orders never touch this ledger. A hash-to-trade mapping
records exposure here that does not exist and misses the transaction where
exposure actually arises. Supervision needs a confidential account-to-party
linkage, so the deposit, the later orders, and the eventual redemption read as
one owner's lifecycle. A machine can check conservation: collateral in equals
complete sets out.

*Close and match.* Orders accumulate until the stated close; at close the
submitted book freezes and a deterministic rule clears it at one consistent
set of prices. The ledger can carry little of this: perhaps a root committing
to the accepted orders, the clearing prices, and aggregate fills. The records
market-conduct examination cares most about --- instructions rejected before
admission, modifications, cancellations, the sequence in which orders
arrived --- may never become ledger bytes at all, because they never changed the
ledger's state. They are reported from the venue's own records or they are not
reported. In the other direction, this milestone is where machine verification
is strongest. Because the clearing rule is deterministic over a frozen book,
anyone holding the book can recompute the result. In my research prototype,
I built the batch verifier to accept a submitted clearing only if
recomputation from the frozen book reproduces it exactly, never trusting the
submitter's claimed quantities. A reporting rule with that shape is more than a format check: the
rule is the check.

*Finality.* Between the close of trading and the close of the observation
window, the honest description of the market is a set of candidates: five
possible settlements, each fully specified, exactly one of which will be
authorized. Reporting vocabulary should be able to say this without
pretending. Pending, disputed, unsupported, and expired are distinct states;
none of them is "resolved," and none is an error. When the frozen observation
program accepts qualifying evidence and the repair period lapses, one
candidate becomes the certified outcome. Even then, a ledger reorganization or
an application-level dispute can supersede what was observed. The record needs
a correction linkage that preserves the superseded event, not a silent
overwrite.

*Settlement.* The realized cell's claims redeem from the pool; the other four
expire worthless. The public bytes show outflows from the pool. They do not
show which redemption closes which position of which owner at what gain --- the
fact an examiner reconstructing a lifecycle actually needs --- and no
transparency purpose requires that they show it. The confidential record links
them; the public record need not.

The walk yields the executive summary's three records: enough public fields to
trust prices; the exact owner-linked lifecycle, including events the ledger
never saw, for the regulator; and a deterministic validation rule for the
machine. The Commissions' three questions take them in turn.

= Question 3: report economic events, not ledger writes

== A normalized lifecycle

The walk shows why the reporting unit must be a normalized economic event with
an exact link to its source, not a transaction hash. A reporting framework for
onchain markets should distinguish at least the following states when they are
economically relevant under the applicable reporting rules:

- policy or order creation;
- signature and authorization;
- funding or collateral lock;
- receipt, acceptance, rejection, modification, and cancellation;
- batch open and close, admission sequence, accepted-set inclusion or
  exclusion, duplicate or nullifier disposition, and data-availability status;
- match, execution, and allocation;
- candidate, disputed, pending, unsupported, or expired resolution;
- finality or resolution certification;
- early exit, compression, unilateral close, unwind, cancellation, or
  replacement;
- settlement, delivery, transfer, maturity termination, or correction;
- proof or verifier result and its exact statement version; and
- replacement or reversal caused by ledger reorganization or an application
  dispute process.

These states are not interchangeable. A transaction may fail before any state
change. A submitted commitment may remain pending. Execution may precede final
settlement. A ledger observation may later be superseded. Mapping every
transaction hash to "trade" creates false positives at funding and omissions
at the close, exactly as the worked example shows. Where several ledger
instructions jointly implement one reportable event, the report should bind
the complete source set; where one transaction contains several reportable
events, each should be identified separately.

== Chain-specific provenance

Onchain reporting should add the fields necessary to interpret the economic
event. It should not replace ordinary economic and counterparty fields.

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

Guidance should specify how to report ledger time, local receipt time,
ordering, economic execution time, and finality time when they differ. It
should also specify correction behavior for probabilistic finality, chain
reorganization, protocol upgrade, cross-chain execution, and application-level
disputes.

== Commitments, proofs, and retained evidence

Where records are encrypted or represented by commitments, every reportable
event should bind to the accepted report payload, the schema used to interpret
it, and the authorized correction history, with separate controls for
completeness, inclusion and exclusion, censorship, custody, retention, and
availability. Proofs can establish useful facts: an authorized signer, a valid
schema, collateral sufficiency, deterministic matching, position-limit
compliance, conservation, consistency between public and confidential
projections, and the absence of duplicate consumption.

A proof is not automatically the evidence an examination needs. It establishes
only the proposition its statement encodes, relative to named committed
inputs, rule and program versions, ledger state, and whatever external facts
were supplied to the relation, under the stated assumptions. An authorization
proof does not establish current legal authority without key and revocation
governance. A collateral proof does not establish custody or absence of
encumbrance unless those facts are inside the relation and bound to the actual
settlement. Unless the Commissions determine that a proof is itself the
required datum, the committed source evidence needed to reconstruct the event
should be preserved, and the regulatory record should identify the proof
system and statement version, the verifier implementation or its hash, setup
parameters where applicable, the public-input digest, the verification result
and time, correction linkage, and the retention location and access policy for
the supporting evidence.

= Question 8: public transparency without involuntary strategy publication

== Identity and policy leakage

Return to the funding milestone. The depositor's wallet, the exact amount, and
the time are public bytes, permanently. Removing a name does not necessarily
remove identity or strategy leakage: a combination of public wallet,
timestamp, exact size, price, product, collateral movement, bridge transfer,
and settlement destination can re-identify a trader. Repeated behavior can
reveal entry policy, inventory limits, hedge timing, liquidation avoidance, or
an algorithm's response to market conditions. Sparse or bespoke markets
heighten the risk, because an otherwise anonymous event can be unique. In the
worked example, the batch conceals individual orders, but a settlement graph
that pays redemptions directly to the original wallets can undo that
concealment after the fact. I offer these as risk analysis --- reasoning about
what public fields make inferable --- not as measurements of any real market;
the basis I do have is described below.

Public dissemination should therefore be defined as a frozen, purpose-limited
leakage policy. Each public field should have a stated transparency objective
and should be examined against re-identification and strategy-inference risk.
The questions to ask include:

- whether exact notional must be public immediately, or whether caps and
  buckets preserve useful transparency for unusually large or sparse trades;
- whether precise sub-second timing is necessary, or whether a short
  aggregation window reduces linkage without harming price discovery;
- which wallet addresses, transaction hashes, and settlement links help public
  understanding, and which primarily expose identity;
- whether related executions may be aggregated without concealing
  manipulation;
- how a correction, cancellation, or reorganization is linked without
  producing a misleading tape; and
- how activity level, participant count, product bespoke-ness, and observable
  settlement graphs change the analysis.

Purpose-limited publication is not a novelty: the real-time public reporting
rules already delay the public print of certain large trades and cap the
disseminated notional.#note_ref(2) What is new for onchain markets is the
number of leakage surfaces outside the regulated tape. A public mempool, a fee
payer, a funding graph, a failed instruction, a relayer path, a transaction
trace, a public token account, or a settlement graph can reveal information
before and beyond official dissemination. Evaluation should measure both the
dissemination policy and the independently observable base-chain and
application-layer leakage.

None of this loosens the confidential record. The lifecycle reporting
framework already expects exact creation and continuation data with
counterparty and transaction identifiers,#note_ref(3) held under repository
confidentiality and access rules.#note_ref(4) Any cap, delay, bucket, or
aggregation policy for the public record should be explicit, deterministic,
versioned, and empirically reviewed, and the exact confidential record should
remain timely and complete however conservative the public record becomes.

== What I have measured, and what I have not

The empirical basis behind my leakage statements is deliberately modest. I
have built a small deterministic laboratory that replays four synthetic
trading traces and records, for each of three transcript designs --- a fully
public one, one with a named executor who sees private inputs, and one
hypothetical design with a fixed disclosure budget --- which fields the design
mechanically reveals and which deductions those fields enable, keeping the two
categories separate. It is transcript bookkeeping over synthetic data. It
measures no anonymity, no cryptographic leakage, no timing behavior, no
endpoints, and no real market. Its value is only that it forces every "this
design hides X" sentence to be checked against an explicit list of what the
transcript actually contains. I recommend the same discipline for any proposed
dissemination policy: enumerate the fields, then defend each one.

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

= Privacy architectures must not be conflated

My research uses three terms exactly; conflating them produces bad reporting
policy in both directions:

- *Clear*: the specified state and computation are public.
- *Shielded*: a named executor, committee, or auditor may learn private
  inputs.
- *Dark*: no actor learns anything beyond a frozen leakage function and its
  own authorized local output, within an explicit corruption model.

An end-to-end claim under this taxonomy must also state its topology and
assumptions and its ingress, intermediate-state, proof-production, settlement,
timing, and availability leakage. A due-process threshold opening path is
regulator-observable Shielded, not Dark. An operator-readable encrypted
database is Shielded. Unpadded ingress, observable timing, a small participant
count, a public account-and-amount settlement, or a plaintext proof producer
can each make a purportedly Dark design Shielded in fact. The distinction
matters for reporting because each architecture provides different evidence,
incident-response, availability, and lawful-access properties.

Clear systems and regulator-observable Shielded systems --- whether
operator-readable or threshold-disclosure --- can be evaluated against
confidential reporting obligations by inspecting their data and governance
paths. A true Dark architecture presents a narrower research question: whether
fixed encrypted compliance queries and bounded leakage can satisfy each
applicable reporting, correction, examination, and enforcement objective
without a general opening capability. I do not claim that question has been
answered. My own research has not produced an end-to-end Dark system; its
strongest composed paths remain Shielded. The question should be tested
obligation by obligation, not answered by the word "private."

= Question 19: governed machine-readable rules

The Commissions should publish machine-readable reporting structures, and
should treat them as governed specifications rather than as an opaque
executable or a substitute for controlling legal text.

== Minimum package

1. *Normative semantic model.* Define economic events, lifecycle states, field
   meanings, valid transitions, and correction history independently of any
   chain or vendor.
2. *Versioned schemas.* Specify types, exact units, integer bounds,
   enumerations, optionality, identities, references, and cross-field
   constraints.
3. *Deterministic validation logic.* Provide small, reviewable predicates for
   syntax and semantic consistency.
4. *Conformance vectors.* Publish valid and invalid records, boundary
   integers, lifecycle sequences, reorganizations, batching, ambiguous
   timestamps, and corrections.
5. *Reference implementations.* Provide reproducible, non-normative code, in
   more than one implementation where feasible.
6. *Differential and property testing.* Require implementations to agree
   across randomized and adversarial corpora, with exact arithmetic for
   prices, quantities, fees, and conservation-sensitive values.
7. *Version and effective-date binding.* Bind each report to the exact schema
   and rule version and retain historical versions for replay.
8. *Public change control.* Publish proposals, rationale, migration rules,
   examples, and interpretation procedures.
9. *Failure taxonomy.* Standardize rejected, pending, unsupported, expired,
   corrected, reorganized, and backend-unavailable states.
10. *Human-readable parity.* Map each rule provision to its data elements,
    validators, examples, and controlling text.

The worked example already exhibited the strongest form of item 3: a clearing
rule deterministic over a frozen book, checkable by anyone through full
recomputation. Where a reporting rule can have that shape, disagreement about
what happened reduces to disagreement about inputs, which is exactly where a
regulator wants it.

== Guarded updates and candidate states

Two structures from my research bear directly on machine-readable reporting.
In my Lean models of guarded
commitments, an update's shape --- the record it modifies, the actor authorized
to modify it, the fields it may touch, the predicate version it must satisfy ---
is fixed before the late value arrives; an accepted update is exactly the
committed transition, and a violating one fails closed, changing nothing. In
the same research, a computation's pending state is represented as the set of
results it could still be, and collapsing that set to one answer is an
explicit act with a stated precondition, not an assumption. These are theorems
about modeled state machines. They are not a deployed reporting adapter, a
compliance implementation, or a proposal that the Commissions adopt any
research calculus.

Their value here is narrower: they show that transition authority, failure,
and ambiguity can be made explicit and machine-checkable --- that "who may
correct this report, to what, under which rule version" can be a typed object
rather than a convention. A finality or correction certificate should be
required before one candidate becomes the final regulatory state, and rejected
or superseded transitions should remain auditable without being counted as
trades. One further lesson from my prototypes: I built the observation accumulator
to refuse a question its retained information cannot support rather than
approximate it. "The rule rejects this" and "the backend cannot answer
this" are different states, and the failure taxonomy of item 9 should keep
them apart.

= Governance and pilot design

A machine-readable regime should be tested against at least these failures:

- code and controlling text diverge;
- new schemas are applied retroactively;
- validators reject valid records or accept incomplete ones;
- implementations disagree about units, time, optionality, or correction;
- public fields enable unexpected re-identification;
- private-data keys are lost, compromised, or used beyond authorization;
- a fixed encrypted compliance query proves too narrow for a later
  investigation;
- proof assumptions, circuits, or setup parameters change;
- chain finality or reorganization semantics are misunderstood; and
- historical-version maintenance disproportionately burdens smaller entities.

Before mandating a machine-readable structure, the Commissions should conduct
a public conformance pilot involving reporting entities, repositories,
market-data users, privacy researchers, cryptographers, and smaller
implementers. The pilot should publish acceptance and rejection rates,
correction latency, disagreement cases, implementation cost, performance, and
privacy findings.

= Requested actions

I respectfully request that the Commissions:

1. define normalized lifecycle events for onchain transactions;
2. require chain provenance and explicit finality and correction semantics
   where necessary to interpret those events;
3. separate the public, confidential regulatory, and machine-verifiable
   records;
4. treat public dissemination as a documented leakage policy and examine its
   identity and strategy effects;
5. recognize the difference between regulator-observable Shielded systems and
   systems whose declared Dark leakage has no general opening path;
6. state when a proof can satisfy a reporting element and when retained source
   evidence remains required; and
7. publish versioned schemas, validators, test vectors, and change-control
   rules with human-readable parity.

= Limits of this comment

This comment does not argue for reduced regulatory access merely because a
transaction is onchain. It does not claim that a transaction hash is a
complete report, that a proof establishes the report behind it, that
zero-knowledge techniques eliminate recordkeeping, or that encryption places
any record beyond lawful process. It does not claim that any presently
available Dark architecture satisfies existing reporting obligations --- my own
research included: the artifacts behind this comment are Lean models and
offline research prototypes, not a reporting system, and none of them is
deployed, funded, offered, or operating. It takes no position on the
classification of any product; classification is the subject of a separate
joint request and is outside this comment's scope. And it does not ask the
Commissions to adopt a particular proof system, blockchain, or local research
formalism.

= Conclusion

Onchain provenance, confidential regulatory data, public transparency, and
machine-verifiable rules are compatible, but only when their boundaries are
explicit. "Onchain" should not mean "already reported." "Public" should not
mean "every strategy-revealing field is published." "Encrypted" should not
mean "unavailable to any lawful process." "Machine-readable" should not mean
"an opaque program silently becomes the law."

The Commissions should standardize normalized lifecycle events and chain
provenance, preserve an exact confidential regulatory record for Clear and
regulator-observable Shielded systems, minimize unnecessary identity and
strategy leakage while preserving the transparency that price discovery,
surveillance, and market integrity require, and publish governed
machine-readable specifications with conformance vectors. A true Dark system
should be evaluated separately against each obligation, rather than assumed to
possess a complete confidential record that its leakage contract intentionally
does not expose.

#block(breakable: false)[
  #v(18pt)
  Respectfully submitted,

  [FULL NAME]  \
  [AFFILIATION, IF ANY]  \
  [DATE]
]

= Appendix: basis of material technical claims

This appendix records the evidentiary basis for each material technical claim
in one line. "Model theorem" means a machine-checked statement about a
simplified formal model reviewed by the submitter. "Prototype test" means a
deterministic offline test in a research prototype reviewed by the submitter.
No artifact behind these claims is deployed market infrastructure, and none
has been independently audited.

#table(
  columns: (1fr, 2.1in),
  table.header([*Claim*], [*Basis*]),
  [The Commissions request comment on blockchain reporting, public dissemination and privacy, and machine-readable reporting logic (Questions 3, 8, and 19)], [Joint request for comment; source note 1],
  [The real-time public reporting rules delay the public print of certain large trades and cap the disseminated notional], [17 C.F.R. part 43; source note 2],
  [The lifecycle reporting framework expects exact creation and continuation data with counterparty and transaction identifiers], [17 C.F.R. part 45; source note 3],
  [Repository confidentiality and access rules govern reported swap data], [17 C.F.R. part 49; source note 4],
  [A guarded update's shape is fixed before the late value; an accepted update equals the committed transition; a violating update fails closed], [Model theorems in the submitter's guarded-commitment research; not deployed controls or a reporting adapter],
  [A pending computation can be represented as the set of results it could still be; collapse to one answer is an explicit act with a stated precondition], [Model theorem in the submitter's candidate-result formalism; no oracle, finality process, or enforceable selection is implemented or validated],
  [The worked example's core accounting --- deposit, recombination, resolution, redemption, with conservation and pool-coverage checks --- has been implemented offline with passing deterministic tests], [Pure-Rust research prototype reviewed by the submitter; tested, not formally verified; not deployed],
  [The prototype's batch verifier was built to accept a submitted clearing only if full recomputation from the frozen book reproduces it], [Prototype source and deterministic tests reviewed by the submitter; offline research code, not a deployed venue],
  [The prototype's observation accumulator was built to refuse questions its retained information cannot support], [Prototype source and deterministic tests reviewed by the submitter; offline research code],
  [The leakage laboratory was built to replay four synthetic traces against three transcript designs, separating mechanically revealed fields from enabled deductions], [Deterministic synthetic-transcript accounting reviewed by the submitter; not an anonymity, cryptographic-leakage, timing, or real-market measurement],
  [The Clear, Shielded, and Dark taxonomy and the associated leakage-surface analysis], [Proposed analytical terminology from the submitter's research; no claim that any Dark system exists, is deployed, or satisfies current rules],
  [No artifact described in this comment is deployed, funded, offered, or operating], [The submitter's repository status records; a statement about the submitter's own artifacts, not about any third party],
)
