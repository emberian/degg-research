//! Withholding, timeout, typed abort, and refund conservation.

mod common;

use common::{CUTOFF, domain, holder, request, seal_statement, sealed_log};
use degg_inclusion_availability::equivocation::{Conflict, EquivocationProof, RootStatement};
use degg_inclusion_availability::lifecycle::{
    AbortClass, BatchMachine, Consequence, Disposition, Entitlement, LifecycleError, Phase,
    RefundError, ReserveLedger, Timeouts,
};
use degg_inclusion_availability::log::{
    AdmissionLog, CutoffRoot, InclusionReceipt, LogDomain, verify_receipt,
};

const SEAL_DEADLINE: u64 = CUTOFF + 1;
const AVAILABILITY_DEADLINE: u64 = CUTOFF + 2;
const FIRST_ATTEMPT_DEADLINE: u64 = CUTOFF + 4;
const SECOND_ATTEMPT_DEADLINE: u64 = CUTOFF + 8;

const RESULT: [u8; 32] = [0xC0; 32];

/// A relation's own public refusal-class code, opaque to this crate. The value
/// is arbitrary here on purpose: the machine carries it and never reads it.
const REFUSAL_CODE: u32 = 17;

/// Reservation amounts, exact integers, distinct so a mix-up shows up in a total.
const AMOUNTS: [u64; 4] = [10, 200, 3_000, 40_000];

fn escrow_for(tags: &[u8]) -> ReserveLedger {
    let mut ledger = ReserveLedger::new();
    for (position, tag) in tags.iter().enumerate() {
        let amount = AMOUNTS[position % AMOUNTS.len()];
        assert!(ledger.escrow(request(*tag).nullifier, amount));
    }
    ledger
}

/// A sealed four-record log, its escrow, and a machine that has seen the root.
fn sealed_fixture() -> (AdmissionLog, CutoffRoot, ReserveLedger, BatchMachine) {
    let domain = domain();
    let (log, cutoff) = sealed_log(domain, 4);
    let ledger = escrow_for(&[0, 1, 2, 3]);
    let mut machine = BatchMachine::new(domain, Timeouts::dark_fba_v0());
    machine
        .observe_cutoff(cutoff, CUTOFF)
        .expect("the root is published on time");
    (log, cutoff, ledger, machine)
}

fn make_available(machine: &mut BatchMachine, count: u32, threshold: u8) {
    for seq in 0..count {
        machine
            .report_availability(seq, threshold)
            .expect("report is admissible");
    }
}

fn receipts(log: &AdmissionLog, count: u32) -> Vec<InclusionReceipt> {
    (0..count)
        .map(|seq| log.receipt(seq).expect("record is admitted"))
        .collect()
}

#[test]
fn the_happy_path_settles_and_conserves() {
    let (log, cutoff, mut ledger, mut machine) = sealed_fixture();
    make_available(&mut machine, 4, 3);
    machine
        .begin_compute(CUTOFF + 1)
        .expect("all inputs present");
    assert_eq!(machine.phase(), Phase::Computing);
    assert_eq!(
        machine
            .deliver_result(cutoff.root, RESULT, CUTOFF + 3)
            .expect("delivery is admissible"),
        Phase::Settled {
            result_digest: RESULT
        }
    );

    let total = ledger.total_escrowed();
    for receipt in receipts(&log, 4) {
        machine
            .release_to_settlement(&mut ledger, receipt.record.nullifier)
            .expect("settled batch releases reservations");
    }
    assert_eq!(ledger.total_settled(), total);
    assert_eq!(ledger.total_refunded(), 0);
    assert_eq!(ledger.total_outstanding(), 0);
    assert!(ledger.conserves());
}

#[test]
fn a_withheld_cutoff_root_refunds_every_escrowed_submission() {
    let domain = domain();
    let mut ledger = escrow_for(&[0, 1, 2, 3]);
    let mut machine = BatchMachine::new(domain, Timeouts::dark_fba_v0());
    assert_eq!(machine.tick(SEAL_DEADLINE), Phase::Open);
    assert_eq!(
        machine.tick(SEAL_DEADLINE + 1),
        Phase::Aborted(AbortClass::CutoffRootWithheld)
    );

    let total = ledger.total_escrowed();
    for tag in 0..4u8 {
        machine
            .claim_refund(
                &mut ledger,
                &Entitlement::Escrowed {
                    nullifier: request(tag).nullifier,
                },
            )
            .expect("escrow alone entitles a refund");
    }
    assert_eq!(ledger.total_refunded(), total);
    assert_eq!(ledger.total_outstanding(), 0);
    assert!(ledger.conserves());
}

