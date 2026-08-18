#import "../shared/template.typ": key_point, note_ref

= Summary of positions

I submit this statement as an independent software and formal-methods
researcher, in connection with the Committee's August 20, 2026 public
meeting; the meeting notice provides for written statements received by
August 27, 2026.#note_ref(9)

The Commission has explained that "event contract" is not itself a defined
term in the CEA or CFTC regulations, that a prediction market offering
event contracts in swap or futures form to the general public must register
as a designated contract market, while a swap execution facility may serve
only eligible contract participants.#note_ref("1, 11") Innovation will press hardest where
software distributes over time the events that traditional vocabulary fuses
into "the trade." This statement takes eight positions about how that
pressure should be analyzed and asks the Committee to recommend the work
that would adopt them; the positions are my own analysis as a researcher,
argued from the statutory and regulatory text and one worked market, and
what I report about my research artifacts stays within what those artifacts
support.

1. The Commission should adopt a milestone taxonomy --- publication,
   instrument creation, funding, close, resolution, settlement --- as the
   shared factual clock
   for classifying, auditing, and supervising staged programmable
   transactions, in place of labels such as "prediction," "token," "smart
   contract," or "decentralized."
2. Publication of market software, without more, is not operation of a
   venue or an intermediary; operation should be found from solicitation,
   order handling, matching, custody, oracle control, upgrade and emergency
   control, and transaction-linked compensation, with guidance stating
   which combinations cross the line.
3. Instrument creation freezes a market instance but locks no claim
   collateral; funding locks collateral and issues gross claims; an
   unbalancing transfer creates net contingent outcome exposure. A fully
   prefunded design fixes maximum payout liability when those claims issue.
4. Prefunding can remove novation, credit extension, and loss mutualization,
   but it does not by itself decide DCO status: the statutory definition also
   reaches multilateral settlement or netting. Analysis should identify the
   function actually performed, custody, settlement control, and failure
   handling.
5. The fields a DCM must read in real time are those its surveillance,
   monitoring, and reconstruction obligations actually consume; other
   fields may remain encrypted where exact linked records are timely
   recoverable by and intelligible to the responsible regulated function
   and the Commission.
6. Governed threshold disclosure can satisfy those access objectives
   without routine plaintext visibility to one commercial operator, and
   regulator-observable Shielded --- defined below --- should be the
   reference architecture for privacy-preserving market pilots.
7. Independently verifiable proofs should be admitted as evidence of
   exactly the propositions their statements encode --- named relation,
   version, published negative cases --- with underlying records preserved,
   and a structured predeployment pathway should exist for presenting these
   facts before anything is at stake.
8. Removing settlement discretion relocates manipulation risk to the
   reference market rather than removing it; contract terms should
   therefore state the full reference specification --- venue, statistic,
   sampling grid, window, source-failure rule --- and surveillance should
   consume an assumption-indexed manipulation-cost envelope, not one
   context-free bound.

One question stays open throughout, stated here once: whether any
reporting, surveillance, or enforcement obligation structurally precludes a
Dark architecture --- no general opening path beyond a frozen leakage
function and authorized local outputs --- is a research question my own
work has not answered, and Dark is a long-horizon research boundary, not a
venue label, compliance conclusion, or deployment recommendation.

*Scope.* Current Regulation 40.11 addresses registered-entity listing or
clearing of contracts involving specified enumerated activities; a June
2026 proposal would revise that framework and establish a structured
public-interest review, and remains proposed, not current
law.#note_ref("10, 11") Every example in this statement references
objectively verifiable crypto-native facts --- ledger states, program
events, prices, ranges, path statistics --- and none targets an enumerated
activity under CEA section 5c(c)(5)(C) or Regulation 40.11. This scope
choice is not a claim that any example falls outside the CEA or any other
law.

= One worked market, six milestones

