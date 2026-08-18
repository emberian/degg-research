//! The pool: swap arithmetic, the price move, and the closed forms.

use core::cmp::Ordering;

use degg_manipulation_cost::pool::{
    BPS, Pool, PoolError, Price, closed_form_quote_in, closed_form_round_trip_cost,
    round_trip_cost_ceiling, round_trip_to,
};

#[test]
fn zero_fee_swap_is_the_bare_constant_product_rule() {
    let pool = Pool::new(1_000_000, 1_000_000, 0).expect("pool");
    // out = x * d / (y + d) exactly, with no fee wedge.
    for input in [1u128, 7, 1_000, 999_983, 4_000_000] {
        let expected = 1_000_000 * input / (1_000_000 + input);
        assert_eq!(pool.quote_in_output(input).expect("output"), expected);
    }
}

#[test]
fn the_invariant_never_shrinks() {
    let mut pool = Pool::new(5_000_000, 3_000_000, 30).expect("pool");
    let mut previous = pool.base() * pool.quote();
    for input in [10u128, 5_000, 250_000, 1_000_000] {
        pool.swap_quote_in(input).expect("swap");
        let current = pool.base() * pool.quote();
        assert!(
            current >= previous,
            "invariant fell: {current} < {previous}"
        );
        previous = current;
    }
}

#[test]
fn the_price_is_strictly_increasing_in_the_quote_input() {
    let pool = Pool::new(2_000_000, 2_000_000, 30).expect("pool");
    let mut previous = pool.price();
    for input in 1u128..500 {
        let mut probe = pool;
        probe.swap_quote_in(input).expect("swap");
        assert_eq!(
            probe.price().compare(&previous),
            Ordering::Greater,
            "price did not rise at input {input}"
        );
        previous = probe.price();
    }
}

#[test]
fn bisection_reaches_the_target_and_one_unit_less_does_not() {
    let pool = Pool::new(1_000_000, 1_000_000, 30).expect("pool");
    for (num, den) in [(3u128, 2u128), (2, 1), (11, 5), (10_007, 10_000)] {
        let target = Price::new(num, den).expect("price");
        let input = pool.min_quote_in_to_reach(&target).expect("bisection");
        assert!(input > 0);

        let mut hit = pool;
        hit.swap_quote_in(input).expect("swap");
        assert_ne!(hit.price().compare(&target), Ordering::Less);

        let mut miss = pool;
        miss.swap_quote_in(input - 1).expect("swap");
        assert_eq!(miss.price().compare(&target), Ordering::Less);
    }
}

#[test]
fn bisection_agrees_with_the_derived_closed_form() {
    // The closed form's discriminant must fit in a u128, so the cross-check
    // runs on small pools and small target denominators.
    //
    // The gap is signed, and its sign is the point: flooring the swap output
    // hands the pool one extra base unit, so the integer pool always needs at
    // least what the continuous solution needs, and at most enough extra to
    // buy back that one unit at the target price.
    for (base, quote) in [(1_000_000u128, 1_000_000u128), (2_000_000, 500_000)] {
        for fee in [0u128, 5, 30, 100] {
            let pool = Pool::new(base, quote, fee).expect("pool");
            for (num, den) in [(3u128, 2u128), (2, 1), (5, 1), (21, 20)] {
                let target = Price::new(num, den).expect("price");
                if pool.price().compare(&target) != Ordering::Less {
                    continue;
                }
                let bisected = pool.min_quote_in_to_reach(&target).expect("bisection");
                let closed = closed_form_quote_in(&pool, &target).expect("closed form");
                let slack = 2 + target.num().div_ceil(target.den());
                assert!(
                    bisected >= closed && bisected - closed <= slack,
                    "base {base} quote {quote} fee {fee} target {num}/{den}: {bisected} vs {closed}"
                );
            }
        }
    }
}

#[test]
fn a_zero_fee_round_trip_costs_nothing_but_the_rounding() {
    let pool = Pool::new(1_000_000, 1_000_000, 0).expect("pool");
    let target = Price::new(4, 1).expect("price");
    let trip = round_trip_to(&pool, &target).expect("round trip");
    assert!(trip.quote_in > 0);
    assert!(
        trip.net_cost <= 2,
        "zero-fee round trip cost {}",
        trip.net_cost
    );
}

