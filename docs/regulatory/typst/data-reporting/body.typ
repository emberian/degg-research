#import "../shared/template.typ": key_point, note_ref

= Summary of positions

The Commissions ask how swap and security-based swap reporting should treat
blockchain-native execution, what public dissemination should reveal when
the underlying ledger is already public, and whether reporting logic should
be published in machine-readable form.#note_ref(1) I write in response to
Questions 3, 8, and 19, and take no position on the other questions.

I am a software and formal-methods researcher. This comment takes positions
and argues them from one concrete design placed on a public ledger. The
positions are my own analysis as a commenter; what the comment reports about
my research artifacts is stated separately, and stays within what those
artifacts support.

1. The unit of reporting should be a normalized economic lifecycle event
   bound to its exact ledger sources, with chain-provenance fields added
   to --- never substituted for --- ordinary economic and counterparty
   fields; a transaction hash is an address of bytes, not a report.
2. Funding of a fully hedged complete claim set should not be reported as a
   contingent position; contingent exposure should be reported where it
   arises, at the first transaction that unbalances the set.
3. Corrections must supersede, never overwrite: a reorganization, dispute,
   or repair should link to and preserve the superseded record.
4. Public dissemination should be adopted as an explicit, versioned,
   deterministic leakage policy --- every public field justified by a stated
   transparency purpose and examined for re-identification and
   strategy-inference risk --- while the confidential regulatory record
   remains exact, complete, and timely however conservative the public
   record becomes.
5. A proof should satisfy a reporting element only where the required
   proposition is exactly what the proof's statement establishes; otherwise
   the committed source evidence must be retained.
6. Machine-readable reporting rules should be published as governed
   executable validators --- versioned schemas, deterministic predicates,
   conformance vectors, historical replay, and human-readable parity ---
   validating the three records defined below.

The three records are the structure the positions defend. For Clear and
regulator-observable systems, define three records rather than forcing one
public ledger event to serve every purpose:

1. a *public transparency record* containing the fields justified by price
   discovery and public market integrity;
2. an *exact confidential regulatory record* containing the owner-linked
   order, funding, execution, position, and settlement lifecycle needed for
   supervision, examination, and enforcement; and
3. a *machine-verifiable validation package* containing versioned schemas,
   deterministic transition rules, conformance vectors, and provenance
   bindings.

One question I leave open, because analysis cannot yet close it: whether a
Dark architecture --- no general opening path beyond a frozen leakage
function and authorized local outputs --- can satisfy every applicable
reporting, correction, examination, and enforcement objective. It should be
tested obligation by obligation, not answered by the word "private."

= The worked example, viewed as records

The design is a fully collateralized conditional-asset market over an
objectively verifiable onchain price band. A depositor locks collateral into
a segregated pool that belongs to one market only and receives one claim for
each cell of an exhaustive, disjoint partition of the possible outcomes.
Claims trade through a batch venue with a stated close. A deterministic
observation program, frozen when the market is created, later identifies the
realized cell, and settlement pays that cell's claims from the pool. I have
built an offline research prototype of the core accounting for this design.
It is not a deployed system, a product, or an offer, and nothing in this
comment asks either Commission to approve it. At each milestone, ask which
records exist, which are public bytes, and which exist only if somebody is
required to keep them.

*Publication.* The market template is created on the ledger: partition rule,
observation-program version, batch policy, settlement terms. Everything
about this milestone is public, and none of it is reportable as a
transaction: there are no parties, no funding, and no exposure. A reporting
adapter that mapped this ledger write to "trade" would invent a trade with
no participants. Ledger writes precede reportable events.

*Funding.* A wallet deposits one unit of collateral and receives a complete
set: one claim per band. The public bytes are precise --- pool address,
wallet address, amount, time. They are not an identification: nothing on the
ledger says who controls the wallet. And they are not a contingent position:
a complete set plus the right to recombine it into its collateral is fully
hedged, interchangeable with the deposit itself. Exposure arises later, when
the depositor sells some claims and keeps others --- and that sale may occur
in a venue whose individual orders never touch this ledger. A hash-to-trade
mapping records exposure here that does not exist and misses the transaction
where exposure actually arises. Supervision needs a confidential
account-to-party linkage, so the deposit, the later orders, and the eventual
redemption read as one owner's lifecycle.

