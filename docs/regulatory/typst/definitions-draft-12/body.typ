#import "../shared/template.typ": claim_table, key_point, note_ref

= What this is, and what it asks

I am a software and formal-methods researcher. I built a staged claim
protocol, Dragon's Clutch, ran it, and use it here as the worked example,
because a concrete system makes the classification questions exact. This
comment answers Questions 1 and 8 of the joint request#note_ref(1) and takes
no position on Questions 12 through 15 concerning alternative compliance.

One finding drives everything below. A staged program has no single moment of
formation: authorship, signature, funding, interaction, resolution, and
settlement are separate dated events, days apart, with different parties
present and different rights enforceable at each. A bilateral confirmation
fuses them into one signing; code separates them. Guidance that does not name
which milestone it analyzes will be applied inconsistently, because there is
no longer one event to point at. Five requests follow.

1. Adopt a *staged formation matrix* --- authorship, signature, revocability,
   funding, interaction, match, resolution, settlement, issuance, secondary
   transfer --- as interpretive guidance within the framework of the 2012
   Product Definitions Adopting Release,#note_ref(3) stating which economic
   facts control at each stage. (Position 1.)
2. State that, on facts like the worked example's, *publication alone issues
   no claim and locks no collateral*; require the operative milestone to be
   identified from the terms and facts; and report gross instrument
   classification separately from net exposure and complete-set operations.
   (Positions 2 through 4.)
3. In answering Question 8, apply the *complete-set diagnostic*: identify the
   classified unit, state whether gross rights or net exposure controls, and
   explain whether and why deposit, separation, transfer, or recombination
   changes the result. (Position 5.)
4. State expressly that full prefunding, bounded participant loss under fully
   paid terms, fail-closed settlement, and precisely scoped formal
   verification are *risk evidence where the operative rule makes them
   relevant --- not classification exclusions*. (Position 6.)
5. Make *separate instrument, venue, intermediary, and clearing findings*;
   analyze software activity by the function performed; state when a listing
   or venue status is a constituent product fact and when it is a separate
   conduct or facility fact; and provide a bounded process through which a
   developer can present a staged design's factual matrix and receive
   stage-specific guidance before deployment. (Position 7.)

= The worked example

Dragon's Clutch asks one kind of question: on a stated future date, what was
the price of a stipulated non-security digital commodity, observed from a
named source over a stated window? A market's terms are frozen at creation
and include a deterministic rule for every edge case --- a missing
observation, a malformed data page, a value exactly on a boundary.

The programs execute in local test banks and in an author-operated research
deployment on Solana's devnet test cluster [DEVNET RECORD: program
identifiers, build hashes, and deployment slot], using valueless test tokens,
fresh keys, and no customer anything. The default build refuses every deposit
until an authenticated data-source release is compiled into it, and none is:
the deposit instruction fails closed. There is no mainnet deployment, no
offer, and no live order from any other person.

The frozen terms of a market:

- *Deposit.* Anyone may deposit one unit of collateral into the market's
  segregated pool and receive a complete set of its claims. The pool belongs
  to this market alone and pays only its claimants.
- *Recombination.* Anyone holding a complete set may return it at any time
  before resolution and withdraw the unit of collateral. A complete set and
  its collateral are interchangeable throughout the market's life.
- *Claim families.* The simplest terms partition the price range into
  exhaustive, non-overlapping bands --- say five --- and a complete set is
  one claim per band; exactly one band realizes and pays the full unit. The
  native generalization replaces bands with a smooth basis: each claim is one
  basis function of an open-clamped B-spline of degree one, two, or three
  over the frozen grid, and a position is a vector of integer coefficients
  over those claims. Settlement evaluates the basis at the observed value in
  exact rational arithmetic and converts to integer payouts by a canonical
  largest-remainder rule. Because the basis functions are nonnegative and sum
  to one everywhere on the grid --- the partition of unity, proved in a
  machine-checked model of the construction --- one claim of every basis
  function pays exactly the fixed denominator at any settled value: the same
  complete-set constancy as the five bands, now as a theorem. A payoff curve
  such as a capped call spread is expressed exactly by integer coefficients
  when it lies in the spline span, and with a certified error bound when it
  does not.
- *Trading.* Individual claims are separately transferable. Orders submitted
  to the market's batch venue accumulate until a stated close; at close the
  book freezes and a deterministic rule clears it at one consistent set of
  prices. An order fixes who is acting, which balances may change, and the
  exact limits of the permitted fill; a fill outside those limits fails and
  changes nothing, a property machine-checked in formal models of the order
  pattern. The venue's selection claim is deliberately "best valid submitted
  candidate," never "optimal": the program verifies submitted clearings; it
  does not certify a global search.
