# 24/7 and perpetuals RFC --- candidate comment: the reference price as an observation program

Status: **candidate text, not filed.** Prepared 2026-08-18 by the
candidate-draft lane as a POSSIBLE fourth filing, for the author's go/no-go
only. *(Status note 2026-08-19: a Typst conversion has since landed —
`docs/regulatory/typst/perpetuals/` and
`output/pdf/cftc-perpetuals-comment-draft-1.pdf`, corrected by its own
manipulation-cost experiment — so "not in the Typst tree, converted nowhere"
no longer describes the tree. The go/no-go itself remains the author's and is
recorded nowhere in-repo; nothing has been filed.)* Absent the author's
express go it is filed nowhere and announced nowhere.
Supporting analysis is in [OPEN_MATTERS_MAP.md](OPEN_MATTERS_MAP.md)
(finding 2.1 and shortlist entry 2); the argument is ported from
[definitions-q15-reference-integrity.md](definitions-q15-reference-integrity.md),
not cited by it.

## Go/no-go

- **What this would be.** The program's fourth filing document of the week
  of August 24: a standalone, CFTC-only comment on the request for comment
  on 24/7 trading and perpetual contracts referencing physically delivered
  or storable energy commodities --- **RIN 3038-AF75, docket
  CFTC-2026-1388, comment document CFTC-2026-1388-0001**. It would be the
  program's first filing outside the joint digital-asset ground and its
  first on an energy-adjacent docket.
- **Deadline.** **Wednesday, August 26, 2026** (91 FR 47158, extending
  91 FR 38334) --- between the two joint filings due Monday August 24 and
  the IAC written statement due Thursday August 27.
- **Workload consequence.** A fourth document drafted, audited, Typst-set,
  and filed in the same seven days that already carry the joint-filing
  edits (Aug 18-22) and the IAC rewrite (Aug 20-26). The drafting window
  collides with both; this candidate exists so that a "go" on Aug 19-20
  costs conversion and audit, not drafting.
- **Case FOR.** The Commission asked this program's exact question, in its
  own words: Question 40 asks for methodologies providing "24/7,
  manipulation-resistant observability at every funding interval";
  Question 50 restates Core Principle 3 as a continuous per-interval
  obligation; Question 53 asks "on what evidentiary basis" the
  demonstration would rest; Question 66 names "demonstrable continuous
  manipulation-resistance" as a candidate listing criterion. The argument
  is already written --- the Question 15 memo ports with no new research
  --- and it is the same argument the IAC statement's Position 8 carries,
  so one argument serves two audiences in reinforcing filings. The docket
  holds 86 comments plus 4 on the extension (Federal Register API,
  2026-08-18); on the evidence available, no filer argues from a
  formal-methods/specification position --- but note the open-matters lane
  could not read any comment body (regulations.gov returned 429 all day),
  so that absence is a characterization of the likely population (energy
  trade associations and exchanges), not a verified fact.
- **Case AGAINST.** An energy-adjacent docket on which the program has no
  data of any kind, answering an RFC that asks commenters to support
  responses "with data, empirical analysis, transaction- or market-level
  statistics, and supporting documents rather than with conclusory
  assertions." Verbatim retrieval sharpened this: Question 40 asks for
  methodologies "**outside of the digital-asset context**" --- the
  Commission brackets exactly the context this program's grounding lives
  in, so the comment must argue the property set, not the substrate, and a
  reader may still discount it as a digital-asset answer to an energy
  question. Roughly ninety filers are ahead, presumably with the data. A
  fourth document filed thin in the same week as three others is the
  failure mode the packet's length discipline exists to avoid.
- **If go, before filing.** (1) Typst conversion into the shared template
  (the text below already uses `---` dashes and the Draft 6 register);
  (2) claim-audit rows against
  [../DRAFT5_CLAIM_LEDGER.md](../DRAFT5_CLAIM_LEDGER.md) --- the two
  central artifact sentences are the V-24 and V-38(b) allowed wordings
  verbatim, and the accumulator-description sentence needs one new
  narrowing row; (3) a John packet row; (4) docket revalidation
  immediately before filing: RIN, docket, deadline, filing method, any
  further extension, and comment count re-checked (with a registered
  regulations.gov key if available); (5) add 91 FR 38334 and 91 FR 47158
  to the LEGAL_ANALYSIS.md citation ledger --- neither is on it; (6)
  identity placeholders and the filing-day evidence re-pin per the
  standing gates.

