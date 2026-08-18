# Manipulation cost of a venue-read TWAP

Status: **PROPOSED** as evidence; **VERIFIED** only as an offline deterministic
Rust computation, at the bounds the tests state, after the commands below pass.
**Synthetic pools only. No market data. No claim about any deployed venue,
oracle, pool, or contract.**

This crate computes what it costs an adversary to move a time-weighted price
print across a decision boundary, when the print is read off a constant-product
pool over a frozen bucket grid. It exists because
[../../docs/regulatory/research-memos/definitions-q15-reference-integrity.md](../../docs/regulatory/research-memos/definitions-q15-reference-integrity.md)
asserts that such a cost "is a computable number" and then gates the assertion:
"no number should appear in a filing before it exists." This makes the number
exist. It puts the number nowhere else.

## The model in one screen

A balanced constant-product pool holds `x` base and `y` quote in integer units
and charges `f` basis points on the input leg. The **print** is the reserve
ratio `y / x`, carried as an exact reduced rational. A window is cut into `n`
uniform buckets, one price is sampled per bucket, and the statistic is the mean
of the `n` samples. A boundary sits `D` basis points above the undistorted
print.

The adversary starts flat, trades only against this one pool, may hold a
position across the sample instant of at most `k` of the `n` buckets, and must
end flat.

Two closed forms carry the whole thing, both derived in `src/pool.rs`:

- **Price move.** With `g = (10000 - f)/10000`, reaching a price `p1` from
  `(x0, y0)` costs a quote input `d` solving
  `g*d^2 + (1 + g)*y0*d + y0^2 - p1*x0*y0 = 0`.
  The discriminant is not generally a perfect square, so the crate computes `d`
  by exact integer bisection and keeps the closed form as an independent
  cross-check (tested to agree within the integer-flooring slack).
- **Round-trip cost.** Buying with `d` and selling the whole position straight
  back costs `c(d) = (1 - g^2)*y0*d / (y0 + g^2*d)`, which is **bounded above
  for every `d`** by `y0 * (F^2 - (F - f)^2) / (F - f)^2` --- about 0.60% of the
  quote reserve at 30 bps, 0.10% at 5 bps, 2.03% at 100 bps.

Moving the mean of `n` samples by `D` while distorting only `k` of them
requires moving each distorted sample by `n*D/k`. That amplification factor is
where the sampling grid enters, and it is the only place it does.

## The recovery model, stated as the assumption it is

The adversary sells its entire position back into the same pool, with no other
flow in between, and keeps everything the pool returns. It recovers its capital
less two fee legs and the price impact of its own reversal. There is no
competing order flow, no external arbitrageur taking a share of the distorted
print, no inventory or borrowing limit, no gas or priority fee, no latency, no
second venue, no price process, no detection, and no cost of *holding* a price
against arriving flow. Every one of those omissions raises a real attacker's
cost.

The reported numbers are therefore a **lower bound under stated assumptions**,
not a prediction, and not a statement about any market. Integer flooring of the
swap output moves each reported figure at most a couple of units above the
continuous-model value, which the tests pin; at the smallest depth in the grid
that slack is a visible fraction of a small number and at the largest it is
nothing.

## What the table holds

`vectors/v1.txt` is 1080 rows over depth (4) x fee (3) x window length and
bucket count (6) x hold count (3) x boundary distance (5), each row carrying
capital, the cost of a continuously held schedule, the cost of a schedule forced
to re-establish, two naive estimators, and the saturation ceiling. It is
byte-stable and compared byte-for-byte by a unit test.

Results the tests assert, including the ones that came out backwards:

- Cost and capital rise strictly with pool depth, with the fee, and with
  boundary distance.
- Cost rises strictly with bucket count at a fixed hold count --- a denser
  sampling grid is a real dial.
- **Window length does not enter the cost at all.** Three window lengths,
  identical numbers, because seconds only matter through flow this model does
  not have. A memo may not claim this experiment prices the window dial.
- **Requiring a longer hold makes the attack cheaper, not dearer.** Holding
  costs nothing once established here, while every extra distorted bucket
  divides the amplification. What a longer hold really buys is time for
  arriving flow to fight the position, which is exactly what is unmodelled.
- Cost is zero exactly when the boundary is already crossed.
- Capital exceeds cost everywhere, by more than a thousandfold in the deepest
  rows. What an attacker must *have* and what the attack *burns* are different
  numbers by orders of magnitude.
- Exhaustive enumeration over every exposure pattern and every level assignment,
  at small bucket counts, selects the schedule the closed form names: one
  excursion to the required price, held across a contiguous block.

## Falsifiers

`tests/falsifiers.rs` names four estimators and the direction each errs in.

- Pricing the boundary as a **spot** move understates the schedule, by roughly
  the amplification factor `n/k`.
- Pricing **one held excursion** understates a schedule whose exposed buckets
  are separated: that adversary pays `k` reversals, and the reversal fee
  compounds.
- **Refutation, recorded deliberately.** The hypothesis this experiment was
  commissioned with --- that a per-bucket-independent estimate *understates* a
  real schedule once reversal fees compound --- is false here, in every row. A
  completed round trip returns the pool to its original base reserve with the
  whole fee residue on the quote side, so the price after an up-manipulation
  sits *above* where it started and each repeat excursion begins closer to its
  target. The fee residue is a ratchet in the adversary's own direction, and
  charging every bucket at the pristine price bills it too much.
- **Linear extrapolation in boundary distance overstates.** Cost is concave in
  the distance and the burn saturates, so scaling a small measured move up
  overshoots.

## Run it

```sh
cargo fmt --manifest-path experiments/manipulation-cost/Cargo.toml --check
cargo clippy --manifest-path experiments/manipulation-cost/Cargo.toml \
  --all-targets -- -D warnings
cargo test --manifest-path experiments/manipulation-cost/Cargo.toml
cargo run --quiet --manifest-path experiments/manipulation-cost/Cargo.toml \
  --bin manipulation-cost-vectors
```

Provenance, digests, and the validation toolchain are in
[PROVENANCE.md](PROVENANCE.md). The experiment conventions this follows are in
[../README.md](../README.md).
