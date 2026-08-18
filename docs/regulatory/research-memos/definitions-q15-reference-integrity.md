# Definitions Question 15 — cross-market surveillance when the reference is a trading venue

Internal research memo. See [README.md](README.md) for status and citation rules.

## The question, verbatim

> 15. Under an alternative compliance regime, how could the Commissions best
> deter market manipulation and trading on material non-public information?
> What steps should the agencies take to ensure robust surveillance and
> oversight of cross-market activities?

— 91 Fed. Reg. 37873, 37877 (June 24, 2026).

Related, from the same notice:

> 14. What considerations should guide surveillance, examination, and
> enforcement under an alternative compliance approach? How could enhanced
> sharing of information and data help fulfil the Commissions' regulatory
> mandates under an alternative compliance approach?

## Positions

1. **When a contract settles by reading a price off a specific trading venue,
   the cost of moving that price for the length of the observation window is a
   computable number, and it belongs in the surveillance picture as a number.**
   For a contract referencing an onchain liquidity pool, the inputs to that
   computation — pool depth, fee, the sampling grid, the window length — are
   public and current. The cost of moving the settlement statistic far enough
   to change which outcome pays can be calculated by anyone, in advance, and
   compared to the amount at stake in the contract.
2. **Removing settlement discretion does not remove manipulation risk; it
   relocates it.** A frozen observation rule means no person chooses the
   reported value, which eliminates reporter discretion entirely. It leaves
   untouched the incentive to move the thing being observed, and it makes that
   attack easier to plan, because the attacker knows exactly which statistic
   over exactly which window decides the payout.
3. **Contract terms should therefore state the reference specification in a
   form a supervisor can act on:** the venue, the statistic, the sampling grid,
   the window, and the rule for every failure of the source. That is the same
   information a surveillance program needs to know what to watch and when to
   watch it, and stating it converts "watch for manipulation" into "watch this
   venue during these minutes."
4. **Cross-market surveillance across two agencies needs a shared way to say
   which instrument settles on which venue's price over which window.** Without
   a machine-readable reference specification carried in the transaction
   record, correlating activity in a derivative with activity in the thing it
   settles on is a manual exercise repeated per product.
5. **In this setting, material non-public information includes pending changes
   to the reference itself** — a scheduled upgrade to the source venue, a
   change to its fee or oracle parameters, a planned reconstitution — because
   those change the settlement value or the cost of moving it, and the people
   who know about them are frequently not the issuer or the counterparties.

## Argument

Positions 1 and 2 follow from what a frozen settlement rule does and does not
accomplish. The program's own comment on the definitions notice states the
limit plainly: a frozen observation program "removes reporting discretion; it
does not prevent trading from influencing the underlying price." The useful
consequence is that the residual risk is quantifiable in a way that
discretionary-reporter risk is not. Where the reference is a continuous
onchain venue, an observer can price the attack rather than describe it.

Position 4 is the reason this question belongs next to the data notice rather
than only in an enforcement discussion. The transaction record is the only
place the link between an instrument and its reference venue lives at scale.
The Commissions already accept that the regulatory record and the public
commercial record need not be identical, 17 C.F.R. 38.7, so carrying a precise
reference specification in the regulatory record does not require publishing
it in the same form.

The counterargument to position 1 is that a computed manipulation cost is a
model output, sensitive to assumptions about depth, latency, and the attacker's
inventory, and that a supervisor should not rely on a number of that kind. That
is right and it is why the recommendation is to require the *inputs* in the
terms rather than to require a *conclusion*. The number is a screening tool for
allocating attention; the inputs are facts.

*Basis for statements about the submitter's artifacts in this memo:* the
program has an offline research prototype implementing an observation
accumulator over supplied observations (source authentication is an assumed
input contract, not implemented) with a stated refusal behavior, and
its architecture documents describe frozen source specifications with sampling
grids, coverage and dispersion bounds, and deterministic failure rules. The
prototype's deterministic tests pass; it is tested, not formally verified, and
it is not deployed. The program has produced no manipulation-cost measurement,
no market data study, and no analysis of any real venue.

## Evidence this program could build

A **manipulation-cost table**. For a stated constant-product pool model with a
stated depth and fee, and a stated time-weighted price rule with a stated
window and sampling grid, compute the capital required to hold the observed
statistic across a band boundary for the window, and report that cost against
the maximum payout the contract puts at stake. Vary window length, sampling
frequency, and pool depth across a small grid and print the ratio. Bounded:
closed-form or short simulation over a synthetic pool, deterministic, offline,
no real venue data and no claim about any deployed market. The deliverable is a
table showing how the ratio moves with window length — which is the concrete
form of the drafting advice "choose the window so that moving the reference
costs more than winning the bet."