Drafting facts. Register: filing register --- first person, declarative
positions, no claim labels, Limits section, basis appendix. Question text
below is quoted verbatim from the GPO full text of 91 FR 38334 (FR Doc
2026-12784, June 25, 2026, pp. 38334-39) and 91 FR 47158 (FR Doc
2026-15216, July 28, 2026), both retrieved 2026-08-18 via
federalregister.gov; every quotation was verified against that retrieval,
none is taken from a secondary memo. Every artifact sentence is at or
below the DRAFT5_CLAIM_LEDGER.md ceilings; the monotone accepted-path rule
is claimed as existing and tested, not formally verified, only. Body
length: about 1,700 words plus a five-row basis appendix --- four pages at
the Draft 6 body density, the top of the stated 3-4 page budget. If the
Typst render runs long, the sanctioned trims, in order, are: the
Question 62 section entire (position 4 and its paragraph; nothing else
references them), then the second sentence of the Question 51 answer in
the first argument section ("As to the timing half ..."); neither touches
a ceiling-bound sentence.

---

## The candidate comment, verbatim

Re: Request for Comment on the Extension of Standard Futures Contracts to
24/7 Trading and on Perpetual Contracts Referencing Physically Delivered
or Storable Energy Commodities, RIN 3038-AF75, 91 FR 38334 (June 25,
2026), comment period extended through August 26, 2026, 91 FR 47158
(July 28, 2026).

### Summary of positions

I write in response to the reference-price and observability questions
only: Questions 38 through 41 (Section F), Questions 50 through 53
(Section I), and, narrowly, Questions 62 and 66. I take no position on whether any 24/7 or
perpetual energy contract should be listed; on the energy-market,
delivery, storage, convergence, and cost-of-carry questions; on margin,
clearing, and default management; on position-limit levels; on customer
protection, leverage, or access; on the Commission's surveillance
resourcing; or on the stayed NYMEX 10-Barrel WTI Crude Oil futures
contract. I have no energy market data, and nothing in this comment
supplies any.

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
   payment, for the length of the window, is computable in advance from
   the same stated terms. Surveillance thresholds and funding-interval
   design should consume that number --- and in a market that never
   closes, window and interval design is the only dial that sets it.
3. The demonstration Question 53 asks about should be specified as a
   recomputation: a reference methodology stated precisely enough that any
   party can recompute every funding-interval print from public inputs.
   The evidentiary basis is then the replay, and disagreement is a fact
   about inputs, not a dispute about process.
4. Any automatic mechanism that consumes the reference --- funding
   calculation or liquidation trigger --- should inherit the reference's
   refusal rule, with its behavior on a refused interval fixed in the
   terms in advance.

### The reference price is an observation program (Questions 38, 40, 50, 51)

Appendix C to part 38, as the Commission summarizes it in Question 38,
accepts a cash-settled contract "only where the settlement price is
reliable, acceptable, publicly available, and timely." A contract with one
settlement asks those four questions once. Question 50 states the
perpetual difference exactly: "the reference price must be reliable at
every funding interval on a continuous basis rather than at a single
settlement." A property demanded continuously cannot rest on episodic
review; it has to be a property of the machinery that produces the print.

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
--- rather than a discretionary fallback. As to the timing half of
Question 51: a frozen grid protects the timing of the calculation
structurally, because no one chooses when to observe; the thin-liquidity
half is addressed below.

Question 40 asks whether reference-price methodologies "outside of the
digital-asset context" could provide "24/7, manipulation-resistant
observability at every funding interval." My research ground is the
context the question brackets, so I offer no methodology for crude oil and
no view on which energy price source, if any, satisfies Appendix C. What I
can report is that the integrity properties above are
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

### Manipulation cost as a number the terms make computable (Questions 40, 41, 51, 66)

Freezing the observation program removes reporting discretion; it does not
prevent trading from influencing the underlying price. It relocates
manipulation risk rather than removing it, and it makes the residual
attack easier to plan, because the attacker knows exactly which statistic
over exactly which window decides the payment. In exchange, the residual
risk becomes computable. For a time-weighted statistic read from a named
venue with a known liquidity surface, the capital required to hold the
print across a decision boundary for the length of the window follows
from stated inputs --- venue depth, fee structure, sampling grid, window
length --- and
anyone can compute it in advance and compare it to what the contract puts
at stake at that interval.

