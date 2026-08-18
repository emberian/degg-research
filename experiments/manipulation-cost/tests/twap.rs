//! The grid statistic, the schedule model, and the exhaustive adversary search.

use core::cmp::Ordering;

use degg_manipulation_cost::pool::{Pool, Price, round_trip_to};
use degg_manipulation_cost::twap::{
    Grid, Schedule, TwapError, boundary_price, execute, exhaustive_min_cost,
    naive_independent_cost, naive_spot_cost, required_bucket_price, twap_at_least,
};

fn balanced(depth: u128, fee: u128) -> Pool {
    Pool::new(depth, depth, fee).expect("pool")
}

#[test]
fn a_grid_must_divide_its_window() {
    assert!(Grid::new(300, 6).is_ok());
    assert_eq!(Grid::new(300, 7), Err(TwapError::BadGrid));
    assert_eq!(Grid::new(300, 0), Err(TwapError::BadGrid));
    let grid = Grid::new(3600, 12).expect("grid");
    assert_eq!(grid.bucket_seconds(), 300);
}

#[test]
fn the_required_bucket_price_carries_the_amplification_exactly() {
    let baseline = Price::new(1, 1).expect("price");
    // Twelve buckets, one distorted, a ten basis point boundary: the one
    // distorted print must move a hundred and twenty basis points.
    let required = required_bucket_price(&baseline, 12, 1, 10).expect("required");
    assert_eq!(required, Price::new(10_120, 10_000).expect("price"));
    // All twelve distorted: no amplification at all.
    let flat = required_bucket_price(&baseline, 12, 12, 10).expect("required");
    assert_eq!(flat, Price::new(10_010, 10_000).expect("price"));
    // A boundary at the baseline needs no move.
    let none = required_bucket_price(&baseline, 12, 3, 0).expect("required");
    assert_eq!(none, baseline);
    assert_eq!(
        required_bucket_price(&baseline, 12, 0, 10),
        Err(TwapError::BadHold)
    );
    assert_eq!(
        required_bucket_price(&baseline, 12, 13, 10),
        Err(TwapError::BadHold)
    );
}

#[test]
fn the_mean_test_is_exact_at_the_boundary() {
    let one = Price::new(1, 1).expect("price");
    let high = Price::new(3, 1).expect("price");
    // Mean of (3, 1, 1) is exactly 5/3.
    let samples = [high, one, one];
    let exactly = Price::new(5, 3).expect("price");
    let just_over = Price::new(50_001, 30_000).expect("price");
    assert!(twap_at_least(&samples, &exactly).expect("compare"));
    assert!(!twap_at_least(&samples, &just_over).expect("compare"));
    assert_eq!(twap_at_least(&[], &one), Err(TwapError::BadSchedule));
}

#[test]
fn a_held_excursion_is_two_swaps_and_a_spread_one_is_two_per_bucket() {
    let pool = balanced(1_000_000_000, 30);
    let target = required_bucket_price(&pool.price(), 8, 3, 100).expect("required");

    let contiguous = execute(
        &pool,
        &Schedule::contiguous(8, 3, &target).expect("schedule"),
    )
    .expect("execute");
    assert_eq!(contiguous.swaps, 2);

    let spread = Schedule::spread(8, 3, &target)
        .expect("spread")
        .expect("separable");
    let spread = execute(&pool, &spread).expect("execute");
    assert_eq!(spread.swaps, 6);
}

#[test]
fn exposed_buckets_sample_at_or_above_the_target_and_flat_ones_do_not() {
    let pool = balanced(1_000_000_000, 30);
    let target = required_bucket_price(&pool.price(), 6, 2, 200).expect("required");
    let schedule = Schedule::contiguous(6, 2, &target).expect("schedule");
    let execution = execute(&pool, &schedule).expect("execute");
    for (sample, entry) in execution.samples.iter().zip(schedule.entries()) {
        match entry {
            Some(_) => assert_ne!(sample.compare(&target), Ordering::Less),
            None => assert_eq!(sample.compare(&target), Ordering::Less),
        }
    }
    assert_eq!(schedule.hold_count(), 2);
    assert!(!schedule.is_empty());
    assert_eq!(schedule.len(), 6);
}

#[test]
fn a_schedule_that_crosses_actually_crosses() {
    let pool = balanced(1_000_000_000, 30);
    for buckets in [4u32, 6, 12] {
        for hold in [1u32, 2, buckets] {
            for boundary in [0u128, 10, 200, 1000] {
                let target =
                    required_bucket_price(&pool.price(), buckets, hold, boundary).expect("target");
                let schedule = Schedule::contiguous(buckets, hold, &target).expect("schedule");
                let execution = execute(&pool, &schedule).expect("execute");
                let boundary_price =
                    boundary_price(&pool.price(), boundary).expect("boundary price");
                assert!(
                    twap_at_least(&execution.samples, &boundary_price).expect("compare"),
                    "buckets {buckets} hold {hold} boundary {boundary} did not cross"
                );
            }
        }
    }
}

