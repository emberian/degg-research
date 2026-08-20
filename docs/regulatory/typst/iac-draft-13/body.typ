#import "../shared/template.typ": claim_table, key_point, note_ref

= What this is, and what it asks

I am an independent software and formal-methods researcher. I built a staged
claim market, Dragon's Clutch, ran it, and measured where it stops. This
statement reports the results and asks the Committee to recommend six work
products; the meeting notice provides for written statements received by
August 27, 2026.#note_ref(9)

The Commission has explained: "event contract" is not a defined term in the
CEA or CFTC regulations; a prediction market offering event contracts in swap
or futures form to the general public must register as a designated contract
market (DCM); a swap execution facility may serve only eligible contract
participants.#note_ref("1, 11") That vocabulary assumes one moment of
formation; a staged program has none --- authorship, funding, matching,
resolution, and settlement happen days apart, to different people, creating
different rights. Eight positions follow from the build; six work products
would adopt them:

1. A *milestone taxonomy* (the six milestones below, and the events between)
   as the shared factual clock for classifying, auditing, and supervising
   staged programmable transactions. (Positions 1 and 3.)
2. *Functional guidance separating software publication from operation*,
   naming which factor combinations cross the line. (Position 2.)
3. *A statement of which facts govern the clearing analysis* of fully
   prefunded, atomically settled designs. (Position 4.)
4. *Privacy-compatible audit-trail criteria*. (Positions 5 and 6.)
5. *Proof and control objectives with published negative cases*, and a
   structured predeployment presentation path. (Position 7.)
6. *A reference-specification requirement* for contracts settling by
   observing a named venue. (Position 8.)

= One worked market, six milestones

Dragon's Clutch is a fully collateralized conditional-asset market over an
objectively verifiable onchain price. The simplest market asks which of five
stated price bands the time-weighted price of a specific digital asset will
occupy on a stated future date; the bands cover every price without
overlapping, a deterministic rule fixes every edge case, and exactly one band
must realize. No borrowing, variation-margin calls, or liquidation; no
transition
calls for funds after acquisition --- not economic unleverage: a payout can
exceed a secondary buyer's price, and external financing is outside the
design.

Five bands are an instance. Natively each claim is one basis function of an
open-clamped B-spline, degree one through three, over the frozen grid;
positions are integer coefficient vectors; evaluation is exact rational
arithmetic under a canonical largest-remainder rule whose integer weights sum
to a fixed payout denominator; portfolios express overlapping ranges and
shaped graded payouts on that finite exact basis. The basis sums to one
everywhere (machine-checked for the construction), so a complete set has the
same aggregate value in every state. No pricing algorithm or arbitrary
continuous computer is established.

The programs run in local test banks and an author-operated research
deployment on Solana's devnet test cluster [DEVNET RECORD: program
identifiers, build hashes, and deployment slot] --- valueless test tokens, no
customer anything. The default build refuses every deposit until an
authenticated data-source release is compiled in; none is. The appendix gives
every claim's basis.

*Publication fixes a reusable specification.* Partition, observation program,
batch rule, payout terms, and edge-case rules become inspectable; no market
instance or claim exists, nobody has signed, deposited, or promised anything,
and no value can move.

*Instrument creation freezes one market instance.* A creation transition
binds a unique identifier to the terms; no claim issues, no claim collateral
is accepted, and in the modeled relation nobody --- author included --- can
substitute a different transition or fill outside a committed authorization
(first negative below). Deployment mutability and upgrade control are
separate facts. It runs today: from a bank holding only the program, an
ordinary wallet creates a real fixed-supply Token-2022 collateral mint;
uploads and seals typed policy, grid, and terms; constructs canonical realm
and profile accounts; and creates categorical or native markets, devnet
included. Predictable program-state and token accounts tolerate honest
over-rent prefunding; a byte-bearing late target refuses atomically. Not yet:
the general feed, epoch, candidate, pot, and receipt plane, or a complete
permissionless lifecycle.

*Funding locks collateral and issues gross claims.* One collateral unit,
locked in the market's own pool, buys a complete set (one transferable claim
per band), returnable together for the collateral until resolution. Each
claim's state carries a one-unit gross face; the states are exclusive, so
aggregate payout liability caps at one unit, a complete set is worth the same
in every state, and net contingent outcome exposure begins when a transfer
unbalances one. Issuance, collateral lock, gross liability, net exposure:
four distinct facts. Sufficiency is structural: required collateral is the largest
liability any payout vector in the immutable set implies at current supply,
rounded against the protocol; every transition, construction included,
refuses lesser states as invariant violations.