#[test]
fn a_root_published_after_the_seal_deadline_is_not_observed() {
    let domain = domain();
    let (_, cutoff) = sealed_log(domain, 2);
    let mut machine = BatchMachine::new(domain, Timeouts::dark_fba_v0());
    assert!(machine.observe_cutoff(cutoff, SEAL_DEADLINE + 1).is_err());
    assert_eq!(
        machine.phase(),
        Phase::Aborted(AbortClass::CutoffRootWithheld)
    );
    assert!(machine.cutoff().is_none());
}

#[test]
fn silence_about_a_payload_counts_as_unavailability() {
    let (_, _, _, mut machine) = sealed_fixture();
    assert_eq!(machine.first_unavailable(), Some((0, 0)));
    make_available(&mut machine, 4, 3);
    assert_eq!(machine.first_unavailable(), None);
    machine
        .report_availability(2, 2)
        .expect("report is admissible");
    assert_eq!(machine.first_unavailable(), Some((2, 2)));
}

#[test]
fn a_withheld_payload_aborts_rather_than_shrinking_the_batch() {
    let (log, _, mut ledger, mut machine) = sealed_fixture();
    make_available(&mut machine, 4, 3);
    machine
        .report_availability(2, 2)
        .expect("report is admissible");

    assert_eq!(
        machine.begin_compute(CUTOFF + 1),
        Err(LifecycleError::AvailabilityBelowThreshold {
            seq: 2,
            recoverable: 2,
            threshold: 3,
        })
    );
    assert_eq!(
        machine.compute_with_subset(&[0, 1, 3]),
        Err(LifecycleError::SubsetSelectionForbidden),
        "there is no transition from a missing payload to a smaller batch"
    );

    assert_eq!(machine.tick(AVAILABILITY_DEADLINE), Phase::Sealed);
    assert_eq!(
        machine.tick(AVAILABILITY_DEADLINE + 1),
        Phase::Aborted(AbortClass::InputWithheld { seq: 2 })
    );

    let total = ledger.total_escrowed();
    for receipt in receipts(&log, 4) {
        machine
            .claim_refund(&mut ledger, &Entitlement::Included(&receipt))
            .expect("an admitted record is refundable");
    }
    assert_eq!(ledger.total_refunded(), total);
    assert!(ledger.conserves());
}

#[test]
fn availability_reports_are_bounded_by_the_admitted_set_and_the_share_count() {
    let (_, cutoff, _, mut machine) = sealed_fixture();
    assert_eq!(
        machine.report_availability(4, 3),
        Err(LifecycleError::SequenceOutOfLog {
            seq: 4,
            leaf_count: cutoff.leaf_count,
        })
    );
    assert_eq!(
        machine.report_availability(0, 5),
        Err(LifecycleError::SharesOutOfRange {
            reported: 5,
            dispersed: 4,
        })
    );
}

#[test]
fn a_compute_timeout_is_retryable_and_then_terminal() {
    let (log, _, mut ledger, mut machine) = sealed_fixture();
    make_available(&mut machine, 4, 3);
    machine
        .begin_compute(CUTOFF + 1)
        .expect("all inputs present");

    assert_eq!(machine.tick(FIRST_ATTEMPT_DEADLINE), Phase::Computing);
    let first = machine.tick(FIRST_ATTEMPT_DEADLINE + 1);
    assert_eq!(
        first,
        Phase::Aborted(AbortClass::ComputeTimeout { attempts: 1 })
    );
    assert!(!first.is_terminal());
    assert_eq!(
        machine.claim_refund(&mut ledger, &Entitlement::Included(&receipts(&log, 1)[0])),
        Err(RefundError::PhaseNotRefundable { phase: first }),
        "a retryable abort does not release funds"
    );

    assert_eq!(
        machine.resume(FIRST_ATTEMPT_DEADLINE + 1).expect("retry"),
        Phase::Computing
    );
    assert_eq!(machine.attempts(), 1);
    assert_eq!(machine.tick(SECOND_ATTEMPT_DEADLINE), Phase::Computing);
    let second = machine.tick(SECOND_ATTEMPT_DEADLINE + 1);
    assert_eq!(
        second,
        Phase::Aborted(AbortClass::ComputeExhausted { attempts: 2 })
    );
    assert!(second.is_terminal());
    assert_eq!(
        machine.resume(SECOND_ATTEMPT_DEADLINE + 2),
        Err(LifecycleError::NotResumable { phase: second })
    );

    let total = ledger.total_escrowed();
    for receipt in receipts(&log, 4) {
        machine
            .claim_refund(&mut ledger, &Entitlement::Included(&receipt))
            .expect("an exhausted batch refunds");
    }
    assert_eq!(ledger.total_refunded(), total);
    assert!(ledger.conserves());
}

#[test]
fn a_sealed_batch_that_never_computes_also_times_out() {
    let (_, _, _, mut machine) = sealed_fixture();
    make_available(&mut machine, 4, 3);
    assert_eq!(machine.tick(AVAILABILITY_DEADLINE + 1), Phase::Sealed);
    assert_eq!(
        machine.tick(FIRST_ATTEMPT_DEADLINE + 1),
        Phase::Aborted(AbortClass::ComputeTimeout { attempts: 1 })
    );
}

