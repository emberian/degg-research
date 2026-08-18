#import "../shared/template.typ": key_point, note_ref

= Summary of positions

The Commissions ask how to distinguish swaps, mixed swaps, security-based
swaps, securities, and instruments excluded from the definition of swap, and
whether new or revised rules or interpretations are warranted.#note_ref(1)
I write in response to Question 1, and I take no position on Questions 12
through 15 concerning alternative compliance.

I am a software and formal-methods researcher; I study programmable markets
whose legal and economic character changes as code is published, signed,
funded, matched, resolved, and settled. This comment takes positions and
argues them. The positions are my own analysis as a commenter, argued from
the statutory text and one worked example; what the comment reports about my
research artifacts is stated separately, and stays within what those
artifacts support.

1. Classification of a staged program should be decided milestone by
   milestone, from the economic facts in force at the milestone being
   classified, through interpretive guidance within the framework of the
   2012 Product Definitions Adopting Release.#note_ref(3)
2. Publication of market software, without more, should not itself be a
   regulated activity, because it creates no agreement, contract, or
   transaction; the operative events are funding and interaction.
3. An instrument is formed where binding effect and consideration coincide
   --- in the worked example below, at funding, when collateral locks against
   the market's frozen terms --- not at publication, and not at a revocable
   signed instruction.
4. Instrument formation and a participant's contingent exposure are separate
   findings: the worked example's claims exist from funding, but a complete
   claim set held with an unconditional right to exchange it for its
   collateral is fully hedged, and the holder's contingent exposure arises
   at the first transaction that unbalances the set.
5. Product category should follow the reference object and the payout
   structure, not the software form: a categorical claim paying on a
   digital-asset price implicates none of the security-based swap
   definition's prongs on its face, and the identical program pointed at a
   single issuer's security would.#note_ref(4)
6. Full prefunding, fixed maximum loss, and deterministic fail-closed
   settlement terms are risk facts --- material to credit and
   customer-protection analysis --- not classification exclusions, and the
   Commissions should say so expressly.
7. The instrument, the venue, the intermediary, and clearing and settlement
   are separate findings that arise at different milestones and can attach
   to different persons; "the software" is not a unit of classification.

One matter I leave as a question, because it turns on policy choice rather
than analysis: how much weight transferability should carry where the
statute does not make it an element.

= The worked example

A market asks one question: on a stated future date, in which of five stated
price bands will the time-weighted price of a specific digital asset in a
specific onchain liquidity pool fall? The bands cover every possible price
and do not overlap --- exactly one must realize --- and the market's terms
include a deterministic rule for every edge case: a missing observation, a
malformed data page, a price exactly on a boundary.

The market's terms are frozen when it is created:

- *Deposit.* Anyone may deposit one unit of collateral into the market's
  segregated pool and receive a complete set: one claim for each of the five
  bands. The pool belongs to this market alone and pays only its claimants.
- *Recombination.* Anyone holding a complete set may return it at any time
  before resolution and withdraw the unit of collateral. A complete set and
  its collateral are interchangeable throughout the market's life.
- *Trading.* Individual claims are ordinary transferable assets. Orders
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

The design creates no debt, margin, leverage, or liquidation. Maximum loss
for every participant is fixed when their collateral or premium is paid.

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
review;#note_ref(5)#note_ref(6) the examples here are chosen to stay away
from that boundary. This scope choice is not a claim that any example falls
outside the CEA or any other law.

The table walks the example through its stages; the right-hand column states
the answer the positions give at each one.