*The close makes the price a fact.* An open batch has no consistent price set
to report; the close creates one, making the clearing arithmetic anyone can
redo. My prototype freezes price grid, tie rule, and remainder rule at book
construction, derives one canonical clearing, and accepts a submitted
candidate only if it matches the frozen book's own recomputed determination
--- never the submitter's claimed quantities.

*Resolution is licensed by admitted evidence.* The qualifying design
identifies the outcome only after the observation window and repair period
close and an authenticated frozen rule admits the complete required source
history; given that admission no discretionary adjudicator chooses the band,
though publishers, proposers, and an evidence-admission or dispute process
may remain. My campaigns derive and authenticate the exact canonical
program-owned sealed source receipt, refusing same-domain evidence or account
substitution; the full source lifecycle (specification, feed, complete
single-page archive, parser-admitted records, maturity receipt) runs against
a deliberately non-production stand-in provider, and the default build
registers no provider release, failing source construction and value
admission with error `0x79` before any state is created or changed. The
selected production profile targets an oracle network whose documentation
defines a unique update per instant --- no transaction-timing discretion over
which observation settles the market --- and is not yet compiled in. Terms
must name the rule, dispute procedure, and source-failure behavior.

*Settlement is exact, one-shot, and serialized in the modeled relation.* The
realized band's claims redeem from the pool, the other four expire worthless,
a redemption executes the authorized transfer at most once, and a
nonconforming presentation changes nothing. My campaigns run it end to end: a
22-transaction signed sequential custody walk through issuance, resolution,
internal and bearer redemption, and withdrawal of all owned collateral;
per-degree walks with source-receipt-authenticated point resolution and
exact-lot bearer redemption (a nondivisible lot refuses before any state
changes); and one same-page, full-fill, single-claim, zero-fee settlement
slice from two reservations and a prefrozen receipt. None of that is a
production source pipeline or general partial, portfolio, cross-page,
fee-bearing, or end-to-end venue settlement.

== Verification has a ceiling; attribution is measurement work

A chain asked to re-derive a complex result in one transaction meets a hard
compute ceiling --- 1,400,000 CU, of which this project admits a route only
under 1,120,000 (25-percent headroom); the alternatives at scale are staging
or succinct proof. Two routes measured as hard stops against that line. Both
stops were real; I attributed both to the architecture, and the attribution
was wrong.

*The stops, as first measured.* Monolithic native occupation-resolution,
degrees one through three: no initial span cleared admission, best row
1,236,364 CU. First-generation batch clearing: full top-three selection
consumed exactly the 1,400,000-CU ceiling and rolled back every watched byte
and lamport. I read both as "single-transaction re-execution does not scale."

*The cause was a hash implementation, not an architecture.* The program
compiled in a software SHA-256. Replacing it with the runtime's hashing
syscall --- no digest value changed; equivalence is tested at every call site
and pinned against independently computed reference hashes --- made every
measured instruction three to eight times cheaper. On the resealed artifact
every measured occupation initial row is admitted (172,665--197,766 CU);
staged resolution (prepaid begin, fold, finalize, and abort steps,
output-equal to the monolithic route) shows per-step maxima of 90,924,
95,505, 164,287, and 46,677 CU; and the formerly stopped selection completes
and commits at 226,071 CU, 16 percent of the ceiling. Spans above degree
three remain unmeasured --- nothing is inferred about them. The admissions
cover exactly the measured routes and their prepaid zero-charge policy: not
a global liveness policy, nor deployment, production-source,
extrapolated-shape, terminal-closure, or transaction-inclusion evidence.

*What survives the correction.* The staged successor stands on growth, not
on the old constants: placement, freeze, abort, submission with full
re-verification, per-candidate staged verification, selection, settlement
with exact asset transfer, and three lapse phases, each a bounded
transaction --- measured worst row 383,909 CU at freeze, candidate
replacement 203,128 CU. One bank profile (five candidates, eleven-tick grid)
is the evidence --- wider grids, exact score ties, and reordered retained
accounts remain model and host evidence, and the staged route carries no
compute, rent, or terminal-admission row in the liveness profile governing
resolution. Selection means the best valid submitted candidate admitted
before an immutable close boundary; the verifier recomputes rather than
trusting a claimed score.

