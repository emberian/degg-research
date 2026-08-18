//! Admission refusals: every occupied slot must pass every rule, and a
//! malformed batch is never reinterpreted as no trade.

mod common;

use common::{BATCH_ID, MARKET_ID, batch, tweak};
use degg_batch_oracle::admit::Refusal;
use degg_batch_oracle::book::{Boundary, Order};
use degg_batch_oracle::book::{Direction, Direction::*, Mode, Slot};
use degg_batch_oracle::curve::Clearing;
use degg_batch_oracle::{Outcome, evaluate, publish};

/// One witness edit paired with the class it must produce.
type BoundaryCase = (fn(&mut Boundary), Refusal);
type OrderCase = (fn(&mut Order), Refusal);

fn refusal(outcome: Outcome) -> Refusal {
    match outcome {
        Outcome::Refused(refusal) => refusal,
        Outcome::Settled(settlement) => panic!("expected refusal, settled {settlement:?}"),
    }
}

fn crossing() -> [(u8, Direction, u8, u32); 2] {
    [(0, Buy, 3, 5), (1, Sell, 0, 5)]
}

#[test]
fn dark_target_is_refused() {
    let mut fixture = batch(&crossing());
    fixture.mode = Mode::DarkTarget;
    assert_eq!(refusal(evaluate(&fixture)), Refusal::DarkTargetUnavailable);
}

#[test]
fn dark_target_refuses_even_an_empty_book() {
    let mut fixture = batch(&[]);
    fixture.mode = Mode::DarkTarget;
    assert_eq!(refusal(evaluate(&fixture)), Refusal::DarkTargetUnavailable);
}

#[test]
fn executable_modes_both_settle() {
    for mode in [Mode::Clear, Mode::ShieldedSingleExecutor] {
        let mut fixture = batch(&crossing());
        fixture.mode = mode;
        assert!(
            matches!(evaluate(&fixture), Outcome::Settled(_)),
            "{mode:?}"
        );
        assert!(degg_batch_oracle::mode_is_executable(mode));
    }
    assert!(!degg_batch_oracle::mode_is_executable(Mode::DarkTarget));
}

#[test]
fn each_absent_boundary_statement_refuses_distinctly() {
    let cases: [BoundaryCase; 4] = [
        (|b| b.log_final = false, Refusal::AdmissionLogNotFinal),
        (|b| b.root_binds_slots = false, Refusal::RootBindingAbsent),
        (|b| b.no_conflicting_root = false, Refusal::RootEquivocation),
        (
            |b| b.payloads_available = false,
            Refusal::PayloadUnavailable,
        ),
    ];
    for (edit, expected) in cases {
        let mut fixture = batch(&crossing());
        edit(&mut fixture.boundary);
        assert_eq!(refusal(evaluate(&fixture)), expected);
    }
}

#[test]
fn quantity_domain_is_one_through_fifteen() {
    for bad in [0u32, 16, 60] {
        let fixture = tweak(batch(&crossing()), 0, |order| order.quantity = bad);
        assert_eq!(
            refusal(evaluate(&fixture)),
            Refusal::QuantityOutOfDomain { slot: 0 },
            "quantity {bad}"
        );
    }
    for good in [1u32, 15] {
        let fixture = tweak(batch(&crossing()), 0, |order| {
            order.quantity = good;
            order.reserved = u64::from(good) * 4;
        });
        assert!(matches!(evaluate(&fixture), Outcome::Settled(_)));
    }
}

#[test]
fn limit_tick_must_be_in_grid() {
    let fixture = tweak(batch(&crossing()), 1, |order| order.limit_index = 4);
    assert_eq!(
        refusal(evaluate(&fixture)),
        Refusal::LimitOutOfDomain { slot: 1 }
    );
}

#[test]
fn owner_must_be_in_domain() {
    let fixture = tweak(batch(&crossing()), 0, |order| order.owner = 4);
    assert_eq!(
        refusal(evaluate(&fixture)),
        Refusal::OwnerOutOfDomain { slot: 0 }
    );
}

#[test]
fn slot_must_bind_its_batch_and_market() {
    let fixture = tweak(batch(&crossing()), 0, |order| order.batch = BATCH_ID + 1);
    assert_eq!(
        refusal(evaluate(&fixture)),
        Refusal::BatchBindingMismatch { slot: 0 }
    );
    let fixture = tweak(batch(&crossing()), 1, |order| order.market = MARKET_ID + 1);
    assert_eq!(
        refusal(evaluate(&fixture)),
        Refusal::MarketBindingMismatch { slot: 1 }
    );
}

#[test]
fn late_arrival_refuses() {
    let fixture = tweak(batch(&crossing()), 1, |order| order.arrival += 1);
    assert_eq!(
        refusal(evaluate(&fixture)),
        Refusal::LateArrival { slot: 1 }
    );
    let fixture = tweak(batch(&crossing()), 1, |order| order.arrival -= 1);
    assert!(matches!(evaluate(&fixture), Outcome::Settled(_)));
}

