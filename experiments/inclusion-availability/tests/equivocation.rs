//! Equivocation: a holder that tells two stories about one cutoff produces a
//! transferable object, and a holder that tells one story never does.

mod common;

use common::{CUTOFF, ack_statement, domain, harness_domain, holder, request, seal_statement};
use degg_inclusion_availability::equivocation::{
    Conflict, EquivocationDefect, EquivocationProof, HolderId, RootStatement, StatementKind,
    verify_equivocation,
};
use degg_inclusion_availability::log::{
    AdmissionLog, CutoffRoot, DomainDefect, InclusionReceipt, LogDomain,
};
use degg_inclusion_availability::mmr::ConsistencyProof;

/// Build and seal a log admitting exactly `tags`, in that order.
fn log_of(domain: LogDomain, tags: &[u8]) -> (AdmissionLog, CutoffRoot) {
    let mut log = AdmissionLog::open(domain).expect("domain is valid");
    for tag in tags {
        log.admit(&request(*tag), CUTOFF).expect("admitted");
    }
    let cutoff = log.seal(CUTOFF).expect("seal at the cutoff");
    (log, cutoff)
}

/// Two divergent logs over one domain, plus the receipts at `seq` under each.
struct Fork {
    domain: LogDomain,
    left: CutoffRoot,
    right: CutoffRoot,
    left_log: AdmissionLog,
    right_log: AdmissionLog,
}

impl Fork {
    fn build(left_tags: &[u8], right_tags: &[u8]) -> Self {
        let domain = domain();
        let (left_log, left) = log_of(domain, left_tags);
        let (right_log, right) = log_of(domain, right_tags);
        assert_ne!(left.root, right.root, "the fork must actually diverge");
        Self {
            domain,
            left,
            right,
            left_log,
            right_log,
        }
    }

    fn receipts(&self, left_seq: u32, right_seq: u32) -> (InclusionReceipt, InclusionReceipt) {
        (
            self.left_log.receipt(left_seq).expect("record is admitted"),
            self.right_log
                .receipt(right_seq)
                .expect("record is admitted"),
        )
    }

    fn proof(&self, conflict: Conflict) -> EquivocationProof {
        EquivocationProof {
            domain: self.domain,
            left: seal_statement(&self.left),
            right: seal_statement(&self.right),
            conflict,
        }
    }
}

#[test]
fn two_sealed_roots_for_one_cutoff_are_an_equivocation() {
    let fork = Fork::build(&[0, 1, 2], &[0, 9, 2]);
    let verdict = verify_equivocation(&fork.proof(Conflict::Roots)).expect("proof verifies");
    assert_eq!(verdict.class, "conflicting-sealed-roots");
    assert_eq!(verdict.holder, holder());
    assert_eq!(verdict.domain_digest, fork.domain.digest());
    assert_eq!(verdict.left_root, fork.left.root);
    assert_eq!(verdict.right_root, fork.right.root);
}

#[test]
fn two_records_at_one_position_are_an_equivocation() {
    let fork = Fork::build(&[0, 1, 2], &[0, 9, 2]);
    let (left, right) = fork.receipts(1, 1);
    assert_ne!(left.record, right.record);
    let verdict = verify_equivocation(&fork.proof(Conflict::Sequence { left, right }))
        .expect("proof verifies");
    assert_eq!(verdict.class, "conflicting-record-at-sequence");
}

#[test]
fn one_nullifier_at_two_positions_is_an_equivocation() {
    let fork = Fork::build(&[0, 1, 2], &[1, 0, 2]);
    let (left, right) = fork.receipts(1, 0);
    assert_eq!(left.record.nullifier, right.record.nullifier);
    assert_ne!(left.record.seq, right.record.seq);
    let verdict = verify_equivocation(&fork.proof(Conflict::Position { left, right }))
        .expect("proof verifies");
    assert_eq!(verdict.class, "nullifier-at-two-positions");
}

