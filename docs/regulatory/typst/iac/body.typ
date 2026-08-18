#import "../shared/template.typ": key_point, note_ref

= Summary of positions

I submit this statement as an independent software and formal-methods
researcher, in connection with the Committee's August 20, 2026 public
meeting; the meeting notice provides for written statements received by
August 27, 2026.#note_ref(9)

The Commission explains that "event contract" is not itself a defined term
in the CEA or CFTC regulations. It states that a prediction market offering
event contracts in swap or futures form to the general public must register
as a designated contract market, while a swap execution facility may make
swaps available for trading only to eligible contract
participants.#note_ref(1)#note_ref(11) Innovation will press on those lines,
and it will press hardest where software distributes over time the events
that traditional vocabulary fuses into "the trade." This statement takes
seven positions about how that pressure should be analyzed and asks the
Committee to recommend the work that would adopt them. The positions are my
own analysis as a researcher, argued from the statutory and regulatory text
and one worked market; what I report about my research artifacts stays
within what those artifacts support.

1. The Commission should adopt a milestone taxonomy --- publication,
   funding, close, resolution, settlement --- as the shared factual clock
   for classifying, auditing, and supervising staged programmable
   transactions, in place of labels such as "prediction," "token," "smart
   contract," or "decentralized."
2. Publication of market software, without more, is not operation of a
   venue or an intermediary; operation should be found from solicitation,
   order handling, matching, custody, oracle control, upgrade and emergency
   control, and transaction-linked compensation, and Commission-level
   guidance should state which combinations cross the line.
3. Economic exposure arises at funding, when collateral locks against a
   contingency; the published template creates none, and a fully prefunded
   design fixes the exposure's ceiling in the same act that creates it.
4. A fully prefunded, non-leveraged, fully collateralized claim set settled
   atomically --- no novation, no credit extension, no loss
   mutualization --- has none of the credit-intermediation features that
   motivate DCO-style regulation, so the clearing analysis of such designs
   should turn on custody, settlement control, and default handling, and
   the Commission should say so.
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
   version, and published negative cases --- with underlying records
   preserved, and a structured predeployment pathway should let developers
   present these facts before anything is at stake.

Two questions stay open. Whether any reporting, surveillance, or
enforcement obligation structurally precludes a Dark architecture --- no
general opening path beyond a frozen leakage function and authorized local
outputs --- is a research question my own work has not answered; Dark is a
long-horizon research boundary, not a venue label, compliance conclusion,
or deployment recommendation. And which procedural vehicle fits
predeployment review is the Commission's choice, not an analytical
conclusion.

*Scope.* Current Regulation 40.11 addresses registered-entity listing or
clearing of contracts involving specified enumerated activities. A June 2026
proposal would revise that framework, define additional terms, and establish
a structured public-interest review; it remains proposed, not current
law.#note_ref(10)#note_ref(11) Every example in this statement references
objectively verifiable crypto-native facts --- ledger states, program
events, prices, ranges, path statistics --- and none targets an enumerated
activity under CEA section 5c(c)(5)(C) or Regulation 40.11. This scope
choice is not a claim that any example falls outside the CEA or any other
law.

= One worked market, five milestones

The design is a fully collateralized conditional-asset market over an
objectively verifiable onchain price band. It asks one question: on a stated
future date, in which of five stated price bands will the time-weighted
price of a specific digital asset in a specific onchain liquidity pool fall?
The bands cover every possible price and do not overlap, the terms include a
deterministic rule for every edge case, and exactly one band must realize. A
depositor locks one unit of collateral into a pool that belongs to this
market alone and receives one claim for each band; a complete set of five
claims can be returned for its collateral at any time before resolution;
individual claims are ordinary transferable assets; orders accumulate in a
batch that closes at a stated time and clears by a frozen deterministic
rule; a frozen observation program later identifies the realized band; and
settlement pays that band's claims from the pool. There is no debt, margin,
leverage, or liquidation anywhere in the design; every participant's
maximum loss is fixed when their collateral or premium is paid.

I have built the core accounting of this design as an offline research
prototype --- pure Rust, integer-exact, with passing deterministic tests. It
is tested, not formally verified. It is not a deployed system, a product, or
an offer, and I do not ask the Commission to approve it.