#[test]
fn a_result_bound_to_another_root_aborts() {
    let (log, cutoff, mut ledger, mut machine) = sealed_fixture();
    make_available(&mut machine, 4, 3);
    machine
        .begin_compute(CUTOFF + 1)
        .expect("all inputs present");

    let mut other = cutoff.root;
    other[0] ^= 0x01;
    assert_eq!(
        machine
            .deliver_result(other, RESULT, CUTOFF + 3)
            .expect("delivery is admissible"),
        Phase::Aborted(AbortClass::ResultUnbound)
    );

    let total = ledger.total_escrowed();
    for receipt in receipts(&log, 4) {
        machine
            .claim_refund(&mut ledger, &Entitlement::Included(&receipt))
            .expect("an unbound result refunds");
    }
    assert_eq!(ledger.total_refunded(), total);
    assert!(ledger.conserves());
}

#[test]
fn a_late_result_is_a_timeout_not_a_settlement() {
    let (_, cutoff, _, mut machine) = sealed_fixture();
    make_available(&mut machine, 4, 3);
    machine
        .begin_compute(CUTOFF + 1)
        .expect("all inputs present");
    assert_eq!(
        machine
            .deliver_result(cutoff.root, RESULT, FIRST_ATTEMPT_DEADLINE + 1)
            .expect("delivery is admissible"),
        Phase::Aborted(AbortClass::ComputeTimeout { attempts: 1 })
    );
}

#[test]
fn a_publicly_refused_relation_aborts_and_refunds_every_admitted_record() {
    // Composition gap C-1, found by the Shielded lane and closed here, in the
    // taxonomy that owns it: a batch the relation publicly refuses produces no
    // allocation, so it is not a settlement, and every admitted record's
    // reservation comes back.
    let (log, cutoff, mut ledger, mut machine) = sealed_fixture();
    make_available(&mut machine, 4, 3);
    machine
        .begin_compute(CUTOFF + 1)
        .expect("all inputs present");

    let phase = machine
        .deliver_refusal(cutoff.root, REFUSAL_CODE, CUTOFF + 3)
        .expect("delivery is admissible");
    assert_eq!(
        phase,
        Phase::Aborted(AbortClass::RelationRefused {
            class_code: REFUSAL_CODE
        })
    );
    assert!(phase.is_terminal());
    assert_eq!(phase.name(), "relation-refused");

    let total = ledger.total_escrowed();
    for receipt in receipts(&log, 4) {
        machine
            .claim_refund(&mut ledger, &Entitlement::Included(&receipt))
            .expect("a refused batch refunds every admitted record");
    }
    assert_eq!(ledger.total_refunded(), total);
    assert_eq!(ledger.total_settled(), 0);
    assert_eq!(ledger.total_outstanding(), 0);
    assert!(ledger.conserves());
}

#[test]
fn a_refused_batch_releases_nothing_to_settlement() {
    // The direction matters: reserved funds go back, never forward. There is
    // no allocation for a settlement relation to consume.
    let (log, cutoff, mut ledger, mut machine) = sealed_fixture();
    make_available(&mut machine, 4, 3);
    machine.begin_compute(CUTOFF + 1).expect("inputs present");
    let phase = machine
        .deliver_refusal(cutoff.root, REFUSAL_CODE, CUTOFF + 3)
        .expect("delivery is admissible");
    let nullifier = receipts(&log, 1)[0].record.nullifier;
    assert_eq!(
        machine.release_to_settlement(&mut ledger, nullifier),
        Err(RefundError::PhaseNotSettled { phase })
    );
    assert_eq!(ledger.total_settled(), 0);
    assert!(ledger.conserves());
}

#[test]
fn a_refusal_bound_to_another_root_is_unbound_rather_than_refused() {
    // A refusal is a statement about one admitted set. Offered against another
    // root it says nothing about this batch, so it is `result-unbound`.
    let (_, cutoff, _, mut machine) = sealed_fixture();
    make_available(&mut machine, 4, 3);
    machine.begin_compute(CUTOFF + 1).expect("inputs present");
    let mut other = cutoff.root;
    other[0] ^= 0x01;
    assert_eq!(
        machine
            .deliver_refusal(other, REFUSAL_CODE, CUTOFF + 3)
            .expect("delivery is admissible"),
        Phase::Aborted(AbortClass::ResultUnbound)
    );
}