#[test]
fn an_abandoned_acknowledged_prefix_is_an_equivocation() {
    let domain = harness_domain(16);
    // The holder acknowledges two records, then seals a log that does not
    // extend what it acknowledged.
    let mut honest = AdmissionLog::open(domain).expect("domain is valid");
    honest.admit(&request(0), CUTOFF).expect("admitted");
    let ack = honest.admit(&request(1), CUTOFF).expect("admitted");

    let (rolled_back, sealed) = log_of(domain, &[0, 9, 2]);
    let consistency = rolled_back
        .consistency_proof(2)
        .expect("prefix is within the log");

    let proof = EquivocationProof {
        domain,
        left: ack_statement(&domain, ack.running_leaf_count, ack.running_root),
        right: seal_statement(&sealed),
        conflict: Conflict::Prefix { consistency },
    };
    let verdict = verify_equivocation(&proof).expect("proof verifies");
    assert_eq!(verdict.class, "acknowledged-prefix-abandoned");
    assert_eq!(verdict.left_root, ack.running_root);
    assert_eq!(verdict.right_root, sealed.root);
}

#[test]
fn a_verdict_digest_is_stable_and_class_separated() {
    let fork = Fork::build(&[0, 1, 2], &[0, 9, 2]);
    let (left, right) = fork.receipts(1, 1);
    let roots = verify_equivocation(&fork.proof(Conflict::Roots)).expect("verifies");
    let sequence =
        verify_equivocation(&fork.proof(Conflict::Sequence { left, right })).expect("verifies");
    assert_ne!(roots.digest, sequence.digest);
    assert_eq!(
        roots.digest,
        verify_equivocation(&fork.proof(Conflict::Roots))
            .expect("verifies")
            .digest
    );

    let other_holder = EquivocationProof {
        left: RootStatement::new(
            HolderId([0x01; 32]),
            fork.domain.digest(),
            StatementKind::Sealed,
            fork.left.leaf_count,
            fork.left.root,
        ),
        right: RootStatement::new(
            HolderId([0x01; 32]),
            fork.domain.digest(),
            StatementKind::Sealed,
            fork.right.leaf_count,
            fork.right.root,
        ),
        ..fork.proof(Conflict::Roots)
    };
    assert_ne!(
        verify_equivocation(&other_holder).expect("verifies").digest,
        roots.digest,
        "the verdict names a holder"
    );
}

#[test]
fn honest_single_root_operation_never_yields_an_equivocation() {
    let domain = domain();
    let mut log = AdmissionLog::open(domain).expect("domain is valid");
    let mut acks = vec![(0u64, log.running_root())];
    for tag in 0..4u8 {
        let ack = log.admit(&request(tag), CUTOFF).expect("admitted");
        acks.push((ack.running_leaf_count, ack.running_root));
    }
    let cutoff = log.seal(CUTOFF).expect("seal at the cutoff");
    let sealed = seal_statement(&cutoff);
    let receipts: Vec<InclusionReceipt> = (0..4)
        .map(|seq| log.receipt(seq).expect("record is admitted"))
        .collect();

    // Every ordered pair of receipts, in both conflict shapes that take a pair.
    let mut attempts = 0usize;
    for left in &receipts {
        for right in &receipts {
            for conflict in [
                Conflict::Sequence {
                    left: left.clone(),
                    right: right.clone(),
                },
                Conflict::Position {
                    left: left.clone(),
                    right: right.clone(),
                },
            ] {
                let proof = EquivocationProof {
                    domain,
                    left: sealed,
                    right: sealed,
                    conflict,
                };
                assert_eq!(
                    verify_equivocation(&proof),
                    Err(EquivocationDefect::RootsIdentical)
                );
                attempts += 1;
            }
        }
    }
    assert_eq!(attempts, 32);

    // Every acknowledged prefix against the holder's own cutoff root.
    for (leaf_count, root) in &acks {
        let ack = ack_statement(&domain, *leaf_count, *root);
        let consistency = log
            .consistency_proof(*leaf_count)
            .expect("prefix is within the log");
        let proof = EquivocationProof {
            domain,
            left: ack,
            right: sealed,
            conflict: Conflict::Prefix { consistency },
        };
        let expected = if *root == cutoff.root {
            EquivocationDefect::RootsIdentical
        } else {
            EquivocationDefect::PrefixAgrees
        };
        assert_eq!(
            verify_equivocation(&proof),
            Err(expected),
            "at {leaf_count}"
        );
    }

    // And the bare root conflict, which is what a naive detector would use.
    assert_eq!(
        verify_equivocation(&EquivocationProof {
            domain,
            left: sealed,
            right: sealed,
            conflict: Conflict::Roots,
        }),
        Err(EquivocationDefect::RootsIdentical)
    );
}

