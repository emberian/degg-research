#import "../shared/template.typ": note_ref

= Summary of positions

The Commission requests comment on the extension of standard futures
contracts to 24/7 trading and on perpetual contracts referencing physically
delivered or storable energy commodities, and has extended the comment
period through August 26, 2026.#note_ref(1)#note_ref(2)
I write in response to the reference-price and observability questions
only: Questions 38 through 41 (Section F), Questions 50 through 53
(Section I), and, narrowly, Question 66. I take no position on
whether any 24/7 or perpetual energy contract should be listed; on the
energy-market, delivery, storage, convergence, and cost-of-carry questions;
on margin, clearing, and default management; on position-limit levels; on
customer protection, leverage, or access; on the Commission's surveillance
resourcing; or on the stayed NYMEX 10-Barrel WTI Crude Oil futures
contract.#note_ref(2) I have no energy market data, and nothing in this
comment supplies any.

I am a software and formal-methods researcher; I study markets that settle
by reading prices from named venues under frozen rules. The positions are
my own analysis as a commenter; what I report about my research artifacts
is stated separately and stays within what those artifacts support.

1. A reference price computed at every funding interval is an observation
   program, and a contract's terms can state that program's integrity
   properties as checkable requirements --- a frozen source specification
   and sampling grid, a monotone accepted-observation path, windows that
   seal, and deterministic refusal on gaps and dispersion --- rather than
   as assurances held by an exchange or a vendor.
2. For a contract settling on a time-weighted statistic read off a named
   venue, the cost of moving that statistic far enough to change a
   payment, for the required fraction of the window's samples, is
   computable in advance from the same stated terms. Surveillance
   thresholds and funding-interval design should consume that number ---
   and in a market that never closes, the sampling structure of the
   window is the only dial that sets it.
3. The demonstration Question 53 asks about should be specified as a
   recomputation: a reference methodology stated precisely enough that any
   party can recompute every funding-interval print from public inputs.
   The evidentiary basis is then the replay, and disagreement is a fact
   about inputs, not a dispute about process.

= The reference price is an observation program (Questions 38, 40, 50)

Appendix C to part 38, as the Commission summarizes it in Question 38,
accepts a cash-settled contract "only where the settlement price is
reliable, acceptable, publicly available, and timely."#note_ref(1) A
contract with one settlement asks those four questions once. Question 50
states the perpetual difference exactly: "the reference price must be
reliable at every funding interval on a continuous basis rather than at a
single settlement."#note_ref(1) A property demanded continuously cannot
rest on episodic review; it has to be a property of the machinery that
produces the print.

That machinery is an observation program: a rule for what counts as an
observation, a grid saying when observations are expected, a statistic
over a window, and a rule for every failure of the source. Each Appendix C
factor becomes checkable per interval when the terms state that program
explicitly: the source specification and sampling grid are frozen in the
contract's terms, so "reliable" and "acceptable" are judged once, in
public, about a named source; the accepted-observation path is monotone
and append-only, so a print cannot be quietly recomputed from revised
history; windows seal, so each print is final on stated evidence at a
stated time; and coverage or dispersion outside stated bounds produces a
deterministic refusal --- a defined failure output carrying its reason
--- rather than a discretionary fallback.

Question 40 asks whether reference-price methodologies "outside of the
digital-asset context" could provide "24/7, manipulation-resistant
observability at every funding interval."#note_ref(1) My research ground
is the context the question brackets, so I offer no methodology for crude
oil and no view on which energy price source, if any, satisfies
Appendix C. What I can report is that the integrity properties above are
substrate-independent: they constrain how observations are admitted,
aggregated, and refused, not what is observed, and a methodology from any
context either exhibits them checkably or does not.
My basis for asserting they are implementable is stated exactly: I have
built an offline research prototype of this observation design --- an
accumulator that absorbs observations over a frozen grid into windows that
seal, whose accepted-observation path is monotone, refusing an input that
appears to rewind rather than believing it; and I built the observation
accumulator to refuse a question its retained information cannot support
rather than approximate it. It is tested, not formally verified; it is not
deployed; source authentication is an assumed input contract, not
implemented; and it has read no energy market data.

= Manipulation cost as a number the terms make computable (Questions 40, 41, 51, 66)

Freezing the observation program removes reporting discretion; it does not
prevent trading from influencing the underlying price. It relocates
manipulation risk rather than removing it, and it makes the residual
attack easier to plan, because the attacker knows exactly which statistic
over exactly which window decides the payment. In exchange, the residual
risk becomes computable. For a time-weighted statistic read from a named
venue with a known liquidity surface, the capital required to hold the
print across a decision boundary for the length of the window follows
from stated inputs --- venue depth, fee structure, sampling grid, window
length --- and anyone can compute it in advance and compare it to what the
contract puts at stake at that interval.