#[test]
fn a_late_refusal_is_a_timeout_not_a_refusal() {
    let (_, cutoff, _, mut machine) = sealed_fixture();
    make_available(&mut machine, 4, 3);
    machine.begin_compute(CUTOFF + 1).expect("inputs present");
    assert_eq!(
        machine
            .deliver_refusal(cutoff.root, REFUSAL_CODE, FIRST_ATTEMPT_DEADLINE + 1)
            .expect("delivery is admissible"),
        Phase::Aborted(AbortClass::ComputeTimeout { attempts: 1 })
    );
}

#[test]
fn a_refusal_is_not_admissible_outside_a_running_attempt() {
    let (_, cutoff, _, mut machine) = sealed_fixture();
    assert_eq!(
        machine.deliver_refusal(cutoff.root, REFUSAL_CODE, CUTOFF + 1),
        Err(LifecycleError::PhaseForbids {
            phase: Phase::Sealed
        })
    );
    make_available(&mut machine, 4, 3);
    machine.begin_compute(CUTOFF + 1).expect("inputs present");
    machine
        .deliver_result(cutoff.root, RESULT, CUTOFF + 3)
        .expect("delivery is admissible");
    assert_eq!(
        machine.deliver_refusal(cutoff.root, REFUSAL_CODE, CUTOFF + 3),
        Err(LifecycleError::Terminal {
            phase: Phase::Settled {
                result_digest: RESULT
            }
        }),
        "a settled batch is not retroactively refused"
    );
}

#[test]
fn a_refusal_carries_the_relations_own_class_code_verbatim() {
    // The code is opaque to this crate: two codes are two distinct phases with
    // the same class name and the same consequence, and the machine never
    // recomputes either. The relation the code belongs to is the one the log
    // domain names.
    let mut phases = Vec::new();
    for class_code in [0u32, REFUSAL_CODE, u32::MAX] {
        let (_, cutoff, _, mut machine) = sealed_fixture();
        make_available(&mut machine, 4, 3);
        machine.begin_compute(CUTOFF + 1).expect("inputs present");
        let phase = machine
            .deliver_refusal(cutoff.root, class_code, CUTOFF + 3)
            .expect("delivery is admissible");
        assert_eq!(
            phase,
            Phase::Aborted(AbortClass::RelationRefused { class_code })
        );
        assert_eq!(phase.name(), "relation-refused");
        assert_eq!(
            AbortClass::RelationRefused { class_code }.consequence(),
            Consequence::RefundEveryAdmittedRecord
        );
        assert!(!AbortClass::RelationRefused { class_code }.is_retryable());
        phases.push(phase);
    }
    phases.dedup();
    assert_eq!(phases.len(), 3, "distinct codes are distinct phases");
}

/// A holder that published two cutoff roots, and the escrow spanning both.
struct Equivocating {
    domain: LogDomain,
    left_log: AdmissionLog,
    right_log: AdmissionLog,
    proof: EquivocationProof,
    ledger: ReserveLedger,
}

fn equivocating() -> Equivocating {
    let domain = domain();
    let build = |tags: &[u8]| {
        let mut log = AdmissionLog::open(domain).expect("domain is valid");
        for tag in tags {
            log.admit(&request(*tag), CUTOFF).expect("admitted");
        }
        let cutoff = log.seal(CUTOFF).expect("seal at the cutoff");
        (log, cutoff)
    };
    let (left_log, left) = build(&[0, 1, 2]);
    let (right_log, right) = build(&[0, 9, 2]);
    Equivocating {
        domain,
        left_log,
        right_log,
        proof: EquivocationProof {
            domain,
            left: seal_statement(&left),
            right: seal_statement(&right),
            conflict: Conflict::Roots,
        },
        ledger: escrow_for(&[0, 1, 2, 9]),
    }
}

#[test]
fn an_equivocation_aborts_and_refunds_each_nullifier_once() {
    let mut case = equivocating();
    let mut machine = BatchMachine::new(case.domain, Timeouts::dark_fba_v0());
    let phase = machine
        .present_equivocation(&case.proof)
        .expect("proof verifies");
    let AbortClass::Equivocation { verdict_digest } = (match phase {
        Phase::Aborted(class) => class,
        other => panic!("expected an abort, got {other:?}"),
    }) else {
        panic!("expected an equivocation abort");
    };
    assert_ne!(verdict_digest, [0u8; 32]);

    let total = case.ledger.total_escrowed();
    let mut claimed = 0u64;
    for (log, statement) in [
        (&case.left_log, case.proof.left),
        (&case.right_log, case.proof.right),
    ] {
        for seq in 0..3u32 {
            let receipt = log.receipt(seq).expect("record is admitted");
            match machine.claim_refund(
                &mut case.ledger,
                &Entitlement::IncludedUnderRepudiatedRoot {
                    statement,
                    receipt: &receipt,
                },
            ) {
                Ok(amount) => claimed += amount,
                Err(RefundError::AlreadyRefunded) => {}
                Err(other) => panic!("unexpected refund error {other:?}"),
            }
        }
    }
    assert_eq!(
        claimed, total,
        "the union of both logs refunds exactly once"
    );
    assert_eq!(case.ledger.total_refunded(), total);
    assert_eq!(case.ledger.total_outstanding(), 0);
    assert!(case.ledger.conserves());
}

