#import "../shared/template.typ": key_point, note_ref

= Summary of positions

The Commissions ask how swap and security-based swap reporting should treat
blockchain-native execution, what public dissemination should reveal when
the underlying ledger is already public, and whether reporting logic should
be published in machine-readable form.#note_ref(1) I write in response to
Questions 3, 8, and 19, and take no position on the other questions.

I am a software and formal-methods researcher. This comment takes
positions, argued from one concrete design placed on a public ledger; the
positions are my own analysis as a commenter, and what the comment reports
about my research artifacts stays within what those artifacts support.

1. The unit of reporting should be a normalized economic lifecycle event
   bound to its exact ledger sources, with chain-provenance fields added
   to --- never substituted for --- ordinary economic and counterparty
   fields; a transaction hash is an address of bytes, not a report.
2. Funding and gross issuance of a fully hedged complete claim set should be
   reported as lifecycle events, with the claims, collateral, parties, and
   provenance preserved exactly in the confidential record. A distinct net
   contingent-exposure field should remain zero while the set and its
   unconditional recombination right remain together, and should change at
   the first transaction that unbalances the set.
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
6. On machine-readable reporting rules I endorse, rather than restate,
   Ariadne Dataworks' filed recommendations --- a machine-readable field
   dictionary, executable openly licensed validation logic with a public
   conformance corpus, and a normative lifecycle event model#note_ref(5)
   --- and add what it does not reach: validators in full-recomputation
   form, bound to the three-record structure defined below, and a failure
   taxonomy distinguishing rejection by rule from backend unavailability.

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
no participants: ledger writes precede reportable events.

*Funding and issuance.* A wallet deposits one unit of collateral and receives
a complete set: one claim per band. This is a reportable funding and gross
issuance event, and the exact confidential record should preserve the
deposit, every claim issued, the parties, amounts, time, and ledger
provenance. The public bytes are precise --- pool address, wallet address,
amount, time --- but they do not establish the controller's legal or
beneficial identity. The separate net contingent-exposure value is
zero at this milestone: a complete set plus the unconditional right to
recombine it into its collateral is fully hedged and interchangeable with
the deposit itself. Net contingent exposure arises later, when the depositor
sells some claims and keeps others --- and that sale may occur in a venue
whose individual orders never touch this ledger. A hash-to-trade mapping
that labels funding itself as net contingent exposure records exposure that
does not exist, while a mapping that ignores offchain events misses the
transaction where it actually arises. Supervision needs a confidential
account-to-party linkage, so gross issuance and funding, the later orders,
the unbalancing transaction, and eventual redemption read as one owner's
exact lifecycle.

*Close and match.* Orders accumulate until the stated close; at close the
book freezes and a deterministic rule clears it at one consistent set of
prices. In the current transparent offline prototype, the frozen book is
available to the verifier and no order concealment is claimed. A proposed
Shielded form could instead let a named executor or committee see the
individual orders while the ledger carries only a root committing to the
accepted set, the clearing prices, and aggregate fills. In either form, the
records market-conduct examination cares most about --- instructions rejected
before admission, modifications, cancellations, arrival sequence --- may
never become ledger bytes at all, because they never changed the ledger's
state. They are reported from the venue's own records or they are not
reported. This milestone is also where machine verification is strongest:
because the clearing rule is deterministic over a frozen book, anyone
authorized to hold that book can recompute the result. In my research
prototype, I built the batch verifier to accept a submitted clearing only if
recomputation from the frozen book reproduces it exactly, never trusting the
submitter's claimed quantities. A reporting rule with that shape is more
than a format check: the rule is the check.

*Resolution.* Between the close of trading and resolution, the outcome is
genuinely undetermined: any band may realize, and each band's claim trades
at a price. An actor may submit evidence, but nothing in the software makes
one outcome authoritative before the frozen observation program admits
qualifying evidence and the repair period lapses. The narrower claim is that,
under the frozen rule and its admitted evidence, no discretionary adjudicator
chooses among outcomes. Reporting vocabulary should be able to say this
without pretending: pending, disputed, unsupported, and expired are distinct
states; none is "resolved," and none is an error. Even after certification,
a ledger reorganization or an application-level dispute can supersede what
was observed. The record needs a correction linkage that preserves the
superseded event, not a silent overwrite.

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
minimum: order creation and authorization; funding or collateral lock; gross
claim issuance and recombination; net contingent exposure, including the
recognized complete-set offset and the event that unbalances it; receipt,
acceptance, rejection, modification, and cancellation; batch close and
accepted-set inclusion; match, execution, and allocation; pending,
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
can be unique. The current transparent offline batch prototype does not
conceal individual orders. In a proposed Shielded form, a named executor or
committee could keep them from public view, but a settlement graph paying
redemptions directly to the original wallets could undo that concealment
after the fact.