A measured stop is evidence about an artifact, not an architecture; only a
cause-level change and re-measurement tell those apart. An earlier draft of
this statement carried the wrong generalization; this one carries the
correction.

= Four machine-checked negatives

The strongest claims above are negatives: each concretely exhibits a way this
pattern fails, and the counterexample --- not the assurance --- is what an
examiner can probe. The first three are formal-model properties I reviewed;
the fourth is a deterministic executable experiment, not a theorem.

#[
#set table.cell(breakable: true)
#table(
  columns: (1fr, 1.8in),
  table.header([*Property, and the counterexample shown*], [*Consequence for supervision*]),
  [*A guarded transition executes only an authorized fill.* Separate formal models fix actor, target, field, predicate, and authorized later value; a guard-violating fill fails closed. Not established: that every possible obligation fixes its amount or obligor at creation], [The authorized transition is readable from the committed object; an unexplained state change is a violation, not an ambiguity],
  [*An outcome is authorized only by accepted evidence.* Declaring an outcome before the evidence window closes is not caution but error --- a declaration the remaining evidence can falsify, exhibited concretely in a formal model], [A declared outcome is worth what its evidence rule is worth; read the rule, not the declaration],
  [*Settlement is one act against one ledger.* Two withdrawals can each be valid against the same pool and jointly overdraw it --- machine-checked], [Balance-type constraints cannot be checked in independent fragments and merged; serialization is a choice to verify, not a guarantee to assume],
  [*Feasibility plus conservation does not establish optimality.* In a bounded synthetic energy-dispatch relation (three padded providers, three periods, two buses, integer output atoms), an exhaustive Clear oracle examines 8,025 trajectory pairs, finds 468 feasible schedules, and selects canonical objective 56; a fully recommitted, physically feasible, exactly conserving cost-60 plan is rejected only by recomputing the frozen global optimum and tie rule. A bounded encrypted evaluator over `tfhe-rs` integer ciphertexts, holding no client key, validates that cost-60 candidate's feasibility and exact conservation and detects a forged cost-59 settlement, with no global search --- encrypted execution establishes neither global optimality nor correct evaluation. Synthetic deterministic experiments: not theorems, energy-market evidence, privacy results, or deployments], [A submission passing every stated predicate can still be the wrong selection under the frozen rule; recomputing the complete rule, not predicate passing, verifies a selection],
)
]

= The eight positions

*1. Adopt the milestone taxonomy* (work product 1) in place of labels such as
"prediction," "token," "smart contract," or "decentralized."

*2. Publication of market software, without more, is not operation of a venue
or an intermediary.* Find operation from solicitation, order handling,
matching, custody, oracle control, upgrade and emergency control, and
transaction-linked compensation. The registration triggers are functional
(soliciting, accepting orders, operating a facility), and the closest
authority, Staff Letter No. 26-09, is a narrow, conditional, nonbinding staff
analysis in which frontend facts mattered, issued expressly until a
rulemaking addresses software providers.#note_ref(3) That rulemaking should
avoid two symmetric errors: treating all publication as operation, and
treating an actively operated financial interface as mere publication because
its backend is open-source or immutable. The facts should decide; no single
factor is dispositive.

*3. The milestones carry distinct accounting*, as the walk shows --- and a
fully prefunded design fixes maximum payout liability when claims issue.
Milestones are events, not product categories; a design lacking one records
its absence.

*4. Prefunding relocates the clearing question; it does not answer it.* The
CEA's DCO definition lists alternative functions: substituting credit through
novation or otherwise; arranging multilateral settlement or netting; or
otherwise providing clearing that mutualizes or transfers participants'
credit risk.#note_ref(2) Prefunded atomic settlement can eliminate the first
and third, not thereby the second. Part 39's treatment of fully
collateralized positions confirms the caution: full collateralization can
change the applicable risk yet coexist with DCO requirements.#note_ref(4) No
categorical conclusion follows from the collateral invariant alone; identify
the statutory function actually performed, who holds the pool, who controls
finality, and what happens on failure.

