//! Shared fixtures for the Shielded baseline suite.
#![allow(dead_code)]

use degg_relation_ir::batch::Side;
use degg_shielded_baseline::scenario::{BookOrder, CUTOFF_EPOCH, Scenario, Session};

/// The epoch every scenario seals and computes at.
pub const NOW: u64 = CUTOFF_EPOCH;

/// The balanced crossing of the relation's anchor corpus: tick 2, volume 5,
/// fills `[5, 0, 3, 2]`, with a pro-rata residual tie.
#[must_use]
pub fn balanced_residual() -> Scenario {
    Scenario::new(
        "balanced-residual",
        vec![
            BookOrder::exact(0, Side::Buy, 2, 5),
            BookOrder::exact(1, Side::Buy, 1, 3),
            BookOrder::exact(2, Side::Sell, 0, 4),
            BookOrder::exact(3, Side::Sell, 2, 4),
        ],
    )
}

/// A maximum-volume tie that must select the lowest tick, with two committed
/// positions and two padding positions.
#[must_use]
pub fn price_tie_low() -> Scenario {
    Scenario::new(
        "price-tie-low",
        vec![
            BookOrder::exact(0, Side::Buy, 2, 4),
            BookOrder::exact(1, Side::Sell, 1, 4),
        ],
    )
}

/// A non-crossing book: a valid no-trade, not a refusal.
#[must_use]
pub fn no_trade() -> Scenario {
    Scenario::new(
        "no-trade",
        vec![
            BookOrder::exact(0, Side::Buy, 0, 2),
            BookOrder::exact(1, Side::Sell, 3, 2),
        ],
    )
}

/// A book whose second order under-reserves. The admission log cannot see it,
/// because reservations live inside the payload; the relation refuses it.
#[must_use]
pub fn under_reserved() -> Scenario {
    let mut orders = vec![
        BookOrder::exact(0, Side::Buy, 2, 4),
        BookOrder::exact(1, Side::Sell, 1, 4),
    ];
    orders[1].reserved = Some(0);
    Scenario::new("under-reserved", orders)
}

/// A book whose second order claims, inside its sealed payload, an arrival
/// after the cutoff. The envelope's arrival epoch is what the log observes and
/// is in time; only the relation checks the claim inside.
#[must_use]
pub fn late_inside_the_seal() -> Scenario {
    let mut orders = vec![
        BookOrder::exact(0, Side::Buy, 2, 4),
        BookOrder::exact(1, Side::Sell, 1, 4),
    ];
    orders[1].arrived_at = Some(CUTOFF_EPOCH + 1);
    Scenario::new("late-inside-the-seal", orders)
}

/// Open a session and seal its padded cutoff.
#[must_use]
pub fn session(scenario: &Scenario) -> Session {
    Session::open(scenario, NOW).expect("scenario opens")
}

/// A deep book: every slot at the quantity ceiling, clearing at tick 0 with
/// volume 30. Large quantities make the pro-rata feasibility interval a proper
/// subset of `0..=quantity`, which is what gives
/// `owner::Finding::ProRataInfeasible` a witness.
#[must_use]
pub fn deep_book() -> Scenario {
    Scenario::new(
        "deep-book",
        vec![
            BookOrder::exact(0, Side::Buy, 3, 15),
            BookOrder::exact(1, Side::Buy, 3, 15),
            BookOrder::exact(2, Side::Sell, 0, 15),
            BookOrder::exact(3, Side::Sell, 0, 15),
        ],
    )
}
