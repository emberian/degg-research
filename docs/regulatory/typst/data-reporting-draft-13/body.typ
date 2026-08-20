#import "../shared/template.typ": claim_table, key_point, note_ref

= What this is, and what it asks

I am a software and formal-methods researcher. I built a staged claim
protocol, Dragon's Clutch, put it on a public ledger, and studied what it
does and does not record. This comment answers Questions 3, 8, and 19 of the joint
request#note_ref(1) from that vantage; on the others it takes no position.

One finding organizes all three answers: a public ledger is not a report. It
omits beneficial ownership, rejected and canceled instructions, offchain
modifications, allocation, and the meaning of a fork or dispute; where
several instructions implement one reportable event, or one transaction holds
several, the hash-to-event mapping fails both ways. A transaction hash is an
address of bytes. The regime needs three records:

1. a *public transparency record*: fields justified by price discovery and
   public market integrity;
2. an *exact confidential regulatory record*: the owner-linked order,
   funding, execution, position, and settlement lifecycle for supervision,
   examination, and enforcement;
3. a *machine-verifiable validation package*: versioned schemas,
   deterministic transition rules, conformance vectors, provenance bindings.

Five requests follow.

1. Define *normalized lifecycle events* for onchain transactions, each bound
   to the complete set of ledger sources implementing it, with chain
   provenance added to --- never substituted for --- economic and
   counterparty fields, explicit finality semantics, and corrections that
   supersede rather than overwrite.
2. Report *funding and gross issuance* of a fully hedged complete claim set
   as lifecycle events, preserving claims, collateral, parties, and
   provenance exactly in the confidential record, and carry a *distinct net
   contingent-exposure field* that stays zero while the set and its
   unconditional recombination right remain together and changes at the first
   transaction that unbalances the set.
3. Separate the three records; treat public dissemination as a documented,
   versioned, deterministic *leakage policy*: every public field justified by
   a stated transparency purpose and examined for re-identification and
   strategy-inference risk, with the confidential record exact, complete, and
   timely however conservative the public record becomes.
4. State *when a proof can satisfy a reporting element* --- only where the
   required proposition is exactly what the proof's statement establishes ---
   and when retained source evidence remains required.
5. Adopt the machine-readable field dictionary, executable validation logic
   with a public conformance corpus, and normative lifecycle model already
   proposed on this docket;#note_ref(5) *bind the resulting validators to the
   three-record structure*, publish them in recomputation form where the rule
   is deterministic over committed inputs, and standardize a failure taxonomy
   --- then pilot publicly, with reporting entities, repositories,
   market-data users, privacy researchers, and smaller implementers, before
   mandating.

Evaluate regulator-observable Shielded systems, defined below, on their data
and governance paths. Whether a Dark architecture --- no opening path beyond
a frozen leakage function and authorized local outputs --- can satisfy every
reporting, correction, examination, and enforcement objective is a question
analysis cannot yet close; test it obligation by obligation, not with the
word "private."

= The worked example, viewed as records

Dragon's Clutch is a fully collateralized conditional-asset market over an
objectively verifiable onchain price. A depositor locks collateral into a
segregated single-market pool and receives a complete claim set: one claim
per cell of an exhaustive partition, or a smooth basis-function family whose
complete set is provably constant in aggregate. Claims trade in a batch venue
with a stated close; a deterministic observation rule binds an authenticated
source history to the outcome; settlement pays from the pool. The programs
run in local test banks and an author-operated research deployment on
Solana's devnet test cluster [DEVNET RECORD: program identifiers, build
hashes, and deployment slot], with no customer anything. The default build
refuses every deposit until an authenticated data-source release is compiled
in (none is); local tests run the source lifecycle against a deliberately
non-production stand-in provider. At each milestone: which records exist,
which are public bytes, and which exist only if somebody must keep them?

*Publication.* The market template --- partition rule, observation-program
version, batch policy, settlement terms --- goes on the ledger: all public,
none reportable. No parties, funding, or exposure exist to report --- a
hash-to-trade adapter would invent a trade with no participants. Ledger
writes precede reportable events.