#[test]
fn missing_external_statements_refuse_distinctly() {
    let cases: [OrderCase; 3] = [
        (|o| o.authorized = false, Refusal::Unauthorized { slot: 0 }),
        (|o| o.eligible = false, Refusal::Ineligible { slot: 0 }),
        (|o| o.included = false, Refusal::InclusionAbsent { slot: 0 }),
    ];
    for (edit, expected) in cases {
        let fixture = tweak(batch(&crossing()), 0, edit);
        assert_eq!(refusal(evaluate(&fixture)), expected);
    }
}

#[test]
fn nullifiers_must_be_nonzero_and_distinct() {
    let fixture = tweak(batch(&crossing()), 0, |order| order.nullifier = 0);
    assert_eq!(
        refusal(evaluate(&fixture)),
        Refusal::NullifierZero { slot: 0 }
    );

    let base = batch(&crossing());
    let Slot::Taken(first) = base.slots[0] else {
        unreachable!()
    };
    let fixture = tweak(base, 1, |order| order.nullifier = first.nullifier);
    assert_eq!(
        refusal(evaluate(&fixture)),
        Refusal::NullifierRepeated { slot: 1, first: 0 }
    );
}

#[test]
fn reservation_must_cover_the_worst_case_obligation() {
    // A buy reserves quote at its own limit price.
    let fixture = tweak(batch(&crossing()), 0, |order| order.reserved -= 1);
    assert_eq!(
        refusal(evaluate(&fixture)),
        Refusal::ReservationInsufficient { slot: 0 }
    );
    // A sell reserves base.
    let fixture = tweak(batch(&crossing()), 1, |order| order.reserved -= 1);
    assert_eq!(
        refusal(evaluate(&fixture)),
        Refusal::ReservationInsufficient { slot: 1 }
    );
    // Surplus reservation is admissible.
    let fixture = tweak(batch(&crossing()), 0, |order| order.reserved += 100);
    assert!(matches!(evaluate(&fixture), Outcome::Settled(_)));
}

#[test]
fn buy_reservation_uses_its_own_limit_not_the_clearing_price() {
    // Buy 5 at tick 3 must reserve 5 * 4 = 20 quote even though the batch
    // clears at tick 0 for a quote cost of 5.
    let fixture = batch(&crossing());
    let Slot::Taken(buy) = fixture.slots[0] else {
        unreachable!()
    };
    assert_eq!(buy.reserved, 20);
    let short = tweak(fixture, 0, |order| order.reserved = 19);
    assert_eq!(
        refusal(evaluate(&short)),
        Refusal::ReservationInsufficient { slot: 0 }
    );
}

#[test]
fn malformed_batch_is_not_reinterpreted_as_no_trade() {
    // A book that would be a valid no-trade if the bad slot were dropped.
    let fixture = tweak(batch(&[(0, Buy, 0, 3), (1, Sell, 3, 3)]), 0, |order| {
        order.included = false
    });
    assert_eq!(
        refusal(evaluate(&fixture)),
        Refusal::InclusionAbsent { slot: 0 }
    );
    let published = publish(&fixture, &evaluate(&fixture));
    assert_eq!(published.refusal, Some("inclusion-absent"));
    assert_eq!(published.tick, None);
    assert_eq!(published.volume, 0);
}

#[test]
fn fully_empty_book_is_a_valid_no_trade() {
    let fixture = batch(&[]);
    let Outcome::Settled(settlement) = evaluate(&fixture) else {
        panic!("empty book must settle")
    };
    assert_eq!(settlement.clearing, Clearing::NoTrade);
    assert_eq!(settlement.fills, [0; 4]);
    let published = publish(&fixture, &evaluate(&fixture));
    assert_eq!(published.refusal, None);
    assert_eq!(published.tick, None);
    assert_eq!(published.volume, 0);
    assert_eq!(published.accepted_input_root, common::ROOT);
}

#[test]
fn refusal_classes_are_pairwise_distinct() {
    let classes = [
        Refusal::DarkTargetUnavailable,
        Refusal::AdmissionLogNotFinal,
        Refusal::RootBindingAbsent,
        Refusal::RootEquivocation,
        Refusal::PayloadUnavailable,
        Refusal::BatchBindingMismatch { slot: 0 },
        Refusal::MarketBindingMismatch { slot: 0 },
        Refusal::OwnerOutOfDomain { slot: 0 },
        Refusal::LimitOutOfDomain { slot: 0 },
        Refusal::QuantityOutOfDomain { slot: 0 },
        Refusal::LateArrival { slot: 0 },
        Refusal::Unauthorized { slot: 0 },
        Refusal::Ineligible { slot: 0 },
        Refusal::InclusionAbsent { slot: 0 },
        Refusal::NullifierZero { slot: 0 },
        Refusal::NullifierRepeated { slot: 1, first: 0 },
        Refusal::ReservationInsufficient { slot: 0 },
        Refusal::AccumulatorOverflow,
    ];
    for (i, left) in classes.iter().enumerate() {
        for right in classes.iter().skip(i + 1) {
            assert_ne!(left.class(), right.class(), "{left:?} vs {right:?}");
        }
    }
}
