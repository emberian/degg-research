# Definitions Question 5 — narrow-based index status as a term of the contract

Internal research memo. See [README.md](README.md) for status and citation rules.

## The question, verbatim

> 5. Regarding the SBS NBSI Prong, is there a need for additional clarity
> regarding when a swap is based on "an index that is [an NBSI] including any
> interest therein or on the value thereof"? Should the Commissions further
> address the circumstances when a swap does or does not satisfy the SBS NBSI
> Prong? For example, what additional clarity, consistent with CEA section
> 1a(35) and Exchange Act section 3(a)(55), might the Commissions provide with
> respect to the characterization of contracts referring to potential changes
> to the composition of an NBSI, as opposed to changes in the price or value of
> an NBSI? The Commissions have adopted rules addressing tolerance periods and
> grace periods for products referencing securities indexes traded on
> designated contract markets, swap execution facilities ("SEFs"), foreign
> boards of trade, security-based SEFs, or national securities exchanges, where
> the securities index temporarily moves from broad-based to narrow-based or
> from narrow-based to broad-based. Should the Commissions revise or clarify
> those rules or provide additional clarity, transition rules, or safe harbors
> for products that are based on a securities index that transitions between
> narrow-based and broad-based, or vice versa?

— 91 Fed. Reg. 37873, 37876 (June 24, 2026).

## Positions

1. **A contract referring to an index's composition is a different instrument
   from a contract referring to its price or level, and the difference is
   readable off the settlement function.** A composition contract settles on a
   membership fact — is this security in the index on this date. A price
   contract settles on a number. The Commissions can state the line in exactly
   those terms, and it will survive products that do not exist yet.
2. **Narrow-based status is a time-varying observable, and a contract whose
   classification depends on it should be required to carry the observation
   rule in its own terms.** The rule has four parts, all of which a drafter can
   fix at inception: the source that establishes status, the times at which
   status is evaluated, the treatment of a missing or ambiguous measurement,
   and the classification consequence of each status path the contract can
   travel.
3. **Tolerance and grace periods should be expressed as a status function over
   a stated measurement grid, not as a narrative standard.** The test of a
   status rule is that two people holding the same rule and the same data
   reach the same answer without negotiating. A rule that leaves the
   measurement time, the source, or the edge cases unstated does not pass that
   test, and the disagreement it produces surfaces at the worst moment — after
   a transition, when the classification determines which agency's rules
   applied to conduct that has already happened.
4. **The safe harbor the question asks about should be available on the basis
   of what the terms say, not on the basis of what the index did.** A contract
   whose terms name a status source, an evaluation grid, and a consequence for
   each path has a determinate classification for its entire life. A contract
   that names none of these has a classification that depends on facts nobody
   committed in advance to observe. Rewarding the first is a rule a drafter can
   comply with before listing.

## Argument

The prong the question is about depends on a defined status. "Narrow-based
security index" is a defined term, 7 U.S.C. 1a(35), and the security-based swap
definition turns on whether the swap is based on "an index that is a
narrow-based security index . . . including any interest therein or on the
value thereof." 15 U.S.C. 78c(a)(68)(A). What makes this prong different from
the single-security prong is that the status can change while the contract is
alive, through no act of either party — which is precisely why the Commissions
already adopted tolerance and grace periods, as the notice recites.

Once a classification depends on a time-varying fact, the classification is
only as determinate as the rule for observing that fact. This is a familiar
drafting problem in a different vocabulary: a settlement term that names a
source, a window, a sampling grid, and a rule for every failure mode produces
the same answer for everyone who reads it, and a settlement term that says
"the market price" does not. The proposal here is to apply that discipline one
level up, to the status input that decides which agency's framework governs,
rather than only to the price input that decides the payout.

The distinction in position 1 has textual support in the question itself: the
Commissions separate "potential changes to the composition of an NBSI" from
"changes in the price or value of an NBSI," which is the same separation as
between a membership fact and a number. Nothing in 1a(35) makes composition and
price interchangeable, and a contract that pays on a reconstitution event is
not measuring value at all.

*Basis for statements about the submitter's artifacts in this memo:* the
"frozen observation rule with a stated failure behavior" discipline described
above is implemented, for price observations only, in an offline research
prototype — an observation accumulator that combines supplied observations — source
authentication is an assumed input contract the prototype does not implement —
and refuses to answer questions its retained information cannot support. Its
deterministic tests pass; it is tested, not formally verified, and it is not
deployed. Nothing in it observes securities index status, and no artifact in
this program does.

## Evidence this program could build

A **status-path corpus**. Generate a bounded set of synthetic index
composition histories — a security added mid-life, a security removed and
re-added, a measurement missing on an evaluation date, a change effective
between two evaluation dates. Encode two or three candidate status rules that a
reasonable drafter might write, differing only in measurement grid and edge-case
handling. For each history, compute the classification each rule produces
across the contract's life, and report: how many distinct classification
outcomes the corpus admits, and which specific histories cause two plausible
rules to diverge. Bounded: synthetic composition data, deterministic, offline,
no market data and no real index. The deliverable is a short list of concrete
divergence cases — the cases a transition rule or safe harbor has to decide.

## Needs verification

The notice states that the Commissions have adopted tolerance-period and
grace-period rules, citing 17 C.F.R. 240.3a55-2, 240.3a55-3, and 17 C.F.R.
part 41, and cites Exchange Act section 3(a)(55) and the rule of construction at
15 U.S.C. 78c(a)(68)(E). None of those provisions was independently retrieved
for the program's citation ledger. This memo relies on the notice's own
characterization only. Any filing use requires fetching and reading the rules
themselves — in particular, the existing tolerance rules may already resolve
part of position 3.

## Filing-worthiness

**Strong on the composition-versus-price distinction and the
terms-carry-the-status-rule proposal; needs evidence for any claim about how
much existing tolerance rules already cover.** The distinction can be argued
from verified text today; the safe-harbor proposal should not be filed before
the existing rules are read.
