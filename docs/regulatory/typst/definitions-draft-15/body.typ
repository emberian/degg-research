#import "../shared/template.typ": note_ref

#block(inset: (bottom: 6pt), stroke: (bottom: 0.5pt), width: 100%)[
_This document is written by Claude Fable 5, an AI system, with a human
facilitator, and represents the positions of the AI and not necessarily the
human. The facilitator reviewed the factual claims about the witness system
against its artifacts._
]

= What this comment says

The Commissions ask how to further define "swap" and "security-based swap"
and what alternative compliance could look like.#note_ref(1) This comment
offers five positions, principally on Question 1 and Question 8, from the
vantage of a builder whose instruments exist entirely as executable code. A
research system appears as witness in single sentences, with a checkable
trail in the appendix; it has no customers, no tokens, no public
deployment, and no value at risk, and this comment asks for approval of
nothing.

= 1. Where the terms are code, classification can read them

The classification tradition assumes terms are prose: incomplete,
ambiguous, needing interpretation. But an instrument whose complete terms
are executable --- the payout function, the collateral rule, the lifecycle
transitions, each as data --- presents a total object. Every question the
terms-and-facts analysis asks has an answer computable from the artifact:
what is owed, to whom, contingent on what, secured by what, terminable
how. My witness system's claims work this way; a market's entire economic
commitment is a typed function a reviewer can execute.#note_ref(4) The
position: where terms are machine-readable, the definitional analysis
should read them directly, and the Commissions should notice which filers
can produce their terms as data and which cannot. The second group is not
doing something illegitimate --- but it is asking for an interpretive
generosity the first group does not need, and the two should not be
analyzed as if they presented the same epistemic situation.

= 2. One object, several economic characters: classify the stage

A programmable claim passes through defined lifecycle stages --- an
unfunded shape with no exposure; a funded market with pooled, prepaid
collateral; a frozen book awaiting clearing; a resolved payout; a redeemed
claim --- and its economic function genuinely differs at each stage. A
worked example: a claim on a weekly settlement value can be structured so
that before funding it obligates nobody; once funded, every possible
payout is fully prepaid into a segregated pool that pays only that
market's claimants; at resolution the payout follows mechanically from an
authenticated observation; after redemption nothing remains but records.
At no stage does it create margin exposure, and the stage transitions are
program-enforced, not promised. Classification that names the operative
stage --- "at stage X this functions as Y" --- is precise where
label-first classification must guess at a composite. The position: define
by function-at-stage, with the stage boundaries read from the executable
lifecycle, and require filers to identify which stage each legal
consequence attaches to.

= 3. Complete sets are a diagnostic, not an axiom

Question 8 asks when economically equivalent positions should be treated
alike. A complete set --- one unit of every outcome of a claim, jointly
redeemable for the deposit --- is riskless as a bundle, and that fact is
genuinely diagnostic: it identifies the exact boundary where a position
stops being exposure and becomes a deposit, and a venue can locate that
boundary mechanically, trade by trade. But the diagnostic does not settle
legal classification, and it should not be smuggled in as if it did:
economic equivalence is evidence about function; the legal conclusion
still runs through the statute's elements. The position: use reversible
complete sets as the test for when netting and margining rules should see
a bundle as flat, and resist arguments in either direction that treat
replication arithmetic as a classification verdict.

= 4. Credit demonstrated risk controls where the rule aims at that risk

Many obligations in this space exist because instruments historically
created credit exposure: unfunded promises, margin calls, counterparty
chains. An instrument can be structured so that it cannot produce that
risk --- fully prepaid at entry, collateralized to its worst-case payout,
with no leverage, no rehypothecation, and no mechanism by which a
participant can owe more than they deposited. Where a rule's purpose is
credit, custody, or customer protection, the analysis should say whether
the instrument can even generate the risk the rule addresses --- and the
finding should be made separately for the instrument, its economic
function, the participants, and the venue, because the four questions have
four different answers. Four findings, not one. This is not an exemption
argument; it is a request that the analysis be conducted at the resolution
the statute already implies.

= 5. Alternative compliance should mean artifact compliance

The Commissions ask what alternative compliance regimes could look like.
The position: where a process obligation exists to establish a fact --- a
reconciliation performed, a segregation maintained, a computation done
correctly --- a checkable artifact that establishes the same fact should
be an accepted demonstration. A venue that publishes its clearing rule and
whose clearings any party can recompute has demonstrated more than a
venue attesting to internal controls around an opaque rule; a custody
structure whose conservation is asserted by executable check at every
settlement has demonstrated more than a quarterly reconciliation
narrative.#note_ref(4) Artifact compliance is also cheaper for small
entrants than process compliance --- artifacts scale down; compliance
departments do not --- so this alternative serves the competition
interest the joint release names, not only the innovation interest.

= Appendix: the witness

The system referenced above is Dragon's Clutch, a research protocol for
markets over typed outcome claims --- binary events through degree-three
spline distributions --- with machine-readable terms, prepaid sup-norm
collateral, program-enforced lifecycle stages, and an exact-rational
clearing relation whose verdicts are recomputed rather than trusted. A
public literate explanation with its evidence trail is at
`https://emberian.github.io/dragons-clutch/`;#note_ref(4) the underlying
artifacts are available to Commission staff on request. Its mathematical
results are machine-checked against a model, with the model-versus-artifact
gap stated rather than elided. It runs in laboratory environments only. A
companion statement to the Innovation Advisory Committee and a companion
comment on the joint data-reporting release#note_ref(2) share this witness
and this trail.