*Funding and issuance.* A wallet deposits one unit of collateral and receives
a complete set, one claim per band: reportable funding and gross issuance, whose confidential
record should keep deposit, claims issued, parties, amounts, time, and
provenance. Public bytes (pool, wallet, amount, time) are precise yet do not
establish the controller's legal or beneficial identity. Net contingent
exposure is zero --- a complete set plus its unconditional recombination
right is fully hedged, interchangeable with the deposit --- and arises when
the depositor sells some claims and keeps others, possibly in a venue whose
orders never touch this ledger; labeling funding itself net contingent
exposure invents exposure, and ignoring offchain events misses where it
arises. Supervision
needs confidential account-to-party linkage reading funding, issuance, later
orders, the unbalancing transaction, and redemption as one owner's lifecycle.

*Close and match.* At the stated close the book freezes and a deterministic
rule clears it at one consistent price set. The transparent offline prototype
gives the verifier the frozen book, claiming no order concealment; a proposed
Shielded form could restrict individual orders to a named executor or
committee, the ledger carrying only a root over the accepted set, clearing
prices, and aggregate fills. Either way, what market-conduct examination most
needs --- rejections before admission, modifications, cancellations, arrival
sequence --- may never become ledger bytes, having never changed ledger
state: the venue reports them or nobody does.

Machine verification peaks here: with the clearing rule deterministic over a
frozen book, any authorized holder can recompute the result, and my batch
verifier accepts a clearing only if recomputation reproduces it exactly,
never trusting claimed quantities --- the rule, not a format check, is the
check. The onchain evidence is narrower. Local campaigns settle one
same-page, full-fill, single-claim, zero-fee slice from funded reservations
and a prefrozen receipt, with replay and substitution refusals. The live
batch path submits bounded full-width candidates; full top-three selection
once consumed exactly the 1,400,000-compute-unit transaction ceiling and
rolled back --- a measured stop whose cause proved to be a software SHA-256
implementation, not the verification design; with the runtime's hashing
syscall it completes and commits at 226,071 compute units. Its staged
successor, split across bounded transactions, clears the ceiling on every
measured row in a single-bank-profile campaign. Onchain selection is not the verification
story; any future selection claim covers only the best valid submitted
candidate admitted before an immutable close boundary.

Two bounded artifacts supply a worked negative. A synthetic energy-dispatch
relation (three providers, three periods, two buses, integer outputs,
deterministic 156-byte witness and 176-byte result codecs): a Clear oracle
examines 8,025 trajectory pairs, finds 468 feasible complete schedules,
selects canonical objective 56, and rejects a fully recommitted, physically
feasible, exactly settlement-conserving cost-60 plan solely by repeating the
frozen global-optimum and tie rule. A `tfhe-rs` evaluator whose evaluation
API holds no client key checks caller-supplied encrypted dispatches for
feasibility and exact settlement conservation: cost 56 and feasible cost 60
pass, a forged cost-59 settlement fails --- but only the Clear enumeration
establishes 56 as optimal; the encrypted evaluator does no global search.
Both are synthetic, single-process experiments: no energy-market validation,
verifiable-encryption proof, custody design, or anything Dark. A
predicate-checking validator can accept a feasible, conserving, wrong
submission; only recomputing the complete frozen rule verifies a selection.

*Resolution.* Between close and resolution any outcome may realize; each
claim trades at a price. Evidence may be submitted, but nothing is
authoritative until the frozen observation rule admits the complete required
history and the repair period lapses --- then no discretionary adjudicator
chooses. Local campaigns derive and authenticate one canonical, sealed,
program-owned source receipt, refusing substitutes. Reporting vocabulary
should distinguish pending, disputed, unsupported, expired, and resolved.
Even certified outcomes can be superseded by ledger reorganization or
application-level dispute: correction linkage, not silent overwrite.

