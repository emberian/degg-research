#import "../shared/template.typ": claim_table, key_point, note_ref

= What this is, and what it asks

I am a software and formal-methods researcher. Dragon's Clutch, a staged
claim protocol I built and ran, is the worked example: a concrete system
makes the classification questions exact. This comment answers Questions 1
and 8 of the joint request#note_ref(1) and takes no position on Questions 12
through 15.

One finding drives everything below: a staged program has no single moment of
formation. Authorship, signature, funding, interaction, resolution, and
settlement are separate dated events, days apart, with different parties and
different enforceable rights at each. A bilateral confirmation fuses them
into one signing; code separates them. Guidance that does not name its
milestone will be applied inconsistently. Five requests:

1. Adopt a *staged formation matrix* --- authorship, signature, revocability,
   funding, interaction, match, resolution, settlement, issuance, secondary
   transfer --- as interpretive guidance under the 2012 Product Definitions
   Adopting Release,#note_ref(3) stating which economic facts control at each
   stage. (Position 1.)
2. State that on facts like these, *publication alone issues no claim and
   locks no collateral*, and require the operative milestone identified from
   terms and facts. (Positions 2 through 4.)
3. Answer Question 8 with the *complete-set diagnostic*. (Position 5.)
4. Treat full prefunding, bounded participant loss under fully paid terms,
   fail-closed settlement, and precisely scoped formal verification as *risk
   evidence, not classification exclusions*. (Position 6.)
5. Make *separate instrument, venue, intermediary, and clearing findings*;
   analyze software activity by function; give developers a bounded
   pre-deployment process to present a staged design's factual matrix and
   receive stage-specific guidance. (Position 7.)

= The worked example

Dragon's Clutch asks one kind of question: on a stated future date, what was
the price of a stipulated non-security digital commodity, observed from a
named source over a stated window? Market terms freeze at creation, with a
deterministic rule for every edge case: missing observation, malformed data
page, value exactly on a boundary.

The programs run in local test banks and, at the strongest, on a real
loopback validator: a 44-transaction signed, confirmed walk dated August 20,
2026 drives the complete general clearing lifecycle, policy artifact through
portfolio full-pair settlement, with 38 watched accounts byte-reloaded per
step and conservation re-derived from the observed bytes --- fresh keys,
valueless test units, no live order from any other person. No public cluster
runs them; a devnet deployment is authorized but unfunded. The default build
refuses every deposit until an authenticated data-source release is compiled
in --- none is; deposits fail closed.

The frozen terms of a market:

- *Deposit.* Depositing one collateral unit into the market's segregated
  pool --- which pays only this market's claimants --- issues a complete set
  of its claims.
- *Recombination.* A holder may return a complete set before resolution and
  withdraw the collateral unit; set and collateral stay interchangeable
  throughout.
- *Claim families.* The simplest terms partition the price range into
  exhaustive, non-overlapping bands --- say five; a complete set is one claim
  per band, and exactly one band realizes and pays the full unit. The native
  generalization is a smooth basis: each claim is one basis function of an
  open-clamped B-spline of degree one, two, or three over the frozen grid,
  and a position is a vector of integer coefficients. Settlement evaluates
  the basis at the observed value in exact rational arithmetic, then
  quantizes to integer payouts by a canonical largest-remainder rule. The
  basis functions are nonnegative and sum to one everywhere on the grid ---
  the partition of unity, proved in a machine-checked model of the
  construction --- so one claim of every basis function pays exactly the
  fixed denominator at any settled value: the five-band complete-set
  constancy, now a theorem. A payoff curve like a capped call spread is
  expressed exactly by integer coefficients within the spline span, with a
  certified error bound outside it.
- *Trading.* Individual claims are separately transferable. Orders accumulate
  in the market's batch venue until a stated close; the frozen book then
  clears by a deterministic rule at one consistent price set. An order fixes
  the actor, the balances that may change, and the fill's exact limits; an
  out-of-limit fill fails and changes nothing (machine-checked in formal
  models of the order pattern). The venue claims "best valid submitted
  candidate," never "optimal": it verifies submitted clearings, not a global
  search.
