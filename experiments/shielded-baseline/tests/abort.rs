//! Corruption and abort: a crashed executor, a withheld payload, a result
//! bound to another root, and what each does to reserved funds.
//!
//! Every path here is the inclusion lane's
//! [`degg_inclusion_availability::lifecycle`] machinery, reused unmodified.
//! What this suite adds is the composition: the reserved amounts are the ones
//! the Shielded submitters escrowed, the receipts are the ones the padded
//! cutoff root issued, and the delivered result is the one the named executor
//! published.

mod common;

use common::{NOW, balanced_residual, price_tie_low, session, under_reserved};
use degg_inclusion_availability::lifecycle::{
    AbortClass, BatchMachine, Consequence, Disposition, Entitlement, Phase, RefundError,
};
use degg_shielded_baseline::executor::Tamper;

#[test]
fn a_crashed_executor_times_out_retries_then_exhausts_and_every_record_refunds() {
    let scenario = balanced_residual();
    let mut session = session(&scenario);
    let escrowed = session.ledger.total_escrowed();
    assert_eq!(escrowed, 15 + 6 + 4 + 4);

    // The executor never computes. The first deadline is retryable.
    let cutoff_epoch = session.domain.cutoff_epoch;
    let first = cutoff_epoch + session.timeouts.compute + 1;
    assert_eq!(
        session.machine.tick(first),
        Phase::Aborted(AbortClass::ComputeTimeout { attempts: 1 })
    );
    assert_eq!(
        AbortClass::ComputeTimeout { attempts: 1 }.consequence(),
        Consequence::RetryAgainstSameCutoffRoot
    );
    // A retryable abort pays nothing.
    assert_eq!(session.ledger.total_refunded(), 0);

    session
        .machine
        .resume(first)
        .expect("one retry is permitted");
    let second = cutoff_epoch + session.timeouts.compute * 2 + 1;
    let phase = session.machine.tick(second);
    assert_eq!(
        phase,
        Phase::Aborted(AbortClass::ComputeExhausted { attempts: 2 })
    );
    assert_eq!(
        AbortClass::ComputeExhausted { attempts: 2 }.consequence(),
        Consequence::RefundEveryAdmittedRecord
    );

    // Every admitted record refunds exactly once, against its own inclusion
    // receipt, and the ledger conserves.
    for seq in 0..u32::try_from(session.cutoff.leaf_count).expect("bounded") {
        let receipt = &session.receipts[usize::try_from(seq).expect("bounded")];
        let claim = session
            .machine
            .claim_refund(&mut session.ledger, &Entitlement::Included(receipt));
        if receipt.record.is_padding(&session.domain) {
            assert_eq!(claim, Err(RefundError::NotEscrowed));
        } else {
            assert!(claim.is_ok(), "position {seq}: {claim:?}");
            assert_eq!(
                session
                    .machine
                    .claim_refund(&mut session.ledger, &Entitlement::Included(receipt)),
                Err(RefundError::AlreadyRefunded)
            );
        }
    }
    assert_eq!(session.ledger.total_refunded(), escrowed);
    assert_eq!(session.ledger.total_settled(), 0);
    assert_eq!(session.ledger.total_outstanding(), 0);
    assert!(session.ledger.conserves());
}

#[test]
fn a_padding_position_has_a_verifying_receipt_and_refunds_nothing() {
    let scenario = price_tie_low();
    let mut session = session(&scenario);
    let cutoff_epoch = session.domain.cutoff_epoch;
    session
        .machine
        .tick(cutoff_epoch + session.timeouts.compute + 1);
    session
        .machine
        .resume(cutoff_epoch + session.timeouts.compute + 1)
        .expect("resumes");
    session
        .machine
        .tick(cutoff_epoch + session.timeouts.compute * 2 + 1);
    for seq in [2usize, 3] {
        let receipt = &session.receipts[seq];
        assert!(receipt.record.is_padding(&session.domain));
        degg_inclusion_availability::log::verify_receipt(&session.cutoff, receipt)
            .expect("a padding receipt verifies");
        assert_eq!(
            session
                .machine
                .claim_refund(&mut session.ledger, &Entitlement::Included(receipt)),
            Err(RefundError::NotEscrowed)
        );
    }
    assert!(session.ledger.conserves());
}