The design is a fully collateralized conditional-asset market over an
objectively verifiable onchain price band. It asks one question: on a
stated future date, in which of five stated price bands will the
time-weighted price of a specific digital asset in a specific onchain
liquidity pool fall? The bands cover every possible price and do not
overlap, the terms include a deterministic rule for every edge case, and
exactly one band must realize. The modeled transition system has no
borrowing, variation-margin call, or liquidation mechanism, and it never
calls for more funds after acquisition. That is not a claim that each claim
is economically unlevered: its payout can exceed a secondary buyer's price,
and external financing is outside the model. The six milestones below walk
the rest of the terms.

I have built the core accounting of this design as an offline research
prototype --- pure Rust, integer-exact, with passing deterministic tests. It
is tested, not formally verified. It is not a deployed system, a product, or
an offer, and I do not ask the Commission to approve it.

Five bands are an instance, not the primitive's limit. The milestone taxonomy
and maximum-liability relation extend to any bounded, frozen finite payout
matrix --- including portfolios representing overlapping ranges and graded
proximity payouts after an explicit discretization and rounding rule. That
does not establish a pricing algorithm for those claims, and a truly
continuous implementation would need separately verified numerical bounds.

*Publication fixes a reusable specification.* The partition, observation
program, batch rule, payout terms, and edge-case rules are inspectable, but
no market instance or claim exists. Nobody has signed, deposited, or
promised anything, and no value can move.

*Instrument creation freezes one market instance.* A creation transition
binds a unique identifier to those terms. It issues no claim and accepts no
claim collateral. It does make later discretion auditable: no one --- author
included --- can substitute a different transition at execution time; an
order executes exactly as committed or not at all (the table's first row).

*Funding locks collateral and issues gross claims.* A depositor locks one
unit of collateral into the market's own pool and receives a complete set:
one transferable claim per band, returnable as a set for its collateral
until resolution. Each claim has a one-unit gross face in its own state, but
the states are exclusive and the payout vector's maximum aggregate liability
is one unit. The complete set therefore has the same outcome value in every
state; net contingent outcome exposure begins only when a transfer unbalances
it. Claim issuance, collateral lock, gross liability, and net exposure are
four distinct facts. In my prototype I made collateral sufficiency
structural:
required collateral is computed as the largest liability any payout vector
in the market's immutable set implies at the current claim supply, rounding
against the protocol, and I built every transition --- market construction
included --- to refuse, as an invariant violation, any state whose
collateral falls below that maximum.

*The close makes the price a fact.* While the batch is open, "the clearing
price" is not yet a fact --- no consistent set of prices exists to report.
The close creates the fact: no further orders will be admitted, and the
clearing becomes arithmetic anyone can redo. I
built my batch prototype to do exactly this: freeze its price grid, tie
rule, and remainder rule when the book is constructed, derive one canonical
clearing, and accept a submitted clearing candidate only if it matches what
the frozen book itself determines, recomputed from scratch --- never
trusting the submitter's claimed quantities.

*Resolution is licensed by admitted evidence.* The observation window and
its repair period close, and the frozen program's admitted evidence
identifies the realized band; until then each band's claim trades at a
price, and nothing in the software makes one outcome authoritative early.
The design may still depend on publishers, proposers, and an evidence-
admission or dispute process. Its narrower property is that, after final
evidence is admitted under the frozen rule, no discretionary adjudicator
chooses the band. The terms must name that rule, the dispute procedure, and
what happens when the source fails; the declared outcome is worth exactly
what that evidence rule is worth.

*Settlement is exact, one-shot, and serialized.* The realized band's claims
redeem from the pool; the other four expire worthless. A redemption executes
exactly the transfer committed at publication, exactly once: a second
presentation of the same accepted redemption is refused, and a presentation
that fails the terms changes nothing. Settlement is also deliberately
serialized --- one act against one ledger --- for the reason the table's
last row exhibits.

= Three machine-checked negatives

The strongest factual claims in the walk above are negatives: each
exhibits, concretely, a way this design pattern fails, and the
counterexample --- not the assurance --- is what an examiner can probe for.
All three are properties of formal models reviewed by me, not of deployed
systems; none is a compliance conclusion.

#table(
  columns: (1.05in, 1fr, 2.05in),
  table.header([*Property*], [*Counterexample shown (formal model)*], [*Consequence for supervision*]),
  [An order can be executed exactly as committed or not at all], [A fill outside the committed limits fails and changes nothing --- a property I have machine-checked in formal models of the commitment pattern; no primitive allows an obligation whose amount or obligor is fixed later], [The permitted fill is readable from the committed order; an unexplained state change is a violation, not an ambiguity],
  [An outcome is authorized only by accepted evidence], [Declaring an outcome before the evidence window closes is not caution but error: a declaration the remaining evidence can falsify, a failure mode I have exhibited concretely in a formal model], [A declared outcome is worth what its evidence rule is worth; read that rule, not the declaration],
  [Settlement is one act against one ledger], [Two withdrawals can each be valid against the same pool and jointly overdraw it --- a fact I have machine-checked], [Balance-type constraints cannot be checked in independent fragments and merged; serialization is a choice to verify, not a guarantee to assume],
)

