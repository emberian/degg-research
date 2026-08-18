//! The honest run, end to end: submitters, log, cutoff, executor, receipts,
//! owner checks, settlement.

mod common;

use common::{
    NOW, balanced_residual, late_inside_the_seal, no_trade, price_tie_low, session, under_reserved,
};
use degg_inclusion_availability::lifecycle::{Disposition, Phase};
use degg_relation_ir::lower::{ClearedTick, Outcome, PublicOutcome};
use degg_relation_ir::receipt::ReceiptStatus;
use degg_relation_ir::refusal::RefusalClass;
use degg_shielded_baseline::executor::Tamper;
use degg_shielded_baseline::owner::{RefusalAttribution, audit};
use degg_shielded_baseline::receipt::{DeliveryEntry, public_check, verify_opening};

#[test]
fn the_honest_run_settles_and_every_owner_check_passes() {
    let scenario = balanced_residual();
    let mut session = session(&scenario);
    let run = session.compute(&Tamper::None, NOW).expect("computes");

    // The public result is the relation's, unchanged by the composition.
    let PublicOutcome::Settled(result) = &run.run.public else {
        panic!("the balanced book settles");
    };
    assert_eq!(result.tick, ClearedTick::Tick(2));
    assert_eq!(result.volume, 5);
    assert_eq!(result.accepted_input_root, session.cutoff.root);

    let Outcome::Settled(settled) = &run.run.outcome else {
        panic!("settled");
    };
    assert_eq!(settled.fills, vec![5, 0, 3, 2]);
    assert_eq!(settled.executed_label, "shielded-single-executor");

    // The lifecycle machine accepted a result bound to the cutoff root.
    assert!(matches!(run.phase, Phase::Settled { .. }));

    // The public role can check the receipt, and does not need the executor.
    public_check(
        &run.run.receipt,
        &session.cutoff,
        &session.executor.module_digest(),
        &run.run.public,
    )
    .expect("the published receipt passes the public check");
    assert_eq!(run.run.receipt.status, ReceiptStatus::Settled);
    assert!(run.run.receipt.is_well_formed());

    // Every committed position opens under the delivery root, at its own index.
    assert_eq!(run.openings.len(), session.cutoff.leaf_count as usize);
    for opening in &run.openings {
        verify_opening(&run.run.receipt, opening).expect("opening verifies");
    }

    // Every owner's battery is clean.
    for owner in 0..session.module.params.owners {
        let evidence = session.owner_evidence(&run, owner);
        let report = audit(
            &session.module,
            &evidence,
            &run.run.receipt,
            &run.run.public,
        );
        assert!(
            report.is_consistent(),
            "owner {owner} findings: {:?}",
            report.findings
        );
        assert_eq!(report.refusal, None);
    }

    // Every derived boundary statement holds, and each names its object.
    for fact in &run.assembly.boundary.facts {
        assert!(fact.holds, "boundary port {} did not hold", fact.port);
        assert!(!fact.object.is_empty());
    }

    // Settlement consumes every escrowed reservation exactly once.
    let total = session.ledger.total_escrowed();
    for submission in &session.submissions {
        session
            .machine
            .release_to_settlement(&mut session.ledger, submission.request.nullifier)
            .expect("a settled batch releases to settlement");
    }
    assert_eq!(session.ledger.total_settled(), total);
    assert_eq!(session.ledger.total_refunded(), 0);
    assert_eq!(session.ledger.total_outstanding(), 0);
    assert!(session.ledger.conserves());
    for submission in &session.submissions {
        assert_eq!(
            session.ledger.disposition(&submission.request.nullifier),
            Some(Disposition::Settled)
        );
    }
}

