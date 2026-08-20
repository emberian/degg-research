#import "../shared/template.typ": claim_table, key_point, note_ref

= What this is, and what it asks

I am an independent software and formal-methods researcher. I built a staged
claim market, Dragon's Clutch, ran it, and measured where it stops. This
statement reports what that produced and asks the Committee to recommend six
work products. The meeting notice provides for written statements received by
August 27, 2026.#note_ref(9)

The Commission has explained that "event contract" is not itself a defined
term in the CEA or CFTC regulations, that a prediction market offering event
contracts in swap or futures form to the general public must register as a
designated contract market, and that a swap execution facility may serve only
eligible contract participants.#note_ref("1, 11") Innovation presses hardest
where that vocabulary assumes one moment of formation. A staged program has
none: authorship, funding, matching, resolution, and settlement happen days
apart, to different people, creating different rights. Eight positions follow
from the build; these six work products would adopt them.

1. A *milestone taxonomy* for staged programmable transactions ---
   publication, instrument creation, funding, close, resolution, settlement,
   and the events between them --- as the shared factual clock for
   classification, audit, and supervision. (Positions 1 and 3.)
2. *Functional guidance separating software publication from operation*,
   naming which combinations of solicitation, order handling,
   intermediation, custody, oracle control, and upgrade and emergency
   control cross the line. (Position 2.)
3. *A statement of which facts govern the clearing analysis* of fully
   prefunded, atomically settled designs. (Position 4.)
4. *Privacy-compatible audit-trail criteria* separating public transparency,
   the exact confidential regulatory record, and governed disclosure.
   (Positions 5 and 6.)
5. *Proof and control objectives with published negative cases*, admitting
   proofs as evidence of exactly what their statements encode, and a
   structured predeployment path for presenting a bounded factual matrix
   before a live product exists. (Position 7.)
6. *A reference-specification requirement* for contracts settling by
   observing a named venue, carried machine-readably in the confidential
   record, with surveillance guidance that consumes the resulting
   assumption-indexed manipulation-cost envelope as a screening input.
   (Position 8.)

= One worked market, six milestones

Dragon's Clutch is a fully collateralized conditional-asset market over an
objectively verifiable onchain price. The simplest market asks one question:
on a stated future date, in which of five stated price bands will the
time-weighted price of a specific digital asset fall? The bands cover every
possible price without overlapping, the terms fix a deterministic rule for
every edge case, and exactly one band must realize. There is no borrowing,
variation-margin call, or liquidation, and no transition ever calls for more
funds after acquisition --- which is not a claim that a claim is economically
unlevered: its payout can exceed a secondary buyer's price, and external
financing is outside the design.

Five bands are an instance, not the limit. The native claim family is smooth:
each claim is one basis function of an open-clamped B-spline of degree one
through three over the frozen grid, positions are integer coefficient
vectors, evaluation is exact rational arithmetic under a canonical
largest-remainder rule whose integer weights sum to a fixed payout
denominator, and portfolios express overlapping ranges and shaped graded
payouts over that finite exact basis. Because the basis functions sum to one
everywhere --- machine-checked for the construction --- a complete set has
the same aggregate value in every state, exactly as one-of-every-band does.
None of this establishes a pricing algorithm or an arbitrary continuous
computer.

The programs execute in local test banks and in an author-operated research
deployment on Solana's devnet test cluster [DEVNET RECORD: program
identifiers, build hashes, and deployment slot], with valueless test tokens
and no customer anything. The default build refuses every deposit until an
authenticated data-source release is compiled into it, and none is. Position
7 below states exactly what my proofs establish and what they do not; the
appendix gives the basis of every claim that follows.

*Publication fixes a reusable specification.* The partition, observation
program, batch rule, payout terms, and edge-case rules are inspectable, but
no market instance or claim exists: nobody has signed, deposited, or promised
anything, and no value can move.