#[test]
fn a_withheld_payload_aborts_before_the_compute_deadline() {
    // Silence is unavailability: a position with no report counts as zero
    // recoverable shares, and the availability deadline fires before the
    // compute deadline so the abort names the withholding.
    let scenario = balanced_residual();
    let mut session = session(&scenario);
    let mut watcher = BatchMachine::new(session.domain, session.timeouts);
    watcher
        .observe_cutoff(session.cutoff, session.domain.cutoff_epoch)
        .expect("observes");
    for seq in 0..3 {
        watcher
            .report_availability(seq, session.domain.availability_shares)
            .expect("reports");
    }
    let phase = watcher.tick(session.domain.cutoff_epoch + session.timeouts.availability + 1);
    assert_eq!(phase, Phase::Aborted(AbortClass::InputWithheld { seq: 3 }));
    assert_eq!(
        AbortClass::InputWithheld { seq: 3 }.consequence(),
        Consequence::RefundEveryAdmittedRecord
    );
    // There is no transition anywhere from a missing payload to a smaller batch.
    assert!(watcher.compute_with_subset(&[0, 1, 2]).is_err());

    let escrowed = session.ledger.total_escrowed();
    for seq in 0..u32::try_from(session.cutoff.leaf_count).expect("bounded") {
        let receipt = &session.receipts[usize::try_from(seq).expect("bounded")];
        let _ = watcher.claim_refund(&mut session.ledger, &Entitlement::Included(receipt));
    }
    assert_eq!(session.ledger.total_refunded(), escrowed);
    assert!(session.ledger.conserves());
}

#[test]
fn a_withheld_cutoff_root_refunds_on_escrow_alone() {
    let scenario = balanced_residual();
    let mut session = session(&scenario);
    let mut watcher = BatchMachine::new(session.domain, session.timeouts);
    let phase = watcher.tick(session.domain.cutoff_epoch + session.timeouts.seal + 1);
    assert_eq!(phase, Phase::Aborted(AbortClass::CutoffRootWithheld));
    assert_eq!(
        AbortClass::CutoffRootWithheld.consequence(),
        Consequence::RefundEveryEscrowedSubmission
    );
    let escrowed = session.ledger.total_escrowed();
    for submission in &session.submissions {
        watcher
            .claim_refund(
                &mut session.ledger,
                &Entitlement::Escrowed {
                    nullifier: submission.request.nullifier,
                },
            )
            .expect("escrow alone entitles the claim");
    }
    assert_eq!(session.ledger.total_refunded(), escrowed);
    assert!(session.ledger.conserves());
    for submission in &session.submissions {
        assert_eq!(
            session.ledger.disposition(&submission.request.nullifier),
            Some(Disposition::Refunded)
        );
    }
}

#[test]
fn a_result_bound_to_another_root_is_refused_and_refunds_every_admitted_record() {
    let scenario = balanced_residual();
    let mut session = session(&scenario);
    let mut watcher = BatchMachine::new(session.domain, session.timeouts);
    let now = session.domain.cutoff_epoch;
    watcher
        .observe_cutoff(session.cutoff, now)
        .expect("observes");
    for seq in 0..u32::try_from(session.cutoff.leaf_count).expect("bounded") {
        watcher
            .report_availability(seq, session.domain.availability_shares)
            .expect("reports");
    }
    watcher.begin_compute(now).expect("begins");
    let phase = watcher
        .deliver_result([0xab; 32], [0xcd; 32], now)
        .expect("delivers");
    assert_eq!(phase, Phase::Aborted(AbortClass::ResultUnbound));
    let escrowed = session.ledger.total_escrowed();
    for seq in 0..u32::try_from(session.cutoff.leaf_count).expect("bounded") {
        let receipt = &session.receipts[usize::try_from(seq).expect("bounded")];
        let _ = watcher.claim_refund(&mut session.ledger, &Entitlement::Included(receipt));
    }
    assert_eq!(session.ledger.total_refunded(), escrowed);
    assert!(session.ledger.conserves());
}

#[test]
fn composition_gap_c1_a_typed_relation_refusal_has_no_refund_path() {
    // A finding about the composition, recorded as a test rather than as a
    // caveat. The relation refuses the admitted batch with a public typed
    // class, the executor publishes a receipt bound to the right cutoff root,
    // and `deliver_result` maps *any* delivered result to `Settled`, because
    // the inclusion lane's abort taxonomy has no class for "the relation
    // refused". The reserved funds therefore have no refund path: the ledger
    // reports `PhaseNotRefundable`, and the only release available is to a
    // settlement relation that does not exist.
    //
    // This packet does not patch the upstream lane; it names the gap. Closing
    // it needs a `relation-refused` abort class upstream, whose consequence is
    // `RefundEveryAdmittedRecord`.
    let scenario = under_reserved();
    let mut session = session(&scenario);
    let run = session.compute(&Tamper::None, NOW).expect("computes");
    assert!(matches!(
        run.run.receipt.status,
        degg_relation_ir::receipt::ReceiptStatus::Refused(_)
    ));
    assert!(matches!(run.phase, Phase::Settled { .. }));
    let receipt = &session.receipts[0];
    assert_eq!(
        session
            .machine
            .claim_refund(&mut session.ledger, &Entitlement::Included(receipt)),
        Err(RefundError::PhaseNotRefundable { phase: run.phase })
    );
    assert_eq!(
        session.ledger.total_outstanding(),
        session.ledger.total_escrowed()
    );
}