A second, cheaper artifact: a **reference-specification field list** — the
minimum set of fields that make an instrument's reference machine-correlatable
with activity on the referenced venue, expressed in the same style as the
chain-provenance field table in the program's data-reporting comment.

## Note for the coordinator

The definitions filing currently states that the submitter "take[s] no position
on Questions 12 through 15 concerning alternative compliance." Question 15 falls
inside that sentence. Folding any of this memo into that filing would require
narrowing or removing the no-position statement. The material may sit more
naturally in a future filing or in the IAC track, where cross-market
surveillance of onchain references is squarely in scope.

## Filing-worthiness

**Strong as a position; needs evidence before the quantitative claim is made.**
Positions 2 through 5 are argued from the program's existing work and verified
sources today. Position 1's numbers require the experiment, and no number should
appear in a filing before it exists.

## Addendum, 2026-08-18 — the manipulation-cost experiment now exists

The experiment described above under "Evidence this program could build" has
been built and run. It is at
[../../../experiments/manipulation-cost/README.md](../../../experiments/manipulation-cost/README.md),
with its provenance and digests at
[../../../experiments/manipulation-cost/PROVENANCE.md](../../../experiments/manipulation-cost/PROVENANCE.md).
This addendum records what it computes, on what assumptions, and what it does
and does not license. It changes nothing in the memo above and inserts no
number into any filing.

### What the experiment is

A synthetic constant-product pool, described entirely by two integer reserves
and a fee in basis points, and a settlement statistic that is the mean of one
price sample per bucket over a window cut into uniform buckets. The observed
quantity is the pool's reserve ratio, which is the quantity such a venue
exposes to a reader. All arithmetic is exact integer arithmetic; nothing is
approximated, sampled, or simulated stochastically. The whole computation is
deterministic and the resulting table is reproducible byte-for-byte.

Two closed forms carry it, both derived from the pool invariant and documented
in the source rather than cited to any implementation. The first gives the
capital that moves the observed price to a target. The second gives the cost of
one excursion and its reversal, `c(d) = (1 - g^2) * y0 * d / (y0 + g^2 * d)`,
where `y0` is the quote reserve, `d` the capital deployed, and `g` the fee
retention factor.

The adversary model is the one the drafting question implies: it starts flat,
trades only against the referenced pool, may be exposed across the sample
instant of at most `k` of the window's `n` buckets, and must end flat.

### The recovery assumption, stated because everything turns on it

The adversary is assumed to sell its entire position straight back into the same
pool and to keep everything the pool returns. It therefore recovers its capital
less two fee legs and the price impact of its own reversal. There is no
competing order flow, no arbitrageur taking a share of the distorted print, no
inventory or borrowing limit, no transaction or priority fee, no latency, no
second venue, no price process, no detection risk, and no cost of *holding* a
price against arriving flow. Every one of those omissions raises a real
attacker's cost. The computed figure is accordingly a **lower bound under
stated assumptions**, and the choice of a maximally generous recovery is what
makes it one.

### One row, in words

Take a pool holding one billion units on each side and charging thirty basis
points. Let the contract settle on the mean of twelve prints taken five minutes
apart across a one-hour observation window, and let the decision boundary sit
ten basis points above the print the pool would show if nobody touched it.

An adversary willing to be exposed in one of the twelve buckets must move that
single print one hundred and twenty basis points, because a mean divides an
adversary's work by the number of samples and multiplies the required
displacement by the same factor. Doing that takes a purchase of 5,991,095 quote
units — just under six tenths of one percent of the pool's quote reserve — and
the round trip of buying, being observed, and selling the whole position back
burns 35,681 quote units, about three and a half hundredths of one percent of
that reserve. Those two figures differ by a factor of one hundred and
sixty-eight. They answer different questions: the first is what an adversary
must have, the second is what the attempt costs it. A surveillance threshold
built on one of them is not built on the other, and the contract's amount at
stake has to be compared against the right one.

The estimate a reader is likeliest to reach for — what does it cost to push
this pool's spot price ten basis points — is 2,998 quote units, and it
understates the schedule by a factor of eleven point nine. The missing factor
is the sampling grid.

All figures are in the model's abstract integer units. They are not
denominated in any currency and they are not calibrated to anything.