*Instrument creation freezes one market instance.* A creation transition
binds a unique identifier to those terms, issuing no claim and accepting no
claim collateral. It makes later discretion auditable: inside the modeled
relation nobody --- author included --- can substitute a different
transition, and a guarded fill executes only within its committed
authorization (the first negative below). Deployment mutability and upgrade
control remain separate facts. The milestone runs today: from a bank holding
only the program, an ordinary wallet creates a real fixed-supply Token-2022
collateral mint, uploads and seals typed policy, grid, and terms, constructs
canonical realm and profile accounts, and creates categorical or native
markets, including on the devnet deployment. Predictable program-state and
token accounts tolerate honest over-rent prefunding; a byte-bearing late
target refuses atomically. It does not yet construct the general feed, epoch,
candidate, pot, and receipt plane or complete a permissionless lifecycle.

*Funding locks collateral and issues gross claims.* A depositor locks one
unit of collateral into the market's own pool and receives a complete set:
one transferable claim per band, returnable as a set for its collateral until
resolution. Each claim carries a one-unit gross face in its own state, but
the states are exclusive and the payout vector's maximum aggregate liability
is one unit --- so a complete set has the same outcome value in every state,
and net contingent outcome exposure begins only when a transfer unbalances
it. Claim issuance, collateral lock, gross liability, and net exposure are
four distinct facts. Collateral sufficiency is structural here: required
collateral is the largest liability any payout vector in the market's
immutable set implies at the current claim supply, rounded against the
protocol, and every transition --- market construction included --- refuses
as an invariant violation any state whose collateral falls below it.

*The close makes the price a fact.* While the batch is open, "the clearing
price" is not yet a fact; no consistent set of prices exists to report. The
close creates it, and the clearing becomes arithmetic anyone can redo. My
batch prototype freezes its price grid, tie rule, and remainder rule when the
book is constructed, derives one canonical clearing, and accepts a submitted
candidate only if it matches what the frozen book itself determines,
recomputed from scratch --- never trusting the submitter's claimed
quantities.

*Resolution is licensed by admitted evidence.* Under the qualifying design
the observation window and repair period close, an authenticated frozen rule
admits the complete required source history, and only then is the outcome
identified. Conditional on that admission no discretionary adjudicator
chooses the band, though the design may still depend on publishers,
proposers, and an evidence-admission or dispute process. In my campaigns
resolution derives and authenticates the exact canonical program-owned sealed
source receipt and refuses same-domain evidence or account substitution. The
full source lifecycle --- specification, feed, complete single-page archive,
parser-admitted records, maturity receipt --- runs against a deliberately
non-production stand-in provider, because the default build registers no
provider release and refuses source construction and value admission with
error `0x79` before any state is created or changed. The production source
profile I selected targets an oracle network whose documentation defines a
unique update per instant, which removes transaction-timing discretion over
which observation settles the market; it is not yet compiled in. The terms
must name the rule, dispute procedure, and source-failure behavior, because
the outcome is worth exactly what that evidence rule is worth.

*Settlement is exact, one-shot, and serialized in the modeled relation.* The
realized band's claims redeem from the pool and the other four expire
worthless; a redemption executes the authorized transfer at most once, and a
nonconforming presentation changes nothing. My campaigns run this end to end:
a 22-transaction signed sequential custody walk through issuance, resolution,
internal and bearer redemption, and withdrawal of all owned collateral;
per-degree walks with source-receipt-authenticated point resolution and
exact-lot bearer redemption, where a nondivisible lot refuses before any
state changes; and one same-page, full-fill, single-claim, zero-fee
settlement slice from two reservations and a prefrozen receipt. None of that
is a production source pipeline or general partial, portfolio, cross-page,
fee-bearing, or end-to-end venue settlement.

== Verification has a ceiling, and it bites

Re-executing a batch clearing inside one Solana transaction consumes exactly
the 1,400,000-unit limit and fails. That is not a peculiarity of my code: any
design that asks a chain to re-derive a complex result in one transaction
meets the same wall, and the alternatives are staging or succinct proof.
Three measured facts, each a boundary rather than a promise.

*Monolithic native occupation-resolution, degrees one through three ---
measured STOP.* The route executes in local banks, but no measured initial
span of one through three clears the project's 1,120,000-CU admission
threshold for retaining 25-percent headroom under the 1,400,000-CU ceiling;
the best measured initial row is 1,236,364 CU. Later-step retries clear the
threshold, but a retry cannot make the first transition reachable. Spans
above three are unmeasured, so nothing is inferred about them. This is a
decomposition and admission-policy result, not a hardware impossibility or a
production-liveness claim.