*Close and match.* Orders accumulate until the stated close; at close the
book freezes and a deterministic rule clears it at one consistent set of
prices. The ledger can carry little of this --- perhaps a root committing to
the accepted orders, the clearing prices, and aggregate fills. The records
market-conduct examination cares most about --- instructions rejected before
admission, modifications, cancellations, arrival sequence --- may never
become ledger bytes at all, because they never changed the ledger's state.
They are reported from the venue's own records or they are not reported. In
the other direction, this milestone is where machine verification is
strongest: because the clearing rule is deterministic over a frozen book,
anyone holding the book can recompute the result. In my research prototype,
I built the batch verifier to accept a submitted clearing only if
recomputation from the frozen book reproduces it exactly, never trusting the
submitter's claimed quantities. A reporting rule with that shape is more
than a format check: the rule is the check.

*Resolution.* Between the close of trading and resolution, the outcome is
genuinely undetermined: any band may realize, and each band's claim trades
at a price. Nothing in the software makes one outcome authoritative before
the frozen observation program accepts qualifying evidence and the repair
period lapses --- there is no reporter and no discretion. Reporting
vocabulary should be able to say this without pretending: pending, disputed,
unsupported, and expired are distinct states; none is "resolved," and none
is an error. Even after certification, a ledger reorganization or an
application-level dispute can supersede what was observed. The record needs
a correction linkage that preserves the superseded event, not a silent
overwrite.

*Settlement.* The realized band's claims redeem from the pool. The public
bytes show outflows. They do not show which redemption closes which position
of which owner at what gain --- the fact an examiner reconstructing a
lifecycle actually needs --- and no transparency purpose requires that they
show it. The confidential record links them; the public record need not.

The walk yields the three records: enough public fields to trust prices; the
exact owner-linked lifecycle, including events the ledger never saw, for the
regulator; and a deterministic validation rule for the machine. The
Commissions' three questions take them in turn.

= Question 3: report economic events, not ledger writes

The counterargument to Position 1 is that the ledger is already a complete,
tamper-evident record. It is neither complete nor a report: it omits
beneficial ownership, rejected and canceled instructions, offchain
modifications, allocation, and the meaning of a fork or dispute --- and
where several ledger instructions jointly implement one reportable event, or
one transaction contains several, the hash-to-event mapping fails in both
directions. A reporting framework for onchain markets should distinguish, at
minimum: order creation and authorization; funding or collateral lock;
receipt, acceptance, rejection, modification, and cancellation; batch close
and accepted-set inclusion; match, execution, and allocation; pending,
disputed, unsupported, or expired resolution; certification; early exit,
unwind, or replacement; settlement, delivery, maturity, or correction; and
replacement caused by reorganization or dispute. Each report should bind the
complete set of ledger sources that implement it.

Provenance fields interpret the economic event; they do not replace economic
and counterparty fields:

#table(
  columns: (1.75in, 1fr),
  table.header([*Field*], [*Purpose*]),
  [Network, chain, and deployment identifier], [Disambiguate ledgers, forks, environments, and redeployments.],
  [Block height, block hash, and transaction and event indexes], [Anchor the event to a particular history and establish exact ordering.],
  [Program address, executable hash, upgrade authority, configuration version], [Bind the event to executable logic and its mutability history rather than a mutable name.],
  [Product or rule identifier, terms digest, and schema version], [Bind the event to its economic semantics and reject incompatible decoders.],
  [Accepted-input or batch root], [Bind an aggregate execution to the set admitted for processing.],
  [Finality status and finality observation time], [Separate observed, confirmed, economically final, and protocol-final states.],
  [Reorganization or replacement reference], [Link superseded events without silent deletion.],
  [Confidential account-to-party linkage], [Preserve ownership continuity for regulators without public identification.],
)

Guidance should specify how to report ledger time, receipt time, execution
time, and finality time when they differ, and correction behavior for
probabilistic finality, reorganization, upgrade, and application disputes.

