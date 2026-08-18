#import "../shared/template.typ": key_point, note_ref

= Summary of positions

The Commissions ask how to distinguish swaps, mixed swaps, security-based
swaps, securities, and instruments excluded from the definition of swap, and
whether new or revised rules or interpretations are warranted.#note_ref(1)
I write in response to Questions 1 and 8, and I take no position on
Questions 12 through 15 concerning alternative compliance.

I am a software and formal-methods researcher; I study programmable markets
whose legal and economic character changes as code is published, signed,
funded, matched, resolved, and settled. This comment takes positions --- my
own analysis as a commenter, argued from the statutory text and one worked
example; what it reports about my research artifacts is stated separately,
and stays within what those artifacts support.

1. Classification of a staged program should be decided milestone by
   milestone, from the economic facts in force at the milestone being
   classified, through interpretive guidance within the framework of the
   2012 Product Definitions Adopting Release.#note_ref(3)
2. On the worked example's stipulated facts, publication alone issues no
   claim and locks no collateral. Deposit is its first instrument-bearing
   milestone, and later order interaction presents distinct venue and conduct
   facts; other terms may produce other milestones.
3. Formation should be analyzed from the rights and obligations created at
   each milestone, not from a universal proxy such as publication, signature,
   or funding. On the worked example's stipulated terms, deposit is the first
   event that locks collateral and issues claims; publication and a revocable,
   unfilled instruction do neither. Other terms may produce a different
   answer.
4. Guidance should distinguish three questions that this architecture makes
   easy to conflate: the gross classification of each issued claim; the
   holder's net economic exposure across claims; and the effect of a
   reversible complete-set operation. In the worked example each band claim
   has a contingent payoff, while one of every band has a constant aggregate
   payout and may be recombined for collateral before resolution.
5. In answer to Question 8, the Commissions should use complete-set
   decomposition as a diagnostic, not assume that economic equivalence is a
   legal axiom. For any proposed criterion, guidance should identify the unit
   classified, state whether gross rights or net exposure controls, and
   explain any change in result when the holder uses an economically
   reversible deposit or recombination operation.
6. In the worked example, full prefunding, bounded loss under fully paid
   terms, and deterministic fail-closed settlement are risk facts --- material
   where credit or customer-protection rules make them relevant --- not
   classification exclusions or proof that economic leverage is absent.
7. The instrument, the venue, the intermediary, and clearing and settlement
   are separate findings that arise at different milestones and can attach
   to different persons; "the software" is not a unit of classification.

For the worked example only, I assume the referenced digital asset is a
non-security commodity and the settlement amount depends only on that
commodity's pool price. On those assumed facts, the example is not offered
as a security-based-swap case. Whether any actual digital asset is a security
or commodity is outside this comment. Substituting a single security, a
narrow-based security index, or an issuer-related event would require the
separate analysis specified in the security-based-swap definition and in
Question 8;#note_ref("1, 4") this comment leaves that classification
open. One further matter I leave open is how much weight transferability
should carry under a particular statutory prong.

= The worked example

A market asks one question: on a stated future date, in which of five stated
price bands will the time-weighted price of a stipulated non-security digital
commodity in a specific onchain liquidity pool fall? The legal status of any
actual asset is not assumed. The bands cover every possible price and do not
overlap --- exactly one must realize --- and the market's terms include a
deterministic rule for every edge case: a missing observation, a malformed
data page, a price exactly on a boundary.

The market's terms are frozen when it is created:

- *Deposit.* Anyone may deposit one unit of collateral into the market's
  segregated pool and receive a complete set: one claim for each of the five
  bands. The pool belongs to this market alone and pays only its claimants.
- *Recombination.* Anyone holding a complete set may return it at any time
  before resolution and withdraw the unit of collateral. A complete set and
  its collateral are interchangeable throughout the market's life.