*Staged resolution over the same sealed archive bytes --- admitted.* Prepaid
begin, fold, finalize, and abort steps executed in local SBF banks, preserved
output equality with the monolithic route, and cleared the selected
25-percent-headroom profile on every measured row, at per-step maxima of
810,992, 815,573, 1,094,832, and 587,197 CU. That admission covers exactly
the measured route and its selected prepaid zero-charge policy; it is not a
global liveness policy, and not deployment, production-source,
extrapolated-shape, terminal-closure, or transaction-inclusion evidence.

*Batch clearing --- measured STOP, then a redesign.* The first-generation
path initializes, freezes, and submits bounded full-width candidates, but
full top-three selection reaches exactly the 1,400,000-CU ceiling and rolls
back every watched byte and lamport: non-promotable, and the reason
single-transaction re-execution does not scale. The successor stages the work
into placement, freeze, abort, submission with full re-verification,
per-candidate staged verification, selection, settlement with exact asset
transfer, and three lapse phases, each its own bounded transaction; every
measured row clears the ceiling, the tightest a candidate replacement at
1,127,892 CU. That evidence is one bank profile --- five candidates on an
eleven-tick grid --- so wider grids, exact score ties, and reordered retained
accounts remain model and host evidence, and the staged route carries no
compute, rent, or terminal-admission row in the liveness profile that governs
the resolution path. Selection claims are limited to the best valid submitted
candidate admitted before an immutable close boundary; the verifier
recomputes rather than trusting a submitter's claimed score.

= Four machine-checked negatives

The strongest claims above are negatives. Each exhibits, concretely, a way
this design pattern fails, and the counterexample --- not the assurance ---
is what an examiner can probe for. The first three are properties of formal
models reviewed by me; the fourth is a deterministic executable experiment,
not a theorem. None is a property of a deployed system or a compliance
conclusion.

#[
#set table.cell(breakable: true)
#table(
  columns: (1fr, 1.8in),
  table.header([*Property, and the counterexample shown*], [*Consequence for supervision*]),
  [*A guarded transition can execute only an authorized fill.* Separate formal models fix the actor, target, field, predicate, and authorized later value; a guard-violating fill fails closed. The model does not establish that every possible obligation fixes its amount or obligor at creation], [The authorized transition is readable from the committed object; an unexplained state change is a violation, not an ambiguity],
  [*An outcome is authorized only by accepted evidence.* Declaring an outcome before the evidence window closes is not caution but error: a declaration the remaining evidence can falsify, a failure mode I have exhibited concretely in a formal model], [A declared outcome is worth what its evidence rule is worth; read that rule, not the declaration],
  [*Settlement is one act against one ledger.* Two withdrawals can each be valid against the same pool and jointly overdraw it --- a fact I have machine-checked], [Balance-type constraints cannot be checked in independent fragments and merged; serialization is a choice to verify, not a guarantee to assume],
  [*Feasibility plus conservation does not establish optimality.* In a bounded synthetic energy-dispatch relation (three padded providers, three periods, two buses, integer output atoms), an exhaustive Clear oracle examines 8,025 trajectory pairs, finds 468 feasible schedules, and selects canonical objective 56; a fully recommitted cost-60 plan is physically feasible and conserves settlement exactly, yet is rejected only because the verifier recomputes the frozen global optimum and tie rule. A separate bounded encrypted evaluator over `tfhe-rs` integer ciphertexts, holding no client key, validates that cost-60 candidate's feasibility and exact conservation and detects a forged cost-59 settlement --- while deliberately performing no global search, so encrypted execution establishes neither global optimality nor correct evaluation. Synthetic deterministic experiments, not theorems, energy-market evidence, privacy results, or deployments], [A submission that passes every stated predicate can still be the wrong selection under the frozen rule; recomputation of the complete rule, not predicate passing, is what verifies a selection],
)
]

= The eight positions