#[test]
fn an_equivocation_refund_needs_a_root_the_holder_actually_published() {
    let mut case = equivocating();
    let mut machine = BatchMachine::new(case.domain, Timeouts::dark_fba_v0());
    machine
        .present_equivocation(&case.proof)
        .expect("proof verifies");

    let (_, unrelated) = sealed_log(case.domain, 2);
    let receipt = case.left_log.receipt(0).expect("record is admitted");
    assert_eq!(
        machine.claim_refund(
            &mut case.ledger,
            &Entitlement::IncludedUnderRepudiatedRoot {
                statement: RootStatement::seal(holder(), &unrelated),
                receipt: &receipt,
            },
        ),
        Err(RefundError::RootNotRepudiated)
    );
    assert_eq!(
        machine.claim_refund(&mut case.ledger, &Entitlement::Included(&receipt)),
        Err(RefundError::WrongEntitlement {
            consequence: Consequence::RefundUnderEitherRepudiatedRoot
        }),
        "an equivocated batch has no single cutoff root to claim against"
    );
    assert_eq!(case.ledger.total_refunded(), 0);
    assert!(case.ledger.conserves());
}

#[test]
fn an_invalid_equivocation_proof_does_not_abort_the_batch() {
    let mut case = equivocating();
    let mut machine = BatchMachine::new(case.domain, Timeouts::dark_fba_v0());
    let mut broken = case.proof.clone();
    broken.right = broken.left;
    assert!(machine.present_equivocation(&broken).is_err());
    assert_eq!(machine.phase(), Phase::Open);

    let elsewhere = LogDomain {
        batch: case.domain.batch + 1,
        ..case.domain
    };
    let mut other_machine = BatchMachine::new(elsewhere, Timeouts::dark_fba_v0());
    assert_eq!(
        other_machine.present_equivocation(&case.proof),
        Err(LifecycleError::EquivocationDomainMismatch)
    );
    let _ = &mut case.ledger;
}

#[test]
fn a_refund_is_paid_at_most_once() {
    let (log, _, mut ledger, mut machine) = sealed_fixture();
    machine.tick(AVAILABILITY_DEADLINE + 1);
    let receipt = &receipts(&log, 1)[0];
    let amount = machine
        .claim_refund(&mut ledger, &Entitlement::Included(receipt))
        .expect("first claim");
    assert_eq!(amount, AMOUNTS[0]);
    assert_eq!(
        machine.claim_refund(&mut ledger, &Entitlement::Included(receipt)),
        Err(RefundError::AlreadyRefunded)
    );
    assert_eq!(
        ledger.disposition(&receipt.record.nullifier),
        Some(Disposition::Refunded)
    );
    assert_eq!(
        machine.release_to_settlement(&mut ledger, receipt.record.nullifier),
        Err(RefundError::PhaseNotSettled {
            phase: machine.phase()
        })
    );
    assert!(ledger.conserves());
}

#[test]
fn a_refund_needs_a_receipt_that_verifies() {
    let (log, cutoff, mut ledger, mut machine) = sealed_fixture();
    machine.tick(AVAILABILITY_DEADLINE + 1);
    let mut tampered = receipts(&log, 2)[1].clone();
    tampered.proof.path[0].hash[0] ^= 0x01;
    assert!(verify_receipt(&cutoff, &tampered).is_err());
    assert!(matches!(
        machine.claim_refund(&mut ledger, &Entitlement::Included(&tampered)),
        Err(RefundError::Receipt(_))
    ));
    assert_eq!(ledger.total_refunded(), 0);
}

#[test]
fn a_refund_needs_an_escrowed_nullifier() {
    let (_, _, mut ledger, mut machine) = sealed_fixture();
    let mut empty = ReserveLedger::new();
    machine.tick(SEAL_DEADLINE + 1);
    let _ = &mut ledger;
    assert_eq!(
        machine.claim_refund(
            &mut empty,
            &Entitlement::Escrowed {
                nullifier: request(0).nullifier
            }
        ),
        Err(RefundError::PhaseNotRefundable {
            phase: machine.phase()
        }),
        "the machine had already observed a cutoff root, so this path is closed"
    );
}

#[test]
fn no_refund_is_paid_while_the_batch_is_live() {
    let (log, _, mut ledger, mut machine) = sealed_fixture();
    let receipt = &receipts(&log, 1)[0];
    for phase in ["sealed", "computing"] {
        assert_eq!(
            machine.claim_refund(&mut ledger, &Entitlement::Included(receipt)),
            Err(RefundError::PhaseNotRefundable {
                phase: machine.phase()
            }),
            "phase {phase}"
        );
        if phase == "sealed" {
            make_available(&mut machine, 4, 3);
            machine.begin_compute(CUTOFF + 1).expect("inputs present");
        }
    }
    assert_eq!(ledger.total_outstanding(), ledger.total_escrowed());
}

