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
accumulator over authenticated observations with a stated refusal behavior, and
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