Purpose-limited publication is not a novelty: the real-time public reporting
rules already delay the public print of certain large trades and cap the
disseminated notional.#note_ref(2) What is new for onchain markets is that
the regulated tape is not the only disclosure channel. On Solana there is no
public global mempool: before landing, a transaction is visible only to the
client and whichever RPC operator, relay or forwarding node, and current or
next leader actually receives it; a direct TPU path can bypass RPC. A
transaction dropped before landing leaves no public ledger record. If a
failed transaction lands in a confirmed block, however, its transaction data
and failure metadata are public.#note_ref("6, 7") Other ledgers have
different ingress and publication models. A dissemination review that scores
only the tape should therefore separate local pre-landing disclosure to
ingress actors from public, durable post-landing leakage:

#table(
  columns: (1.55in, 1fr),
  table.header([*Leakage surface*], [*What it reveals, independent of any tape*]),
  [RPC ingress], [An RPC operator sees the signed transaction it accepts before landing; preflight rejection or later dropping may remain only in client and operator records, not on the public ledger.],
  [Relay or forwarding ingress], [A relay service or forwarding node sees the packets it actually handles. This is topology- and trust-specific disclosure, not global broadcast.],
  [Leader ingress], [A current or next leader receiving the transaction directly or from an RPC path can observe intent before deciding whether it lands; that visibility is not public mempool visibility.],
  [Fee payer], [For a landed transaction, the account paying execution fees links otherwise separate transactions; one reused funding account can join strategies into one actor.],
  [Funding graph], [Landed transfers can make the path from an exchange withdrawal or a prior market's payout into a deposit public and durable, connecting positions across markets and time.],
  [Landed failed transaction], [A failed transaction included in a block exposes its signed message and failure metadata without an execution succeeding; a dropped or preflight-rejected transaction is not public ledger bytes.],
  [Settlement graph], [Public redemptions to original wallets can defeat a proposed Shielded batch's public-order concealment after the fact, reattaching outcomes to accounts that traded.],
)

Evaluation should therefore measure both the dissemination policy and this
independently observable protocol and operational leakage, stating per
surface who can observe it, when, and whether a design closes, narrows, or
leaves it open. Each public field should carry a stated transparency
objective and a reviewed re-identification analysis; caps, delays, buckets,
and aggregation should be explicit, deterministic, and versioned.

None of this loosens the confidential record. The lifecycle reporting
framework already expects exact creation and continuation data with
counterparty and transaction identifiers,#note_ref(3) held under repository
confidentiality and access rules.#note_ref(4) The basis for my leakage
statements is deliberately modest: a small deterministic laboratory that
replays four synthetic trading traces against three transcript designs and
records which fields each design mechanically reveals and which deductions
those fields enable, keeping the two categories separate. It measures no
anonymity, cryptographic leakage, timing behavior, or real market. Its
value is the discipline --- every "this design hides X" sentence is checked
against an explicit list of what the transcript contains --- and I
recommend the same for any dissemination policy: enumerate the fields, then
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

The Commissions have already received the general form of the right answer.
The comment of Ariadne Dataworks Ltd. on this docket recommends publishing the
reporting framework itself as structured data: a joint machine-readable
field dictionary maintained as a versioned schema, validation logic in
executable, openly licensed form with a public conformance corpus, and a
normative lifecycle event model with worked examples.#note_ref(5) I endorse those three recommendations rather
than restating them; adopted with the governance that comment describes,
they would carry most of what a machine-readable regime needs. This
comment's space goes to three layers the proposal does not cover.

*Bind the validators to the three-record structure.* Each of the three
records defined above needs its own schema and validators, and the
validation package is itself the third record --- the one that can be
fully public at no confidentiality cost, because it contains no
transaction data at all: schemas, deterministic predicates, conformance
vectors, and provenance bindings, publishable in their entirety even where
everything they validate is confidential. Publishing it buys independent
implementations that agree on what the rules accept.

*Where a rule is deterministic over committed inputs, publish the validator
in recomputation form.* A conformance corpus checks outcomes against
expected outcomes; it establishes that an implementation agrees with the
published examples. A recomputation validator is stronger: it re-derives the
reported answer from the committed inputs and accepts only exact
agreement --- the rule is the check. The worked example exhibited the
strongest form --- a clearing rule deterministic over a frozen book,
checkable by anyone through full recomputation --- and where a reporting
rule can have that shape, disagreement about what happened reduces to
disagreement about inputs, exactly where a regulator wants it. Correction authority
can be equally concrete: in my formal models I have machine-checked that a
correction's authorized actor, target record, permitted fields, and
governing rule version can be fixed in advance and enforced mechanically ---
"who may correct this report, to what, under which version" can be a typed
object rather than a convention. A certification should be required before a
pending outcome becomes the final regulatory state, and rejected or
superseded transitions should remain auditable without being counted as
trades.

*Standardize a failure taxonomy whose states are genuinely distinct* ---
rejected, pending, unsupported, expired, corrected, reorganized, and
backend-unavailable --- and hold the first and last apart deliberately. I
built the observation accumulator to refuse a question its retained
information cannot support rather than approximate it: "the rule rejects
this" and "the backend cannot answer this" are different facts, and a
validator that conflates them will
misreport both --- a rejection stream that silently absorbs outages
understates data quality exactly when the system is least healthy, and the
converse hides violations behind operational noise. A reporting party should not satisfy an obligation by submitting
into an outage, and an examiner should be able to tell, from the record
alone, which of the two happened.

= Specific requests

1. Define normalized lifecycle events for onchain transactions, with chain
   provenance and explicit finality and correction semantics. (Positions 1
   through 3.)
2. Separate the public, confidential regulatory, and machine-verifiable
   records, and treat public dissemination as a documented, versioned
   leakage policy. (Position 4.)
3. State when a proof can satisfy a reporting element and when retained
   source evidence remains required. (Position 5.)
4. Adopt the machine-readable field dictionary, executable validation logic
   with a public conformance corpus, and normative lifecycle model already
   proposed on this docket;#note_ref(5) bind the resulting validators to
   the three-record structure, publish them in recomputation form where the
   rule is deterministic over committed inputs, and standardize the failure
   taxonomy --- then pilot publicly, with reporting entities, repositories,
   market-data users, privacy researchers, and smaller implementers, before
   mandating. (Position 6.)
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
  [The description of the Ariadne Dataworks recommendations], [The filed comment; source note 5],
  [Solana has no public global mempool; pre-landing visibility follows the RPC, relay or forwarding, and leader path actually used; a landed failure is public ledger data only when included in a confirmed block], [Official Solana transaction-ingress guide and RPC JSON structures; source notes 6 and 7],
  [The worked example's core accounting --- deposit, recombination, resolution, redemption, with conservation and pool-coverage checks --- has been implemented offline with passing deterministic tests], [Pure-Rust research prototype reviewed by the submitter; tested, not formally verified; not deployed],
  [The prototype's batch verifier was built to accept a submitted clearing only if full recomputation from the frozen book reproduces it], [Prototype source and deterministic tests reviewed by the submitter; offline research code, not a deployed venue],
  [The prototype's observation accumulator was built to refuse questions its retained information cannot support], [Prototype source and deterministic tests reviewed by the submitter; offline research code],
  [A correction's authorized actor, target record, permitted fields, and rule version can be fixed in advance and enforced mechanically], [Model theorems in the submitter's guarded-commitment research; not deployed controls or a reporting adapter],
  [The leakage laboratory was built to replay four synthetic traces against three transcript designs, separating mechanically revealed fields from enabled deductions], [Deterministic synthetic-transcript accounting reviewed by the submitter; measures no anonymity, cryptographic leakage, timing, or real market],
  [The Clear, Shielded, and Dark taxonomy and the associated leakage-surface analysis], [Proposed analytical terminology from the submitter's research; no claim that any Dark system exists, is deployed, or satisfies current rules],
  [No artifact described in this comment is deployed, funded, offered, or operating], [The submitter's repository status records; a statement about the submitter's own artifacts, not about any third party],
)