= The argument

Positions 1 and 3 are argued by the walk above; the milestones give
classification, audit, and enforcement one shared factual clock, and they
are events, not product categories --- a design that lacks one simply
records its absence.

*Position 2: publication versus operation.* The registration triggers are
functional --- soliciting, accepting orders, operating a facility --- and
the closest authority, Staff Letter No. 26-09, is a narrow, conditional,
nonbinding staff analysis in which frontend facts mattered, issued
expressly until a rulemaking addresses software
providers.#note_ref(3) That rulemaking should avoid two symmetric errors:
treating all publication as operation, and treating an
actively operated financial interface as mere publication because its
backend is open-source or immutable; the facts in the position should
decide, with no single factor dispositive.

*Position 4: the DCO boundary.* The CEA's DCO definition lists alternative
functions: substituting credit through novation or otherwise; arranging
multilateral settlement or netting; or otherwise providing clearing that
mutualizes or transfers participants' credit risk.#note_ref(2) Prefunded
atomic settlement can eliminate the first and third features, but that does
not answer whether a particular arrangement performs the second. Part 39's
treatment of fully collateralized positions confirms the caution: full
collateralization can change the applicable risk, yet coexist with DCO
requirements.#note_ref(4) No categorical conclusion follows from the
collateral invariant alone. Analysis should identify the statutory function
actually performed, who holds the pool, who controls finality, and what
happens on failure.

*Positions 5 and 6: privacy-compatible audit.* Current DCM rules include
surveillance, real-time monitoring, trader and position data,
order-lifecycle tracking, reconstruction, and retention
requirements.#note_ref(5) A public transaction hash performs none of those
functions --- so "it's on chain" is not an audit trail --- and Regulation
38.7 already establishes that regulatory data and public commercial data
need not be identical,#note_ref(6) which is precisely the separation a
privacy-compatible audit trail formalizes: a public market layer; a
confidential regulatory layer of owner-linked orders, identity, funding,
positions, and settlement; and governed disclosure. My research uses three words exactly: *Clear* --- the specified state and computation are public;
*Shielded* --- a named executor, committee, or auditor may learn private
inputs; *Dark* --- no actor learns anything beyond a frozen leakage function
and its own authorized local output, within an explicit corruption model. A
due-process threshold opening path is regulator-observable Shielded, not
Dark, and it is the design target these positions defend: the regulated
function and the Commission get exact, timely, intelligible records; no
single commercial operator gets routine plaintext visibility into everyone's
positions. Whether a true Dark architecture is structurally precluded is
the question fenced above.