#[test]
fn padding_positions_commit_no_local_output_and_that_is_correct() {
    let scenario = price_tie_low();
    let mut session = session(&scenario);
    let run = session.compute(&Tamper::None, NOW).expect("computes");

    assert_eq!(session.cutoff.leaf_count, 4);
    assert_eq!(session.admitted.len(), 2);
    for (seq, opening) in run.openings.iter().enumerate() {
        let record = session.receipts[seq].record;
        let is_padding = record.is_padding(&session.domain);
        match opening.entry {
            DeliveryEntry::NoLocalOutput => assert!(is_padding, "position {seq} was dropped"),
            DeliveryEntry::Produced(_) => assert!(!is_padding),
        }
    }
    let PublicOutcome::Settled(result) = &run.run.public else {
        panic!("settles");
    };
    assert_eq!(result.tick, ClearedTick::Tick(1));
    assert_eq!(result.volume, 4);
}

#[test]
fn a_non_crossing_book_is_a_valid_no_trade_with_full_reservations_released() {
    let scenario = no_trade();
    let mut session = session(&scenario);
    let run = session.compute(&Tamper::None, NOW).expect("computes");
    let PublicOutcome::Settled(result) = &run.run.public else {
        panic!("settles");
    };
    assert_eq!(result.tick, ClearedTick::NoTrade);
    assert_eq!(result.volume, 0);
    for owner in 0..session.module.params.owners {
        let evidence = session.owner_evidence(&run, owner);
        let report = audit(
            &session.module,
            &evidence,
            &run.run.receipt,
            &run.run.public,
        );
        assert!(report.is_consistent(), "{:?}", report.findings);
    }
    let delivered = run.delivered[0].as_ref().expect("owner 0 is delivered");
    assert_eq!(delivered.released_quote_reservation, 2);
}

#[test]
fn the_log_cannot_see_a_reservation_so_the_relation_refuses_it() {
    // The admission log's ten refusal classes are about envelopes: seal state,
    // timing, wire shape, share count, nullifiers, capacity. It admits this
    // book because it cannot read a reservation, and the relation then refuses
    // with the section 4.1 class.
    let scenario = under_reserved();
    let mut session = session(&scenario);
    assert!(session.refused.is_empty(), "the log admits both envelopes");
    let run = session.compute(&Tamper::None, NOW).expect("computes");
    assert_eq!(
        run.run.receipt.status,
        ReceiptStatus::Refused(RefusalClass::ReservationInsufficient)
    );
    let PublicOutcome::Refused(refusal) = &run.run.public else {
        panic!("refused");
    };
    assert_eq!(refusal.class, RefusalClass::ReservationInsufficient);
    assert_eq!(refusal.accepted_input_root, session.cutoff.root);
    public_check(
        &run.run.receipt,
        &session.cutoff,
        &session.executor.module_digest(),
        &run.run.public,
    )
    .expect("a refusal receipt passes the public check too");
}

#[test]
fn the_envelope_arrival_epoch_and_the_sealed_arrival_claim_are_different_numbers() {
    // The log records the epoch it observed; the payload carries the
    // submitter's own claim, which only the relation checks. A book that is in
    // time on the envelope and late inside the seal is admitted and then
    // refused, which is the composition working, not a gap.
    let scenario = late_inside_the_seal();
    let mut session = session(&scenario);
    assert!(session.refused.is_empty());
    let run = session.compute(&Tamper::None, NOW).expect("computes");
    assert_eq!(
        run.run.receipt.status,
        ReceiptStatus::Refused(RefusalClass::LateArrival)
    );
    // The owner whose own order is late can attribute the class; the other
    // owner cannot, and neither can the public.
    let late = session.owner_evidence(&run, 1);
    let report = audit(&session.module, &late, &run.run.receipt, &run.run.public);
    assert!(matches!(
        report.refusal,
        Some(RefusalAttribution::OwnPositionViolates { .. })
    ));
    let innocent = session.owner_evidence(&run, 0);
    let report = audit(
        &session.module,
        &innocent,
        &run.run.receipt,
        &run.run.public,
    );
    assert_eq!(report.refusal, Some(RefusalAttribution::NotAttributable));
}