Three consequences. First, funding-interval design is not an operational
detail but the manipulation-cost dial: a futures contract has a close, a
perpetual has only its windows, and window length and sampling density are
where "manipulation-resistant" is purchased or forfeited. Overnight and
weekend liquidity reduction (Question 51) enters the same computation as
an input --- a thinner surface raises no new category of risk, it lowers a
computable cost, and the terms can respond by widening the window or
refusing the interval. Second, surveillance should consume the number: a
stated reference specification converts "watch for manipulation" into
"watch this venue during these minutes," and the computed cost is a
screening input for allocating attention --- which answers the
funding-linkage concern in Question 41, since the linkage's cross-market
minutes are exactly the stated windows. Third, for Question 66,
"demonstrable continuous manipulation-resistance" is demonstrable
precisely when the inputs to this computation are in the contract's
terms. A computed cost is a model output, sensitive to assumptions about
depth, latency, and attacker inventory, and the Commission should not be
asked to trust a number of that kind; the requirement belongs on the
*inputs*. The number is a screening tool; the inputs are facts. I have
produced no manipulation-cost measurement and no analysis of any real
venue, and I state no number here.

### The evidentiary basis is a replay (Questions 52, 53)

Question 53 asks what a DCM would be required to demonstrate, "and if so,
on what evidentiary basis?" The demonstration should be specified as a
recomputation. If the reference methodology is stated precisely enough
that any party --- the DCM, the Commission, a counterparty, a member of
the public --- can recompute every funding-interval print from public
inputs and reach the same value or the same refusal, then the evidentiary
basis is the replay itself: performed by every recomputation,
continuously, rather than asserted in a filing and audited episodically.
Disagreement, when it comes, is located in facts --- which observations
were admitted, from which source, under which stated rule --- not in a
dispute about how a number was produced. A methodology that cannot be
recomputed from its stated terms cannot make the continuous demonstration
Core Principle 3 is being read to require; that, and not any
energy-specific judgment, is the objective criterion I offer under
Question 66. The same specification does Core Principle 4's cross-market
work (Question 52) by naming which external venue and which minutes are
settlement-relevant. Recomputation-as-acceptance is implementable: in a
different setting, I built the batch verifier to accept a submitted
clearing only if recomputation from the frozen book reproduces it exactly,
never trusting the submitter's claimed quantities. That is offline
research code about cleared batches, not energy references; I offer it
only as evidence that the acceptance rule is workable, not as a system.

### Refusal and automatic mechanisms (Question 62)

Question 62 asks, of automatic liquidation mechanisms, "how can they be
designed to avoid amplifying price movements during periods of market
stress?" I answer only the slice that touches the reference: stress is
when observation degrades, so a liquidation trigger that consumes the
reference price will meet refused intervals exactly when its actions
matter most. A mechanism that acts anyway, on a degraded or interpolated
print, converts an observation failure into forced flow --- amplification
by construction. The mechanism should inherit the reference's refusal
rule: its behavior on a refused interval --- defer, halt, or fall to a
stated conservative rule --- should be fixed in the terms in advance, so
that what happens when the market cannot be observed is itself a term
anyone can read, not a decision made during the stress.

### Limits

I have no energy market data, no view of any energy cash or futures
market's liquidity, storage, or delivery mechanics, and no position on
whether any contract discussed in the request should be listed. The
artifacts behind this comment are offline research prototypes: tested,
not formally verified; not deployed; holding no funds. I
have produced no manipulation-cost measurement, no market data study, and
no analysis of any real venue. The positions are my analysis as a
commenter of how reference-price integrity should be specified and
demonstrated; none is a compliance conclusion, a product proposal, or
legal advice.

Respectfully submitted,

[FULL NAME]
[AFFILIATION, IF ANY]
[DATE]

### Appendix: basis of material technical claims

| Claim | Basis |
|---|---|
| The request's question text, the Appendix C factor summary, and the Section F and I framings quoted above | 91 FR 38334, 38337-39 (June 25, 2026); GPO full text retrieved 2026-08-18 |
| The comment period extension to August 26, 2026, and the NYMEX self-certification and stay | 91 FR 47158 (July 28, 2026); GPO full text retrieved 2026-08-18 |
| An observation accumulator with a frozen grid, sealing windows, a monotone accepted-observation path, and refusal of unsupportable questions has been implemented offline with passing deterministic tests | Pure-Rust research prototype reviewed by the submitter; tested, not formally verified; not deployed; source authentication an assumed input contract |
| The batch verifier accepts a submitted clearing only if recomputation from the frozen book reproduces it exactly | Same prototype family; offline research code; tested, not formally verified; not deployed |
| No energy market data, no manipulation-cost measurement, no analysis of any real venue | The submitter's repository status records; a statement about the submitter's own artifacts, not about any third party |