- *Trading.* Individual claims are separately transferable. Orders
  submitted to the market's batch venue accumulate until a stated close; at
  close the book freezes and a deterministic rule clears it at one
  consistent set of prices. An order fixes who is acting, which balances may
  change, and the exact limits of the permitted fill; a fill outside those
  limits fails and changes nothing --- a property I have machine-checked in
  formal models of this order pattern.
- *Observation.* A frozen program reads authenticated price history from the
  named onchain source over the stated window and computes the realized
  band. No person chooses the reported value: a transaction either carries
  evidence that satisfies the frozen rule or it is rejected. This removes
  reporting discretion; it does not prevent trading from influencing the
  underlying price.
- *Settlement.* After the observation window and its repair period close,
  the realized band's claims redeem from the pool at the stated payout; the
  other four expire worthless. Because every complete set was issued against
  a full deposit, the pool covers the maximum payout the terms allow, by
  construction of the terms.

The worked example permits no borrowing and no undercollateralized issuance:
complete sets are issued only against the full stated payout. It has no
margin-call or liquidation mechanism. Those facts do not imply an absence of
economic leverage; a claim bought for a limited price may have a larger
state-contingent payout. Under the example's fully paid, long-only terms, a
participant's maximum contractual payoff loss is the collateral deposited or
purchase price paid, apart from fees. That statement does not address custody,
implementation, collateral-value, or other operational loss.

I have implemented the core of this design as an offline research prototype:
a pure-Rust transition kernel with integer-exact arithmetic covering the
worked example's accounting --- deposit, recombination, resolution,
redemption --- with its conservation and pool-coverage checks, together with
observation-accumulation and batch-clearing prototypes. Its deterministic
tests pass. It is tested, not formally verified. It is not a deployed
system, a product, or an offer, and I do not ask either Commission to
approve it; it has no deployed program, no keys, no customers, and no funds.
I use it here to show that the staged structure is concrete enough to build
and test.

*Scope.* Every example in this comment references objectively verifiable
crypto-native facts: ledger states, program events, prices, ranges, and path
statistics. None references politics, sports, gaming, or subjective social
events. Regulation 40.11 and a pending June 2026 proposal address event
contracts involving enumerated activities and public-interest
review;#note_ref("5, 6") the examples here are chosen to stay away
from that boundary. This scope choice is not a claim that any example falls
outside the CEA or any other law.

The table walks the example through its stages; the right-hand column states
the answer the positions give at each one.

