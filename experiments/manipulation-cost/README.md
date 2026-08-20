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

It computes that cost twice: once on a single pool at a single depth
(`vectors/v1.txt`), and once on the same window designs read from an off-hours
session carrying a fraction of normal depth (`vectors/v2-offhours.txt`), which is
the computation the perpetuals comment says would show whether thin sessions are
defensible.

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

## What the first table holds

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

## What the off-hours table asks

`vectors/v2-offhours.txt` is 8640 rows over the same designs, evaluated when the
session the manipulation happens in is thin. The perpetuals comment at
[../../docs/regulatory/typst/perpetuals-draft-3/body.typ](../../docs/regulatory/typst/perpetuals-draft-3/body.typ)
says of the stayed 24/7 crude contract that "the computation of Position 2, run
on the off-hours windows specifically, is the evidence that would show whether
thin sessions are defensible." This is that run.

The session layer, in `src/session.rs`, adds no arithmetic --- a test asserts
every row is the model above evaluated at a different depth and a different hold
count. A window opens inside a thin session of `session_s` seconds; samples are
taken at bucket ends, so `thin_buckets = min(buckets, floor(session_s /
bucket_s))` of the window's samples fall inside it. The thin session's pool holds
`depth_ratio` of **both** reserves, so thinning does not move the print of a
balanced pool, only what it costs to move it. The adversary is confined to that
session: it is exposed in `hold_eff = min(hold, thin_buckets)` buckets and must
therefore move each of them by `buckets * boundary_bps / hold_eff`. Depth ratios
are 1, 1/2, 1/4, 1/10, 1/50, each dividing every depth in the grid exactly;
sessions are 150 s and 600 s, the two lengths that generate every distinct
`(buckets, thin_buckets)` pair the six bucket grids can produce.

**The finding: what an off-hours attack pays, relative to the same attack in
daylight, is the depth ratio times the amplification the session forces on it
(`hold / hold_eff`) --- so a window lying wholly inside the thin session loses
cost in proportion to depth, retaining 10000, 5003, 2501, 1004, 203 basis points
of the daylight cost across the ratio grid (`window_s=300|session_s=600` at
`depth=1000000000|fee_bps=30|buckets=12|hold=12|boundary_bps=10`), while a window
sampling that session in only 2 of its 12 buckets claws back sixfold and keeps
its whole daylight cost down to a quarter depth --- 59776, 29889, 14946, 5980,
1200 (`window_s=3600|session_s=600`, same parameters) --- and no design in the
grid keeps it at a tenth of normal depth (best row 8000) or a fiftieth (4000).**

The rest of what the rows say:

- **A retention ratio above 10000 is not a design that is dearer to attack.** It
  means only that the thin session offers no discount; an adversary indifferent
  to the hour attacks the normal session at `cost_v1`. The cheapest attack on a
  design is `min(cost, cost_v1)`, and 2028 of the 7200 computed rows are rows
  where the off-hours attack is the dearer one.
- **The clawback is sublinear in the boundary distance, because cost is concave
  in displacement.** The same sixfold amplification returns 59776 at a 10 bp
  boundary, 58903 at 50, 55923 at 200, and 45035 at 1000
  (`depth=1000000000|fee_bps=30|window_s=900|session_s=150|buckets=6|hold=6|depth_ratio=1/1`).
  A window design that looks protective at a near boundary protects less at a
  far one.
- **Window length, inert in v1, is live here --- and only through the bucket
  count.** Seconds enter solely by deciding how many samples fall inside the
  session, and a test asserts rows agreeing on `(buckets, thin_buckets)` agree on
  every computed column whatever their seconds say.
- **A bucket grid coarser than the session takes no sample inside it.** The 3600 s
  windows against the 150 s session sample nothing off-hours, so no attack
  mounted from inside that session touches the print at all; those 1440 rows
  carry a dash in every computed column. Whether a thin session is manipulable at
  all is decided by the sampling schedule before any of the rest applies.
- **Cost never rises as the session thins**, in every one of the 8640 rows. The
  only four steps that do not fall are rows whose cost is already one integer
  unit at a thin reserve of 200000, where the flooring is the whole quantity.
- Capital thins with cost, and stays far above it: 3000014 against 17921 in the
  sixfold-amplification row above.

`cost` is the cost of *that* schedule, an attack from inside the thin session. It
is not a minimum over all attacks. Carrying inventory across the return of
liquidity --- so that a distortion bought at thin depth stands into the normal
session --- is named in the header as omitted and would lower these figures; so
would splitting an attack across both sessions.

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

To regenerate the off-hours table, whose bytes a test compares against the
checked-in file:

```sh
cargo run --quiet --manifest-path experiments/manipulation-cost/Cargo.toml \
  --bin manipulation-cost-vectors-offhours \
  > experiments/manipulation-cost/vectors/v2-offhours.txt
```

Provenance, digests, and the validation toolchain are in
[PROVENANCE.md](PROVENANCE.md). The experiment conventions this follows are in
[../README.md](../README.md).
