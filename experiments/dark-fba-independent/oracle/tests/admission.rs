//! Admission refusals: every occupied slot must pass every rule, and a
//! malformed batch is never reinterpreted as no trade.

mod common;

use common::{BATCH_ID, CUTOFF, MARKET_ID, batch, tweak};
use degg_batch_oracle::admit::{Refusal, required_reservation};
use degg_batch_oracle::book::{Batch, Boundary, Order};
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
    let cases: [OrderCase; 4] = [
        (|o| o.authorized = false, Refusal::Unauthorized { slot: 0 }),
        (|o| o.eligible = false, Refusal::Ineligible { slot: 0 }),
        (|o| o.included = false, Refusal::InclusionAbsent { slot: 0 }),
        (
            |o| o.custody_bound = false,
            Refusal::CustodyBindingAbsent { slot: 0 },
        ),
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
        Refusal::CustodyBindingAbsent { slot: 0 },
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

// ---------------------------------------------------------------------------
// Frozen admission-check order, `DARK_FBA_RELATION.md` section 4.1.
//
// Section 8 publishes the refusal class, so which rule wins on a witness that
// violates several is a public observable and is frozen for v0. These tests
// pin the numbered order rather than any single witness.
// ---------------------------------------------------------------------------

/// Every per-slot rule of section 4.1 that slot `index` actually violates,
/// each checked independently rather than at first failure.
///
/// This is a deliberately naive re-derivation of the per-slot predicate: it
/// tells the order tests which rules a multiply-invalid witness really breaks,
/// so they can assert that the lowest-numbered one is the reported class.
fn violated_rules(fixture: &Batch, index: usize) -> Vec<u8> {
    let Slot::Taken(order) = fixture.slots[index] else {
        return Vec::new();
    };
    let mut rules = Vec::new();
    if order.batch != fixture.batch {
        rules.push(6);
    }
    if order.market != fixture.market {
        rules.push(7);
    }
    if order.owner >= 4 {
        rules.push(8);
    }
    if order.quantity == 0 || order.quantity > 15 {
        rules.push(9);
    }
    if usize::from(order.limit_index) >= 4 {
        rules.push(10);
    }
    if order.nullifier == 0 {
        rules.push(11);
    }
    if fixture.slots[..index].iter().any(|earlier| match earlier {
        Slot::Taken(other) => other.nullifier == order.nullifier,
        Slot::Vacant => false,
    }) {
        rules.push(12);
    }
    if order.arrival > fixture.cutoff {
        rules.push(13);
    }
    if !order.authorized {
        rules.push(14);
    }
    if !order.eligible {
        rules.push(15);
    }
    if !order.included {
        rules.push(16);
    }
    if !order.custody_bound {
        rules.push(17);
    }
    // Rule 18 reads the tick grid, so it is only meaningful once rule 10 holds.
    if usize::from(order.limit_index) < 4
        && order.reserved < required_reservation(order.direction, order.limit_index, order.quantity)
    {
        rules.push(18);
    }
    rules
}

/// One per-slot rule, its number, and an edit that violates it alone.
type RuleEdit = (u8, &'static str, fn(&mut Order));

const PER_SLOT_EDITS: [RuleEdit; 13] = [
    (6, "wrong-batch", |o| o.batch += 1),
    (7, "wrong-market", |o| o.market += 1),
    (8, "owner-out-of-domain", |o| o.owner = 4),
    (9, "quantity-out-of-domain", |o| o.quantity = 0),
    (10, "limit-out-of-domain", |o| o.limit_index = 4),
    (11, "nullifier-zero", |o| o.nullifier = 0),
    (12, "nullifier-repeated", |o| o.nullifier = 1),
    (13, "late-arrival", |o| o.arrival = CUTOFF + 1),
    (14, "unauthorized", |o| o.authorized = false),
    (15, "ineligible", |o| o.eligible = false),
    (16, "inclusion-absent", |o| o.included = false),
    (17, "custody-binding-absent", |o| o.custody_bound = false),
    (18, "reservation-insufficient", |o| o.reserved = 0),
];

#[test]
fn each_per_slot_edit_violates_exactly_its_own_rule() {
    for (number, name, edit) in PER_SLOT_EDITS {
        let fixture = tweak(batch(&crossing()), 1, edit);
        assert_eq!(violated_rules(&fixture, 1), vec![number], "{name}");
        assert_eq!(refusal(evaluate(&fixture)).rule(), Some(number), "{name}");
    }
}

#[test]
fn a_multiply_invalid_slot_reports_its_lowest_numbered_violated_rule() {
    for (index, (_, left_name, left)) in PER_SLOT_EDITS.iter().enumerate() {
        for (_, right_name, right) in PER_SLOT_EDITS.iter().skip(index + 1) {
            let fixture = tweak(tweak(batch(&crossing()), 1, *left), 1, *right);
            let violated = violated_rules(&fixture, 1);
            let lowest = violated
                .iter()
                .copied()
                .min()
                .unwrap_or_else(|| panic!("{left_name} + {right_name} violates nothing"));
            assert_eq!(
                refusal(evaluate(&fixture)).rule(),
                Some(lowest),
                "{left_name} + {right_name} violated {violated:?}"
            );
        }
    }
}

#[test]
fn per_slot_rules_are_applied_slot_major() {
    let book = [(0, Buy, 3, 5), (1, Sell, 0, 5), (2, Sell, 0, 1)];
    // Slot 1 repeats slot 0's nullifier (rule 12); slot 2 is zero (rule 11).
    // The earlier slot decides, even though its rule is numbered higher.
    let fixture = tweak(tweak(batch(&book), 1, |o| o.nullifier = 1), 2, |o| {
        o.nullifier = 0
    });
    assert_eq!(
        refusal(evaluate(&fixture)),
        Refusal::NullifierRepeated { slot: 1, first: 0 }
    );
    // Mirror image: the earlier slot again decides, with the roles swapped.
    let fixture = tweak(tweak(batch(&book), 1, |o| o.nullifier = 0), 2, |o| {
        o.nullifier = 1
    });
    assert_eq!(
        refusal(evaluate(&fixture)),
        Refusal::NullifierZero { slot: 1 }
    );
}

#[test]
fn mode_and_boundary_rules_precede_every_per_slot_rule() {
    let slots_broken = tweak(batch(&crossing()), 0, |order| {
        order.nullifier = 0;
        order.included = false;
        order.reserved = 0;
    });
    assert_eq!(refusal(evaluate(&slots_broken)).rule(), Some(11));

    let mut unavailable = slots_broken;
    unavailable.boundary.payloads_available = false;
    assert_eq!(refusal(evaluate(&unavailable)), Refusal::PayloadUnavailable);

    let mut also_unfinalized = unavailable;
    also_unfinalized.boundary.log_final = false;
    assert_eq!(
        refusal(evaluate(&also_unfinalized)),
        Refusal::AdmissionLogNotFinal
    );

    let mut dark = also_unfinalized;
    dark.mode = Mode::DarkTarget;
    assert_eq!(refusal(evaluate(&dark)), Refusal::DarkTargetUnavailable);
}

#[test]
fn the_frozen_order_numbers_every_admission_class_exactly_once() {
    let classes = [
        Refusal::DarkTargetUnavailable,
        Refusal::AdmissionLogNotFinal,
        Refusal::RootBindingAbsent,
        Refusal::RootEquivocation,
        Refusal::PayloadUnavailable,
        Refusal::BatchBindingMismatch { slot: 0 },
        Refusal::MarketBindingMismatch { slot: 0 },
        Refusal::OwnerOutOfDomain { slot: 0 },
        Refusal::QuantityOutOfDomain { slot: 0 },
        Refusal::LimitOutOfDomain { slot: 0 },
        Refusal::NullifierZero { slot: 0 },
        Refusal::NullifierRepeated { slot: 1, first: 0 },
        Refusal::LateArrival { slot: 0 },
        Refusal::Unauthorized { slot: 0 },
        Refusal::Ineligible { slot: 0 },
        Refusal::InclusionAbsent { slot: 0 },
        Refusal::CustodyBindingAbsent { slot: 0 },
        Refusal::ReservationInsufficient { slot: 0 },
    ];
    let numbers: Vec<u8> = classes.iter().map(|class| class.rule().unwrap()).collect();
    assert_eq!(numbers, (1..=18).collect::<Vec<u8>>());
    assert_eq!(Refusal::AccumulatorOverflow.rule(), None);
}