#[test]
fn a_running_root_is_not_evidence_of_a_second_cutoff() {
    let domain = harness_domain(16);
    let mut log = AdmissionLog::open(domain).expect("domain is valid");
    let first = log.admit(&request(0), CUTOFF).expect("admitted");
    log.admit(&request(1), CUTOFF).expect("admitted");
    let cutoff = log.seal(CUTOFF).expect("seal at the cutoff");

    // The two roots genuinely differ; the holder is still honest.
    assert_ne!(first.running_root, cutoff.root);
    let proof = EquivocationProof {
        domain,
        left: ack_statement(&domain, first.running_leaf_count, first.running_root),
        right: seal_statement(&cutoff),
        conflict: Conflict::Roots,
    };
    assert_eq!(
        verify_equivocation(&proof),
        Err(EquivocationDefect::NotSealed),
        "growing a log is not equivocating about its cutoff"
    );
}

#[test]
fn a_malformed_statement_is_refused() {
    let fork = Fork::build(&[0, 1, 2], &[0, 9, 2]);
    let mut proof = fork.proof(Conflict::Roots);
    proof.left.binding[0] ^= 0x01;
    assert_eq!(
        verify_equivocation(&proof),
        Err(EquivocationDefect::MalformedStatement)
    );

    let mut swapped = fork.proof(Conflict::Roots);
    swapped.left.root = fork.right.root;
    assert_eq!(
        verify_equivocation(&swapped),
        Err(EquivocationDefect::MalformedStatement),
        "a statement rewritten without rebinding is malformed"
    );
}

#[test]
fn statements_about_another_log_are_refused() {
    let fork = Fork::build(&[0, 1, 2], &[0, 9, 2]);
    let elsewhere = LogDomain {
        batch: fork.domain.batch + 1,
        ..fork.domain
    };
    let mut proof = fork.proof(Conflict::Roots);
    proof.left = RootStatement::new(
        holder(),
        elsewhere.digest(),
        StatementKind::Sealed,
        fork.left.leaf_count,
        fork.left.root,
    );
    assert_eq!(
        verify_equivocation(&proof),
        Err(EquivocationDefect::DomainMismatch)
    );
}

#[test]
fn statements_by_different_holders_are_refused() {
    let fork = Fork::build(&[0, 1, 2], &[0, 9, 2]);
    let mut proof = fork.proof(Conflict::Roots);
    proof.right = RootStatement::new(
        HolderId([0x77; 32]),
        fork.domain.digest(),
        StatementKind::Sealed,
        fork.right.leaf_count,
        fork.right.root,
    );
    assert_eq!(
        verify_equivocation(&proof),
        Err(EquivocationDefect::DifferentHolders),
        "two holders disagreeing is not one holder equivocating"
    );
}

#[test]
fn an_invalid_domain_is_refused() {
    let fork = Fork::build(&[0, 1, 2], &[0, 9, 2]);
    let mut proof = fork.proof(Conflict::Roots);
    proof.domain = LogDomain {
        capacity: 0,
        ..fork.domain
    };
    assert_eq!(
        verify_equivocation(&proof),
        Err(EquivocationDefect::Domain(DomainDefect::ZeroCapacity))
    );
}

#[test]
fn a_receipt_that_does_not_verify_against_its_own_root_is_refused() {
    let fork = Fork::build(&[0, 1, 2], &[0, 9, 2]);
    let (left, right) = fork.receipts(1, 1);
    // Both receipts are taken against the left root, so the right one fails.
    let stolen = fork.left_log.receipt(2).expect("record is admitted");
    let proof = fork.proof(Conflict::Sequence {
        left,
        right: stolen,
    });
    assert!(matches!(
        verify_equivocation(&proof),
        Err(EquivocationDefect::Receipt(_))
    ));
    drop(right);
}

#[test]
fn receipts_at_different_positions_are_not_a_sequence_conflict() {
    let fork = Fork::build(&[0, 1, 2], &[0, 9, 2]);
    let (left, right) = fork.receipts(1, 2);
    assert_eq!(
        verify_equivocation(&fork.proof(Conflict::Sequence { left, right })),
        Err(EquivocationDefect::NotSameSequence { left: 1, right: 2 })
    );
}