*Settlement.* Realized claims redeem from the pool. My campaigns run this end
to end: a 22-transaction signed custody walk through issuance, resolution,
internal and bearer redemption, and full withdrawal, plus per-degree walks
for the smooth claim families. Public bytes show outflows, not which
redemption closes which position of which legal or beneficial owner at what
gain --- the confidential record's linkage, not the public record's.

= Question 3: report economic events, not ledger writes

The counterargument --- the ledger is already a complete, tamper-evident
record --- fails: it is neither complete nor a report. A framework should
distinguish, at minimum: order creation and authorization; funding or
collateral lock; gross claim issuance and recombination; net contingent
exposure, including the complete-set offset and the unbalancing event;
receipt, acceptance, rejection, modification, and cancellation; batch close
and accepted-set inclusion; match, execution, and allocation; pending,
disputed, unsupported, or expired resolution; certification; early exit,
unwind, or replacement; settlement, delivery, maturity, or correction; and
replacement from reorganization or dispute. Request 1 already binds each
event to its ledger sources; the provenance fields:

#table(
  columns: (1.6in, 1fr),
  table.header([*Field*], [*Purpose*]),
  [Network, chain, and deployment identifier], [Disambiguate ledgers, forks, environments, and redeployments.],
  [Block height, block hash, and transaction and event indexes], [Anchor the event to one history; establish exact ordering.],
  [Program address, executable hash, upgrade authority, configuration version], [Bind the event to executable logic and its mutability history, not a mutable name.],
  [Product or rule identifier, terms digest, and schema version], [Bind the event to its economic semantics; reject incompatible decoders.],
  [Accepted-input or batch root], [Bind an aggregate execution to the set admitted for processing.],
  [Finality status and finality observation time], [Separate observed, confirmed, economically final, and protocol-final states.],
  [Reorganization or replacement reference], [Link superseded events without silent deletion.],
  [Confidential account-to-party linkage], [Preserve ownership continuity for regulators without public identification.],
)

Guidance should specify how to report ledger, receipt, execution, and
finality times when they differ, and correction behavior for probabilistic
finality, reorganization, upgrade, and application disputes.

*On proofs.* A proof establishes only the proposition its statement encodes,
relative to named committed inputs, rule versions, and stated assumptions: an
authorization proof does not establish current legal authority without key
and revocation governance, nor a collateral proof custody or absence of
encumbrance, unless those facts are inside the relation and bound to the
actual settlement. Unless the Commissions determine the proof is itself the
required datum, preserve the committed evidence needed to reconstruct the
event, and record proof system, statement version, verifier, public-input
digest, result, and evidence retention location. The encrypted
candidate-validation experiment above is the concrete case: encrypted
execution establishes neither optimality nor its own correct evaluation.

= Question 8: public transparency without strategy publication

At funding, wallet, exact amount, and time became public bytes, permanently;
removing a name does not remove identity. Wallet, timestamp, size, price,
product, and settlement destination can jointly re-identify a trader;
repeated behavior can reveal entry policy, inventory limits, hedge timing, or
an algorithm's responses; in sparse or bespoke markets a single event can be
unique.

Purpose-limited publication is not novel: the real-time public reporting
rules already delay the public print of certain large trades and cap the
disseminated notional.#note_ref(2) New onchain: the regulated tape is not the
only disclosure channel. Solana has no public global mempool --- pre-landing,
a transaction is visible only to the client and whichever RPC operator, relay
or forwarding node, and current or next leader actually receives it (a direct
TPU path can bypass RPC) --- and only a transaction included in a confirmed
block, succeeded or failed, becomes public ledger data.#note_ref("6, 7")
Other ledgers differ. A dissemination review should separate local
pre-landing disclosure to ingress actors from public, durable post-landing
leakage:

#table(
  columns: (1.35in, 1fr),
  table.header([*Leakage surface*], [*What it reveals, independent of any tape*]),
  [RPC ingress], [The operator sees the signed transaction it accepts before landing; preflight rejection or later dropping may survive only in client and operator records, off-ledger.],
  [Relay or forwarding ingress], [Sees the packets it handles: topology- and trust-specific disclosure, not global broadcast.],
  [Leader ingress], [A current or next leader, receiving directly or via RPC, can observe intent before deciding whether it lands --- not public mempool visibility.],
  [Fee payer], [A landed transaction's fee-paying account links otherwise separate transactions; one reused funding account can join strategies into one actor.],
  [Funding graph], [Landed transfers can make the path from an exchange withdrawal or a prior market's payout into a deposit public and durable, connecting positions across markets and time.],
  [Landed failed transaction], [Exposes its signed message and failure metadata without a successful execution; a dropped or preflight-rejected transaction is not public ledger bytes.],
  [Settlement graph], [Public redemptions to original wallets reattach outcomes to the accounts that traded, defeating a proposed Shielded batch's order concealment after the fact.],
)

Per surface, state who can observe it, when, and whether a design closes,
narrows, or leaves it open, measuring the dissemination policy and this
independent leakage both. Caps, delays, buckets, and aggregation must be
explicit, deterministic, and versioned. None of this loosens the confidential
record: exact creation and continuation data with counterparty and
transaction identifiers,#note_ref(3) under repository confidentiality and
access rules.#note_ref(4)

My leakage statements have a deliberately modest basis: a deterministic
laboratory replaying four synthetic trading traces against three transcript
designs, recording which fields each design mechanically reveals and,
separately, which deductions those enable --- no anonymity, cryptographic
leakage, timing, or real-market measurement. Its value is discipline: every
"this design hides X" sentence is checked against an explicit field list ---
the discipline any dissemination policy deserves: enumerate the fields, then
defend each one.

*Clear, Shielded, and Dark.* Three terms, used exactly --- conflating them
yields bad reporting policy in both directions. *Clear:* the specified state
and computation are public. *Shielded:* a named executor, committee, or
auditor may learn private inputs. *Dark:* no actor learns anything beyond a
frozen leakage function and its own authorized local output, within an
explicit corruption model. A due-process threshold opening path is
regulator-observable Shielded, not Dark; an operator-readable encrypted
database is Shielded; unpadded ingress, observable timing, or a public
settlement graph makes a purportedly Dark design Shielded in fact. My
research has produced no end-to-end Dark system --- the TFHE experiment
validates an encrypted candidate without holding the client key but is a
single process, with no custody, release, network, or settlement design ---
and no composed Clear, Shielded, or Dark venue exists.

= Question 19: governed machine-readable rules

The general form of the right answer is already on this docket: the comment
of Ariadne Dataworks Ltd. recommends publishing the reporting framework
itself as structured data --- a joint machine-readable field dictionary as a
versioned schema, executable, openly licensed validation logic with a public
conformance corpus, and a normative lifecycle event model with worked
examples.#note_ref(5) I endorse all three; with the governance that comment
describes, they carry most of what a machine-readable regime needs. Three
layers remain.

*Bind the validators to the three-record structure.* Each record needs its
own schema and validators; the validation package is itself the third record,
containing no transaction data and hence fully publishable even where
everything it validates is confidential. Publication buys independent
implementations that agree on what the rules accept.

*Where a rule is deterministic over committed inputs, publish the validator
in recomputation form.* A conformance corpus establishes only agreement with
published examples; a recomputation validator re-derives the reported answer
from committed inputs and accepts only exact agreement. The worked example's
clearing rule is the strongest form; wherever a reporting rule can take that
shape, disagreement about what happened reduces to disagreement about inputs
--- exactly where a regulator wants it. Correction authority can be equally
concrete: my formal models machine-check that a correction's authorized
actor, target record, permitted fields, and governing rule version can be
fixed in advance and enforced mechanically; "who may correct this report, to
what, under which version" becomes a typed object, not a convention. Require
certification before a pending outcome becomes final regulatory state; keep
rejected or superseded transitions auditable without counting them as trades.

*Standardize a failure taxonomy whose states are genuinely distinct* ---
rejected, pending, unsupported, expired, corrected, reorganized,
backend-unavailable --- holding the first and last apart deliberately.
*VERIFIED (host-side kernel only):* my observation accumulator refuses a
question its retained information cannot support rather than approximate it.
"The rule rejects this" and "the backend cannot answer this" are different
facts; conflating them misreports both --- absorbed outages understate data
quality exactly when the system is least healthy, and the converse hides
violations behind operational noise. Submitting into an outage should not
satisfy an obligation, and the record alone should tell an examiner which
happened.

= Scope

This comment takes no position on the classification of any product;
classification is the subject of a separate joint request. Nothing here
argues for reduced regulatory access because a transaction is onchain, and no
design of mine is claimed to satisfy any current reporting obligation. The
positions are my analysis as a commenter, not legal opinions. The devnet
deployment is research operation with valueless test tokens, not a reporting
system or an offer, and nothing here asks either Commission to approve
anything.

#block(breakable: false)[
  #v(14pt, weak: true)
  Respectfully submitted,

  [FULL NAME]  \
  [AFFILIATION, IF ANY]  \
  [DATE]
]

= Appendix: basis of material technical claims

Each material technical claim, with its basis in one line. None of the
artifacts has been independently audited.

#claim_table(
  columns: (1fr, 2.1in),
  table.header([*Claim*], [*Basis*]),
  [Joint-request coverage (Questions 3, 8, and 19)], [Joint request for comment; source note 1],
  [Delay and notional caps on certain large-trade prints], [17 C.F.R. part 43; source note 2],
  [Exact creation and continuation data, counterparty and transaction identifiers], [17 C.F.R. part 45; source note 3],
  [Repository confidentiality and access rules], [17 C.F.R. part 49; source note 4],
  [The Ariadne Dataworks recommendations as described], [The filed comment; source note 5],
  [Solana ingress visibility; landed-failure publicity], [Solana transaction-ingress guide and RPC JSON structures; source notes 6 and 7],
  [Exact categorical and degree-one through degree-three B-spline payout semantics], [Exact-rational Rust kernels with canonical largest-remainder quantization; Lean proofs of the named construction properties; no whole-implementation refinement proof],
  [Local SBF and devnet execution of construction, resolution, redemption, custody, staged resolution, submission, one settlement slice], [Signed and bank campaigns: the 22-transaction custody walk, per-degree blank-bank walks; full top-three selection completing at 226,071 CU after a hasher fix dissolved its measured 1,400,000-CU rollback stop, its staged successor clearing the ceiling on one bank profile; dated devnet deployment record],
  [Batch-verifier acceptance only on exact recomputation from the frozen book], [Program source and deterministic tests],
  [Accumulator refusal of unsupported questions], [Program source and deterministic tests],
  [One canonical, sealed, program-owned source receipt; deposits refused absent a compiled source release], [Receipt-binding and substitution-refusal tests; source construction against the non-production stand-in provider],
  [Objective 56 selection; cost-60 rejection via frozen optimum and tie rule], [Exact Rust oracle and recomputation verifier over a frozen synthetic relation; deterministic locked corpus],
  [Encrypted feasibility and exact-conservation validation, forged-settlement detection, no client key held], [A `tfhe-rs` experiment reproduced on arm64 and x86_64; no encrypted search, optimality check, custody, or release protocol],
  [Correction authority fixable in advance and mechanically enforced], [Machine-checked theorems in guarded-commitment formal models],
  [Leakage laboratory: four synthetic traces, three transcript designs, revealed fields separated from enabled deductions], [Deterministic synthetic-transcript accounting; measures no anonymity, timing, or real market],
  [The Clear, Shielded, and Dark taxonomy and leakage-surface analysis], [Proposed analytical terminology; no claim that any Dark system exists],
  [Author-operated devnet research deployment, valueless test tokens, deposits refused by default], [Repository status records; the dated devnet deployment record],
)
