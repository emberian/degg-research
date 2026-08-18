#import "../shared/template.typ": note_ref

= Summary of positions

The Commission requests comment on the extension of standard futures
contracts to 24/7 trading and on perpetual contracts referencing physically
delivered or storable energy commodities, and has extended the comment
period through August 26, 2026.#note_ref("1, 2")
I write in response to the reference-price and observability questions
only: Questions 38 through 41 (Section F), Questions 50 through 53
(Section I), and, narrowly, Question 66. I take no position on
whether any 24/7 or perpetual energy contract should be listed; on the
energy-market, delivery, storage, convergence, and cost-of-carry questions;
on margin, clearing, and default management; on position-limit levels; on
customer protection, leverage, or access; on the Commission's surveillance
resourcing; or on the stayed NYMEX 10-Barrel WTI Crude Oil futures
contract.#note_ref(2) I have no energy market data, and nothing in this
comment supplies any. This is a separate research comment, not part of a
filing packet, and it should not be treated as filing-ready merely because
it compiles.

I am a software and formal-methods researcher; I study markets that settle
by reading prices from named venues under versioned rules. The positions are
my own analysis as a commenter; what I report about my research artifacts
is stated separately and stays within what those artifacts support.

1. A reference price computed at every funding interval is an observation
   program, and a contract's terms can state that program's integrity
   properties as checkable requirements --- a versioned source
   specification and sampling grid, an append-only accepted-observation
   path with explicit correction records, windows that seal, and stated
   outputs on gaps and dispersion. Those properties aid auditability; they
   do not establish source reliability or remove liveness, censorship,
   timestamping, or operator-discretion risk.
2. For a contract settling on a time-weighted statistic read off a named
   venue, a model can produce an assumption-indexed manipulation-cost
   envelope, not a venue fact or guaranteed lower bound. The filing should
   expose the assumptions, parameter ranges, sensitivities, and cases in
   which no useful bound is supported. Window design is one risk control
   among source selection, exposure limits, margin and liquidation design,
   surveillance, and operational failure rules.
3. The demonstration Question 53 asks about should be specified as a
   recomputation: a reference methodology stated precisely enough that any
   authorized verifier with the admitted inputs can recompute every
   funding-interval print.
   Replay demonstrates deterministic recomputation from the same admitted
   inputs. It does not demonstrate that the inputs are authentic, complete,
   timely, representative, continuously available, or resistant to
   manipulation; those propositions require separate evidence.

= The reference price is an observation program (Questions 38, 40, 50)

Appendix C to part 38, as the Commission summarizes it in Question 38,
accepts a cash-settled contract "only where the settlement price is
reliable, acceptable, publicly available, and timely."#note_ref(1)
Appendix C itself asks for the index methodology and supporting information
showing those properties and that the index reflects the underlying cash
market and is not readily subject to manipulation or distortion.#note_ref(3)
A
contract with one final settlement evaluates its settlement statistic at
that event; a perpetual repeats the reference-price task. Question 50
states the perpetual difference exactly: "the reference price must be
reliable at every funding interval on a continuous basis rather than at a
single settlement."#note_ref(1) A property demanded continuously cannot
be demonstrated by an episodic replay alone; the machinery that produces
the print, the source that supplies it, and the people and systems that
operate both all matter.

That machinery includes an observation program: a rule for what counts as
an observation, a grid saying when observations are expected, a statistic
over a window, and a rule for every failure of the source. The program can
make some questions checkable per interval. Its source specification,
sampling grid, aggregation rule, and version identifier can be fixed in
the contract's terms. Its accepted-observation history can be append-only
without pretending that erroneous data are immutable: a correction is a
new record that identifies the superseded record, the authority and reason
for correction, its effective time, and whether already-sealed prints are
restated or only future prints change. The prior record remains available.
Windows can seal on stated evidence at a stated time, and coverage or
dispersion outside stated bounds can produce a defined failure output
carrying its reason.

These are determinism and provenance controls, not proof of the Appendix C
factors. A frozen grid does not cause a source to publish, prevent a source
or relay from censoring or delaying an observation, establish that venue
timestamps correspond to the intended economic instant, or eliminate
judgment in source governance and emergency operation. Even with fixed
code, choices about admission, ordering, clock synchronization, outage
classification, and activation of an emergency rule can retain timing
discretion. A complete design therefore has to identify liveness
assumptions, who can delay or exclude an observation, correction and
supersession authority, maximum tolerated delay, and the consequences of
missing the deadline. I offer no evidence that any energy source meets
those conditions.