#[test]
fn a_boundary_at_the_baseline_costs_nothing_and_moves_nothing() {
    let pool = balanced(10_000_000_000, 5);
    for buckets in [4u32, 6, 12] {
        for hold in [1u32, 3, buckets] {
            let target = required_bucket_price(&pool.price(), buckets, hold, 0).expect("target");
            let execution = execute(
                &pool,
                &Schedule::contiguous(buckets, hold, &target).expect("schedule"),
            )
            .expect("execute");
            assert_eq!(execution.net_cost, 0);
            assert_eq!(execution.peak_capital, 0);
            assert_eq!(execution.swaps, 0);
            assert_eq!(execution.pool_after, pool);
        }
    }
    assert_eq!(naive_spot_cost(&pool, 0).expect("spot"), 0);
    assert_eq!(
        naive_independent_cost(&pool, 6, 3, 0).expect("independent"),
        0
    );
}

#[test]
fn a_window_too_short_to_separate_the_buckets_has_no_spread_schedule() {
    let target = Price::new(2, 1).expect("price");
    assert!(Schedule::spread(6, 6, &target).expect("spread").is_none());
    assert!(Schedule::spread(6, 4, &target).expect("spread").is_none());
    assert!(Schedule::spread(7, 4, &target).expect("spread").is_some());
    assert!(Schedule::spread(6, 1, &target).expect("spread").is_some());
}

#[test]
fn exhaustive_search_selects_the_contiguous_uniform_schedule() {
    // Whenever the exact required level is on the enumerated lattice, the
    // cheapest schedule over every exposure pattern and every level assignment
    // is the one the closed form names: one excursion to the required price,
    // held across a contiguous block of exposed buckets.
    let pool = balanced(1_000_000, 30);
    let cases: [(u32, u32, u128, &[u128]); 5] = [
        (5, 1, 100, &[0, 100, 200, 300, 400, 500, 600]),
        (5, 2, 100, &[0, 50, 100, 150, 200, 250, 300, 350, 400]),
        (4, 2, 200, &[0, 100, 200, 300, 400, 500, 600]),
        (6, 3, 100, &[0, 50, 100, 150, 200, 250, 300]),
        (5, 4, 100, &[0, 125, 250, 375]),
    ];
    for (buckets, hold, boundary, levels) in cases {
        let best = exhaustive_min_cost(&pool, buckets, hold, boundary, levels)
            .expect("search")
            .expect("a feasible schedule exists");
        let target = required_bucket_price(&pool.price(), buckets, hold, boundary).expect("target");
        let closed = execute(
            &pool,
            &Schedule::contiguous(buckets, hold, &target).expect("schedule"),
        )
        .expect("execute");
        assert_eq!(
            best.net_cost, closed.net_cost,
            "buckets {buckets} hold {hold} boundary {boundary}: search {} closed form {}",
            best.net_cost, closed.net_cost
        );
        assert_eq!(best.mask, (1u32 << hold) - 1);
        assert_eq!(best.peak_capital, closed.peak_capital);
    }
}

#[test]
fn exhaustive_search_prices_a_coarse_lattice_above_the_closed_form() {
    // The converse guard: when the required level is *not* on the lattice the
    // search must overpay, which is a statement about the lattice and not about
    // the adversary.
    let pool = balanced(1_000_000, 30);
    let levels = [0u128, 100, 200, 300];
    let best = exhaustive_min_cost(&pool, 5, 4, 100, &levels)
        .expect("search")
        .expect("feasible");
    let target = required_bucket_price(&pool.price(), 5, 4, 100).expect("target");
    let closed = execute(
        &pool,
        &Schedule::contiguous(5, 4, &target).expect("schedule"),
    )
    .expect("execute");
    assert!(best.net_cost > closed.net_cost);
}

#[test]
fn the_search_refuses_a_hold_or_lattice_it_cannot_use() {
    let pool = balanced(1_000_000, 30);
    assert_eq!(
        exhaustive_min_cost(&pool, 4, 0, 100, &[0, 100]),
        Err(TwapError::BadHold)
    );
    assert_eq!(
        exhaustive_min_cost(&pool, 4, 5, 100, &[0, 100]),
        Err(TwapError::BadHold)
    );
    assert_eq!(
        exhaustive_min_cost(&pool, 4, 2, 100, &[]),
        Err(TwapError::BadSchedule)
    );
}

#[test]
fn the_ratchet_makes_each_repeated_excursion_cheaper_than_the_last() {
    // The fee residue of a completed round trip sits on the quote side, so the
    // pool starts the next excursion closer to the target.
    let pool = balanced(1_000_000_000, 100);
    let target = Price::new(3, 2).expect("price");
    let first = round_trip_to(&pool, &target).expect("round trip");
    let second = round_trip_to(&first.pool_after, &target).expect("round trip");
    let third = round_trip_to(&second.pool_after, &target).expect("round trip");
    assert!(second.net_cost < first.net_cost);
    assert!(third.net_cost < second.net_cost);
    assert!(second.quote_in < first.quote_in);
}