#[test]
fn the_round_trip_cost_matches_its_closed_form() {
    for depth in [1_000_000u128, 1_000_000_000] {
        for fee in [5u128, 30, 100] {
            let pool = Pool::new(depth, depth, fee).expect("pool");
            for (num, den) in [(21u128, 20u128), (3, 2), (2, 1), (4, 1)] {
                let target = Price::new(num, den).expect("price");
                let trip = round_trip_to(&pool, &target).expect("round trip");
                let closed = closed_form_round_trip_cost(depth, fee, trip.quote_in)
                    .expect("closed-form cost");
                // Two floored outputs, each losing at most one unit, the first
                // of them valued at the peak price.
                let slack = 2 + trip.peak_price.num().div_ceil(trip.peak_price.den());
                assert!(
                    trip.net_cost >= closed && trip.net_cost - closed <= slack,
                    "depth {depth} fee {fee} target {num}/{den}: measured {} closed {closed}",
                    trip.net_cost
                );
            }
        }
    }
}

#[test]
fn no_round_trip_ever_exceeds_the_saturation_ceiling() {
    for depth in [1_000_000u128, 1_000_000_000] {
        for fee in [5u128, 30, 100] {
            let pool = Pool::new(depth, depth, fee).expect("pool");
            let ceiling = round_trip_cost_ceiling(depth, fee).expect("ceiling");
            for multiple in [2u128, 4, 16, 64, 256, 1024] {
                let target = Price::new(multiple, 1).expect("price");
                let trip = round_trip_to(&pool, &target).expect("round trip");
                assert!(
                    trip.net_cost <= ceiling,
                    "depth {depth} fee {fee} x{multiple}: cost {} exceeded ceiling {ceiling}",
                    trip.net_cost
                );
            }
        }
    }
}

#[test]
fn the_cost_ceiling_is_approached_but_never_reached() {
    // Cost saturates; capital does not. A millionfold price move on a 30 bps
    // pool burns within a tenth of a percent of the ceiling while deploying
    // more than a hundred thousand times that burn in capital.
    let depth = 1_000_000_000u128;
    let pool = Pool::new(depth, depth, 30).expect("pool");
    let ceiling = round_trip_cost_ceiling(depth, 30).expect("ceiling");
    let trip = round_trip_to(&pool, &Price::new(1_000_000, 1).expect("price")).expect("round trip");
    assert!(trip.net_cost < ceiling);
    assert!(trip.net_cost * 1_000 > ceiling * 998);
    assert!(trip.quote_in > 100_000 * trip.net_cost);
}

#[test]
fn the_fee_residue_ratchets_the_price_in_the_direction_of_the_push() {
    let pool = Pool::new(1_000_000, 1_000_000, 30).expect("pool");
    let up = round_trip_to(&pool, &Price::new(2, 1).expect("price")).expect("round trip");
    assert_eq!(up.pool_after.base(), pool.base());
    assert_eq!(
        up.pool_after.quote(),
        pool.quote() + up.net_cost,
        "the whole residue lands on the quote side"
    );
    assert_eq!(
        up.pool_after.price().compare(&pool.price()),
        Ordering::Greater
    );
}

#[test]
fn the_two_directions_are_mirror_images() {
    // Selling into (x, y) to halve the price costs the same, in base units,
    // as buying into (y, x) to double it costs in quote units.
    let up = Pool::new(1_000_000, 1_000_000, 30).expect("pool");
    let down = Pool::new(1_000_000, 1_000_000, 30).expect("pool");
    let up_input = up
        .min_quote_in_to_reach(&Price::new(2, 1).expect("price"))
        .expect("up");
    let down_input = down
        .min_base_in_to_fall_to(&Price::new(1, 2).expect("price"))
        .expect("down");
    assert_eq!(up_input, down_input);
}

#[test]
fn construction_refuses_what_the_model_cannot_carry() {
    assert_eq!(Pool::new(0, 1, 30), Err(PoolError::ZeroReserve));
    assert_eq!(Pool::new(1, 0, 30), Err(PoolError::ZeroReserve));
    assert_eq!(Pool::new(1, 1, BPS), Err(PoolError::FeeOutOfRange));
    assert_eq!(Pool::new(u128::MAX, 1, 30), Err(PoolError::ReserveTooLarge));
    assert_eq!(Price::new(1, 0), Err(PoolError::ZeroDenominator));
}

#[test]
fn an_unreachable_target_refuses_rather_than_looping() {
    let pool = Pool::new(1_000_000_000_000, 1_000_000_000_000, 30).expect("pool");
    let target = Price::new(u128::MAX / 4, 1).expect("price");
    assert_eq!(
        pool.min_quote_in_to_reach(&target),
        Err(PoolError::PriceUnreachable)
    );
}