### What moves the number, and what does not

Across the full grid — four depths, three fees, six window-and-bucket
combinations, three exposure counts, five boundary distances, 1080 rows — the
cost rises strictly with pool depth, strictly with the fee, and strictly with
boundary distance, and it is zero exactly when the boundary is already crossed.
Three further results are worth the memo's attention because two of them run
against the intuition the argument above reaches for.

**Sampling density is a real dial; window length, in this model, is not.** Three
window lengths produce identical numbers, because seconds enter an attacker's
cost only through the order flow that arrives during them, and this model has
no other flow. What does move the number is the number of samples in the window:
at a ten basis point boundary, twelve buckets with one distorted costs 1.99
times what six buckets with one distorted costs, and the multiple falls to 1.56
at a thousand basis points because the cost is concave in displacement. The drafting advice that survives this experiment is therefore
about the *grid*, not the *clock*. The memo's phrasing above — "choose the
window so that moving the reference costs more than winning the bet" — is not
supported by this experiment as stated, and should be re-stated in terms of
sampling density before it is relied on.

**Requiring a longer hold makes the attack cheaper here, not dearer.** Holding
costs nothing once the position is established in a model with no competing
flow, while every additional distorted bucket divides the amplification factor.
The real content of "the attacker must hold it for most of the window" is that
a longer hold gives arriving flow more time to trade against the position — and
that is precisely the effect this experiment does not price. The number should
not be offered as though it captured it.

**The burn is bounded; the capital is not.** The round-trip cost saturates: no
excursion against a given pool, at any target price whatsoever, can burn more
than `y0 * (F^2 - (F - f)^2) / (F - f)^2`, which is 0.6027% of the quote reserve
at thirty basis points, 0.10% at five, and 2.03% at one hundred. Against a thirty basis point pool, moving the price ten
thousandfold rather than fourfold takes ninety-nine times the capital and less
than twice the burn. Under this recovery model the binding constraint on an
adversary is not what the attack costs but what it must be able to hold, which
is a different fact about a market and a different thing for a supervisor to
watch for.

### One hypothesis refuted

The experiment was designed to test whether pricing each distorted bucket
independently *understates* a real schedule once reversal fees compound. It does
not. It overstates, in every row, for a structural reason: a completed round
trip returns the pool to its original base reserve with the entire fee residue
sitting on the quote side, so the price after an up-manipulation sits above
where it started, and each repeat excursion begins closer to its target than the
last. The fee residue is a ratchet in the adversary's own direction.

Two estimators do understate, and both are recorded as falsifiers in the
experiment's tests: pricing the boundary as a spot move understates by roughly
the amplification factor, and pricing one continuously held excursion
understates a schedule whose exposed buckets are separated and which therefore
pays for `k` reversals rather than one.

### The ceiling on all of it

Synthetic pools. No market data of any kind. No calibration to, and no claim
about, any deployed venue, pool, oracle, index, or contract. A lower bound under
the stated assumptions, not a prediction, and not a measurement of anything in
the world. The number is exact arithmetic over a chosen model, and its value as
evidence is entirely a function of whether a reader accepts the model.

### The gate is unchanged

This experiment makes the number exist. It does not place the number anywhere.
Whether any figure from it may appear in a filing remains the author's decision,
and the standard this memo already sets — the recommendation is to require the
*inputs* in the contract's terms rather than to require a *conclusion*, because
"the number is a screening tool for allocating attention; the inputs are facts"
— is if anything strengthened by what the table shows about how sensitive the
conclusion is to the model.

Two consequences the coordinator should weigh before any conversion:

1. The memo's basis paragraph above states that "the program has produced no
   manipulation-cost measurement, no market data study, and no analysis of any
   real venue," and the candidate 24/7 and perpetuals comment carries the same
   sentence verbatim in its Limits section and its basis appendix. The second
   and third clauses remain exactly true. The first now needs a decision: this
   experiment is an exact computation over a synthetic model, not a measurement
   of anything, so the sentence is defensible as written — but a filing that
   keeps it while the repository holds this table should say so deliberately
   rather than by inattention. The narrow wording that stays true in every
   reading is "no measurement of any real venue and no market data study."
2. Any figure lifted from the table must travel with its recovery assumption,
   its lower-bound status, and its synthetic provenance in the same sentence.
   The three results above show why: two of the drafting intuitions this memo
   states in prose come out of the model with the opposite sign or with no
   support at all.