*5 and 6. Privacy-compatible audit is achievable, and "it's on chain" is not
it.* A DCM must read in real time exactly the fields its surveillance,
monitoring, and reconstruction obligations consume; other fields may stay
encrypted where exact linked records remain timely recoverable by and
intelligible to the responsible regulated function and the Commission ---
objectives governed threshold disclosure can meet. Current DCM rules require
surveillance, real-time monitoring, trader and position data, order-lifecycle
tracking, reconstruction, and retention,#note_ref(5) and a public transaction
hash performs none of them. Regulation 38.7 already establishes that
regulatory and public commercial data need not be identical#note_ref(6) ---
the separation a privacy-compatible audit trail formalizes: a public market
layer; a confidential regulatory layer of owner-linked orders, identity,
funding, positions, and settlement; governed disclosure. My research uses
three words exactly. *Clear:* the specified state and computation are public.
*Shielded:* a named executor, committee, or auditor may learn private inputs.
*Dark:* no actor learns anything beyond a frozen leakage function and its own
authorized local output, within an explicit corruption model. A due-process
threshold opening path is regulator-observable Shielded, not Dark --- and
regulator-observable Shielded should be the reference architecture for
privacy-preserving market pilots: the regulated function and the Commission
get exact records; no single commercial operator gets routine plaintext
visibility into everyone's positions. Whether any reporting, surveillance, or
enforcement obligation structurally precludes Dark is a question my work has
not answered; Dark is a long-horizon research boundary, not a venue label,
compliance conclusion, or deployment recommendation.

*7. Admit proofs as evidence of exactly the propositions their statements
encode.* Useful targets: accepted-input binding, collateral sufficiency,
conservation, deterministic matching, duplicate prevention, consistency
between public and confidential records. My own proofs show the scoping
discipline: named Lean theorems establish model-level complete-set, solvency,
guarded-transition, and B-spline construction and quantization properties;
one pinned Verus run checks a single internal-transfer arithmetic seam; none
is refined to the complete Rust or SBF runtime. A proof
claim should name the exact relation and rule version, committed inputs,
verifier and result, assumptions, and what it does *not* establish, with
underlying records preserved; the Commission could publish machine-testable
positive and negative conformance examples per control. Proofs complement
surveillance, governance, and examination, never replacing them; the cheapest
examination is predeployment --- a bounded factual matrix presented before a
live product exists, through the Commission's existing innovation
channels#note_ref(7) (which vehicle, the Commission's choice).

*8. Removing settlement discretion relocates manipulation risk to the
reference market; it does not remove it.* The agenda names market
surveillance and manipulation concerns for the prediction-markets
session.#note_ref(12) Given evidence admitted under the frozen rule, the
qualifying design removes the adjudicator's discretion (resolution milestone
above) --- not reporting, not evidence admission, and not the incentive to
move the thing observed; the attacker knows in advance which statistic and
sampling structure decide the payout. Computable is an envelope, not one
universal bound: for a declared adversary and recovery model, the capital to
displace the necessary prints and the net loss after unwind are separate
estimates, indexed by pool state, fees, boundary distance, sample count,
required hold fraction, recovery, external flow, arbitrage, capital
constraints, latency, transaction costs, and detection. Surveillance should
compare that envelope with the amount at stake as a screening input, not
treat one number as a safety certificate.#note_ref(5) My deterministic
experiment gives exact arithmetic only for synthetic constant-product pools
under a generous same-pool recovery model: a lower bound under those stated
assumptions, not a measurement or bound for any real venue, and no number
appears here. The ask is for the *inputs* --- require the reference
specification (venue, statistic, sampling grid, window, source-failure rule)
in the contract's terms and, machine-readably, in the confidential
record.#note_ref(6) That converts "watch for manipulation" into "watch this
venue at these sample instants, under these declared assumptions."

= The operatorless agent

The Committee's agenda includes artificial intelligence.#note_ref(9)
*PROPOSED research question.* The sharpest form of Position 2 is a market
participant that is itself an AI agent with no operator: a published
specification fixes the operating loop, and prepaid, permissionless executors
submit steps a ledger accepts only when their certificates verify.

Part of that certificate stack is real: my local research artifacts implement
a Lean-authored parse/guard STARK and a genuine TLSNotary 2PC integration,
joined by a shared content commitment and tested for refusal --- no live
model-provider session, no onchain posting path, no verifiable-inference
backend. Part is not: proving the whole execution history is a named,
machine-readable gap, so the executing host is trusted, and the transcript
leg pins a named notary, an operator for that function. The artifacts have
enumerated trusted roles; they are not an operatorless system; no such agent
exists. The market artifacts are no further along: an ordinary wallet can
invoke some local transitions against frozen program state, but provider
availability, transaction inclusion, deployment and upgrade control,
source-release registration, unresolved-work funding, private-key release,
and terminal recovery remain named dependencies; no current artifact has
eliminated an operator function.

