//! The refund lane: terminal aborts return every reservation exactly once,
//! and the custody ledger agrees with the upstream reserve ledger.

mod common;

use common::{balanced_residual, computed, under_reserved};
use degg_inclusion_availability::lifecycle::{AbortClass, Entitlement, Phase};
use degg_settlement_model::custody::{Balances, CustodyError, CustodyState};
use degg_settlement_model::harness::{escrow_admitted, settle_all};
use degg_settlement_model::relation::{ObserveError, RefundRefusal, SettlementBook, refund};
use degg_shielded_baseline::scenario::Session;

#[test]
fn a_publicly_refused_batch_refunds_every_admitted_record() {
    let (mut session, run, mut custody) = computed(&under_reserved());
    assert!(matches!(
        run.phase,
        Phase::Aborted(AbortClass::RelationRefused { .. })
    ));

    // The refused receipt obligates nothing.
    let mut book = SettlementBook::new();
    assert!(matches!(
        book.observe_settled(&mut custody, &run.run.receipt, run.phase),
        Err(ObserveError::ReceiptStatusRefused { .. })
    ));

    // Both lanes refund the same amounts: 12 for the covered buy, 0 for the
    // uncovered sell.
    let mut refunded = Vec::new();
    for index in session.admitted.clone() {
        let submission = session.submissions[index];
        let seq = session.seq_of(index).expect("admitted");
        let upstream = session
            .machine
            .claim_refund(
                &mut session.ledger,
                &Entitlement::Included(&session.receipts[usize::try_from(seq).expect("bounded")]),
            )
            .expect("upstream refunds");
        let here = refund(
            &mut custody,
            run.phase,
            &submission.request.nullifier,
            common::NOW,
        )
        .expect("custody refunds");
        assert_eq!(upstream, here, "the two ledgers agree on the amount");
        refunded.push(here);
    }
    assert_eq!(refunded, vec![12, 0]);
    assert_eq!(custody.pool(), Balances::zero());
    assert!(custody.conserves());
    for (_, entry) in custody.entries() {
        assert_eq!(entry.state, CustodyState::Refunded);
    }
}

#[test]
fn a_crashed_computation_exhausts_retries_and_refunds() {
    let scenario = balanced_residual();
    let mut session = Session::open(&scenario, common::NOW).expect("opens");
    session
        .machine
        .begin_compute(common::NOW)
        .expect("availability was reported");
    // First attempt times out; the retry is spent; the abort is terminal.
    let after_timeout = session.machine.tick(15);
    assert!(matches!(
        after_timeout,
        Phase::Aborted(AbortClass::ComputeTimeout { attempts: 1 })
    ));
    session.machine.resume(15).expect("one retry is permitted");
    let phase = session.machine.tick(19);
    assert!(matches!(
        phase,
        Phase::Aborted(AbortClass::ComputeExhausted { attempts: 2 })
    ));

    let mut custody = escrow_admitted(&session);
    let mut total = 0u64;
    for index in &session.admitted {
        let submission = &session.submissions[*index];
        total += refund(&mut custody, phase, &submission.request.nullifier, 19)
            .expect("a terminal abort refunds");
    }
    assert_eq!(total, 29, "15 + 6 quote and 4 + 4 base");
    assert_eq!(custody.pool(), Balances::zero());
    assert!(custody.conserves());
}

#[test]
fn a_live_batch_is_not_refundable() {
    let scenario = balanced_residual();
    let session = Session::open(&scenario, common::NOW).expect("opens");
    let mut custody = escrow_admitted(&session);
    let nullifier = session.submissions[0].request.nullifier;
    let phase = session.machine.phase();
    assert_eq!(
        refund(&mut custody, phase, &nullifier, common::NOW),
        Err(RefundRefusal::PhaseNotRefundable { phase })
    );
}

#[test]
fn a_retryable_timeout_is_not_refundable() {
    let scenario = balanced_residual();
    let mut session = Session::open(&scenario, common::NOW).expect("opens");
    session
        .machine
        .begin_compute(common::NOW)
        .expect("availability was reported");
    let phase = session.machine.tick(15);
    assert!(matches!(
        phase,
        Phase::Aborted(AbortClass::ComputeTimeout { .. })
    ));
    let mut custody = escrow_admitted(&session);
    let nullifier = session.submissions[0].request.nullifier;
    assert_eq!(
        refund(&mut custody, phase, &nullifier, 15),
        Err(RefundRefusal::PhaseNotRefundable { phase })
    );
}

#[test]
fn a_settled_batch_is_not_refundable_and_releases_upstream_instead() {
    let mut ready = common::ready(&balanced_residual());
    settle_all(
        &ready.session,
        &ready.run,
        &mut ready.custody,
        &mut ready.book,
        common::NOW,
    );
    let nullifier = ready.session.submissions[0].request.nullifier;
    assert_eq!(
        refund(&mut ready.custody, ready.run.phase, &nullifier, common::NOW),
        Err(RefundRefusal::PhaseNotRefundable {
            phase: ready.run.phase
        })
    );
    // The upstream reserve ledger's settled release agrees with custody: same
    // nullifier, same amount, disposition settled on both sides.
    let mut upstream = ready.session.ledger.clone();
    for index in ready.session.admitted.clone() {
        let submission = ready.session.submissions[index];
        let amount = ready
            .session
            .machine
            .release_to_settlement(&mut upstream, submission.request.nullifier)
            .expect("upstream releases");
        assert_eq!(amount, submission.plain.reserved);
        let entry = ready
            .custody
            .entry(&submission.request.nullifier)
            .expect("entry");
        assert_eq!(entry.amount, amount);
        assert!(matches!(entry.state, CustodyState::Settled { .. }));
    }
    assert!(upstream.conserves());
}

#[test]
fn a_second_refund_claim_is_typed() {
    let (session, run, mut custody) = computed(&under_reserved());
    let nullifier = session.submissions[0].request.nullifier;
    refund(&mut custody, run.phase, &nullifier, common::NOW).expect("first refund");
    assert_eq!(
        refund(&mut custody, run.phase, &nullifier, common::NOW),
        Err(RefundRefusal::Custody(CustodyError::AlreadyRefunded))
    );
}
