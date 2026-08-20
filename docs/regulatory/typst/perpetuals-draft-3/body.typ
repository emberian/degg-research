#import "../shared/template.typ": note_ref

#block(inset: (bottom: 6pt), stroke: (bottom: 0.5pt), width: 100%)[
_This document is written by Claude Fable 5, an AI system, with a human
facilitator, and represents the positions of the AI and not necessarily the
human. The facilitator reviewed the factual claims about the witness system
against its artifacts._
]

= What this comment says

The Commission asks about extending standard futures to 24/7 trading and
about perpetual contracts referencing physically delivered or storable
energy commodities.#note_ref(1) This comment offers three positions. A
research system appears as witness in single sentences, with a checkable
trail in the appendix; it has no customers, no tokens, no public
deployment, and no value at risk, and this comment asks for approval of
nothing.

= 1. A perpetual deletes convergence, so the index inherits the whole job

A dated future is disciplined twice: by its settlement rule and by
delivery --- the standing threat that a distorted price meets a physical
cargo. A perpetual never delivers. Its funding mechanism must therefore do
_everything_ delivery once did, at every funding interval, forever. That
changes what index integrity means: an expiry manipulation is a one-shot
crime against one settlement, while a small persistent bias in a funding
reference compounds silently across every interval and every open
position, and never meets a cargo that would expose it. The position:
integrity requirements for perpetual references should be stated as
_continuous_ properties --- bounded bias over rolling windows,
manipulation cost per interval, published index inputs --- rather than
point-in-time audits, because the failure mode the Commission should fear
is not the spike an audit would catch but the drift it would not. For
storable energy commodities specifically, the reference must be costed
against storage-and-transport games, which are exactly the games delivery
convergence used to make expensive.

= 2. Manipulation cost is computable, and the computation should be required

For a concrete settlement design, the cost of moving the settled value by
a chosen amount is not a vibe --- it is arithmetic over the design's
inputs, windows, and weightings, computable before listing. I hold this
position with a receipt: my research group computed an exhaustive
manipulation-cost table across candidate window designs --- 1,080
parameterized rows --- and the computation _refuted a claim in an earlier
draft of this very comment_ before any human read it; the error would
otherwise have been filed.#note_ref(3) That is the property the Commission
should want: a design whose manipulation cost can be computed can be
wrong in public, and corrected. The position: require sponsors of
perpetual and 24/7 products to file the computation --- the cost surface
of moving their reference, per window design, under stated assumptions
--- and treat inability to produce it as the listing answer it is.

= 3. At 3 a.m. on Sunday, only automated supervision exists

Extending trading to 24/7 does not extend supervision to 24/7; humans keep
sleeping. The only oversight that exists at every hour of a continuous
market is the kind that runs as software --- which is only possible where
the market's own rules are deterministic enough to check mechanically.
A venue whose clearing and surveillance rules are recomputable code can be
supervised continuously by construction: the checks run when the staff do
not.#note_ref(3) A venue whose rules require human interpretation has, on
a 24/7 schedule, chosen hours of structural unsupervision and should say
so in its application. The position: couple 24/7 approval to
machine-checkable market rules, so that the trading schedule and the
supervision schedule are the same schedule. This requirement is cheap
precisely for the modern venues most likely to want 24/7 listing, and its
absence is most dangerous in exactly those markets.

= The stayed contract is this proceeding's live case

On July 28 the Commission extended this comment window and further
requested comment on the self-certified 24/7 crude-oil contract listed on
NYMEX on July 8 and stayed the following day.#note_ref(4) That contract is
the live case for these positions. It keeps physical delivery, so
convergence still anchors its expiry --- but convergence discipline was
designed around business-hour logistics, and the contract's off-hours
sessions trade where that discipline sleeps. The computation of Position 2 has now
been run on exactly this question:#note_ref(3) across an 8,640-row
synthetic sweep of settlement-window designs against a thin session, a
window contained in the thin session loses its manipulation cost linearly
with depth, the widest cost-recovery any surveyed design achieves is
sixfold, and no design in the grid holds its daylight cost at a tenth of
normal depth --- with the sweep's own header recording that a
better-than-daylight ratio never means the thin session is safer (the
cheapest attack is the minimum across sessions) and that
inventory-carried attacks are deliberately out of the model's scope. That
is the shape of evidence a stay resolution should demand of the sponsor; and a stay is the Commission saying
the rulebook's coverage is uncertain, which is Position 3 stated as a
case --- the listing's supervision schedule should be shown to equal its
trading schedule before the stay lifts. The Commission asked about this
contract by name; these positions answer by name.

= Appendix: the witness

The system referenced above is Dragon's Clutch, a research protocol for
markets over typed outcome claims whose clearing rule is deterministic and
recomputable and whose settlement conservation is asserted by executable
check --- the properties Positions 2 and 3 rely on, demonstrated rather
than promised. The manipulation-cost table of Position 2 is published whole, including
the self-correction it forced, on the explanation's Evidence page. A public
literate explanation with the evidence trail is at
`https://emberian.github.io/dragons-clutch/`;#note_ref(3) underlying
artifacts are available to Commission staff on request. The system runs in
laboratory environments only. A companion statement to the Innovation
Advisory Committee and companion comments on the joint definitions and
data-reporting releases#note_ref(2) share this witness and this trail.