#table(
  columns: (1.15in, 1.55in, 1fr),
  table.header([*Stage*], [*Economic fact*], [*The position's answer*]),
  [Authored policy], [Reusable software text; no parties], [Nothing exists to classify; regulation should not attach here.],
  [Signed instruction], [Authenticated, possibly revocable direction], [Formation begins only with binding effect; a revocable instruction binds nobody.],
  [Funded commitment], [Consideration passes; collateral locks], [The instrument is formed; a fully hedged whole carries no net exposure.],
  [Early exit, compression, or unwind], [A right is canceled, netted, transferred, or closed before maturity], [Termination of the existing instrument, unless the terms deliver a new right or reintroduce discretion --- read the terms for exactly that.],
  [Admitted evidence], [The frozen rule accepts qualifying evidence], [Evidence of a fixed fact where the rule leaves no choice among economic outcomes; otherwise a discretion fact, analyzed as such.],
  [Interaction or match], [Binding interests meet; prices exist], [Venue and intermediary functions are performed here; a separate finding.],
  [Resolution], [One outcome becomes authorized], [The named evidence rule, dispute procedure, and source-failure behavior carry the outcome's authority.],
  [Settlement or issuance], [Balances move or claims are delivered], [Performance of the earlier instrument; a delivered continuing claim restarts analysis.],
  [Secondary transfer], [A resulting claim continues to trade], [Same instrument; new venue facts, found separately.],
)

#key_point("Core distinction", [
  Technical incompleteness is not economic incompleteness. At batch close,
  nobody knows which band will realize --- yet consideration is paid,
  collateral is locked, the outcomes are exhaustively enumerated, each
  payout is stated, and nobody retains discretion to change any of it. The
  reverse also holds: the same arithmetic run as an unfunded local
  simulation binds nobody and moves nothing. Classification should read the
  economic facts, not the software's description of its own state.
])

= The argument

== Position 1: classify milestone by milestone

The statute classifies transactions, not codebases: CEA section 1a(47)
reaches "any agreement, contract, or transaction" of the described
kinds.#note_ref(1) A staged program separates into distinct dated events
what a bilateral confirmation fuses into one signing --- authorship,
signature, funding, interaction, resolution, and settlement can be days
apart, with different parties present and different rights enforceable at
each --- so the statutory question has a potentially different answer at each
milestone, and guidance that does not name the milestone it is analyzing
will be applied inconsistently. Question 1 asks whether new interpretations
are warranted "[t]aking into account" the 2012 release;#note_ref(3) a
milestone matrix is an answer in exactly that form. The strongest
counterargument is gamesmanship --- designers relabeling stages to defer the
regulated moment. But the matrix reads binding effect, consideration, and
exposure, which are facts; renaming a stage changes nothing unless the facts
change, and where they genuinely change, different treatment is accuracy,
not evasion.

== Position 2: publication is not the operative event

At publication there are no parties, no consideration, and no binding
effect; nothing satisfying the statutory predicate exists, and analysis that
attaches to publication is classifying a text. The counterargument is that
publication can be one step in a larger course of conduct --- solicitation,
interface operation, transaction-linked compensation. True, and that is why
the position says "without more": those are facts about a person's conduct,
analyzed under Position 7, and publication is no safe harbor for what
surrounds it. A template that deploys itself with the author's
pre-authorized funding is not publication without more --- it is funding.

== Position 3: formation at binding effect plus consideration

The swap definition's option prong reaches "a put, call, cap, floor,
collar, or similar option of any kind";#note_ref(1) an option exists when
the premium passes and the writer is bound, not when the option form is
drafted. So here: at funding, consideration passes and the depositor
acquires rights only the market's frozen terms can defeat. Where a design
instead makes signed instructions irrevocable and enforceable before value
moves, binding effect arrives earlier, and formation with it. The general
rule is the coincidence of binding effect and consideration; in fully
prefunded onchain designs the two coincide at funding.

== Position 4: hedged wholes and unbalanced positions

The event prong turns on a payment "dependent on the occurrence,
nonoccurrence, or the extent of the occurrence of an event or
contingency."#note_ref(1) A complete set plus the unconditional
recombination right returns the deposit in every state of the world, by the
terms themselves: the whole is not dependent on the contingency. The claims
are instruments from funding --- transferable and priced --- but the holder
of the balanced whole has no contingent exposure until a transaction
unbalances it. Both findings are needed because they serve different rules:
the instrument finding governs product treatment; the exposure finding
governs when a participant's position becomes economically operative. The
administrability objection --- use funding for everything --- collapses the
two and misstates the economics of every depositor who never sells.

== Position 5: the reference object allocates the category