*Publication fixes the terms.* The template is published: the partition, the
observation program, the batch rule, the payout terms, and the edge-case
rules are all inspectable. Nobody has signed, deposited, or promised
anything, and no value can move. Everything that will ever be discretionary
about this market is already visible, and nothing is yet at risk. The design
gives no one --- author included --- a way to owe an amount fixed later or
to substitute a different transition at execution time: an order can be
executed exactly as committed or not at all, a property I have machine-checked
in formal models of the commitment pattern.

*Funding creates the exposure and fixes its ceiling.* A depositor locks
collateral and receives the complete set of five claims. Exposure now
exists --- and the same act that creates it fixes its ceiling, because the
pool holds the full deposit and covers every payout the frozen terms allow,
by construction of the terms. In my prototype I made the check structural:
required collateral is computed as the largest liability any payout vector
in the market's immutable set implies at the current claim supply, rounding
against the protocol, and I built every transition --- market construction
included --- to refuse, as an invariant violation, any state whose
collateral falls below that maximum. One refinement matters: a complete set
plus the recombination right is fully hedged, interchangeable with its own
collateral, so funding and the first unbalancing sale are distinct facts,
and a participant's contingent exposure begins with the sale.

*The close makes the price a fact.* While the batch is open, "the clearing
price" is not yet a fact --- different gateways may have seen different
orders, and no consistent set of prices exists to report. The close is the
act that creates the fact: a frozen rule that no further orders will be
admitted. After the close, the clearing is arithmetic anyone can redo. I
built my batch prototype to do exactly this: freeze its price grid, tie
rule, and remainder rule when the book is constructed, derive one canonical
clearing, and accept a submitted clearing candidate only if it matches what
the frozen book itself determines, recomputed from scratch --- never
trusting the submitter's claimed quantities.

*Resolution is licensed by evidence.* The observation window and its repair
period close, and the frozen program's accepted evidence identifies the
realized band. Until then the outcome is genuinely undetermined: each band's
claim trades at a price, and nothing in the software makes one outcome
authoritative early --- there is no reporter and no discretion. Declaring an
outcome before the evidence window closes is not caution but error: a
declaration the remaining evidence can falsify, a failure mode I have
exhibited concretely in a formal model. The terms must name, in advance,
the evidence rule, the dispute procedure, and what happens when the source
fails; the declared outcome is worth exactly what that evidence rule is
worth.

*Settlement is exact, one-shot, and serialized.* The realized band's claims
redeem from the pool; the other four expire worthless. A redemption executes
exactly the transfer committed at publication, exactly once: a second
presentation of the same accepted redemption is refused, and a presentation
that fails the terms changes nothing. Settlement is also deliberately
serialized. Two withdrawals can each be valid against the same pool and
jointly overdraw it, so anything that behaves like a balance or a position
limit cannot be checked in independent fragments and merged afterward --- a
fact I have machine-checked, and one the design treats as load-bearing:
settlement is one act against one ledger.

#table(
  columns: (1in, 1.85in, 1fr),
  table.header([*Milestone*], [*What becomes fixed*], [*What the analysis reads here*]),
  [Publication], [The complete terms: parties possible, states, conditions, effects, deadlines], [No party, no funding, no exposure; inspection begins, regulation should not attach.],
  [Funding], [The exposure and its ceiling], [Exposure exists; its ceiling and funding source are readable from the terms.],
  [Close], [Admission ends; the book freezes], [The clearing becomes verifiable by recomputation; venue functions are performed here.],
  [Resolution], [One outcome, selected by accepted evidence], [The named evidence rule, dispute procedure, and source-failure behavior carry the outcome's authority.],
  [Settlement], [The committed transfers execute], [Performance of the earlier instrument, serialized against one ledger.],
)

= The argument

Positions 1 and 3 are argued by the walk above. Every product description
currently invents its own clock; the milestones give classification, audit,
and enforcement one shared factual clock, and they are events, not product
categories --- a design that lacks one simply records its absence.

*Position 2: publication versus operation.* The registration triggers are
functional --- soliciting, accepting orders, operating a facility --- and
the closest authority is Staff Letter No. 26-09, a narrow, conditional,
nonbinding staff analysis in which frontend facts mattered, issued expressly
until a Commission rulemaking or guidance addresses software
providers.#note_ref(3) That rulemaking should avoid two symmetric errors:
treating all software publication as market operation, and treating an
actively operated financial interface as mere publication because its
backend is open-source or immutable. The facts in the position should
decide, and no single factor should be dispositive.