#[test]
fn no_refund_is_paid_after_settlement() {
    let (log, cutoff, mut ledger, mut machine) = sealed_fixture();
    make_available(&mut machine, 4, 3);
    machine.begin_compute(CUTOFF + 1).expect("inputs present");
    machine
        .deliver_result(cutoff.root, RESULT, CUTOFF + 3)
        .expect("delivery is admissible");
    let receipt = &receipts(&log, 1)[0];
    assert_eq!(
        machine.claim_refund(&mut ledger, &Entitlement::Included(receipt)),
        Err(RefundError::PhaseNotRefundable {
            phase: machine.phase()
        })
    );
}

#[test]
fn the_abort_matrix_is_exactly_this() {
    let matrix: [(AbortClass, &str, bool, bool, Consequence); 7] = [
        (
            AbortClass::CutoffRootWithheld,
            "cutoff-root-withheld",
            false,
            true,
            Consequence::RefundEveryEscrowedSubmission,
        ),
        (
            AbortClass::InputWithheld { seq: 0 },
            "input-withheld",
            false,
            true,
            Consequence::RefundEveryAdmittedRecord,
        ),
        (
            AbortClass::ComputeTimeout { attempts: 1 },
            "compute-timeout",
            true,
            false,
            Consequence::RetryAgainstSameCutoffRoot,
        ),
        (
            AbortClass::ComputeExhausted { attempts: 2 },
            "compute-exhausted",
            false,
            true,
            Consequence::RefundEveryAdmittedRecord,
        ),
        (
            AbortClass::Equivocation {
                verdict_digest: [0u8; 32],
            },
            "equivocation",
            false,
            true,
            Consequence::RefundUnderEitherRepudiatedRoot,
        ),
        (
            AbortClass::ResultUnbound,
            "result-unbound",
            false,
            true,
            Consequence::RefundEveryAdmittedRecord,
        ),
        (
            AbortClass::RelationRefused { class_code: 0 },
            "relation-refused",
            false,
            true,
            Consequence::RefundEveryAdmittedRecord,
        ),
    ];
    for (class, name, retryable, terminal, consequence) in matrix {
        assert_eq!(class.class(), name);
        assert_eq!(class.is_retryable(), retryable, "{name} retryable");
        assert_eq!(class.is_terminal(), terminal, "{name} terminal");
        assert_eq!(class.consequence(), consequence, "{name} consequence");
        assert_eq!(Phase::Aborted(class).name(), name);
        assert_eq!(Phase::Aborted(class).is_terminal(), terminal);
    }
}

#[test]
fn every_abort_class_is_reachable_and_no_terminal_phase_moves_again() {
    let terminals: Vec<Phase> = vec![
        {
            let mut machine = BatchMachine::new(domain(), Timeouts::dark_fba_v0());
            machine.tick(SEAL_DEADLINE + 1)
        },
        {
            let (_, _, _, mut machine) = sealed_fixture();
            machine.tick(AVAILABILITY_DEADLINE + 1)
        },
        {
            let (_, _, _, mut machine) = sealed_fixture();
            make_available(&mut machine, 4, 3);
            machine.begin_compute(CUTOFF + 1).expect("inputs present");
            machine.tick(FIRST_ATTEMPT_DEADLINE + 1);
            machine.resume(FIRST_ATTEMPT_DEADLINE + 1).expect("retry");
            machine.tick(SECOND_ATTEMPT_DEADLINE + 1)
        },
        {
            let case = equivocating();
            let mut machine = BatchMachine::new(case.domain, Timeouts::dark_fba_v0());
            machine
                .present_equivocation(&case.proof)
                .expect("proof verifies")
        },
        {
            let (_, cutoff, _, mut machine) = sealed_fixture();
            make_available(&mut machine, 4, 3);
            machine.begin_compute(CUTOFF + 1).expect("inputs present");
            let mut other = cutoff.root;
            other[31] ^= 0x01;
            machine
                .deliver_result(other, RESULT, CUTOFF + 3)
                .expect("delivery is admissible")
        },
        {
            let (_, cutoff, _, mut machine) = sealed_fixture();
            make_available(&mut machine, 4, 3);
            machine.begin_compute(CUTOFF + 1).expect("inputs present");
            machine
                .deliver_refusal(cutoff.root, REFUSAL_CODE, CUTOFF + 3)
                .expect("delivery is admissible")
        },
        {
            let (_, cutoff, _, mut machine) = sealed_fixture();
            make_available(&mut machine, 4, 3);
            machine.begin_compute(CUTOFF + 1).expect("inputs present");
            machine
                .deliver_result(cutoff.root, RESULT, CUTOFF + 3)
                .expect("delivery is admissible")
        },
    ];
    let names: Vec<&'static str> = terminals.iter().map(Phase::name).collect();
    assert_eq!(
        names,
        vec![
            "cutoff-root-withheld",
            "input-withheld",
            "compute-exhausted",
            "equivocation",
            "result-unbound",
            "relation-refused",
            "settled",
        ]
    );
    assert!(terminals.iter().all(Phase::is_terminal));

    // Rebuild each terminal machine and confirm no event moves it.
    for expected in &terminals {
        let (_, cutoff, _, _) = sealed_fixture();
        let mut machine = BatchMachine::new(domain(), Timeouts::dark_fba_v0());
        drive_to(&mut machine, *expected, cutoff);
        assert_eq!(machine.phase(), *expected);
        assert_eq!(
            machine.observe_cutoff(cutoff, CUTOFF),
            Err(LifecycleError::Terminal { phase: *expected })
        );
        assert_eq!(
            machine.report_availability(0, 4),
            Err(LifecycleError::Terminal { phase: *expected })
        );
        assert_eq!(
            machine.begin_compute(CUTOFF + 1),
            Err(LifecycleError::Terminal { phase: *expected })
        );
        assert_eq!(
            machine.deliver_result(cutoff.root, RESULT, CUTOFF + 3),
            Err(LifecycleError::Terminal { phase: *expected })
        );
        assert_eq!(
            machine.deliver_refusal(cutoff.root, REFUSAL_CODE, CUTOFF + 3),
            Err(LifecycleError::Terminal { phase: *expected })
        );
        assert_eq!(
            machine.resume(CUTOFF + 3),
            Err(LifecycleError::NotResumable { phase: *expected })
        );
        assert_eq!(machine.tick(CUTOFF + 1_000), *expected);
    }
}