On proofs (Position 5): a proof establishes only the proposition its
statement encodes, relative to named committed inputs, rule versions, and
stated assumptions. An authorization proof does not establish current legal
authority without key and revocation governance; a collateral proof does not
establish custody or absence of encumbrance unless those facts are inside
the relation and bound to the actual settlement. Unless the Commissions
determine that a proof is itself the required datum, the committed evidence
needed to reconstruct the event should be preserved, and the record should
identify the proof system, statement version, verifier, public-input digest,
result, and the retention location of the supporting evidence.

= Question 8: public transparency without strategy publication

Return to the funding milestone: wallet, exact amount, and time are public
bytes, permanently. Removing a name does not remove identity: the
combination of wallet, timestamp, size, price, product, and settlement
destination can re-identify a trader, and repeated behavior can reveal entry
policy, inventory limits, hedge timing, or an algorithm's responses. Sparse
or bespoke markets heighten the risk, because an otherwise anonymous event
can be unique. In the worked example, the batch conceals individual orders,
but a settlement graph paying redemptions directly to the original wallets
can undo that concealment after the fact.

Purpose-limited publication is not a novelty: the real-time public reporting
rules already delay the public print of certain large trades and cap the
disseminated notional.#note_ref(2) What is new for onchain markets is the
number of leakage surfaces outside the regulated tape --- a public mempool,
a fee payer, a funding graph, a failed instruction, a settlement graph ---
so evaluation should measure both the dissemination policy and the
independently observable base-chain leakage. Each public field should
therefore carry a stated transparency objective and a reviewed
re-identification analysis; caps, delays, buckets, and aggregation should be
explicit, deterministic, and versioned.

None of this loosens the confidential record. The lifecycle reporting
framework already expects exact creation and continuation data with
counterparty and transaction identifiers,#note_ref(3) held under repository
confidentiality and access rules.#note_ref(4) The basis for my leakage
statements is deliberately modest: a small deterministic laboratory that
replays four synthetic trading traces against three transcript designs and
records which fields each design mechanically reveals and which deductions
those fields enable, keeping the two categories separate. It measures no
anonymity, cryptographic leakage, timing behavior, or real market. Its value
is the discipline: every "this design hides X" sentence is checked against
an explicit list of what the transcript contains. I recommend the same
discipline for any dissemination policy --- enumerate the fields, then
defend each one.

*Clear, Shielded, and Dark.* My research uses three terms exactly, because
conflating them produces bad reporting policy in both directions:

- *Clear*: the specified state and computation are public.
- *Shielded*: a named executor, committee, or auditor may learn private
  inputs.
- *Dark*: no actor learns anything beyond a frozen leakage function and its
  own authorized local output, within an explicit corruption model.

A due-process threshold opening path is regulator-observable Shielded, not
Dark; an operator-readable encrypted database is Shielded; unpadded ingress,
observable timing, or a public settlement graph can make a purportedly Dark
design Shielded in fact. Clear and regulator-observable Shielded systems can
be evaluated against confidential reporting obligations by inspecting their
data and governance paths. Whether a true Dark architecture can satisfy each
obligation is the open question stated above; my own research has not
produced an end-to-end Dark system --- its strongest composed paths remain
Shielded.

= Question 19: governed machine-readable rules

The Commissions should publish machine-readable reporting structures, and
should treat them as governed specifications rather than as an opaque
executable or a substitute for controlling legal text. The minimum package:

1. a normative semantic model --- events, states, field meanings, valid
   transitions, correction history --- independent of any chain or vendor;
2. versioned schemas with exact units, integer bounds, enumerations, and
   cross-field constraints;
3. deterministic validation predicates, small enough to review;
4. conformance vectors: valid and invalid records, boundary integers,
   lifecycle sequences, reorganizations, and corrections;
5. version and effective-date binding, public change control, and
   historical versions retained for replay;
6. a failure taxonomy standardizing rejected, pending, unsupported, expired,
   corrected, reorganized, and backend-unavailable states; and
7. human-readable parity mapping each rule provision to its data elements,
   validators, and controlling text.

The worked example exhibited the strongest form of item 3: a clearing rule
deterministic over a frozen book, checkable by anyone through full
recomputation. Where a reporting rule can have that shape, disagreement
about what happened reduces to disagreement about inputs, which is exactly
where a regulator wants it. Correction authority can be equally concrete: in
my formal models I have machine-checked that a correction's authorized
actor, target record, permitted fields, and governing rule version can be
fixed in advance and enforced mechanically --- "who may correct this report,
to what, under which version" can be a typed object rather than a
convention. A certification should be required before a pending outcome
becomes the final regulatory state, and rejected or superseded transitions
should remain auditable without being counted as trades.