- *Observation.* The terms freeze a source specification naming one data
  provider, parser, and deployment; the program authenticates the complete
  price history against that specification and refuses substitutes,
  truncations, and replays. The production source profile I have selected
  targets an oracle network whose own documentation defines a unique update
  for any instant --- for any time t, the unique update with
  prev_publish_time < t <= publish_time#note_ref(8) --- which removes
  transaction-timing discretion over which observation settles the market.
  Until such a release is compiled into the default build, deposits refuse;
  local tests exercise the full source lifecycle against a deliberately
  non-production stand-in provider.
- *Settlement.* After the observation window and its repair period close,
  claims redeem from the pool at their computed payouts. Every complete set
  was issued against a full deposit, so the pool covers the maximum payout
  the terms allow, by construction.

The terms permit no borrowing and no undercollateralized issuance, and there
is no margin-call or liquidation mechanism. That does not imply an absence of
economic leverage: a claim bought for a limited price may have a much larger
state-contingent payout. Under these fully paid, long-only terms a
participant's maximum contractual payoff loss is the collateral deposited or
purchase price paid, apart from fees; custody, implementation, and
collateral-value risks are separate matters.

The table walks the example through its stages; the right-hand column states
the answer the positions give at each one.

#table(
  columns: (1.0in, 1.4in, 1fr),
  table.header([*Stage*], [*Economic fact*], [*The position's answer*]),
  [Authored policy], [Reusable software text; no parties], [No claim, collateral lock, or transaction yet; surrounding conduct remains a separate inquiry.],
  [Signed instruction], [Authenticated, revocable, and unfilled direction], [Issues no claim and locks no collateral here; different legal or technical terms may differ.],
  [Funded commitment], [Collateral locks; the complete set issues], [The first instrument-bearing milestone: contingent gross claims, with constant aggregate exposure while held as a complete set.],
  [Early exit, compression, or unwind], [A right is canceled, netted, transferred, or closed before maturity], [Termination of the existing instrument, unless the terms deliver a new right or reintroduce discretion --- read the terms for exactly that.],
  [Admitted evidence], [The frozen rule accepts qualifying evidence], [Evidence of a fixed fact where the rule leaves no choice among economic outcomes; otherwise a discretion fact, analyzed as such.],
  [Interaction or match], [Orders interact; a transfer may execute], [Venue or intermediary functions may arise here; make that finding separately from product classification.],
  [Resolution], [One outcome becomes authorized], [The named evidence rule, dispute procedure, and source-failure behavior carry the outcome's authority.],
  [Settlement or issuance], [Balances move or claims are delivered], [Performance of the earlier instrument; a delivered continuing claim restarts analysis.],
  [Secondary transfer], [A resulting claim continues to trade], [A continuing claim if its terms are unchanged; new venue and participant-conduct facts, found separately.],
)

The core distinction the table applies: technical incompleteness is not
economic incompleteness. At batch close nobody knows which outcome will
realize --- yet consideration is paid, collateral is locked, the outcomes are
exhaustively enumerated, each payout rule is stated, and nobody retains
discretion to change any of it. The same arithmetic run as an unfunded local
simulation binds nobody and moves nothing. Classification should read the
economic facts, not the software's description of its own state.

= The seven positions

*1. Classify a staged program milestone by milestone, from the economic facts
in force at the milestone being classified.* The statute classifies
transactions, not codebases: CEA section 1a(47) reaches "any agreement,
contract, or transaction" of the described kinds.#note_ref(1) Because a
staged program separates into dated events what a bilateral confirmation
fuses into one signing, the statutory question has a potentially different
answer at each milestone. Question 1 asks whether new interpretations are
warranted "[t]aking into account" the 2012 release;#note_ref(3) a milestone
matrix is an answer in exactly that form. The strongest counterargument is
gamesmanship --- designers relabeling stages to defer the regulated moment
--- but the matrix reads rights, obligations, collateral lock, issuance,
transfer, and exposure, which are facts. Renaming a stage changes nothing
unless the facts change, and where they genuinely change, different treatment
is accuracy, not evasion.

*2. Publication alone is not the operative event here.* At publication there
are no transaction parties, no issued claims, and no collateral lock. On
those facts, treating publication as the product-formation event would
classify reusable text rather than an agreement, contract, or transaction.
Publication can still be part of a larger course of conduct ---
solicitation, interface operation, transaction-linked compensation, or
pre-authorized funding --- and those surrounding facts require their own
analysis under Position 7; publication supplies no safe harbor for them.

*3. Identify the first instrument-bearing milestone from the terms, not from
a universal proxy.* The swap definition reaches an "agreement, contract, or
transaction" within its enumerated prongs,#note_ref(1) so the example asks at
each milestone what rights and obligations the terms have actually created.
Authorship creates reusable text. A revocable, unfilled instruction creates
no claim and locks no collateral. Deposit atomically locks one collateral
unit and issues the complete set, and is therefore this design's first
instrument-bearing milestone. That conclusion is deliberately
example-specific: mutual promises may bind before funding, an executable
order may create different rights, a purported deposit may fail to bind
anyone. Guidance should require a terms-and-facts analysis that names the
operative milestone, not replace the analysis with a universal software
event.

*4. Separate gross instruments, net exposure, and complete-set operations.*
Each claim pays by its own rule, so its gross payoff is contingent. The
aggregate payoff of a complete set is one unit of the same collateral in
every permitted resolution state --- for bands because exactly one pays, for
the smooth basis because the basis functions sum to one at every point, a
property proved in a machine-checked model of the construction. Before
resolution the holder may recombine the set for the collateral unit. The
terms therefore support three descriptions at once: *gross instrument
classification*, separately issued and transferable rights, each read from
its own terms and the governing statutory criterion; *net economic exposure*,
a constant aggregate payout for a complete-set holder under the stipulated
resolution rule, with contingent net exposure appearing when a component is
bought or sold; and the *complete-set operation* itself, which transforms
collateral and the claim set at the stated one-for-one rate before
resolution. "Economically reversible" describes the stipulated protocol terms
and payoff arithmetic, not transaction fees, latency, implementation failure,
insolvency, or other deployment frictions. The second and third facts do not
erase the first; conversely, classifying each claim does not establish that a
balanced holder has directional net risk. Guidance should say which layer
matters to the rule being applied, including whether that rule permits or
requires portfolio netting.

*5. Use reversible complete sets as a diagnostic for Question 8, not as an
axiom that economic equivalence settles legal classification.* Question 8
asks when an event contract referencing securities is a "put, call, straddle,
option, or privilege on" a security for purposes of the statutory exclusion
from both swap definitions, and what distinguishes such contracts from
options.#note_ref(1) A complete-set architecture supplies a demanding
diagnostic for any proposed answer. In a security-reference hypothetical
corresponding to the commodity example, deposit would create several
separately transferable cash-or-nothing claims on disjoint bands --- or a
smooth coefficient vector --- holding the complete set would produce a
constant aggregate payout, and recombination would return the collateral
before resolution. Economic reversibility does not itself compel one legal
classification for the collateral, the complete portfolio, and every
component: statutory text may classify a gross right by its own reference and
payoff, another rule may measure net position, and legal form,
transferability, issuance, or a status predicate may matter. The diagnostic
forces those choices into the open. For each proposed Question 8 criterion,
the Commissions should state:

1. whether the classified unit is an individual claim, a documented
   complete-set arrangement, or a participant's net position;
2. whether the result turns on gross contingent rights or net economic
   exposure;
3. whether deposit, separation, transfer of one component, or recombination
   changes the result, and why; and
4. which statutory words justify any different treatment of economically
   reversible states.

This does not presume packaging is irrelevant. It identifies when packaging
is doing legal work and asks the Commissions to say whether that is because
the underlying instrument changed, because a portfolio-risk rule allows
netting, or because a separate status or conduct rule applies. The
explanation matters because the options exclusion uses the broad terms "put,
call, straddle, option, or privilege" on a security or security index while
the security-based-swap definition separately identifies security and
issuer-related references.#note_ref("1, 4") This comment asks the Commissions
to resolve that boundary rather than supplying a legal identity axiom in its
place.

*6. Prefunding, bounded loss, and determinism are risk facts, not
classification exclusions.* Nothing in section 1a(47) excludes an instrument
for being prefunded, collateralized, or deterministic; the statutory
exclusions are instrument types, not risk controls.#note_ref(1) Full
prefunding reduces counterparty credit exposure and, under these terms,
prevents issuance above the pool's stated payout capacity. It does not decide
product category and does not establish an absence of economic leverage.
Deterministic, fail-closed settlement terms describe how a system handles
specified inputs and failures; they do not establish oracle integrity,
operational availability, or legal compliance. Formal verification, where it
exists, is evidence for the named property under the named model and
assumptions --- nothing more. My Lean models establish specified
complete-set, solvency, guarded-transition, and B-spline construction and
quantization claims; one pinned Verus run checks an internal-transfer
arithmetic seam; no checked refinement connects the complete Rust and SBF
implementation to those theorems. Guidance should credit demonstrated risk
controls when a credit, custody, or customer-protection rule makes them
relevant, without converting scoped proofs into whole-system assurance or
classification exclusions.

*7. Make separate findings for separate functions; "the software" is not a
unit of classification.* The CEA separately defines trading-facility,
swap-execution-facility, and derivatives-clearing-organization
functions.#note_ref(2) In the example the instrument question arises at
funding and the venue question at match --- different milestones,
potentially different persons. Guidance should make four findings, not one:
the instrument (rights, contingency, reference, payout, transferability); the
venue (interaction, matching, interface operation, control); the intermediary
(solicitation, order handling, discretion, compensation); and clearing and
settlement (custody, novation or its absence, netting, credit substitution,
default handling). The same discipline applies to software activity:
publishing a general-purpose language, authoring product-specific terms,
operating an interface, holding custody, and receiving transaction-linked
compensation are different facts, and treating them all as "code" is too
coarse in both directions.

A recent comment on this docket shows why the role of status predicates needs
stating. FalconX Bravo, Inc. would classify a cash-settled perpetual contract
on a single security or narrow-based security index by whether it is listed
under the security-futures framework: listed, a security futures product;
offered bilaterally, over the counter, on a non-U.S. venue, or through a
decentralized finance protocol, a security-based swap.#note_ref(7) That
proposal makes listing status part of the category test, not merely evidence
about where trading occurs. This comment does not resolve whether the
governing security-futures provisions require that result; it asks the
Commissions to state expressly when listing or venue status is a constituent
statutory product fact and when it is a separate conduct or facility fact.
Reference, payout, contingency, listing, and venue can all be objective
facts, and guidance should identify which legal element each one satisfies
instead of allowing one label to substitute silently for another. Nothing
here asks the proceeding to decide any facility's registration status.

= Scope

For the worked example I assume the referenced digital asset is a
non-security commodity, so the example is not offered as a
security-based-swap case; whether any actual digital asset is a security or
commodity is outside this comment. Substituting a single security, a
narrow-based security index, or an issuer-related event would require the
separate analysis specified in the security-based-swap definition and in
Question 8.#note_ref("1, 4") I also leave open how much weight
transferability should carry under a particular statutory prong. Every
example here references objectively verifiable crypto-native facts --- ledger
states, program events, prices, ranges, and path statistics --- and none
references politics, sports, gaming, or subjective social events, staying
away from the boundary addressed by Regulation 40.11 and a pending June 2026
proposal;#note_ref("5, 6") that scope choice is not a claim that any example
falls outside the CEA. The positions are my analysis as a commenter, not
legal opinions, and this comment classifies neither Dragon's Clutch, nor its
security-reference variant, nor any third-party product. The devnet
deployment is research operation with valueless test tokens, not an offer,
and nothing here requests permission to deploy anything.

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
  columns: (1fr, 2.1in),
  table.header([*Claim*], [*Basis*]),
  [The Commissions request objective criteria distinguishing the product categories and ask when event contracts referencing securities fall within the options exclusion (Questions 1 and 8)], [Joint request for comment; source note 1],
  [The CEA separately defines trading-facility, swap-execution-facility, and derivatives-clearing-organization functions], [7 U.S.C. section 1a; source note 2],
  [The 2012 joint release adopted the current product definitions], [77 Fed. Reg. 48208; source note 3],
  [The Exchange Act defines security-based swap], [15 U.S.C. section 78c(a)(68); source note 4],
  [Regulation 40.11 and a pending 2026 proposal address event contracts and public-interest review], [Source notes 5 and 6],
  [The description of the FalconX Bravo listing-status proposal], [The filed comment; source note 7],
  [One claim from every band pays one collateral unit in aggregate in every permitted resolution state], [Direct arithmetic from the band terms],
  [The smooth basis functions are nonnegative and sum to one everywhere on the grid, so a smooth complete set pays the fixed denominator at any settled value], [Machine-checked theorems (partition of unity, solvency, quantization) in a Lean model of the construction; not a proof of the Rust or SBF implementation],
  [A guard-violating fill fails and changes nothing], [Named theorems in guarded-commitment formal models; a design property, not a deployed control],
  [Payout evaluation uses exact rational arithmetic and canonical largest-remainder integer quantization], [Pure-Rust kernels with independent exact-oracle differential tests],
  [Construction, resolution, redemption, custody, staged resolution, and one settlement slice execute in local SBF test banks], [Local signed and bank campaigns, including a 22-transaction custody walk and blank-bank market construction per degree; tested subsets, not a complete venue],
  [Resolution is controlled by one canonical, sealed, program-owned source receipt and refuses substitution], [Local real-SBF receipt-binding and substitution-refusal tests; source construction exercised against a deliberately non-production stand-in provider],
  [The selected production source profile has a documented unique update per instant], [The provider's published interface documentation; source note 8],
  [The default build refuses deposits with no compiled source release; the devnet deployment is author-operated research with valueless tokens], [Repository status records and the dated devnet deployment record],
)