*1. Adopt a milestone taxonomy.* Publication, instrument creation, funding,
close, resolution, and settlement should be the shared factual clock for
classifying, auditing, and supervising staged programmable transactions, in
place of labels such as "prediction," "token," "smart contract," or
"decentralized."

*2. Publication of market software, without more, is not operation of a venue
or an intermediary.* Operation should be found from solicitation, order
handling, matching, custody, oracle control, upgrade and emergency control,
and transaction-linked compensation, with guidance stating which combinations
cross the line. The registration triggers are functional --- soliciting,
accepting orders, operating a facility --- and the closest authority, Staff
Letter No. 26-09, is a narrow, conditional, nonbinding staff analysis in
which frontend facts mattered, issued expressly until a rulemaking addresses
software providers.#note_ref(3) That rulemaking should avoid two symmetric
errors: treating all publication as operation, and treating an actively
operated financial interface as mere publication because its backend is
open-source or immutable. The facts should decide, with no single factor
dispositive.

*3. The milestones carry distinct accounting.* Instrument creation freezes a
market instance but locks no claim collateral; funding locks collateral and
issues gross claims; an unbalancing transfer creates net contingent outcome
exposure; and a fully prefunded design fixes maximum payout liability when
those claims issue. The walk above argues Positions 1 and 3 together:
milestones are events, not product categories, and a design that lacks one
records its absence.

*4. Prefunding relocates the clearing question; it does not answer it.* The
CEA's DCO definition lists alternative functions: substituting credit through
novation or otherwise; arranging multilateral settlement or netting; or
otherwise providing clearing that mutualizes or transfers participants'
credit risk.#note_ref(2) Prefunded atomic settlement can eliminate the first
and third, but not thereby the second. Part 39's treatment of fully
collateralized positions confirms the caution: full collateralization can
change the applicable risk yet coexist with DCO requirements.#note_ref(4) No
categorical conclusion follows from the collateral invariant alone. Analysis
should identify the statutory function actually performed, who holds the
pool, who controls finality, and what happens on failure.

*5 and 6. Privacy-compatible audit is achievable, and "it's on chain" is not
it.* The fields a DCM must read in real time are those its surveillance,
monitoring, and reconstruction obligations actually consume; other fields may
remain encrypted where exact linked records stay timely recoverable by and
intelligible to the responsible regulated function and the Commission.
Governed threshold disclosure can meet those objectives. Current DCM rules
include surveillance, real-time monitoring, trader and position data,
order-lifecycle tracking, reconstruction, and retention
requirements,#note_ref(5) and a public transaction hash performs none of
them. Regulation 38.7 already establishes that regulatory data and public
commercial data need not be identical,#note_ref(6) which is the separation a
privacy-compatible audit trail formalizes: a public market layer; a
confidential regulatory layer of owner-linked orders, identity, funding,
positions, and settlement; and governed disclosure. My research uses three
words exactly. *Clear:* the specified state and computation are public.
*Shielded:* a named executor, committee, or auditor may learn private inputs.
*Dark:* no actor learns anything beyond a frozen leakage function and its own
authorized local output, within an explicit corruption model. A due-process
threshold opening path is regulator-observable Shielded, not Dark, and
regulator-observable Shielded should be the reference architecture for
privacy-preserving market pilots: the regulated function and the Commission
get exact, timely, intelligible records, and no single commercial operator
gets routine plaintext visibility into everyone's positions. Whether any
reporting, surveillance, or enforcement obligation structurally precludes a
Dark architecture is a research question my own work has not answered; Dark
is a long-horizon research boundary, not a venue label, a compliance
conclusion, or a deployment recommendation.

*7. Admit proofs as evidence of exactly the propositions their statements
encode.* Useful targets exist: accepted-input binding, collateral
sufficiency, conservation, deterministic matching, duplicate prevention,
consistency between public and confidential records. My own proofs show the
scoping discipline --- named Lean theorems establish model-level
complete-set, solvency, guarded-transition, and B-spline construction and
quantization properties, one pinned Verus run checks a single
internal-transfer arithmetic seam, and no checked refinement connects them to
the complete Rust or SBF runtime. A proof claim should name the exact
relation and rule version, committed inputs, verifier and result,
assumptions, and what the proof does *not* establish, with the underlying
records preserved; the Commission could publish machine-testable positive and
negative conformance examples for each control. Proofs complement
surveillance, governance, and examination rather than replacing them, and the
cheapest moment to examine any of this is before deployment, through a
structured path in the Commission's existing innovation
channels#note_ref(7) --- which vehicle, the Commission's choice.

