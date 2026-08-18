//! Shared fixtures for the settlement-model suite.
#![allow(dead_code)]

use degg_relation_ir::batch::Side;
use degg_settlement_model::custody::CustodyLedger;
use degg_settlement_model::harness::escrow_admitted;
use degg_settlement_model::relation::SettlementBook;
use degg_shielded_baseline::executor::Tamper;
use degg_shielded_baseline::scenario::{BookOrder, CUTOFF_EPOCH, Run, Scenario, Session};

/// The epoch every scenario seals, computes, and settles at.
pub const NOW: u64 = CUTOFF_EPOCH;

/// The balanced crossing of the relation's anchor corpus: tick 2, volume 5,
/// fills `[5, 0, 3, 2]`.
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

/// A non-crossing book: a valid no-trade with two padding positions.
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

/// A book whose second order under-reserves; the relation publicly refuses.
#[must_use]
pub fn under_reserved() -> Scenario {
    let mut orders = vec![
        BookOrder::exact(0, Side::Buy, 2, 4),
        BookOrder::exact(1, Side::Sell, 1, 4),
    ];
    orders[1].reserved = Some(0);
    Scenario::new("under-reserved", orders)
}

/// One driven, computed, escrowed, observed batch, ready to settle.
pub struct Ready {
    /// The session after compute.
    pub session: Session,
    /// The published run.
    pub run: Run,
    /// The custody ledger, escrowed and obligated.
    pub custody: CustodyLedger,
    /// The settlement book, with the receipt observed.
    pub book: SettlementBook,
}

/// Drive a scenario to the point where settlement instructions can execute.
#[must_use]
pub fn ready(scenario: &Scenario) -> Ready {
    ready_tampered(scenario, &Tamper::None)
}

/// Drive a scenario with a dishonest executor.
#[must_use]
pub fn ready_tampered(scenario: &Scenario, tamper: &Tamper) -> Ready {
    let mut session = Session::open(scenario, NOW).expect("scenario opens");
    let run = session.compute(tamper, NOW).expect("scenario computes");
    let mut custody = escrow_admitted(&session);
    let mut book = SettlementBook::new();
    book.observe_settled(&mut custody, &run.run.receipt, run.phase)
        .expect("a settled run obligates");
    Ready {
        session,
        run,
        custody,
        book,
    }
}

/// Drive a scenario to a computed run without observing the receipt.
#[must_use]
pub fn computed(scenario: &Scenario) -> (Session, Run, CustodyLedger) {
    let mut session = Session::open(scenario, NOW).expect("scenario opens");
    let run = session
        .compute(&Tamper::None, NOW)
        .expect("scenario computes");
    let custody = escrow_admitted(&session);
    (session, run, custody)
}