*Position 4: the DCO boundary.* The CEA defines clearing functionally ---
substituting credit, netting, mutualizing or allocating counterparty
risk.#note_ref(2) An atomically settled, fully prefunded claim set performs
none of those functions: nothing is novated, no credit is extended, no loss
is mutualized, and the pool can never owe more than it holds. The
counterargument comes from Part 39 itself, which shows that full
collateralization can coexist with substantial institutional
requirements#note_ref(4) --- but that shows collateralization does not
excuse an entity that performs clearing functions, not that prefunded
atomic settlement performs them. The analysis of such designs should
therefore turn on the facts that remain live: who holds the pool, who
controls finality, and what happens on failure.

*Positions 5 and 6: privacy-compatible audit.* Current DCM rules include
surveillance, real-time monitoring, trader and position data,
order-lifecycle tracking, reconstruction, and retention
requirements.#note_ref(5) A public transaction hash performs none of those
functions --- so "it's on chain" is not an audit trail --- and Regulation
38.7 already establishes that regulatory data and public commercial data
need not be identical,#note_ref(6) which is precisely the separation a
privacy-compatible audit trail formalizes: a public market layer, a
confidential regulatory layer of owner-linked orders, identity, funding,
positions, and settlement, and governed disclosure separating commercial
access from authorized regulatory access. My research uses three words with exact
meanings: *Clear* --- the specified state and computation are public;
*Shielded* --- a named executor, committee, or auditor may learn private
inputs; *Dark* --- no actor learns anything beyond a frozen leakage function
and its own authorized local output, within an explicit corruption model. A
due-process threshold opening path is regulator-observable Shielded, not
Dark, and it is the design target these positions defend: the regulated
function and the Commission get exact, timely, intelligible records; no
single commercial operator gets routine plaintext visibility into everyone's
positions. Whether any obligation structurally precludes a true Dark
architecture is the open research question fenced above.

*Position 7: proofs as scoped evidence.* Useful proof targets exist ---
accepted-input binding, collateral sufficiency, conservation, deterministic
matching, duplicate prevention, consistency between public and confidential
records. A proof claim should name the exact relation and rule version, the
committed inputs, the verifier and result, the assumptions, and what the
proof does *not* establish; the Commission could publish machine-testable
positive and negative conformance examples for each control. Proofs
complement surveillance, governance, and examination rather than replacing
them --- and the cheapest moment to examine any of this is before
deployment, through a structured path in the Commission's existing
innovation channels.#note_ref(7)

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
  #v(18pt)
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
  [The Commission's description of event-contract terminology, general-public DCM registration, and the SEF/ECP limitation], [Commission releases; source notes 1 and 11],
  [Regulation 40.11's current scope, and the June 2026 proposal's status as proposed, not current law], [17 C.F.R. section 40.11; 91 Fed. Reg. 35806; source notes 10 and 11],
  [The CEA separately defines trading-facility, swap-execution-facility, and derivatives-clearing-organization functions], [7 U.S.C. section 1a; source note 2],
  [Staff Letter No. 26-09 is a narrow, conditional, nonbinding staff analysis in which frontend facts mattered], [Source note 3],
  [Part 39 accommodates fully collateralized positions alongside substantial institutional requirements], [17 C.F.R. part 39; source note 4],
  [Current DCM rules include surveillance, monitoring, audit-trail, reconstruction, and retention functions; regulatory and public data need not be identical], [17 C.F.R. part 38 and section 38.7; source notes 5 and 6],
  [An order can be executed exactly as committed or not at all; a committed redemption executes at most once; no primitive allows an obligation whose amount or obligor is fixed later], [Model theorems in the submitter's guarded-commitment research (a weak prototype and a current generalization); not deployed controls],
  [An outcome declared before the evidence window closes can be falsified by remaining evidence; balance-type constraints cannot be checked in independent fragments and merged], [Model theorems in the submitter's candidate-result formalism; no oracle or legal finality process is implemented or validated],
  [The worked market's accounting: structural refusal of undercollateralized states (required collateral as the maximum liability over the immutable payout set); batch clearing verified by full recomputation of a frozen book], [Offline pure-Rust research prototype reviewed by the submitter; deterministic tests pass; tested, not formally verified; not deployed],
  [No artifact described in this statement is deployed, funded, offered, or operating, and the research artifacts do not presently compose into an end-to-end system], [The submitter's repository status records; a statement about the submitter's own artifacts, not about any third party],
)