The failure taxonomy deserves its two distinct states. I built the
observation accumulator to refuse a question its retained information cannot
support rather than approximate it: "the rule rejects this" and "the backend
cannot answer this" are different facts, and a validator that conflates them
will misreport both.

= Specific requests

1. Define normalized lifecycle events for onchain transactions, with chain
   provenance and explicit finality and correction semantics. (Positions 1
   through 3.)
2. Separate the public, confidential regulatory, and machine-verifiable
   records, and treat public dissemination as a documented, versioned
   leakage policy. (Position 4.)
3. State when a proof can satisfy a reporting element and when retained
   source evidence remains required. (Position 5.)
4. Publish versioned schemas, validators, conformance vectors, and
   change-control rules with human-readable parity, and pilot them publicly
   --- with reporting entities, repositories, market-data users, privacy
   researchers, and smaller implementers --- before mandating them.
   (Position 6.)
5. Evaluate regulator-observable Shielded systems on their data and
   governance paths, and treat Dark satisfiability as an open question to be
   tested obligation by obligation.

= Limits

This comment takes no position on the classification of any product;
classification is the subject of a separate joint request. The artifacts
behind it are formal models and offline research prototypes, not a reporting
system, and none is deployed, funded, offered, or operating. Nothing here
argues for reduced regulatory access because a transaction is onchain, and
no design of mine is claimed to satisfy any current reporting obligation.

#block(breakable: false)[
  #v(18pt, weak: true)
  Respectfully submitted,

  [FULL NAME]  \
  [AFFILIATION, IF ANY]  \
  [DATE]
]

= Appendix: basis of material technical claims

Each material technical claim, with its evidentiary basis in one line.
"Model theorem" means a machine-checked statement about a simplified formal
model reviewed by the submitter; "prototype" means a deterministic offline
research prototype reviewed by the submitter. No artifact behind these
claims is deployed market infrastructure, and none has been independently
audited.

#table(
  columns: (1fr, 2.1in),
  table.header([*Claim*], [*Basis*]),
  [The Commissions request comment on blockchain reporting, public dissemination and privacy, and machine-readable reporting logic (Questions 3, 8, and 19)], [Joint request for comment; source note 1],
  [The real-time public reporting rules delay the public print of certain large trades and cap the disseminated notional], [17 C.F.R. part 43; source note 2],
  [The lifecycle reporting framework expects exact creation and continuation data with counterparty and transaction identifiers], [17 C.F.R. part 45; source note 3],
  [Repository confidentiality and access rules govern reported swap data], [17 C.F.R. part 49; source note 4],
  [The worked example's core accounting --- deposit, recombination, resolution, redemption, with conservation and pool-coverage checks --- has been implemented offline with passing deterministic tests], [Pure-Rust research prototype reviewed by the submitter; tested, not formally verified; not deployed],
  [The prototype's batch verifier was built to accept a submitted clearing only if full recomputation from the frozen book reproduces it], [Prototype source and deterministic tests reviewed by the submitter; offline research code, not a deployed venue],
  [The prototype's observation accumulator was built to refuse questions its retained information cannot support], [Prototype source and deterministic tests reviewed by the submitter; offline research code],
  [A correction's authorized actor, target record, permitted fields, and rule version can be fixed in advance and enforced mechanically], [Model theorems in the submitter's guarded-commitment research; not deployed controls or a reporting adapter],
  [The leakage laboratory was built to replay four synthetic traces against three transcript designs, separating mechanically revealed fields from enabled deductions], [Deterministic synthetic-transcript accounting reviewed by the submitter; not an anonymity, cryptographic-leakage, timing, or real-market measurement],
  [The Clear, Shielded, and Dark taxonomy and the associated leakage-surface analysis], [Proposed analytical terminology from the submitter's research; no claim that any Dark system exists, is deployed, or satisfies current rules],
  [No artifact described in this comment is deployed, funded, offered, or operating], [The submitter's repository status records; a statement about the submitter's own artifacts, not about any third party],
)