Three consequences. First, funding-interval design is not an operational
detail but the manipulation-cost dial: a futures contract has a close, a
perpetual has only its windows, and the sampling structure --- how many
independent prints a window takes and what fraction an adversary must
hold --- is where "manipulation-resistant" is purchased or forfeited;
lengthening a window without adding samples buys nothing. Overnight and
weekend liquidity reduction (Question 51) enters the same computation as
an input --- a thinner surface raises no new category of risk, it lowers a
computable cost, and the terms can respond by widening the window or
refusing the interval. Second, surveillance should consume the number: a
stated reference specification converts "watch for manipulation" into
"watch this venue during these minutes," and the computed cost is a
screening input for allocating attention --- which answers the
funding-linkage concern in Question 41, since the linkage's cross-market
minutes are exactly the stated windows. Third, for Question 66,
"demonstrable continuous manipulation-resistance"#note_ref(1) is
demonstrable precisely when the inputs to this computation are in the
contract's terms. A computed cost is a model output, sensitive to
assumptions about depth, latency, and attacker inventory, and the
Commission should not be asked to trust a number of that kind; the
requirement belongs on the *inputs*. The number is a screening tool; the
inputs are facts. I have produced no measurement of any real venue and no
analysis of any real venue, and I state no number here.

= The evidentiary basis is a replay (Questions 52, 53)

Question 53 asks what a DCM would be required to demonstrate, "and if so,
on what evidentiary basis?"#note_ref(1) The demonstration should be
specified as a recomputation. If the reference methodology is stated
precisely enough that any party --- the DCM, the Commission, a
counterparty, a member of the public --- can recompute every
funding-interval print from public inputs and reach the same value or the
same refusal, then the evidentiary basis is the replay itself: performed
by every recomputation, continuously, rather than asserted in a filing and
audited episodically. Disagreement, when it comes, is located in facts ---
which observations were admitted, from which source, under which stated
rule --- not in a dispute about how a number was produced. A methodology
that cannot be recomputed from its stated terms cannot make the continuous
demonstration Core Principle 3 is being read to require; that, and not any
energy-specific judgment, is the objective criterion I offer under
Question 66. The same specification does the cross-market work of Core
Principle 4 (Question 52) by naming which external venue and which minutes
are settlement-relevant. Recomputation-as-acceptance is implementable: in
a different setting, I built the batch verifier to accept a submitted
clearing only if recomputation from the frozen book reproduces it exactly,
never trusting the submitter's claimed quantities. That is offline
research code about cleared batches, not energy references; I offer it
only as evidence that the acceptance rule is workable, not as a system.

= Limits

I have no energy market data, no view of any energy cash or futures
market's liquidity, storage, or delivery mechanics, and no position on
whether any contract discussed in the request should be listed. The
artifacts behind this comment are offline research prototypes: tested,
not formally verified; not deployed; holding no funds. I
have produced no measurement of any real venue, no market data study, and
no analysis of any real venue. The positions are my analysis as a
commenter of how reference-price integrity should be specified and
demonstrated; none is a compliance conclusion, a product proposal, or
legal advice.

#block(breakable: false)[
  #v(18pt, weak: true)
  Respectfully submitted,

  [FULL NAME]  \
  [AFFILIATION, IF ANY]  \
  [DATE]
]

= Appendix: basis of material technical claims

Each material technical claim, with its evidentiary basis in one line. No
artifact behind these claims is deployed market infrastructure, and none
has been independently audited.

#table(
  columns: (1fr, 2.1in),
  table.header([*Claim*], [*Basis*]),
  [The request's question text, the Appendix C factor summary, and the Section F and I framings quoted above], [91 Fed. Reg. 38334, 38337-39 (June 25, 2026); GPO full text retrieved August 18, 2026; source note 1],
  [The comment period extension to August 26, 2026, and the NYMEX self-certification and stay], [91 Fed. Reg. 47158 (July 28, 2026); GPO full text retrieved August 18, 2026; source note 2],
  [An observation accumulator with a frozen grid, sealing windows, a monotone accepted-observation path, and refusal of unsupportable questions has been implemented offline with passing deterministic tests], [Pure-Rust research prototype reviewed by the submitter; tested, not formally verified; not deployed; source authentication an assumed input contract],
  [The batch verifier accepts a submitted clearing only if recomputation from the frozen book reproduces it exactly], [Same prototype family; offline research code; tested, not formally verified; not deployed],
  [No energy market data, no measurement of any real venue, no market data study], [The submitter's repository status records; a statement about the submitter's own artifacts, not about any third party],
)