/// Force `machine` into `target`, using only the machine's own transitions
/// where a transition exists and the equivocation path otherwise.
fn drive_to(machine: &mut BatchMachine, target: Phase, cutoff: CutoffRoot) {
    match target {
        Phase::Aborted(AbortClass::CutoffRootWithheld) => {
            machine.tick(SEAL_DEADLINE + 1);
        }
        Phase::Aborted(AbortClass::InputWithheld { .. }) => {
            machine.observe_cutoff(cutoff, CUTOFF).expect("on time");
            machine.tick(AVAILABILITY_DEADLINE + 1);
        }
        Phase::Aborted(AbortClass::ComputeExhausted { .. }) => {
            machine.observe_cutoff(cutoff, CUTOFF).expect("on time");
            make_available(machine, 4, 3);
            machine.begin_compute(CUTOFF + 1).expect("inputs present");
            machine.tick(FIRST_ATTEMPT_DEADLINE + 1);
            machine.resume(FIRST_ATTEMPT_DEADLINE + 1).expect("retry");
            machine.tick(SECOND_ATTEMPT_DEADLINE + 1);
        }
        Phase::Aborted(AbortClass::Equivocation { .. }) => {
            let case = equivocating();
            machine
                .present_equivocation(&case.proof)
                .expect("proof verifies");
        }
        Phase::Aborted(AbortClass::ResultUnbound) => {
            machine.observe_cutoff(cutoff, CUTOFF).expect("on time");
            make_available(machine, 4, 3);
            machine.begin_compute(CUTOFF + 1).expect("inputs present");
            let mut other = cutoff.root;
            other[31] ^= 0x01;
            machine
                .deliver_result(other, RESULT, CUTOFF + 3)
                .expect("delivery is admissible");
        }
        Phase::Aborted(AbortClass::RelationRefused { class_code }) => {
            machine.observe_cutoff(cutoff, CUTOFF).expect("on time");
            make_available(machine, 4, 3);
            machine.begin_compute(CUTOFF + 1).expect("inputs present");
            machine
                .deliver_refusal(cutoff.root, class_code, CUTOFF + 3)
                .expect("delivery is admissible");
        }
        Phase::Settled { .. } => {
            machine.observe_cutoff(cutoff, CUTOFF).expect("on time");
            make_available(machine, 4, 3);
            machine.begin_compute(CUTOFF + 1).expect("inputs present");
            machine
                .deliver_result(cutoff.root, RESULT, CUTOFF + 3)
                .expect("delivery is admissible");
        }
        other => panic!("no driver for {other:?}"),
    }
}

#[test]
fn a_ledger_refuses_a_repeated_escrow_and_stays_conserved() {
    let mut ledger = ReserveLedger::new();
    let nullifier = request(0).nullifier;
    assert!(ledger.escrow(nullifier, 100));
    assert!(!ledger.escrow(nullifier, 999));
    assert_eq!(ledger.amount(&nullifier), Some(100));
    assert_eq!(ledger.total_escrowed(), 100);
    assert_eq!(
        ledger.disposition(&nullifier),
        Some(Disposition::Outstanding)
    );
    assert_eq!(ledger.disposition(&[0u8; 32]), None);
    assert!(ledger.conserves());
}

