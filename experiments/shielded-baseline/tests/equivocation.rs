//! Outcome equivocation: one executor, one cutoff root, two incompatible runs.
//!
//! The shape mirrors `degg_inclusion_availability::equivocation` deliberately,
//! including its limitation: nothing signs, so a verdict is a contradiction
//! between published objects and never an attribution.

mod common;

use common::{NOW, balanced_residual, no_trade, session};
use degg_relation_ir::receipt::ReceiptStatus;
use degg_relation_ir::refusal::RefusalClass;
use degg_shielded_baseline::dispute::{
    OutcomeEquivocationDefect, OutcomeEquivocationProof, verify_outcome_equivocation,
};
use degg_shielded_baseline::executor::Tamper;
use degg_shielded_baseline::receipt::{ShieldedReceipt, SlotOutcome};
use degg_shielded_baseline::roles::ExecutorId;
use degg_shielded_baseline::scenario::Scenario;

fn receipt_for(scenario: &Scenario, tamper: &Tamper) -> ShieldedReceipt {
    let mut session = session(scenario);
    session.compute(tamper, NOW).expect("computes").run.receipt
}

#[test]
fn two_runs_of_one_cutoff_are_bound_to_the_same_root() {
    // The whole packet is deterministic, so two independent sessions over one
    // scenario commit the same admitted set under the same cutoff root. That
    // is what makes the pairs below genuine equivocations rather than two
    // statements about two different markets.
    let scenario = balanced_residual();
    let left = receipt_for(&scenario, &Tamper::None);
    let right = receipt_for(&scenario, &Tamper::None);
    assert_eq!(left.cutoff, right.cutoff);
    assert_eq!(left, right);
    assert_eq!(
        verify_outcome_equivocation(&OutcomeEquivocationProof { left, right }),
        Err(OutcomeEquivocationDefect::ReceiptsAgree)
    );
}

#[test]
fn two_different_public_outcomes_for_one_cutoff_yield_a_verdict() {
    let scenario = balanced_residual();
    let left = receipt_for(&scenario, &Tamper::None);
    let mut substituted = session(&scenario).submissions[0].plain;
    substituted.limit_tick = 0;
    let right = receipt_for(
        &scenario,
        &Tamper::SubstitutePlaintext {
            seq: 0,
            plain: substituted,
        },
    );
    assert_ne!(left.outcome_digest, right.outcome_digest);
    let verdict =
        verify_outcome_equivocation(&OutcomeEquivocationProof { left, right }).expect("verifies");
    assert_eq!(verdict.class, "conflicting-outcome");
    assert_eq!(verdict.cutoff, left.cutoff);
    // Content addressed, and order sensitive exactly as the inclusion lane's
    // verdict is: the left and right bindings enter the digest in order.
    let swapped = verify_outcome_equivocation(&OutcomeEquivocationProof {
        left: right,
        right: left,
    })
    .expect("verifies");
    assert_ne!(swapped.digest, verdict.digest);
    assert_eq!(swapped.class, verdict.class);
}

#[test]
fn one_public_outcome_with_two_delivery_commitments_is_its_own_class() {
    // Economically the loaded case: the tick and the volume agree, and the
    // allocation does not. `DARK_FBA_RELATION.md` section 11 records that
    // swapping equal-remainder orders preserves the public result and changes
    // who receives the residual atom, so a public result cannot witness it.
    let scenario = balanced_residual();
    let left = receipt_for(&scenario, &Tamper::None);
    let plain = session(&scenario).submissions[1].plain;
    let forged = SlotOutcome::derive(1, &plain, 3, 1).expect("derives");
    let right = receipt_for(
        &scenario,
        &Tamper::ForgeSlotOutcome {
            seq: 1,
            outcome: forged,
        },
    );
    assert_eq!(left.outcome_digest, right.outcome_digest);
    assert_ne!(left.delivery_root, right.delivery_root);
    let verdict =
        verify_outcome_equivocation(&OutcomeEquivocationProof { left, right }).expect("verifies");
    assert_eq!(verdict.class, "conflicting-delivery-commitment");
}

#[test]
fn one_outcome_and_one_delivery_with_two_witnesses_is_a_conflicting_assembly() {
    // A nullifier is inside the relation's witness and inside no curve, no
    // allocation, and no slot outcome. Substituting one therefore leaves the
    // public outcome and every delivery entry identical and changes only the
    // assembled input digest, which is a contradiction because assembly is a
    // function of the committed set and the payloads.
    let scenario = balanced_residual();
    let left = receipt_for(&scenario, &Tamper::None);
    let mut substituted = session(&scenario).submissions[0].plain;
    substituted.nullifier = 9_999;
    let right = receipt_for(
        &scenario,
        &Tamper::SubstitutePlaintext {
            seq: 0,
            plain: substituted,
        },
    );
    assert_eq!(left.outcome_digest, right.outcome_digest);
    assert_eq!(left.delivery_root, right.delivery_root);
    assert_ne!(left.input_digest, right.input_digest);
    let verdict =
        verify_outcome_equivocation(&OutcomeEquivocationProof { left, right }).expect("verifies");
    assert_eq!(verdict.class, "conflicting-assembled-input");
}

#[test]
fn one_outcome_with_two_statuses_is_its_own_class() {
    let scenario = balanced_residual();
    let left = receipt_for(&scenario, &Tamper::None);
    let right = ShieldedReceipt::new(
        left.executor,
        left.module_digest,
        left.cutoff,
        left.input_digest,
        left.outcome_digest,
        left.delivery_root,
        ReceiptStatus::Refused(RefusalClass::PayloadUnavailable),
    );
    let verdict =
        verify_outcome_equivocation(&OutcomeEquivocationProof { left, right }).expect("verifies");
    assert_eq!(verdict.class, "conflicting-status");
}

#[test]
fn every_rejection_class_has_a_witness() {
    let scenario = balanced_residual();
    let left = receipt_for(&scenario, &Tamper::None);

    let mut malformed = left;
    malformed.binding[0] ^= 1;
    assert_eq!(
        verify_outcome_equivocation(&OutcomeEquivocationProof {
            left,
            right: malformed
        }),
        Err(OutcomeEquivocationDefect::MalformedReceipt)
    );

    let other_executor = ShieldedReceipt::new(
        ExecutorId::named("another-executor"),
        left.module_digest,
        left.cutoff,
        left.input_digest,
        [0u8; 32],
        left.delivery_root,
        left.status,
    );
    assert_eq!(
        verify_outcome_equivocation(&OutcomeEquivocationProof {
            left,
            right: other_executor
        }),
        Err(OutcomeEquivocationDefect::DifferentExecutors)
    );

    let other_cutoff = receipt_for(&no_trade(), &Tamper::None);
    assert_ne!(left.cutoff, other_cutoff.cutoff);
    assert_eq!(
        verify_outcome_equivocation(&OutcomeEquivocationProof {
            left,
            right: other_cutoff
        }),
        Err(OutcomeEquivocationDefect::DifferentCutoffs)
    );

    let other_module = ShieldedReceipt::new(
        left.executor,
        [7u8; 32],
        left.cutoff,
        left.input_digest,
        [0u8; 32],
        left.delivery_root,
        left.status,
    );
    assert_eq!(
        verify_outcome_equivocation(&OutcomeEquivocationProof {
            left,
            right: other_module
        }),
        Err(OutcomeEquivocationDefect::DifferentModules)
    );
}