Question 40 asks whether reference-price methodologies "outside of the
digital-asset context" could provide "24/7, manipulation-resistant
observability at every funding interval."#note_ref(1) My research ground
is the context the question brackets, so I offer no methodology for crude
oil and no view on which energy price source, if any, satisfies
Appendix C. What I can report is that the integrity properties above are
expressible without selecting a substrate: they constrain how observations
are admitted, aggregated, corrected, and refused, not what is observed.
That syntactic generality does not establish operational suitability for
an energy reference.
My basis for asserting they are implementable is stated exactly: I have
built an offline research prototype of this observation design --- an
accumulator that absorbs observations over a frozen grid into windows that
seal, whose accepted-observation path is monotone, refusing an input that
appears to rewind rather than believing it; and I built the observation
accumulator to refuse a question its retained information cannot support
rather than approximate it. It is tested, not formally verified; it is not
deployed; source authentication is an assumed input contract, not
implemented; it does not establish a correction-governance regime; and it
has read no energy market data.

= Manipulation cost as an assumption-indexed envelope (Questions 40, 41, 51, 66)

Freezing part of the observation program narrows some forms of reporting
discretion; it does not remove the operational discretion just identified
or prevent trading from influencing the underlying price. For a
time-weighted statistic read from a named venue, a specified model can
estimate what selected attack strategies would cost under selected
assumptions. It cannot turn the cost into a known venue property. Displayed
depth may disappear, hidden liquidity may appear, impact may be transient
or persistent, other traders and venues may respond, an attacker may
already own inventory or profit in correlated positions, and latency may
change which observations enter the window.

The honest output is therefore an assumption-indexed envelope or scenario
set. Each reported range should name, at minimum, the attack objective and
horizon; admissible strategies; depth and replenishment model; fees and
financing; cross-venue and correlated-position assumptions; latency and
sampling semantics; expected defensive response; and the distribution of
liquidity across ordinary and stressed periods. The analysis should show
sensitivity to each material assumption and say when the available data do
not support a useful lower or upper bound. Public contract terms can make
the payment exposure and observation rule known; they cannot by themselves
make market impact or attacker economics known.

Three consequences follow. First, funding-interval and window design are
material risk parameters, but not the only ones. Source and venue
composition, observation independence, maximum open interest and payment
exposure, margin and liquidation rules, position accountability, price and
funding caps, pauses, fallback authority, and surveillance can all alter
the attack payoff or the system's response. More samples need not be more
independent, and a longer window can change both attack cost and the
amount of legitimate price information suppressed or delayed. Overnight
and weekend conditions (Question 51) require separate stressed parameter
ranges; I have no energy data from which to estimate them.

Second, surveillance can consume the scenario set without treating it as
a fact. A stated reference specification identifies venues, observations,
and times relevant to a funding transfer, while the envelope can help rank
hypotheses for investigation. It does not reduce cross-market surveillance
to those minutes: inventory can be accumulated earlier, profits can sit in
other instruments, and effects can propagate after the reference window.

Third, for Question 66, "demonstrable continuous
manipulation-resistance"#note_ref(1) is not demonstrated merely by putting
model inputs in contract terms. A useful submission would disclose the
model, data provenance, assumptions, parameter ranges, sensitivity and
stress results, validation limits, and residual risks, alongside the
separate operational and market evidence relevant to Core Principles 3
and 4.#note_ref(4) I have produced no manipulation-cost measurement, no
study of any real venue, and no number or envelope for an energy market.

= Failure rules transfer risk; they do not erase it

A deterministic failure output is auditable, but it is not economically
neutral. Refusing a funding print can leave a transfer unresolved and
permit basis risk to accumulate. Carrying forward the last value can make
the reference stale and create a jump when updates resume. Switching to a
fallback source can create a discontinuity between hedges and the
contract. Catch-up funding can concentrate transfers, margin calls, and
liquidations; a pause can strand a hedge when liquidity is already scarce.
Because data outages, liquidity withdrawal, and operational stress may be
correlated, a rule intended as a safeguard can amplify stress.