#[test]
fn a_padding_record_is_committed_but_refunds_nothing() {
    let domain = domain();
    let mut log = AdmissionLog::open(domain).expect("domain is valid");
    for tag in 0..2u8 {
        log.admit(&request(tag), CUTOFF).expect("admitted");
    }
    let cutoff = log.seal_padded(CUTOFF).expect("seal at the cutoff");
    let mut ledger = escrow_for(&[0, 1]);
    let mut machine = BatchMachine::new(domain, Timeouts::dark_fba_v0());
    machine
        .observe_cutoff(cutoff, CUTOFF)
        .expect("the root is published on time");
    machine.tick(AVAILABILITY_DEADLINE + 1);

    let total = ledger.total_escrowed();
    for seq in 0..4u32 {
        let receipt = log.receipt(seq).expect("every position is committed");
        assert_eq!(verify_receipt(&cutoff, &receipt), Ok(()));
        let outcome = machine.claim_refund(&mut ledger, &Entitlement::Included(&receipt));
        if receipt.record.is_padding(&domain) {
            assert_eq!(
                outcome,
                Err(RefundError::NotEscrowed),
                "padding at {seq} has no reservation behind it"
            );
        } else {
            assert!(outcome.is_ok(), "real record at {seq} refunds");
        }
    }
    assert_eq!(ledger.total_refunded(), total);
    assert_eq!(ledger.total_outstanding(), 0);
    assert!(ledger.conserves());
}

#[test]
fn a_second_cutoff_root_is_not_observed() {
    let (_, cutoff, _, mut machine) = sealed_fixture();
    assert_eq!(
        machine.observe_cutoff(cutoff, CUTOFF),
        Err(LifecycleError::PhaseForbids {
            phase: Phase::Sealed
        })
    );
    make_available(&mut machine, 4, 3);
    machine.begin_compute(CUTOFF + 1).expect("inputs present");
    assert_eq!(
        machine.report_availability(0, 4),
        Err(LifecycleError::PhaseForbids {
            phase: Phase::Computing
        })
    );
    assert_eq!(
        machine.begin_compute(CUTOFF + 1),
        Err(LifecycleError::PhaseForbids {
            phase: Phase::Computing
        })
    );
}

#[test]
fn a_cutoff_root_from_another_log_is_not_observed() {
    let elsewhere = LogDomain {
        market: domain().market + 1,
        ..domain()
    };
    let (_, other) = sealed_log(elsewhere, 2);
    let mut machine = BatchMachine::new(domain(), Timeouts::dark_fba_v0());
    assert_eq!(
        machine.observe_cutoff(other, CUTOFF),
        Err(LifecycleError::CutoffDomainMismatch)
    );
    assert_eq!(machine.phase(), Phase::Open);
}

#[test]
fn a_cutoff_root_claiming_more_than_its_capacity_is_not_observed() {
    let (_, cutoff) = sealed_log(domain(), 2);
    let malformed = CutoffRoot {
        leaf_count: u64::from(domain().capacity) + 1,
        ..cutoff
    };
    let mut machine = BatchMachine::new(domain(), Timeouts::dark_fba_v0());
    assert_eq!(
        machine.observe_cutoff(malformed, CUTOFF),
        Err(LifecycleError::CutoffRootMalformed)
    );
    assert_eq!(machine.phase(), Phase::Open);
}

#[test]
fn an_unverifiable_equivocation_names_its_own_defect() {
    let case = equivocating();
    let mut machine = BatchMachine::new(case.domain, Timeouts::dark_fba_v0());
    let mut broken = case.proof.clone();
    broken.right = broken.left;
    assert!(matches!(
        machine.present_equivocation(&broken),
        Err(LifecycleError::EquivocationInvalid(_))
    ));
    assert_eq!(machine.phase(), Phase::Open);
}

#[test]
fn a_reservation_is_released_to_settlement_at_most_once() {
    let (log, cutoff, mut ledger, mut machine) = sealed_fixture();
    make_available(&mut machine, 4, 3);
    machine.begin_compute(CUTOFF + 1).expect("inputs present");
    machine
        .deliver_result(cutoff.root, RESULT, CUTOFF + 3)
        .expect("delivery is admissible");
    let nullifier = receipts(&log, 1)[0].record.nullifier;
    assert_eq!(
        machine
            .release_to_settlement(&mut ledger, nullifier)
            .expect("first release"),
        AMOUNTS[0]
    );
    assert_eq!(
        machine.release_to_settlement(&mut ledger, nullifier),
        Err(RefundError::AlreadySettled)
    );
    assert!(ledger.conserves());
}