*8. Removing settlement discretion relocates manipulation risk to the
reference market; it does not remove it.* The Committee's agenda names market
surveillance and manipulation concerns for its prediction-markets
session,#note_ref(12) and my contribution there is self-critical. The
qualifying frozen observation design does not eliminate reporting or evidence
admission; conditional on final authenticated evidence admitted under the
frozen rule, it removes a discretionary adjudicator's choice of outcome. The
incentive to move the thing being observed is untouched, and the attacker
knows in advance which statistic and sampling structure decide the payout.
What becomes computable is not one universal bound. For a declared adversary
and recovery model one can separately estimate the capital required to
displace the necessary prints and the net loss after unwind; varying pool
state, fees, boundary distance, sample count, required hold fraction,
recovery, external flow, arbitrage, capital constraints, latency, transaction
costs, and detection produces an assumption-indexed envelope. Surveillance
should compare that envelope with the amount at stake as a screening input,
not treat one number as a safety certificate.#note_ref(5) My deterministic
experiment supplies exact arithmetic only for synthetic constant-product
pools and a generous same-pool recovery model; its result is a lower bound
under those stated assumptions, not a measurement or bound for any real
venue, and no number appears here. The ask is therefore for the *inputs*:
require the reference specification --- venue, statistic, sampling grid,
window, source-failure rule --- in the contract's terms and, machine-readably,
in the confidential record.#note_ref(6) That converts "watch for
manipulation" into "watch this venue at these sample instants, under these
declared assumptions."

= The operatorless agent

The Committee's agenda includes artificial intelligence.#note_ref(9)
*PROPOSED research question.* The sharpest form of Position 2 is a market
participant that is itself an AI agent and has no operator: a published
specification fixes the operating loop, and prepaid, permissionless executors
submit steps a ledger accepts only when their certificates verify.

Part of the certificate stack this needs is real. My local research artifacts
implement a Lean-authored parse/guard STARK and a genuine TLSNotary 2PC
integration, joined by a shared content commitment and tested for refusal,
with no live model-provider session, no onchain posting path, and no
verifiable-inference backend. Part is not: proving the whole execution
history is a named, machine-readable gap, so the executing host is currently
trusted, and the transcript leg pins a named notary, who is an operator for
that function. These artifacts have enumerated trusted roles; they are not an
operatorless system, and no such agent exists. The market artifacts are no
further along --- some local transitions can be invoked by an ordinary wallet
against frozen program state, but provider availability, transaction
inclusion, deployment and upgrade control, source-release registration,
unresolved-work funding, private-key release, and terminal recovery remain
named dependencies, and no current artifact has eliminated an operator
function.

The question is worth taking up before such an agent exists: when no one
operates, which operator functions --- supervision, recordkeeping, emergency
authority, accountability for harm --- can verifiable conduct evidence
satisfy, which attach to the specification's author or its executors, and
which have no bearer at all.

= Scope

Current Regulation 40.11 addresses registered-entity listing or clearing of
contracts involving specified enumerated activities; a June 2026 proposal
would revise that framework and establish a structured public-interest
review, and remains proposed, not current law.#note_ref("10, 11") Every
example here references objectively verifiable crypto-native facts --- ledger
states, program events, prices, ranges, path statistics --- and none targets
an enumerated activity under CEA section 5c(c)(5)(C) or Regulation 40.11;
that scope choice is not a claim that any example falls outside the CEA or
any other law. The positions are my analysis as a researcher, not legal
opinions, and I request approval of nothing. The components do not presently
compose into a production, permissionless, source-authenticated, end-to-end
market system of any privacy modality, and the devnet deployment is not a
product or an offer. Machine-checked properties are properties of models, and
no refinement proof connects them to whole-protocol runtime behavior. The
Committee's duties are solely advisory,#note_ref(8) and I ask only that it
recommend work.