- *Observation.* The terms freeze a source specification --- one data
  provider, parser, and deployment; the program authenticates the complete
  price history against it, refusing substitutes, truncations, and replays.
  The selected production source profile targets an oracle network whose
  documentation defines a unique update per instant --- for any time t, the
  unique update with prev_publish_time < t <= publish_time#note_ref(8) ---
  removing transaction-timing discretion over which observation settles the
  market. Local tests run the full source lifecycle against a deliberately
  non-production stand-in provider.
- *Settlement.* After the observation window and repair period close, claims
  redeem from the pool at computed payouts. Every complete set was issued
  against a full deposit, so the pool covers the maximum payout the terms
  allow, by construction.

The terms permit no borrowing, undercollateralized issuance, margin calls, or
liquidations. Economic leverage can still exist: a claim bought for a limited
price may carry a much larger state-contingent payout. Under these fully
paid, long-only terms, maximum contractual payoff loss is the collateral
deposited or price paid, apart from fees; custody, implementation, and
collateral-value risks are separate matters.

#table(
  columns: (1.0in, 1.4in, 1fr),
  table.header([*Stage*], [*Economic fact*], [*The position's answer*]),
  [Authored policy], [Reusable text; no parties], [No claim, lock, or transaction yet; surrounding conduct is a separate inquiry.],
  [Signed instruction], [Authenticated, revocable, unfilled direction], [Issues no claim, locks no collateral; different legal or technical terms may differ.],
  [Funded commitment], [Collateral locks; complete set issues], [First instrument-bearing milestone: contingent gross claims, constant aggregate exposure while held complete.],
  [Early exit, compression, or unwind], [A right is canceled, netted, transferred, or closed early], [Termination of the existing instrument, unless the terms deliver a new right or reintroduce discretion.],
  [Admitted evidence], [The frozen rule accepts qualifying evidence], [Evidence of a fixed fact where the rule leaves no outcome choice; otherwise a discretion fact.],
  [Interaction or match], [Orders interact; a transfer may execute], [Venue or intermediary functions may arise; find them separately from product classification.],
  [Resolution], [One outcome becomes authorized], [The named evidence rule, dispute procedure, and source-failure behavior carry the outcome's authority.],
  [Settlement or issuance], [Balances move or claims are delivered], [Performance of the earlier instrument; a delivered continuing claim restarts analysis.],
  [Secondary transfer], [A resulting claim keeps trading], [Same claim if terms unchanged; new venue and conduct facts, found separately.],
)

The core distinction: technical incompleteness is not economic
incompleteness. At batch close nobody knows the outcome, yet consideration is
paid, collateral locked, outcomes exhaustively enumerated, payout rules
stated, and nobody retains discretion to change any of it; the same
arithmetic as an unfunded local simulation binds nobody and moves nothing.
Read the economic facts, not the software's description of its own state.

= The seven positions

*1. Classify milestone by milestone, from the economic facts in force at
each.* The statute classifies transactions, not codebases: CEA section 1a(47)
reaches "any agreement, contract, or transaction" of the described
kinds,#note_ref(1) and formation here separates into dated events, so the
answer may differ at each. Question 1 asks whether new interpretations are
warranted "[t]aking into account" the 2012 release;#note_ref(3) a milestone
matrix answers in that form. The gamesmanship objection --- relabeling stages
to defer the regulated moment --- fails: the matrix reads rights,
obligations, collateral lock, issuance, transfer, and exposure --- facts ---
and renaming changes nothing unless they change; where they do, different
treatment is accuracy, not evasion.

*2. Publication alone is not the operative event here.* No transaction
parties, no issued claims, no collateral lock; classifying at publication
would classify reusable text, not an agreement, contract, or transaction.
Publication may still join a larger course of conduct --- solicitation,
interface operation, transaction-linked compensation, pre-authorized
funding --- analyzed under Position 7; it is no safe harbor.

*3. Find the first instrument-bearing milestone in the terms, not a universal
proxy.* The swap definition reaches an "agreement, contract, or transaction"
within its enumerated prongs,#note_ref(1) so ask what rights each milestone's
terms actually create. Here, deposit --- atomically locking one collateral
unit and issuing the complete set --- is the first instrument-bearing
milestone. Deliberately example-specific: mutual promises may bind before
funding, an executable order may create different rights, a purported deposit
may bind nobody. Require a terms-and-facts analysis naming the operative
milestone, not a universal software event.

*4. Separate gross instruments, net exposure, and complete-set operations.*
Each claim's gross payoff is contingent under its own rule; a complete set
pays one collateral unit in every permitted resolution state (bands: exactly
one pays; smooth basis: the machine-checked partition of unity) and
recombines for the collateral before resolution. Three descriptions hold at
once: *gross instrument classification* --- separately issued transferable
rights, each read from its own terms and statutory criterion; *net economic
exposure* --- constant aggregate payout for a complete-set holder under the
stipulated resolution rule, contingent once a component trades; the
*complete-set operation* --- collateral and claim set interchange one-for-one
before resolution. "Economically reversible" describes stipulated terms and
payoff arithmetic, not fees, latency, implementation failure, insolvency, or
other deployment frictions. The later layers do not erase the first;
per-claim classification gives a balanced holder no directional net risk.
Guidance should name the layer each rule reads and whether it permits or
requires portfolio netting.

*5. Treat reversible complete sets as a Question 8 diagnostic, not an axiom
that economic equivalence settles legal classification.* Question 8 asks when
an event contract referencing securities is a "put, call, straddle, option,
or privilege on" a security --- the statutory exclusion from both swap
definitions --- and what distinguishes such contracts from
options.#note_ref(1) In a security-reference hypothetical mirroring the
commodity example, deposit would create separately transferable
cash-or-nothing claims on disjoint bands (or a smooth coefficient vector),
the complete set would pay a constant aggregate, and recombination would
return the collateral before resolution. Reversibility alone compels no
single classification for collateral, portfolio, and components: statutory
text may classify a gross right by its own reference and payoff, another
rule may measure net position, and legal form, transferability, issuance, or
a status predicate may matter. For each proposed Question 8 criterion, the
Commissions should state:

1. the classified unit --- individual claim, documented complete-set
   arrangement, or a participant's net position;
2. whether gross contingent rights or net economic exposure controls;
3. whether deposit, separation, single-component transfer, or recombination
   changes the result, and why; and
4. which statutory words justify different treatment of economically
   reversible states.

This does not presume packaging is irrelevant; where packaging does legal
work, the Commissions should say whether the underlying instrument changed, a
portfolio-risk rule allows netting, or a separate status or conduct rule
applies. The explanation matters: the options exclusion uses the broad terms
"put, call, straddle, option, or privilege" on a security or security index,
while the security-based-swap definition separately identifies security and
issuer-related references.#note_ref("1, 4") The Commissions should resolve
that boundary.

*6. Prefunding, bounded loss, and determinism are risk facts, not
classification exclusions.* Section 1a(47) excludes instrument types, not
risk controls; nothing in it excludes a prefunded, collateralized, or
deterministic instrument.#note_ref(1) Full prefunding reduces counterparty
credit exposure and here caps issuance at the pool's stated payout capacity;
it decides no product category and establishes no absence of economic
leverage. Deterministic, fail-closed settlement terms describe how a system
handles specified inputs and failures --- not oracle integrity, operational
availability, or legal compliance. Formal verification evidences the named
property under the named model and assumptions, nothing more: my 212
zero-sorry Lean theorems establish specified complete-set, solvency,
guarded-transition, and B-spline construction and quantization claims, with
Lean the adopted proof substrate of record; one pinned Verus run checks an
internal-transfer arithmetic seam; no checked refinement connects the
complete Rust and SBF implementation to those theorems. Credit demonstrated
risk controls where a credit, custody, or customer-protection rule makes them
relevant, without converting scoped proofs into whole-system assurance.

*7. Make separate findings for separate functions; "the software" is not a
unit of classification.* The CEA separately defines trading-facility,
swap-execution-facility, and derivatives-clearing-organization
functions.#note_ref(2) Here the instrument question arises at funding, the
venue question at match --- different milestones, potentially different
persons. Four findings, not one: instrument (rights, contingency, reference,
payout, transferability); venue (interaction, matching, interface operation,
control); intermediary (solicitation, order handling, discretion,
compensation); clearing and settlement (custody, novation or its absence,
netting, credit substitution, default handling). Software activity likewise:
publishing a general-purpose language, authoring product-specific terms,
operating an interface, holding custody, and receiving transaction-linked
compensation are different facts; "code" is too coarse in both directions.

A recent docket comment shows why status predicates need stating. FalconX
Bravo, Inc. would classify a cash-settled perpetual contract on a single
security or narrow-based security index by listing status: listed under the
security-futures framework, a security futures product; offered bilaterally,
over the counter, on a non-U.S. venue, or through a decentralized finance
protocol, a security-based swap.#note_ref(7) Listing becomes part of the
category test, not merely evidence of where trading occurs. Whether the
governing security-futures provisions require that result is not resolved
here; the Commissions should state expressly when listing or venue status is
a constituent statutory product fact and when a separate conduct or facility
fact. Reference, payout, contingency, listing, and venue can all be objective
facts; guidance should name the legal element each satisfies, not let one
label silently substitute for another.

= Scope

For the worked example I assume the referenced digital asset is a
non-security commodity; whether any actual digital asset is a security or
commodity is outside this comment. Substituting a single security, a
narrow-based security index, or an issuer-related event would require the
separate analysis in the security-based-swap definition and Question
8.#note_ref("1, 4") I leave open how much weight transferability carries
under a particular statutory prong. Every example references objectively
verifiable crypto-native facts --- ledger states, program events, prices,
ranges, path statistics --- never politics, sports, gaming, or subjective
social events, away from the boundary of Regulation 40.11 and a pending June
2026 proposal.#note_ref("5, 6") This comment classifies neither Dragon's
Clutch, its security-reference variant, nor any third-party product.

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
  [The Commissions seek objective criteria distinguishing product categories and ask when security-referencing event contracts fall within the options exclusion (Questions 1 and 8)], [Joint request for comment; source note 1],
  [The CEA separately defines trading-facility, swap-execution-facility, and derivatives-clearing-organization functions], [7 U.S.C. section 1a; source note 2],
  [The 2012 joint release adopted the current product definitions], [77 Fed. Reg. 48208; source note 3],
  [The Exchange Act defines security-based swap], [15 U.S.C. section 78c(a)(68); source note 4],
  [Regulation 40.11 and a pending 2026 proposal address event contracts and public-interest review], [Source notes 5 and 6],
  [The description of the FalconX Bravo listing-status proposal], [The filed comment; source note 7],
  [One claim from every band pays one collateral unit in aggregate in every permitted resolution state], [Direct arithmetic from the band terms],
  [The smooth basis functions are nonnegative and sum to one everywhere on the grid, so a smooth complete set pays the fixed denominator at any settled value], [Machine-checked theorems (partition of unity, solvency, quantization) among 212 zero-sorry Lean theorems modeling the construction; not a proof of the Rust or SBF implementation],
  [A guard-violating fill fails and changes nothing], [Named theorems in guarded-commitment formal models; a design property, not a deployed control],
  [Payout evaluation uses exact rational arithmetic and canonical largest-remainder integer quantization], [Pure-Rust kernels with independent exact-oracle differential tests],
  [Construction, resolution, redemption, custody, staged resolution, general clearing, and portfolio settlement execute in local SBF banks and, end to end, in the 44-transaction signed validator walk], [Local signed and bank campaigns, including the 22-transaction custody walk, blank-bank market construction per degree, and the August 20, 2026 validator walk against a sealed, independently attested build (100/100 executed manifest gates; portable attestation 44/44); laboratory evidence, unpromoted, fees forced zero],
  [Resolution is controlled by one canonical, sealed, program-owned source receipt and refuses substitution], [Local real-SBF receipt-binding and substitution-refusal tests; source construction exercised against a deliberately non-production stand-in provider],
  [The selected production source profile has a documented unique update per instant], [The provider's published interface documentation; source note 8],
  [The default build refuses deposits with no compiled source release; no public-cluster deployment exists (devnet authorized but unfunded)], [Repository status and decision records dated August 20, 2026],
)