The security-based swap definition has three prongs: a narrow-based security
index, a single security or loan, and an event relating to a single issuer
that directly affects its financial statements, condition, or
obligations.#note_ref(4) The worked example references an onchain
digital-asset price and no security, loan, issuer, or securities index; none
of the three prongs is implicated on its face. Point the identical program
at a single issuer's security and the prongs engage; the mixed-swap
category, which the Commissions have said is intentionally narrow, enters
only when both kinds of reference are present. Software generality is the
reason for this position, not an objection to it: because one program can
span both agencies' jurisdictions, only the reference object and payout
structure can carry the allocation.

== Position 6: risk facts, not classification exclusions

Nothing in section 1a(47) excludes an instrument for being prefunded,
collateralized, or deterministic; the statutory exclusions are instrument
types, not risk controls.#note_ref(1) Silence here breeds two symmetric
errors. Industry will argue that full prefunding takes a product out of the
definitions; it does not --- it changes the credit risk, not the category.
Regulators may discount real controls as window dressing; that is also
wrong --- where a loss ceiling is encoded in the terms, correctly
implemented, and bound to settlement, credit and customer-protection
analysis should credit it. An express statement that these are risk facts
with risk-analysis weight, and nothing more, forecloses both errors.

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
directions. This comment does not ask this proceeding to decide the
registration status of any facility; it asks that the findings be kept
separate so that question can be decided accurately when it arises.

= Specific requests

1. Adopt a staged formation matrix --- authorship, signature, revocability,
   funding, interaction, match, resolution, settlement, issuance, secondary
   transfer --- as interpretive guidance within the 2012 framework, stating
   which economic facts control at each stage. (Position 1.)
2. State that publication of market software, without more, creates no
   agreement, contract, or transaction; that formation occurs where binding
   effect and consideration coincide; and that a participant's contingent
   exposure is found separately, distinguishing a fully hedged holding from
   an unbalanced one. (Positions 2 through 4.)
3. Publish paired examples that hold the computation constant and vary the
   reference object, so the swap, security-based swap, and mixed-swap
   boundaries are illustrated by nexus facts rather than software labels.
   (Position 5.)
4. State expressly that full prefunding, fixed maximum loss, fail-closed
   settlement terms, and formal verification are risk facts relevant where
   risk is the operative question, not classification exclusions.
   (Position 6.)
5. Use separate instrument, venue, intermediary, and clearing findings;
   analyze software activity by function performed; and provide a bounded
   process through which a developer can present a staged design's factual
   matrix and receive stage-specific guidance before deployment.
   (Position 7.)

= Limits

The worked example is a research design, and the artifacts behind it are an
offline prototype and formal models, not production market infrastructure.
No artifact described in this comment is deployed, funded, offered, or
operating, and nothing here requests permission to deploy one. The positions
are my analysis of how the definitions should treat a class of staged
structures; none is a claim that any design of mine falls outside either
Commission's jurisdiction or satisfies any current rule.

#block(breakable: false)[
  #v(18pt)
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
  [The Commissions request objective criteria distinguishing the product categories (Question 1)], [Joint request for comment; source note 1],
  [The CEA separately defines trading-facility, swap-execution-facility, and derivatives-clearing-organization functions], [7 U.S.C. section 1a; source note 2],
  [The 2012 joint release adopted the current product definitions], [77 Fed. Reg. 48208; source note 3],
  [The Exchange Act defines security-based swap], [15 U.S.C. section 78c(a)(68); source note 4],
  [Regulation 40.11 and a pending 2026 proposal address event contracts and public-interest review], [Source notes 5 and 6],
  [An order can fix actor, affected balances, and the exact limits of the permitted fill, with a nonconforming fill failing and changing nothing], [Model theorems in the submitter's guarded-commitment research; not deployed controls],
  [The worked example's core accounting --- deposit, recombination, resolution, redemption, with conservation and pool-coverage checks --- has been implemented offline with passing deterministic tests], [Pure-Rust research prototype reviewed by the submitter; tested, not formally verified; not deployed],
  [No artifact described in this comment is deployed, funded, offered, or operating], [The submitter's repository status records; a statement about the submitter's own artifacts, not about any third party],
)
