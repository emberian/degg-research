//! Estimators a reader might reach for, and the direction each one errs in.
//!
//! Every test here names a naive estimate, states which way it is wrong, and
//! fails if the sign flips. Two of them refute a hypothesis this experiment was
//! commissioned with; the refutations are recorded here rather than smoothed
//! over, and the memo addendum carries them.

use degg_manipulation_cost::pool::{Pool, Price, round_trip_to};
use degg_manipulation_cost::table::{Row, grid_rows};
use degg_manipulation_cost::twap::{Schedule, execute, required_bucket_price};

fn rows() -> Vec<Row> {
    grid_rows().expect("grid")
}

#[test]
fn pricing_the_boundary_as_a_spot_move_understates_the_schedule() {
    // The amplification falsifier. Moving the mean of n samples by D while
    // only k of them may be distorted requires moving each distorted sample by
    // n*D/k. An estimator that prices "what does it cost to push the spot
    // across B" answers a different question and always answers low.
    let mut worst_ratio = 0u128;
    for row in rows() {
        if row.boundary_bps == 0 {
            continue;
        }
        if row.hold == row.buckets {
            // No amplification: the two estimates are the same computation.
            assert_eq!(row.naive_spot, row.contiguous, "{row:?}");
            continue;
        }
        assert!(
            row.naive_spot < row.contiguous,
            "spot estimate did not understate: {row:?}"
        );
        worst_ratio = worst_ratio.max(row.contiguous / row.naive_spot.max(1));
    }
    // At twelve buckets with one distorted the understatement is roughly the
    // amplification factor itself.
    assert!(worst_ratio >= 6, "worst understatement ratio {worst_ratio}");
}

#[test]
fn pricing_one_held_excursion_understates_a_schedule_forced_to_re_establish() {
    // The compounding falsifier, in the direction it actually runs. A hold that
    // may be continuous is one excursion and one reversal. A hold the same size
    // that must be interrupted --- because the exposed buckets are separated ---
    // is k excursions and k reversals, and the reversal fee is paid every time.
    let mut any_strict = false;
    for row in rows() {
        let Some(spread) = row.spread else { continue };
        if row.boundary_bps == 0 || row.hold < 2 {
            continue;
        }
        assert!(
            spread > row.contiguous,
            "separated schedule was not dearer: {row:?}"
        );
        assert!(
            spread >= row.contiguous * u128::from(row.hold) * 9 / 10,
            "compounding was weaker than expected: {row:?}"
        );
        any_strict = true;
    }
    assert!(any_strict);
}

#[test]
fn the_per_bucket_independent_estimate_overstates_rather_than_understating() {
    // REFUTATION, recorded deliberately. The hypothesis this experiment was
    // handed is that pricing each distorted bucket independently *understates*
    // a real schedule once reversal fees compound. In this model it does the
    // opposite, and the reason is structural: a completed round trip returns
    // the pool to its original base reserve with the entire fee residue on the
    // quote side, so the pool's price after an up-manipulation is *above* where
    // it started. Every repeat excursion therefore begins closer to the target
    // than the last. Charging each bucket at the pristine price ignores that
    // ratchet and bills the adversary too much.
    let mut strict_cases = 0usize;
    for row in rows() {
        let Some(spread) = row.spread else { continue };
        assert!(
            row.naive_independent >= spread,
            "independent estimate came in under the schedule: {row:?}"
        );
        if row.naive_independent > spread {
            strict_cases += 1;
        }
    }
    assert!(
        strict_cases > 0,
        "the ratchet never showed up in the grid at all"
    );
}

#[test]
fn the_per_bucket_independent_estimate_also_overstates_a_continuous_hold() {
    for row in rows() {
        if row.hold < 2 || row.boundary_bps == 0 {
            continue;
        }
        assert!(
            row.naive_independent > row.contiguous,
            "independent estimate did not overstate the held schedule: {row:?}"
        );
    }
}

#[test]
fn linear_extrapolation_in_boundary_distance_overstates() {
    // A second natural estimator, wrong the other way. Cost is concave in the
    // boundary distance --- the input needed grows like a square root and the
    // round-trip burn saturates --- so scaling a small measured move up to a
    // large one always overshoots.
    let rows = rows();
    let small = rows
        .iter()
        .find(|row| {
            row.depth == 10_000_000_000
                && row.fee_bps == 100
                && row.window_seconds == 3600
                && row.buckets == 12
                && row.hold == 1
                && row.boundary_bps == 10
        })
        .expect("row");
    let large = rows
        .iter()
        .find(|row| {
            row.depth == 10_000_000_000
                && row.fee_bps == 100
                && row.window_seconds == 3600
                && row.buckets == 12
                && row.hold == 1
                && row.boundary_bps == 1000
        })
        .expect("row");
    let extrapolated = small.contiguous * 100;
    assert!(
        extrapolated > large.contiguous,
        "linear extrapolation {extrapolated} did not exceed the computed {}",
        large.contiguous
    );
    assert!(extrapolated > large.contiguous * 3 / 2);
}

#[test]
fn the_reversal_is_where_the_cost_lives_and_it_is_capped() {
    // The blunt version of the ceiling result, restated as a falsifier of "a
    // deeper boundary always costs proportionally more". Beyond a certain
    // displacement the burn stops growing while the capital keeps going.
    let pool = Pool::new(1_000_000_000, 1_000_000_000, 30).expect("pool");
    let near = round_trip_to(&pool, &Price::new(4, 1).expect("price")).expect("trip");
    let far = round_trip_to(&pool, &Price::new(10_000, 1).expect("price")).expect("trip");
    // Ninety times the capital buys less than twice the burn.
    assert!(far.quote_in > 90 * near.quote_in);
    assert!(far.net_cost < 2 * near.net_cost);
}

#[test]
fn moving_a_twelve_bucket_print_from_one_bucket_costs_more_than_from_all_twelve() {
    // The concrete shape of the hold-count result, isolated from the table.
    let pool = Pool::new(1_000_000_000, 1_000_000_000, 30).expect("pool");
    let one = required_bucket_price(&pool.price(), 12, 1, 100).expect("target");
    let all = required_bucket_price(&pool.price(), 12, 12, 100).expect("target");
    let from_one =
        execute(&pool, &Schedule::contiguous(12, 1, &one).expect("schedule")).expect("execute");
    let from_all = execute(
        &pool,
        &Schedule::contiguous(12, 12, &all).expect("schedule"),
    )
    .expect("execute");
    assert!(from_one.net_cost > from_all.net_cost);
    assert!(from_one.peak_capital > 10 * from_all.peak_capital);
}
