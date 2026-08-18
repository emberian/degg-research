//! Every finding the owner check battery can report has a witness.
//!
//! The discipline is the inclusion lane's: a class that looks reachable and is
//! not is worse than no class, so each one is exhibited here or its
//! unreachability is recorded as an argument. The fill-family findings have
//! their witnesses in `tests/detection.rs`, where the adversary handles that
//! produce them live; this file covers the evidence-level ones by handing the
//! battery an owner bundle that is wrong in exactly one way.

mod common;

use common::{NOW, balanced_residual, no_trade, session};
use degg_relation_ir::lower::OwnerOutput;
use degg_shielded_baseline::executor::Tamper;
use degg_shielded_baseline::owner::{Finding, OwnedPosition, OwnerEvidence, audit};
use degg_shielded_baseline::receipt::ReceiptRejection;
use degg_shielded_baseline::scenario::{Run, Session};

fn evidence_for<'a>(session: &'a Session, run: &'a Run, owner: u8) -> OwnerEvidence<'a> {
    session.owner_evidence(run, owner)
}

fn findings(session: &Session, evidence: &OwnerEvidence<'_>, run: &Run) -> Vec<Finding> {
    audit(&session.module, evidence, &run.run.receipt, &run.run.public).findings
}

#[test]
fn a_receipt_bound_to_another_cutoff_fails_the_public_check() {
    let mut here = session(&balanced_residual());
    let run = here.compute(&Tamper::None, NOW).expect("computes");
    let mut elsewhere = session(&no_trade());
    let other = elsewhere.compute(&Tamper::None, NOW).expect("computes");
    let evidence = evidence_for(&here, &run, 0);
    let report = audit(
        &here.module,
        &evidence,
        &other.run.receipt,
        &other.run.public,
    );
    assert!(
        report
            .findings
            .contains(&Finding::PublicCheck(ReceiptRejection::CutoffMismatch))
    );
}

#[test]
fn a_tampered_inclusion_receipt_is_refused() {
    let mut session = session(&balanced_residual());
    let run = session.compute(&Tamper::None, NOW).expect("computes");
    let mut inclusion = session.receipts[0].clone();
    inclusion.record.submitter[0] ^= 1;
    let evidence = OwnerEvidence {
        owner: 0,
        positions: vec![OwnedPosition {
            submission: &session.submissions[0],
            inclusion: &inclusion,
            opening: run.openings.first(),
        }],
        cutoff: &session.cutoff,
        delivered: run.delivered[0].as_ref(),
    };
    let found = findings(&session, &evidence, &run);
    assert!(
        found
            .iter()
            .any(|finding| matches!(finding, Finding::InclusionReceiptInvalid { seq: 0, .. }))
    );
}

#[test]
fn a_record_committing_another_payload_is_caught_by_the_owner() {
    let mut session = session(&balanced_residual());
    let run = session.compute(&Tamper::None, NOW).expect("computes");
    let mut mine = session.submissions[0];
    mine.sealed = mine
        .sealed
        .adversarially_replace(&session.submissions[1].sealed);
    let evidence = OwnerEvidence {
        owner: 0,
        positions: vec![OwnedPosition {
            submission: &mine,
            inclusion: &session.receipts[0],
            opening: run.openings.first(),
        }],
        cutoff: &session.cutoff,
        delivered: run.delivered[0].as_ref(),
    };
    assert!(
        findings(&session, &evidence, &run)
            .contains(&Finding::PayloadCommitmentMismatch { seq: 0 })
    );
}

#[test]
fn a_record_committing_another_nullifier_is_caught_by_the_owner() {
    let mut session = session(&balanced_residual());
    let run = session.compute(&Tamper::None, NOW).expect("computes");
    let mut mine = session.submissions[0];
    mine.plain.nullifier = 4_242;
    let evidence = OwnerEvidence {
        owner: 0,
        positions: vec![OwnedPosition {
            submission: &mine,
            inclusion: &session.receipts[0],
            opening: run.openings.first(),
        }],
        cutoff: &session.cutoff,
        delivered: run.delivered[0].as_ref(),
    };
    assert!(findings(&session, &evidence, &run).contains(&Finding::NullifierMismatch { seq: 0 }));
}

#[test]
fn a_missing_or_tampered_delivery_opening_is_refused() {
    let mut session = session(&balanced_residual());
    let run = session.compute(&Tamper::None, NOW).expect("computes");

    let absent = OwnerEvidence {
        owner: 0,
        positions: vec![OwnedPosition {
            submission: &session.submissions[0],
            inclusion: &session.receipts[0],
            opening: None,
        }],
        cutoff: &session.cutoff,
        delivered: run.delivered[0].as_ref(),
    };
    assert!(
        findings(&session, &absent, &run).contains(&Finding::DeliveryOpeningMissing { seq: 0 })
    );

    let mut tampered = run.openings[0].clone();
    tampered.proof.path[0].hash[0] ^= 1;
    let bad = OwnerEvidence {
        owner: 0,
        positions: vec![OwnedPosition {
            submission: &session.submissions[0],
            inclusion: &session.receipts[0],
            opening: Some(&tampered),
        }],
        cutoff: &session.cutoff,
        delivered: run.delivered[0].as_ref(),
    };
    assert!(
        findings(&session, &bad, &run)
            .iter()
            .any(|finding| matches!(finding, Finding::DeliveryOpeningInvalid { seq: 0, .. }))
    );
}

#[test]
fn every_delivered_local_output_defect_has_a_witness() {
    let mut session = session(&balanced_residual());
    let run = session.compute(&Tamper::None, NOW).expect("computes");
    let truth = run.delivered[0].clone().expect("owner 0 is delivered");
    let position = |opening| OwnedPosition {
        submission: &session.submissions[0],
        inclusion: &session.receipts[0],
        opening,
    };
    fn bundle<'a>(
        session: &'a Session,
        position: OwnedPosition<'a>,
        delivered: Option<&'a OwnerOutput>,
    ) -> OwnerEvidence<'a> {
        OwnerEvidence {
            owner: 0,
            positions: vec![position],
            cutoff: &session.cutoff,
            delivered,
        }
    }
    let bundle = |delivered| bundle(&session, position(run.openings.first()), delivered);

    assert!(findings(&session, &bundle(None), &run).contains(&Finding::LocalOutputMissing));

    let mut wrong_owner = truth.clone();
    wrong_owner.owner = 1;
    assert!(
        findings(&session, &bundle(Some(&wrong_owner)), &run)
            .contains(&Finding::LocalOutputWrongOwner { claimed: 1 })
    );

    let mut extra = truth.clone();
    extra.owned_slot_fills[2] = Some(0);
    assert!(
        findings(&session, &bundle(Some(&extra)), &run)
            .contains(&Finding::UnexpectedOwnedPosition { seq: 2 })
    );

    let mut missing = truth.clone();
    missing.owned_slot_fills[0] = None;
    assert!(
        findings(&session, &bundle(Some(&missing)), &run)
            .contains(&Finding::MissingOwnedPosition { seq: 0 })
    );

    let mut skewed = truth.clone();
    skewed.bought += 1;
    assert!(findings(&session, &bundle(Some(&skewed)), &run).contains(&Finding::AggregateMismatch));
}
