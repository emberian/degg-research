//! The custody state machine: `Reserved -> Obligated -> Settled | Refunded`,
//! exactly once, with a checked pool.

mod common;

use degg_settlement_model::custody::{Asset, CustodyError, CustodyLedger, CustodyState};

const N1: [u8; 32] = [1u8; 32];
const N2: [u8; 32] = [2u8; 32];
const BINDING: [u8; 32] = [7u8; 32];
const OTHER_BINDING: [u8; 32] = [8u8; 32];
const EXECUTION: [u8; 32] = [9u8; 32];

fn escrowed() -> CustodyLedger {
    let mut custody = CustodyLedger::new();
    custody
        .escrow(N1, 0, Asset::Quote, 15, 10)
        .expect("first escrow");
    custody
        .escrow(N2, 1, Asset::Base, 4, 10)
        .expect("second escrow");
    assert!(custody.conserves());
    custody
}

#[test]
fn a_duplicate_nullifier_is_refused() {
    let mut custody = escrowed();
    assert_eq!(
        custody.escrow(N1, 2, Asset::Base, 1, 10),
        Err(CustodyError::DuplicateNullifier)
    );
    assert!(custody.conserves());
}

#[test]
fn settlement_before_obligation_is_not_a_transition() {
    let mut custody = escrowed();
    assert_eq!(
        custody.settle(
            &N1,
            &BINDING,
            EXECUTION,
            (Asset::Quote, 0),
            (Asset::Base, 5)
        ),
        Err(CustodyError::NotObligated {
            state: CustodyState::Reserved
        })
    );
    assert!(custody.conserves());
}

#[test]
fn obligation_is_idempotent_under_one_binding_and_refuses_another() {
    let mut custody = escrowed();
    custody.obligate(&N1, BINDING).expect("first obligation");
    custody.obligate(&N1, BINDING).expect("same binding again");
    assert_eq!(
        custody.obligate(&N1, OTHER_BINDING),
        Err(CustodyError::ObligationMismatch)
    );
    assert_eq!(
        custody.entry(&N1).expect("entry").state,
        CustodyState::Obligated {
            receipt_binding: BINDING
        }
    );
}

#[test]
fn an_obligated_reservation_is_not_refundable() {
    let mut custody = escrowed();
    custody.obligate(&N1, BINDING).expect("obligation");
    assert_eq!(
        custody.refund(&N1, 11),
        Err(CustodyError::ObligatedNotRefundable)
    );
    assert!(custody.conserves());
}

#[test]
fn settlement_resolves_exactly_once() {
    let mut custody = escrowed();
    custody.obligate(&N1, BINDING).expect("obligation");
    let amount = custody
        .settle(
            &N1,
            &BINDING,
            EXECUTION,
            (Asset::Quote, 3),
            (Asset::Base, 4),
        )
        .expect("settles");
    assert_eq!(amount, 15);
    assert_eq!(
        custody.settle(
            &N1,
            &BINDING,
            EXECUTION,
            (Asset::Quote, 3),
            (Asset::Base, 4)
        ),
        Err(CustodyError::AlreadySettled {
            execution: EXECUTION
        })
    );
    assert_eq!(
        custody.refund(&N1, 11),
        Err(CustodyError::AlreadySettled {
            execution: EXECUTION
        })
    );
    assert_eq!(custody.account(0).base, 4);
    assert_eq!(custody.account(0).quote, 3);
    assert!(custody.conserves());
}

#[test]
fn a_refund_resolves_exactly_once_and_forecloses_settlement() {
    let mut custody = escrowed();
    let amount = custody.refund(&N2, 11).expect("refunds");
    assert_eq!(amount, 4);
    assert_eq!(custody.refund(&N2, 11), Err(CustodyError::AlreadyRefunded));
    assert_eq!(
        custody.obligate(&N2, BINDING),
        Err(CustodyError::AlreadyRefunded)
    );
    assert_eq!(
        custody.settle(
            &N2,
            &BINDING,
            EXECUTION,
            (Asset::Base, 4),
            (Asset::Quote, 0)
        ),
        Err(CustodyError::AlreadyRefunded)
    );
    assert_eq!(custody.account(1).base, 4);
    assert!(custody.conserves());
}

#[test]
fn an_insolvent_payout_is_refused_and_nothing_is_applied() {
    let mut custody = escrowed();
    custody.obligate(&N1, BINDING).expect("obligation");
    let before_pool = custody.pool();
    let before_account = custody.account(0);
    // The pool holds 4 base; a payout of 5 base cannot be covered.
    assert_eq!(
        custody.settle(
            &N1,
            &BINDING,
            EXECUTION,
            (Asset::Quote, 0),
            (Asset::Base, 5)
        ),
        Err(CustodyError::PoolInsolvent {
            asset: Asset::Base,
            needed: 5,
            available: 4
        })
    );
    assert_eq!(custody.pool(), before_pool);
    assert_eq!(custody.account(0), before_account);
    assert_eq!(
        custody.entry(&N1).expect("entry").state,
        CustodyState::Obligated {
            receipt_binding: BINDING
        }
    );
    assert!(custody.conserves());
}

#[test]
fn the_missing_nullifier_is_typed_everywhere() {
    let mut custody = escrowed();
    let absent = [3u8; 32];
    assert_eq!(
        custody.obligate(&absent, BINDING),
        Err(CustodyError::NotEscrowed)
    );
    assert_eq!(
        custody.settle(
            &absent,
            &BINDING,
            EXECUTION,
            (Asset::Base, 0),
            (Asset::Quote, 0)
        ),
        Err(CustodyError::NotEscrowed)
    );
    assert_eq!(custody.refund(&absent, 11), Err(CustodyError::NotEscrowed));
}

#[test]
fn deposits_and_refunds_are_recorded_as_public_rows() {
    let mut custody = escrowed();
    custody.refund(&N2, 11).expect("refunds");
    assert_eq!(custody.deposits().len(), 2);
    assert_eq!(custody.refunds().len(), 1);
    let refund = custody.refunds()[0];
    assert_eq!(refund.account, 1);
    assert_eq!(refund.asset, Asset::Base);
    assert_eq!(refund.amount, 4);
    assert_eq!(refund.epoch, 11);
}