#table(
  columns: (1.15in, 1.55in, 1fr),
  table.header([*Stage*], [*Economic fact*], [*The position's answer*]),
  [Authored policy], [Reusable software text; no parties], [On the stipulated facts, no claim, collateral lock, or transaction yet; surrounding conduct remains a separate inquiry.],
  [Signed instruction], [Authenticated, revocable, and unfilled direction], [In this example it issues no claim and locks no collateral; different legal or technical terms may differ.],
  [Funded commitment], [Collateral locks; the complete set issues], [The example's first instrument-bearing milestone: five gross claims, with constant aggregate exposure while held as a complete set.],
  [Early exit, compression, or unwind], [A right is canceled, netted, transferred, or closed before maturity], [Termination of the existing instrument, unless the terms deliver a new right or reintroduce discretion --- read the terms for exactly that.],
  [Admitted evidence], [The frozen rule accepts qualifying evidence], [Evidence of a fixed fact where the rule leaves no choice among economic outcomes; otherwise a discretion fact, analyzed as such.],
  [Interaction or match], [Orders interact; a transfer may execute], [Venue or intermediary functions may arise here; make that finding separately from product classification.],
  [Resolution], [One outcome becomes authorized], [The named evidence rule, dispute procedure, and source-failure behavior carry the outcome's authority.],
  [Settlement or issuance], [Balances move or claims are delivered], [Performance of the earlier instrument; a delivered continuing claim restarts analysis.],
  [Secondary transfer], [A resulting claim continues to trade], [A continuing claim if its terms are unchanged; new venue and participant-conduct facts, found separately.],
)

The core distinction the table applies: technical incompleteness is not
economic incompleteness. At batch close nobody knows which band will
realize --- yet consideration is paid, collateral is locked, the outcomes
are exhaustively enumerated, each payout is stated, and nobody retains
discretion to change any of it; the same arithmetic run as an unfunded
local simulation binds nobody and moves nothing. Classification should
read the economic facts, not the software's description of its own state.

= The argument

== Position 1: classify milestone by milestone

The statute classifies transactions, not codebases: CEA section 1a(47)
reaches "any agreement, contract, or transaction" of the described
kinds.#note_ref(1) A staged program separates into distinct dated events
what a bilateral confirmation fuses into one signing --- authorship,
signature, funding, interaction, resolution, and settlement can be days
apart, with different parties present and different rights enforceable at
each --- so the statutory question has a potentially different answer at each
milestone, and guidance that does not name the milestone it analyzes will
be applied inconsistently. Question 1 asks whether new interpretations
are warranted "[t]aking into account" the 2012 release;#note_ref(3) a
milestone matrix is an answer in exactly that form. The strongest
counterargument is gamesmanship --- designers relabeling stages to defer the
regulated moment. But the matrix reads rights, obligations, collateral lock,
issuance, transfer, and exposure, which are facts; renaming a stage changes
nothing unless the facts change, and where they genuinely change, different
treatment is accuracy, not evasion.

== Position 2: publication alone is not the worked example's operative event

At publication in the worked example there are no transaction parties, no
issued claims, and no collateral lock. On those facts, treating publication
as the product-formation event would classify reusable text rather than an
agreement, contract, or transaction. Publication can still be part of a
larger course of conduct --- solicitation, interface operation,
transaction-linked compensation, or pre-authorized funding. Those surrounding
facts require their own analysis under Position 7; publication supplies no
safe harbor for them.

== Position 3: identify the first instrument-bearing milestone from the terms

The swap definition reaches an "agreement, contract, or transaction" within
its enumerated prongs.#note_ref(1) The worked example therefore asks at each
milestone what rights and obligations the stipulated terms have actually
created. Authorship creates reusable text. The assumed revocable, unfilled
instruction creates no claim and locks no collateral. Deposit, by contrast,
atomically locks one collateral unit and issues five claims governed by the
frozen terms. For this example, deposit is therefore the first
instrument-bearing milestone.

That conclusion is deliberately example-specific. This comment does not
propose that consideration and binding effect must always coincide, or that
funding is either necessary or sufficient in every architecture. Mutual
promises may bind before funding; an executable order may create different
rights; a purported deposit may fail to bind anyone. The requested guidance
should require a terms-and-facts analysis and name the operative milestone,
not replace that analysis with a universal software event.

== Position 4: separate gross instruments, net exposure, and operations

Each band claim in the worked example pays only if its band realizes, so its
gross payoff is contingent. But the aggregate payoff of one claim from every
band is one unit of the same collateral in every permitted resolution state:
exactly one claim pays one and the other four pay zero. This is a constant
nominal payoff in collateral units, not a claim that the collateral's external
value is stable. Before resolution, the holder may also recombine that set
for the collateral unit. Thus the terms support three different descriptions
at once:

- *Gross instrument classification:* five separately issued and transferable
  rights, each read from its own terms and the governing statutory criterion.
- *Net economic exposure:* a holder of one complete set has a constant
  aggregate payout under the stipulated resolution rule; selling or buying a
  component can introduce contingent net exposure.
- *Complete-set operation:* deposit and recombination transform collateral
  and the five-claim set at the stated one-for-one rate before resolution.

"Economically reversible" here describes only those stipulated protocol
terms and payoff arithmetic. It does not assume away transaction fees,
latency, implementation failure, insolvency, legal restrictions, or other
deployment frictions.

The second and third facts do not erase the first. Conversely, classifying
each claim does not establish that a balanced holder has directional or
contingent net risk. Guidance should say which layer matters to the rule being
applied, including whether that rule permits or requires portfolio netting.

== Position 5: use reversible complete sets as a diagnostic

Question 8 asks when an event contract referencing securities is a "put,
call, straddle, option, or privilege on" a security for purposes of the
statutory exclusion from both swap definitions, and what distinguishes such
contracts from options.#note_ref(1) A complete-set architecture supplies a
useful diagnostic for any proposed answer. In a security-reference
hypothetical corresponding to the commodity example, deposit would create
several separately transferable, cash-or-nothing claims on disjoint price
bands. Holding one of every band would produce a constant aggregate payout,
and recombination would return the collateral before resolution.

Economic reversibility does not itself compel one legal classification for
the collateral, the complete portfolio, and every component. Statutory text
may classify a gross right by its own reference and payoff; another rule may
measure a person's net position; legal form, transferability, issuance, or a
specific status predicate may matter. The diagnostic instead requires the
decisionmaker to expose those choices. For each proposed Question 8
criterion, the Commissions should state:

1. whether the classified unit is an individual claim, a documented
   complete-set arrangement, or a participant's net position;
2. whether the result turns on gross contingent rights or net economic
   exposure;
3. whether deposit, separation, transfer of one component, or recombination
   changes the result, and why; and
4. which statutory words justify any different treatment of economically
   reversible states.

This test does not presume that packaging is irrelevant. It identifies when
packaging is doing legal work and asks the Commissions to say whether that is
because the underlying instrument changed, because a portfolio-risk rule
allows netting, or because a separate status or conduct rule applies. That
explanation is especially important here because the options exclusion uses
the broad terms "put, call, straddle, option, or privilege" on a security or
security index, including an interest based on its value, while the
security-based-swap definition separately identifies security and
issuer-related references.#note_ref("1, 4") This comment asks the
Commissions to resolve that statutory boundary; it does not supply a legal
identity axiom in its place.

== Position 6: risk facts, not classification exclusions

Nothing in section 1a(47) excludes an instrument for being prefunded,
collateralized, or deterministic; the statutory exclusions are instrument
types, not risk controls.#note_ref(1) Full prefunding may reduce counterparty
credit exposure and, under the worked example's terms, prevents issuance
above the pool's stated payout capacity. It does not decide product category,
and it does not establish an absence of economic leverage. Likewise,
deterministic or fail-closed settlement terms describe how a system handles
specified inputs and failures; they do not establish oracle integrity,
operational availability, or legal compliance.

Formal verification, where it exists, is evidence only for the named property,
model, assumptions, and implementation correspondence. The prototype
described here is not formally verified. Guidance should credit demonstrated
risk controls when a credit, custody, or customer-protection rule makes them
relevant, without converting those controls into classification exclusions.

== Position 7: separate findings for separate functions

The CEA separately defines trading-facility, swap-execution-facility, and
derivatives-clearing-organization functions.#note_ref(2) In the worked
example the instrument question arises at funding and the venue question at
match --- different milestones, potentially different persons. Guidance
should make four findings, not one: the instrument (rights, contingency,
reference, payout, transferability); the venue (interaction, matching,
interface operation, control); the intermediary (solicitation, order
handling, discretion, compensation); and clearing and settlement (custody,
novation or its absence, netting, credit substitution, default handling).
The same discipline applies to software activity: publishing a
general-purpose language, authoring product-specific terms, operating an
interface, holding custody, and receiving transaction-linked compensation
are different facts, and treating them all as "code" is too coarse in both
directions.

A recent comment on this docket illustrates the need to identify the role of
status predicates. FalconX Bravo, Inc. would classify a
cash-settled perpetual contract on a single security or narrow-based
security index by whether it is listed under the security-futures
framework: listed, a security futures product; offered bilaterally, over
the counter, on a non-U.S. venue, or through a decentralized finance
protocol, a security-based swap.#note_ref(7) That proposal makes listing
status part of the category test, not merely evidence about where trading
occurs. This comment does not resolve whether the governing security-futures
provisions require that result. It asks the Commissions to state expressly
when listing or venue status is a constituent statutory product fact and when
it is a separate conduct or facility fact. Reference, payout, contingency,
listing, and venue can all be objective facts; the guidance should identify
which legal element each one satisfies instead of allowing one label to
silently substitute for another. Nothing here asks the proceeding to decide
any facility's registration status.

= Specific requests

1. Adopt a staged formation matrix --- authorship, signature, revocability,
   funding, interaction, match, resolution, settlement, issuance, secondary
   transfer --- as interpretive guidance within the 2012 framework, stating
   which economic facts control at each stage. (Position 1.)
2. State that, on facts like the worked example's, publication alone issues
   no claim and locks no collateral; require the operative milestone to be
   identified from the terms and facts; and report gross instrument
   classification separately from net exposure and complete-set operations.
   (Positions 2 through 4.)
3. In answering Question 8, apply the complete-set diagnostic: identify the
   classified unit, state whether gross rights or net exposure controls, and
   explain whether and why deposit, separation, transfer, or recombination
   changes the result. (Position 5.)
4. State expressly that full prefunding, bounded participant loss under
   fully paid terms, fail-closed settlement, and any precisely scoped formal
   verification are risk evidence where the operative rule makes them
   relevant, not classification exclusions and not proof that economic
   leverage is absent.
   (Position 6.)
5. Use separate instrument, venue, intermediary, and clearing findings;
   analyze software activity by function performed; state when a listing or
   venue status is a constituent product fact and when it is a separate
   conduct or facility fact; and provide a bounded process through which a
   developer can present a staged design's factual matrix and receive
   stage-specific guidance before deployment.
   (Position 7.)

= Limits

The worked example is a research design, and the artifacts behind it are an
offline prototype and formal models, not production market infrastructure.
No artifact described in this comment is deployed, funded, offered, or
operating, and nothing here requests permission to deploy one. The positions
are my analysis of questions raised by one stipulated staged structure; none
is a legal opinion or a claim that any design of mine falls outside either
Commission's jurisdiction or satisfies any current rule. The assumed
non-security-commodity reference is an analytic stipulation, not a conclusion
about any actual digital asset. The comment does not classify the worked
example, a security-reference variant, or any third-party product.

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
model reviewed by the submitter. No artifact behind these claims is deployed
market infrastructure, and none has been independently audited.

#table(
  columns: (1fr, 2.1in),
  table.header([*Claim*], [*Basis*]),
  [The Commissions request objective criteria distinguishing the product categories and ask when event contracts referencing securities fall within the options exclusion (Questions 1 and 8)], [Joint request for comment; source note 1],
  [The CEA separately defines trading-facility, swap-execution-facility, and derivatives-clearing-organization functions], [7 U.S.C. section 1a; source note 2],
  [The 2012 joint release adopted the current product definitions], [77 Fed. Reg. 48208; source note 3],
  [The Exchange Act defines security-based swap], [15 U.S.C. section 78c(a)(68); source note 4],
  [Regulation 40.11 and a pending 2026 proposal address event contracts and public-interest review], [Source notes 5 and 6],
  [The description of the FalconX Bravo listing-status proposal], [The filed comment; source note 7],
  [Under the worked example's stipulated payout table, one claim from every band pays one collateral unit in aggregate in every permitted resolution state], [Direct arithmetic from the five-band terms stated in this comment; an economic observation, not a legal-classification conclusion],
  [An order can fix actor, affected balances, and the exact limits of the permitted fill, with a nonconforming fill failing and changing nothing], [Model theorems in the submitter's guarded-commitment research; not deployed controls],
  [The worked example's core accounting --- deposit, recombination, resolution, redemption, with conservation and pool-coverage checks --- has been implemented offline with passing deterministic tests], [Pure-Rust research prototype reviewed by the submitter; tested, not formally verified; not deployed],
  [No artifact described in this comment is deployed, funded, offered, or operating], [The submitter's repository status records; a statement about the submitter's own artifacts, not about any third party],
)