#[test]
fn identical_records_at_one_position_are_not_a_sequence_conflict() {
    let fork = Fork::build(&[0, 1, 2], &[0, 9, 2]);
    let (left, right) = fork.receipts(0, 0);
    assert_eq!(left.record, right.record);
    assert_eq!(
        verify_equivocation(&fork.proof(Conflict::Sequence { left, right })),
        Err(EquivocationDefect::RecordsIdentical),
        "the two logs agreeing about position 0 is not a contradiction"
    );
}

#[test]
fn different_nullifiers_are_not_a_position_conflict() {
    let fork = Fork::build(&[0, 1, 2], &[0, 9, 2]);
    let (left, right) = fork.receipts(1, 1);
    assert_eq!(
        verify_equivocation(&fork.proof(Conflict::Position { left, right })),
        Err(EquivocationDefect::NotSameNullifier)
    );
}

#[test]
fn one_nullifier_at_one_position_is_not_a_position_conflict() {
    let fork = Fork::build(&[0, 1, 2], &[0, 9, 2]);
    let (left, right) = fork.receipts(0, 0);
    assert_eq!(
        verify_equivocation(&fork.proof(Conflict::Position { left, right })),
        Err(EquivocationDefect::SamePosition { seq: 0 })
    );
}

#[test]
fn a_prefix_conflict_needs_an_acknowledgement_and_a_seal() {
    let fork = Fork::build(&[0, 1, 2], &[0, 9, 2]);
    let consistency = fork
        .right_log
        .consistency_proof(2)
        .expect("prefix is within the log");
    assert_eq!(
        verify_equivocation(&fork.proof(Conflict::Prefix {
            consistency: consistency.clone()
        })),
        Err(EquivocationDefect::NotAck),
        "a sealed root is not an acknowledgement"
    );

    let both_acks = EquivocationProof {
        domain: fork.domain,
        left: ack_statement(&fork.domain, fork.left.leaf_count, fork.left.root),
        right: ack_statement(&fork.domain, fork.right.leaf_count, fork.right.root),
        conflict: Conflict::Prefix { consistency },
    };
    assert_eq!(
        verify_equivocation(&both_acks),
        Err(EquivocationDefect::NotSealed)
    );
}

#[test]
fn a_prefix_proof_at_the_wrong_size_is_refused() {
    let fork = Fork::build(&[0, 1, 2], &[0, 9, 2]);
    let consistency = fork
        .right_log
        .consistency_proof(1)
        .expect("prefix is within the log");
    let proof = EquivocationProof {
        domain: fork.domain,
        left: ack_statement(&fork.domain, 2, fork.left_log.root_at(2)),
        right: seal_statement(&fork.right),
        conflict: Conflict::Prefix { consistency },
    };
    assert_eq!(
        verify_equivocation(&proof),
        Err(EquivocationDefect::PrefixSizeMismatch {
            acknowledged: 2,
            proved: 1
        })
    );
}

#[test]
fn a_tampered_prefix_proof_is_refused() {
    let fork = Fork::build(&[0, 1, 2], &[0, 9, 2]);
    let mut consistency = fork
        .right_log
        .consistency_proof(2)
        .expect("prefix is within the log");
    consistency.prefix_peaks[0].hash[0] ^= 0x01;
    let proof = EquivocationProof {
        domain: fork.domain,
        left: ack_statement(&fork.domain, 2, fork.left_log.root_at(2)),
        right: seal_statement(&fork.right),
        conflict: Conflict::Prefix { consistency },
    };
    assert!(matches!(
        verify_equivocation(&proof),
        Err(EquivocationDefect::Consistency(_))
    ));

    let empty = EquivocationProof {
        domain: fork.domain,
        left: ack_statement(&fork.domain, 2, fork.left_log.root_at(2)),
        right: seal_statement(&fork.right),
        conflict: Conflict::Prefix {
            consistency: ConsistencyProof {
                prefix_leaf_count: 2,
                prefix_peaks: Vec::new(),
            },
        },
    };
    assert!(matches!(
        verify_equivocation(&empty),
        Err(EquivocationDefect::Consistency(_))
    ));
}