*Position 7: proofs as scoped evidence.* Useful proof targets exist ---
accepted-input binding, collateral sufficiency, conservation, deterministic
matching, duplicate prevention, consistency between public and confidential
records. A proof claim should name the exact relation and rule version, the
committed inputs, the verifier and result, the assumptions, and what the
proof does *not* establish; the Commission could publish machine-testable
positive and negative conformance examples for each control. Proofs
complement surveillance, governance, and examination rather than replacing
them, and the cheapest moment to examine any of this is before deployment,
through a structured path in the Commission's existing innovation
channels#note_ref(7) --- which vehicle, the Commission's choice.

*Position 8: manipulation cost, stated against my own design pattern.* The
Committee's published agenda names market surveillance and manipulation
concerns for its prediction-markets session.#note_ref(12) My contribution
there is self-critical: the worked market's frozen observation program
does not eliminate reporting or evidence admission. Conditional on final
evidence admitted under the frozen rule, it removes a discretionary
adjudicator's choice of outcome. The incentive to move the thing being
observed is untouched, and the attacker knows in advance which statistic
and sampling structure decide the payout: risk is relocated to the reference
market, not removed.

What becomes computable is not one universal bound. For a declared adversary
and recovery model, one can separately estimate capital required to displace
the necessary prints and net loss after unwind. Vary pool state, fees,
boundary distance, sample count, required hold fraction, recovery, external
flow, arbitrage, capital constraints, latency, transaction costs, and
detection, and the outputs form an assumption-indexed envelope. Surveillance
should compare that envelope with amount at stake as a screening input, not
treat one number as a safety certificate.#note_ref(5) My deterministic
experiment supplies exact arithmetic only for synthetic constant-product
pools and a generous same-pool recovery model; its result is a lower bound
under those stated assumptions, not a measurement or bound for any real
venue. No number appears here. The ask is therefore for the *inputs*, not a
conclusion: require the reference
specification --- venue, statistic, sampling grid, window, source-failure
rule --- in the contract's terms and, machine-readably, in the confidential
record.#note_ref(6) That converts "watch for manipulation" into "watch this
venue at these sample instants, under these declared assumptions."

= The operatorless agent

The Committee's agenda includes artificial intelligence.#note_ref(9) The
architecture in this statement extends to a market participant that is
itself an AI agent and has no operator. A published specification fixes the
agent's entire operating loop in advance, and execution is prepaid,
permissionless work: anyone may perform a step offchain and submit the
result, and the ledger accepts a step only if its certificates verify
against the specification. This is the publication-versus-operation
question of Position 2 in its sharpest form: in such a design there is no
operator to register, and no operator to trust.

Part of the certificate stack this needs is real: my local research
artifacts implement a Lean-authored parse/guard STARK and a genuine
TLSNotary 2PC integration, joined by a shared content commitment and
tested for refusal, with no live model-provider session, no onchain
posting path, and no verifiable-inference backend. Proving the whole
execution history is a named, machine-readable gap, so the executing host
is currently trusted; and the transcript leg pins a named notary, which is
an operator for that function. These artifacts have enumerated trusted
roles; they are not an operatorless system, and no such agent exists.

I request approval of nothing. I ask the Committee to take up one question
early: when no one operates, which operator functions --- supervision,
recordkeeping, emergency authority, accountability for harm --- can
verifiable conduct evidence satisfy, which attach to the specification's
author or its executors, and which have no bearer at all.

= Requested work products

1. A milestone taxonomy for staged programmable transactions ---
   publication, instrument creation, funding, close, resolution, settlement,
   and the events
   between them. (Positions 1 and 3.)
2. Functional guidance distinguishing software publication from operation,
   solicitation, order handling, intermediation, and control. (Position 2.)
3. A statement of which facts govern the clearing analysis of fully
   prefunded, atomically settled designs. (Position 4.)
4. Privacy-compatible audit-trail criteria separating public transparency,
   confidential regulatory records, and governed disclosure. (Positions 5
   and 6.)
5. Independently reviewable proof and control objectives with published
   negative cases, and a structured predeployment path for presenting
   bounded factual matrices before a live product exists. (Position 7.)
