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
only eligible contract participants.#note_ref(1)#note_ref(11) Innovation will press hardest where
software distributes over time the events that traditional vocabulary fuses
into "the trade." This statement takes eight positions about how that
pressure should be analyzed and asks the Committee to recommend the work
that would adopt them; the positions are my own analysis as a researcher,
argued from the statutory and regulatory text and one worked market, and
what I report about my research artifacts stays within what those artifacts
support.

1. The Commission should adopt a milestone taxonomy --- publication,
   funding, close, resolution, settlement --- as the shared factual clock
   for classifying, auditing, and supervising staged programmable
   transactions, in place of labels such as "prediction," "token," "smart
   contract," or "decentralized."
2. Publication of market software, without more, is not operation of a
   venue or an intermediary; operation should be found from solicitation,
   order handling, matching, custody, oracle control, upgrade and emergency
   control, and transaction-linked compensation, with guidance stating
   which combinations cross the line.
3. Economic exposure arises at funding, when collateral locks against a
   contingency; the published template creates none, and a fully prefunded
   design fixes the exposure's ceiling in the same act that creates it.
4. A fully prefunded, fully collateralized claim set settled atomically ---
   no novation, credit extension, or loss mutualization --- has none of the
   credit-intermediation features that motivate DCO-style regulation;
   clearing analysis should turn on custody, settlement control, and
   default handling, and the Commission should say so.
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
   consume the manipulation-cost bound those inputs make computable.

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
law.#note_ref(10)#note_ref(11) Every example in this statement references
objectively verifiable crypto-native facts --- ledger states, program
events, prices, ranges, path statistics --- and none targets an enumerated
activity under CEA section 5c(c)(5)(C) or Regulation 40.11. This scope
choice is not a claim that any example falls outside the CEA or any other
law.

= One worked market, five milestones

The design is a fully collateralized conditional-asset market over an
objectively verifiable onchain price band. It asks one question: on a
stated future date, in which of five stated price bands will the
time-weighted price of a specific digital asset in a specific onchain
liquidity pool fall? The bands cover every possible price and do not
overlap, the terms include a deterministic rule for every edge case, and
exactly one band must realize. There is no debt, margin, leverage, or
liquidation anywhere in the design; every participant's maximum loss is
fixed when their collateral or premium is paid. The five milestones below
walk the rest of the terms.

I have built the core accounting of this design as an offline research
prototype --- pure Rust, integer-exact, with passing deterministic tests. It
is tested, not formally verified. It is not a deployed system, a product, or
an offer, and I do not ask the Commission to approve it.

*Publication fixes the terms.* The template is published: partition,
observation program, batch rule, payout terms, and edge-case rules are all
inspectable. Nobody has signed, deposited, or promised anything; no value
can move; everything that will ever be discretionary about this market is
already visible. The design gives no one --- author included --- a way to
owe an amount fixed later or to substitute a different transition at
execution time: an order executes exactly as committed or not at all (the
table's first row).

*Funding creates the exposure and fixes its ceiling.* A depositor locks one
unit of collateral into the market's own pool and receives a complete set:
one transferable claim per band, returnable as a set for its collateral
until resolution. Exposure now exists --- and the same act fixes its ceiling, because the pool holds the
full deposit and covers every payout the frozen terms allow, by
construction. In my prototype I made the check structural:
required collateral is computed as the largest liability any payout vector
in the market's immutable set implies at the current claim supply, rounding
against the protocol, and I built every transition --- market construction
included --- to refuse, as an invariant violation, any state whose
collateral falls below that maximum. One refinement matters: a complete
set plus the recombination right is fully hedged, so funding and the first
unbalancing sale are distinct facts, and contingent exposure begins with
the sale.

*The close makes the price a fact.* While the batch is open, "the clearing
price" is not yet a fact --- no consistent set of prices exists to report.
The close creates the fact: no further orders will be admitted, and the
clearing becomes arithmetic anyone can redo. I
built my batch prototype to do exactly this: freeze its price grid, tie
rule, and remainder rule when the book is constructed, derive one canonical
clearing, and accept a submitted clearing candidate only if it matches what
the frozen book itself determines, recomputed from scratch --- never
trusting the submitter's claimed quantities.

*Resolution is licensed by evidence.* The observation window and its repair
period close, and the frozen program's accepted evidence identifies the
realized band; until then each band's claim trades at a price, and nothing
in the software makes one outcome authoritative early --- there is no
reporter and no discretion. The terms must name, in advance, the evidence
rule, the dispute procedure, and what happens when the source fails; the
declared outcome is worth exactly what that evidence rule is worth.

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

*Position 4: the DCO boundary.* The CEA defines clearing functionally ---
substituting credit, netting, mutualizing or allocating counterparty
risk.#note_ref(2) An atomically settled, fully prefunded claim set performs
none of those functions: nothing is novated, no credit is extended, no loss
is mutualized, and the pool can never owe more than it holds. Part 39
itself supplies the counterargument --- full collateralization coexists
there with substantial institutional requirements#note_ref(4) --- but that
shows collateralization does not excuse an entity performing clearing
functions, not that prefunded atomic settlement performs them. The analysis of such designs should
therefore turn on the facts that remain live: who holds the pool, who
controls finality, and what happens on failure.

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
eliminates reporter discretion --- no person chooses the reported value ---
and that is exactly why the residual risk must be named. The incentive to
move the thing being observed is untouched, and the attack is easier to
plan, because the attacker knows in advance which statistic over which
window decides the payout: the risk is relocated, from a reporter who could
lie to the reference market itself, not removed. The relocation buys
quantifiability --- for a contract settling on a specific onchain liquidity
pool, the inputs that price the attack (pool depth, fee, sampling grid,
window length) are public, so the cost of moving the settlement statistic
far enough to change which outcome pays can be computed in advance and
compared with the amount at stake --- and surveillance#note_ref(5) should
consume that bound as a screening input for allocating attention. A
computed cost is a model output, sensitive to assumptions, so the ask is
for the *inputs* rather than a conclusion: require the reference
specification --- venue, statistic, sampling grid, window, source-failure
rule --- in the contract's terms and, machine-readably, in the confidential
record.#note_ref(6) That converts "watch for manipulation" into "watch this
venue during these minutes." I have produced no manipulation-cost
measurement and no study of any real venue, and no number appears in this
statement.

= Requested work products

1. A milestone taxonomy for staged programmable transactions ---
   publication, funding, close, resolution, settlement, and the events
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
   manipulation-cost bound as a screening input. (Position 8.)

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
  [The worked market's accounting: structural refusal of undercollateralized states (required collateral as the maximum liability over the immutable payout set); batch clearing verified by full recomputation of a frozen book], [Offline pure-Rust research prototype reviewed by the submitter; deterministic tests pass; tested, not formally verified; not deployed],
  [The manipulation-cost observation: the inputs that price a settlement-moving trade on a named onchain venue are public, and the cost bound is computable in advance], [Drafting analysis; no manipulation-cost measurement or study of any real venue has been produced, and no number appears in this statement],
  [No artifact described in this statement is deployed, funded, offered, or operating, and the research artifacts do not presently compose into an end-to-end system], [The submitter's repository status records; a statement about the submitter's own artifacts, not about any third party],
)
