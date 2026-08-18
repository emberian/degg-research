# Clearing Relation V0

Status: PROPOSED research relation. Not a production mechanism.

## Purpose

Define one small, exact uniform-price batch relation that can be exhaustively
tested and implemented across clear, proof, MPC, and FHE backends.

This relation intentionally avoids:

- continuous price-time priority;
- arbitrary user programs;
- hidden dynamic market creation;
- cross-margin;
- leverage;
- liquidation;
- unconstrained price precision;
- probabilistic rounding;
- operator discretion.

## Domains

Frozen batch parameters:

- batch identifier B;
- market identifier M;
- ordered price ticks P[0..K-1];
- maximum accepted slots N;
- quantity bound Qmax;
- accepted-input commitment R;
- cutoff C;
- public leakage declaration L;
- deterministic allocation policy A;
- fee policy F.

Each padded slot contains either:

- empty; or
- owner commitment, side, limit tick, quantity, reserved-balance commitment,
  nullifier, and admission proof.

The private encoding must not reveal occupancy when the declared mode says only
N is public.

## Admission

For every nonempty slot:

- side is Buy or Sell;
- 0 < quantity <= Qmax;
- limit tick is in range;
- nullifier is unique in B;
- order commitment binds every field and B/M;
- reservation proves sufficient authorized balance;
- eligibility policy passes;
- order arrived before C;
- inclusion root R contains the exact slot.

Empty slots have a canonical zero contribution and cannot carry value.

This list is a set of requirements, not a sequence: it fixes which slots are
admissible, not which class a slot violating several rules is refused with. A
specialization that publishes the refusal class as an output makes that choice
a public observable and owes a frozen check priority, or an explicit
declaration that the reported class is underdetermined. The specialization
`dark-fba/n4-k4-q15/v0` freezes one, as a numbered order over these rules plus
its own boundary statements, in section 4.1 of
[`DARK_FBA_RELATION.md`](../docs/research/DARK_FBA_RELATION.md).

## Aggregate curves

For tick k:

    Demand[k] = sum quantity of accepted Buy orders with limit >= k
    Supply[k] = sum quantity of accepted Sell orders with limit <= k
    Volume[k] = min(Demand[k], Supply[k])

All sums use a width proven sufficient for N * Qmax. Overflow is refusal, never
modular wrap.

## Clearing tick

Let Vmax be max Volume[k].

If Vmax = 0, the batch clears with no fills and the canonical no-trade tick
sentinel.

Otherwise choose the smallest k with Volume[k] = Vmax. This ties-low rule is a
research choice, not yet an economic recommendation. The experiment program
must compare alternatives and manipulation surfaces.

The public base output is:

- B, M, R;
- selected tick or no-trade;
- matched volume Vmax;
- result commitment;
- relation and parameter identifiers;
- proof or explicit verification status.

## Allocation

V0 allocates each side separately across all orders eligible at the clearing
tick.

For a side with total eligible quantity T and target Vmax:

    base_i = floor(quantity_i * Vmax / T)
    rem_i  = (quantity_i * Vmax) mod T

The remaining Vmax - sum base_i atoms go to orders in descending rem_i order,
with ties broken by a frozen canonical rank derived from the accepted commitment
sequence.

This exactly allocates Vmax on each side, but the rank policy may create gaming
or builder power. It is explicitly a falsifiable component. Candidate
alternatives include:

- precommitted unbiased rotation;
- pro-rata at only the marginal price with price-improvement priority;
- uniform lottery with a bias-resistant beacon;
- deterministic residual credit carried to the next batch;
- allocation certificates proposed offchain and verified.

No dust silently goes to a treasury.

## Fees

V0 carries a fee-policy identifier but the base semantic vector uses zero fees.
Fee research is orthogonal to correctness of price and allocation.

A later fee relation must state:

- payer;
- exact base;
- rate and cap;
- rounding;
- recipient split;
- maker/taker definition;
- self-match behavior;
- conservation;
- whether fees alter clearing or only settlement.

## Settlement transition

For every owner, compute private local deltas:

- base asset;
- quote asset;
- fees;
- remaining reservation;
- order terminal status.

Global properties:

- sum bought quantity = sum sold quantity = Vmax;
- quote debits equal quote credits plus explicit fees;
- no fill violates its limit;
- no owner spends beyond reservation;
- every accepted order is filled, unfilled, or refused exactly once;
- every nullifier becomes terminal exactly once;
- result is bound to B, M, R, relation version, and parameters.

## Visibility

Clear mode publishes slots and deltas.

Shielded mode may reveal slots and deltas to the named executor/committee.

Dark target publishes only the frozen public output and delivers each local
delta to its owner. Whether clearing price and volume should be public is a
mechanism and regulatory question, not an automatic cryptographic choice.

## Verification targets

- clear oracle exactness;
- monotone aggregate curves;
- maximum-volume/ties-low selection;
- exact pro-rata total;
- limit satisfaction;
- conservation;
- nullifier uniqueness;
- frozen refusal-class priority wherever the class is a public output;
- commitment binding;
- inclusion completeness;
- leakage noninterference;
- settlement refinement.

## Known omissions

- cancellation races;
- commit/reveal;
- order modification;
- maker age;
- multi-asset or multi-outcome netting;
- external AMM interaction;
- path-dependent claims;
- chain account layouts;
- regulatory records;
- malicious-key behavior;
- liveness and abort compensation.

Those omissions are explicit so V0 cannot be mistaken for a launch design.