6. A reference-specification requirement for contracts that settle by
   observing a named venue, carried machine-readably in the confidential
   record, with surveillance guidance that consumes the resulting
   assumption-indexed manipulation-cost envelope as a screening input.
   (Position 8.)

= Limits

The formal artifacts behind this statement are research models; the market
prototype is offline research code. Independently provenanced repositories
contain separately scoped prototype clearing, proof, and privacy
components; they do not presently compose into a production, permissionless,
end-to-end Dark market system. This statement describes no deployed product,
accepted customer funds, or live orders, and requests approval of nothing.
Machine-checked properties are properties of models, not of deployed
systems, and not compliance conclusions. The Committee's duties are solely
advisory,#note_ref(8) and I ask only that it recommend work.

#block(breakable: false)[
  #v(18pt, weak: true)
  Respectfully submitted,

  [FULL NAME]  \
  [AFFILIATION, IF ANY]  \
  [DATE]
]

= Appendix: basis of material technical claims

Each material technical claim, with its evidentiary basis in one line. A
"model theorem" is a machine-checked statement about a simplified formal
model reviewed by the submitter. No artifact behind these claims is
deployed market infrastructure, and none has been independently audited.

#table(
  columns: (1fr, 2.1in),
  table.header([*Claim*], [*Basis*]),
  [The statement's legal recitals: event-contract terminology and the DCM/SEF statements; the scope of Regulation 40.11 and the June 2026 proposal (proposed, not current law); the CEA's separate facility and clearing definitions; Staff Letter No. 26-09 as narrow, conditional, and nonbinding; the treatment of fully collateralized positions in part 39; the surveillance and audit-trail functions of part 38 and section 38.7], [Primary sources cited in text; source notes 1 through 6, 10, and 11],
  [An order can be executed exactly as committed or not at all; a committed redemption executes at most once; no primitive allows an obligation whose amount or obligor is fixed later], [Model theorems in the submitter's guarded-commitment research; not deployed controls],
  [An outcome declared before the evidence window closes can be falsified by remaining evidence; balance-type constraints cannot be checked in independent fragments and merged], [Model theorems in the submitter's candidate-result formalism; no oracle or legal finality process is implemented or validated],
  [The worked market's staged accounting: instance creation, claim-collateral lock and gross issuance, and unbalancing transfer are distinct; a complete set has constant outcome value until unbalanced; undercollateralized states are refused; a frozen book is fully recomputed; finite payout-matrix generality does not establish pricing or continuous numerics], [Direct consequence of the stated relation plus the offline pure-Rust prototype reviewed by the submitter; deterministic tests pass; tested, not formally verified; not deployed or a legal classification],
  [The manipulation-cost observation: capital required and net unwind loss are different outputs, and declared model assumptions produce an envelope rather than a universal venue bound], [Offline deterministic exact-integer experiment over synthetic constant-product pools, cross-checked by independent computations; lower bound only under its stated recovery model; no market data or measurement or bound for any real venue, and no number appears in this statement],
  table.cell(colspan: 2)[The operatorless-agent artifact sentence --- offline research artifacts and one pinned third-party integration, reviewed by the submitter; test suites independently reproduced from the pinned committed tree: 86 tests, zero failures, under the repository's own toolchain pin (record of August 18, 2026), with the Lean emit step not re-run: the committed emitted descriptor, whose Lean pinning is inherited from the commit, is what the tamper canaries exercised],
  table.cell(colspan: 2)[The operatorless-agent boundary statements --- no live model-provider session (a live exchange-API MPC-TLS session was recorded July 11, 2026), no onchain posting path, no verifiable-inference backend, no deployed agent, no funded market; the executing host and the pinned notary are trusted --- rest on the submitter's repository status records and the pinned session record; research artifacts and open design questions, not products, offers, or compliance conclusions],
  [No artifact described in this statement is deployed, funded, offered, or operating, and the research artifacts do not presently compose into an end-to-end system], [The submitter's repository status records; a statement about the submitter's own artifacts, not about any third party],
)