The terms should therefore state not only the failure label, but also the
economic disposition of the missed transfer, accrual and catch-up rules,
caps, margin and liquidation interaction, resumption criteria, correction
authority, and the operator authorized to act. Those policies need
scenario and stress analysis. My prototype tests deterministic refusal; it
does not model these market-wide consequences, and I have no energy-market
evidence about them.

= Replay is one evidentiary layer (Questions 52, 53)

Question 53 asks what a DCM would be required to demonstrate, "and if so,
on what evidentiary basis?"#note_ref(1) The demonstration should be
specified to include recomputation. If the reference methodology is stated precisely
enough, an authorized party with the same admitted input bytes, rule
version, and time semantics should reproduce the same funding-interval
value or failure output. That replay is evidence of deterministic
recomputation and of whether the implementation matches the specified
calculation.

Replay proves no more. Two implementations can reproduce a poisoned or
unrepresentative input exactly. Replay does not authenticate the source,
show that excluded observations were not censored, prove public or timely
availability, establish that a price reflects the underlying cash market,
or show that either the source or the resulting contract is not readily
susceptible to manipulation. Nor does a deterministic specification alone
perform the broader surveillance, compliance, and enforcement work of
Core Principle 4.#note_ref(4) Disagreement can be localized to input bytes,
versions, time semantics, or implementation behavior, but agreement is
only agreement conditional on those inputs and rules.

For Question 53, the evidentiary record should separate at least: (i)
executable specification, versioned test vectors, and interval replays;
(ii) source provenance, authentication, completeness, latency, correction,
and availability evidence; (iii) empirical evidence that the reference
represents the relevant cash market; (iv) manipulation analysis with the
assumption-indexed sensitivity just described; and (v) failure, recovery,
governance, surveillance, and stress evidence. I supply only a small piece
of the first category. In a different setting, I built a batch verifier to
accept a submitted clearing only if recomputation from a frozen book
reproduces it exactly. That is offline research code about cleared batches,
not energy references; it is evidence that a deterministic acceptance test
can be implemented, not evidence of source reliability, manipulation
resistance, or regulatory compliance.

= Limits

I have no energy market data, no view of any energy cash or futures
market's liquidity, storage, or delivery mechanics, and no position on
whether any contract discussed in the request should be listed. The
artifacts behind this comment are offline research prototypes: tested,
not formally verified; not deployed; holding no funds. I
have produced no measurement of any real venue, no market data study, and
no analysis of any real venue. I have no professional energy-market
expertise and have not interviewed energy producers, merchants, consumers,
price-reporting agencies, exchanges, clearing organizations, or commercial
hedgers for this draft. The positions are my analysis as a
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
  [Appendix C asks for supporting information that a cash-settlement index reflects the underlying cash market, is not readily subject to manipulation or distortion, and uses a reliable, acceptable, publicly available, and timely price series], [17 CFR part 38, Appendix C(a)(2); eCFR current through August 14, 2026 and retrieved August 18, 2026; source note 3],
  [Core Principle 3 addresses contracts not readily susceptible to manipulation; Core Principle 4 addresses capacity and responsibility to prevent manipulation, price distortion, and delivery or cash-settlement disruption through surveillance, compliance, and enforcement], [Commodity Exchange Act section 5(d)(3)-(4), 7 U.S.C. section 7(d)(3)-(4); official U.S. House text retrieved August 18, 2026; source note 4],
  [An observation accumulator with a frozen grid, sealing windows, a monotone accepted-observation path, and refusal of unsupportable questions has been implemented offline with passing deterministic tests], [VERIFIED as to the submitter's inspected artifact only: pure-Rust research prototype; tested, not formally verified; not deployed; source authentication and correction governance not implemented],
  [Replay from identical admitted inputs and rules can test deterministic recomputation, but does not establish source reliability or manipulation resistance], [INFERRED scope distinction; the batch verifier artifact supports only the deterministic-recomputation component],
  [Manipulation-cost results depend on assumptions and should be reported as a disclosed envelope or scenario set rather than as a venue fact], [PROPOSED analytical discipline; no manipulation-cost measurement or real-venue study supports a numerical claim in this comment],
  [Failure rules can shift or concentrate economic risk during correlated operational and liquidity stress], [INFERRED failure-mode analysis; not empirically tested by the prototype and not evaluated with energy-market data],
  [No energy expertise, energy market data, measurement of any real venue, or market data study], [VERIFIED as a statement about the submitter and inspected research artifacts, not about any third party],
)