The question is worth taking up before such an agent exists: when no one
operates, which operator functions (supervision, recordkeeping, emergency
authority, accountability for harm) can verifiable conduct evidence satisfy,
which attach to the specification's author or its executors, and which have
no bearer at all.

= Scope

Current Regulation 40.11 addresses registered-entity listing or clearing of
contracts involving specified enumerated activities; a June 2026 proposal
would revise that framework and establish a structured public-interest
review, and remains proposed, not current law.#note_ref("10, 11") Every
example here references objectively verifiable crypto-native facts (ledger
states, program events, prices, ranges, path statistics) and none targets an
enumerated activity under CEA section 5c(c)(5)(C) or Regulation 40.11; that
scope choice is not a claim that any example falls outside the CEA or any
other law. The positions are my analysis as a researcher, not legal opinions,
and I request approval of nothing. The components do not presently compose
into a production, permissionless, source-authenticated, end-to-end market
system of any privacy modality; the devnet deployment is not a product or an
offer. Machine-checked properties are properties of models, and no refinement
proof connects them to whole-protocol runtime behavior. The Committee's
duties are solely advisory,#note_ref(8) and I ask only that it recommend
work.

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
  [The legal recitals (event-contract terminology, DCM/SEF statements, Regulation 40.11 and the June 2026 proposal, CEA facility and clearing definitions, Staff Letter No. 26-09, part 39 fully collateralized positions, part 38 and section 38.7 audit functions)], [Primary sources cited in text; source notes 1 through 6, 10, and 11],
  [The guarded-transition properties: authorized fills fixing actor, target, field, predicate, and later value; fail-closed guards; one-shot modeled redemption], [Named theorems in separate guarded-commitment models; no deployed control],
  [Premature-outcome falsifiability; non-mergeability of fragment-checked balance constraints], [Model theorems in the submitter's candidate-result formalism; no oracle or legal finality process implemented or validated],
  [The staged accounting; machine-checked B-spline complete-set and solvency properties], [Exact Rust kernels with oracle tests; Lean theorems for the named model properties; one Verus-checked arithmetic seam; no whole-implementation refinement proof],
  [Milestone execution (construction, custody, point-resolution, redemption, staged resolution, staged clearing, settlement) in local SBF tests and on devnet], [Local signed and bank campaigns: the 22-transaction custody walk, per-degree blank-bank walks, a five-candidate staged-clearing campaign on one bank profile; the measured compute boundaries above; the dated devnet deployment record],
  [Receipt-controlled resolution; default-build deposit refusal], [Local receipt-binding and substitution-refusal tests; source construction exercised against the non-production stand-in provider],
  [The Clear relation's canonical objective 56 and its rejection of the conserving cost-60 plan], [Exact Rust oracle and recomputation verifier over a frozen synthetic relation; deterministic locked corpus],
  [The encrypted-evaluator validation and forgery detection], [A `tfhe-rs` experiment reproduced on arm64 and x86_64; no encrypted search, optimality check, custody, or release protocol],
  [The manipulation-cost observation], [Offline deterministic exact-integer experiment over synthetic constant-product pools, cross-checked independently; a lower bound only under its stated recovery model; no market data, no measurement or bound for any real venue, no number in this statement],
  table.cell(colspan: 2)[The operatorless-agent artifact sentence (offline research artifacts and one pinned third-party integration, reviewed by the submitter): test suites independently reproduced from the pinned committed tree, 86 tests, zero failures, under the repository's own toolchain pin (record of August 18, 2026), the Lean emit step not re-run --- the tamper canaries exercised the committed emitted descriptor, whose Lean pinning is inherited from the commit],
  table.cell(colspan: 2)[The operatorless-agent boundary statements (no live model-provider session --- a live exchange-API MPC-TLS session was recorded July 11, 2026 --- no onchain posting path, no verifiable-inference backend, no deployed agent, no funded market; the executing host and the pinned notary are trusted) rest on the submitter's repository status records and the pinned session record; research artifacts and open design questions, not products, offers, or compliance conclusions],
  [The devnet-deployment characterization: author-operated research, valueless test tokens, no end-to-end composition], [Repository status records and the dated devnet deployment record],
)