#block(breakable: false)[
  #v(14pt, weak: true)
  Respectfully submitted,

  [FULL NAME]  \
  [AFFILIATION, IF ANY]  \
  [DATE]
]

= Appendix: basis of material technical claims

Each material technical claim, with its evidentiary basis in one line. None
of the artifacts has been independently audited.

#claim_table(
  columns: (1fr, 1.85in),
  table.header([*Claim*], [*Basis*]),
  [The legal recitals: event-contract terminology and the DCM/SEF statements; the scope of Regulation 40.11 and the June 2026 proposal (proposed, not current law); the CEA's separate facility and clearing definitions; Staff Letter No. 26-09 as narrow, conditional, and nonbinding; fully collateralized positions under part 39; the surveillance and audit-trail functions of part 38 and section 38.7], [Primary sources cited in text; source notes 1 through 6, 10, and 11],
  [A guarded transition can fix actor, target, field, predicate, and an authorized later value; a guard-violating fill fails closed; a modeled redemption is one-shot], [Named theorems in separate guarded-commitment models; no claim that every obligation fixes amount or obligor at creation and no deployed control],
  [An outcome declared before the evidence window closes can be falsified by remaining evidence; balance-type constraints cannot be checked in independent fragments and merged], [Model theorems in the submitter's candidate-result formalism; no oracle or legal finality process is implemented or validated],
  [The staged accounting distinguishes creation, collateral lock, gross issuance, and unbalancing transfer; the B-spline claim families have machine-checked complete-set and solvency properties], [Exact Rust kernels with oracle tests; Lean theorems for the named model properties; one Verus-checked arithmetic seam; no whole-implementation refinement proof],
  [Construction, custody, point-resolution, redemption, staged resolution, the staged clearing lifecycle, and settlement execute in local SBF tests and the devnet research deployment], [Local signed and bank campaigns including the 22-transaction custody walk, per-degree blank-bank walks, and a five-candidate staged-clearing campaign on one bank profile; the measured compute boundaries stated above; the dated devnet deployment record],
  [Resolution is controlled by one canonical, sealed, program-owned source receipt; the default build refuses deposits with no compiled source release], [Local receipt-binding and substitution-refusal tests; source construction exercised against the non-production stand-in provider],
  [The Clear energy-dispatch relation selects canonical objective 56 and rejects a feasible, conserving cost-60 plan only by recomputing the frozen optimum and tie rule], [Exact Rust oracle and recomputation verifier over a frozen synthetic relation; deterministic locked corpus],
  [An encrypted evaluator validates a candidate's feasibility and exact settlement conservation, and detects a forged settlement, without holding the client key], [A `tfhe-rs` experiment reproduced on arm64 and x86_64; no encrypted search, optimality check, custody, or release protocol],
  [The manipulation-cost observation: capital required and net unwind loss are different outputs, and declared model assumptions produce an envelope rather than a universal venue bound], [Offline deterministic exact-integer experiment over synthetic constant-product pools, cross-checked independently; a lower bound only under its stated recovery model; no market data, no measurement or bound for any real venue, and no number in this statement],
  table.cell(colspan: 2)[The operatorless-agent artifact sentence --- offline research artifacts and one pinned third-party integration, reviewed by the submitter; test suites independently reproduced from the pinned committed tree: 86 tests, zero failures, under the repository's own toolchain pin (record of August 18, 2026), with the Lean emit step not re-run: the committed emitted descriptor, whose Lean pinning is inherited from the commit, is what the tamper canaries exercised],
  table.cell(colspan: 2)[The operatorless-agent boundary statements --- no live model-provider session (a live exchange-API MPC-TLS session was recorded July 11, 2026), no onchain posting path, no verifiable-inference backend, no deployed agent, no funded market; the executing host and the pinned notary are trusted --- rest on the submitter's repository status records and the pinned session record; research artifacts and open design questions, not products, offers, or compliance conclusions],
  [The devnet deployment is author-operated research with valueless test tokens; the default build refuses deposits; the components do not compose into an end-to-end system], [Repository status records and the dated devnet deployment record],
)
