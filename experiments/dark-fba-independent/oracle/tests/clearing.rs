//! Aggregate curves and maximum-volume / ties-low tick selection.

mod common;

use common::batch;
use degg_batch_oracle::admit::screen;
use degg_batch_oracle::book::Direction::*;
use degg_batch_oracle::curve::{Clearing, curves, select};
use degg_batch_oracle::params::{QUANTITY_CEILING, SLOTS, TICK_PRICES, TICKS, VOLUME_CEILING};
use degg_batch_oracle::{Outcome, evaluate, publish};

fn clear(specs: &[(u8, degg_batch_oracle::book::Direction, u8, u32)]) -> Clearing {
    let fixture = batch(specs);
    match evaluate(&fixture) {
        Outcome::Settled(settlement) => settlement.clearing,
        Outcome::Refused(refusal) => panic!("unexpected refusal {refusal:?}"),
    }
}

#[test]
fn curves_follow_the_stated_sums() {
    let fixture = batch(&[
        (0, Buy, 2, 5),
        (1, Buy, 0, 3),
        (2, Sell, 1, 4),
        (3, Sell, 3, 6),
    ]);
    let book = screen(&fixture).expect("admissible");
    let curves = curves(&book).expect("no overflow");
    // Demand[k] counts buys with limit >= k: {5,3} at k=0, {5} at k in 1..=2.
    assert_eq!(curves.demand, [8, 5, 5, 0]);
    // Supply[k] counts sells with limit <= k: {} at k=0, {4} at k in 1..=2, {4,6} at k=3.
    assert_eq!(curves.supply, [0, 4, 4, 10]);
    assert_eq!(curves.volume, [0, 4, 4, 0]);
}

#[test]
fn curves_are_monotone_on_every_admissible_two_slot_book() {
    for buy_limit in 0..TICKS as u8 {
        for sell_limit in 0..TICKS as u8 {
            for quantity in 1..=QUANTITY_CEILING {
                let fixture = batch(&[
                    (0, Buy, buy_limit, quantity),
                    (1, Sell, sell_limit, quantity),
                ]);
                let book = screen(&fixture).expect("admissible");
                let curves = curves(&book).expect("no overflow");
                for k in 1..TICKS {
                    assert!(curves.demand[k] <= curves.demand[k - 1], "demand rises");
                    assert!(curves.supply[k] >= curves.supply[k - 1], "supply falls");
                }
                for k in 0..TICKS {
                    assert_eq!(
                        curves.volume[k],
                        curves.demand[k].min(curves.supply[k]),
                        "volume is the pointwise min"
                    );
                }
            }
        }
    }
}

#[test]
fn no_crossing_book_is_a_valid_no_trade() {
    // Every buy limit sits strictly below every sell limit.
    assert_eq!(clear(&[(0, Buy, 0, 9), (1, Sell, 1, 9)]), Clearing::NoTrade);
    assert_eq!(
        clear(&[(0, Buy, 2, 15), (1, Sell, 3, 15)]),
        Clearing::NoTrade
    );
    // One-sided books never trade either.
    assert_eq!(clear(&[(0, Buy, 3, 15)]), Clearing::NoTrade);
    assert_eq!(clear(&[(0, Sell, 0, 15)]), Clearing::NoTrade);
}

#[test]
fn selection_maximizes_volume_rather_than_taking_the_lowest_crossing_tick() {
    // Volume is [0, 5, 0, 0]: tick 0 does not cross, so the maximum is at 1.
    assert_eq!(
        clear(&[(0, Buy, 1, 5), (1, Sell, 1, 5)]),
        Clearing::Trade {
            tick: 1,
            price: TICK_PRICES[1],
            volume: 5
        }
    );
    // Volume is [2, 2, 7, 7]: the maximum plateau starts at tick 2, not tick 0.
    assert_eq!(
        clear(&[(0, Buy, 3, 7), (1, Sell, 0, 2), (2, Sell, 2, 5)]),
        Clearing::Trade {
            tick: 2,
            price: TICK_PRICES[2],
            volume: 7
        }
    );
}

#[test]
fn tied_maximum_volume_selects_the_lowest_tick() {
    // A buy spanning the whole grid against a sell spanning the whole grid ties
    // every tick at volume 5.
    let fixture = batch(&[(0, Buy, 3, 5), (1, Sell, 0, 5)]);
    let book = screen(&fixture).expect("admissible");
    let curves = curves(&book).expect("no overflow");
    assert_eq!(curves.volume, [5, 5, 5, 5]);
    assert_eq!(
        select(&curves),
        Clearing::Trade {
            tick: 0,
            price: TICK_PRICES[0],
            volume: 5
        }
    );
}

#[test]
fn tie_low_holds_on_an_interior_plateau() {
    // Volume is [0, 6, 6, 0]: ticks 1 and 2 tie, so tick 1 wins.
    assert_eq!(
        clear(&[(0, Buy, 2, 6), (1, Sell, 1, 6)]),
        Clearing::Trade {
            tick: 1,
            price: TICK_PRICES[1],
            volume: 6
        }
    );
}

#[test]
fn public_result_carries_tick_and_volume_only() {
    let fixture = batch(&[(0, Buy, 3, 5), (1, Sell, 0, 5)]);
    let published = publish(&fixture, &evaluate(&fixture));
    assert_eq!(published.relation, "dark-fba/n4-k4-q15/v0");
    assert_eq!(published.tick, Some(0));
    assert_eq!(published.volume, 5);
    assert_eq!(published.refusal, None);
}

#[test]
fn volume_never_exceeds_the_declared_accumulator_width() {
    let fixture = batch(&[
        (0, Buy, 3, QUANTITY_CEILING),
        (1, Buy, 3, QUANTITY_CEILING),
        (2, Sell, 0, QUANTITY_CEILING),
        (3, Sell, 0, QUANTITY_CEILING),
    ]);
    let book = screen(&fixture).expect("admissible");
    let curves = curves(&book).expect("no overflow");
    assert_eq!(VOLUME_CEILING, QUANTITY_CEILING * SLOTS as u32);
    for k in 0..TICKS {
        assert!(curves.demand[k] <= VOLUME_CEILING);
        assert!(curves.supply[k] <= VOLUME_CEILING);
    }
    assert_eq!(
        select(&curves),
        Clearing::Trade {
            tick: 0,
            price: 1,
            volume: 30
        }
    );
}
