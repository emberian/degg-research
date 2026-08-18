//! Inclusion receipts: verified against a root alone, and refused whenever the
//! log holder is the only thing vouching for them.

mod common;

use common::{CUTOFF, domain, harness_domain, log_with, request, sealed_log};
use degg_inclusion_availability::log::{
    AdmissionLog, CutoffRoot, DARK_FBA_CAPACITY, DomainDefect, InclusionReceipt, LogDomain,
    ReceiptDefect, verify_receipt,
};
use degg_inclusion_availability::mmr::{ProofDefect, Side, verify_consistency};

/// One named edit to an otherwise valid receipt.
type Mutation = (&'static str, fn(&mut InclusionReceipt));

fn receipts(log: &AdmissionLog, count: u32) -> Vec<InclusionReceipt> {
    (0..count)
        .map(|seq| log.receipt(seq).expect("record is admitted and sealed"))
        .collect()
}

#[test]
fn every_admitted_record_has_a_receipt_that_verifies_against_the_root_alone() {
    for count in 0..=DARK_FBA_CAPACITY {
        let (log, cutoff) = sealed_log(domain(), count);
        for seq in 0..count {
            let receipt = log.receipt(seq).expect("record is admitted");
            assert_eq!(receipt.record.seq, seq);
            assert_eq!(
                verify_receipt(&cutoff, &receipt),
                Ok(()),
                "count {count} seq {seq}"
            );
        }
        assert!(log.receipt(count).is_none(), "no receipt past the log");
    }
}

#[test]
fn receipts_are_available_only_after_the_cutoff() {
    let log = log_with(domain(), 3);
    assert!(log.cutoff().is_none());
    for seq in 0..3 {
        assert!(
            log.receipt(seq).is_none(),
            "there is no committed set to prove membership in before the cutoff"
        );
    }
}

#[test]
fn a_receipt_does_not_verify_against_another_batchs_root() {
    let (log, cutoff) = sealed_log(domain(), 3);
    let receipt = log.receipt(1).expect("record is admitted");
    let other = LogDomain {
        batch: cutoff.domain.batch + 1,
        ..cutoff.domain
    };
    let (_, other_cutoff) = sealed_log(other, 3);
    assert_eq!(
        verify_receipt(&other_cutoff, &receipt),
        Err(ReceiptDefect::Proof(ProofDefect::RootMismatch))
    );
}

#[test]
fn a_receipt_does_not_verify_against_a_root_from_a_different_admitted_set() {
    let (log, _) = sealed_log(domain(), 3);
    let receipt = log.receipt(1).expect("record is admitted");

    let mut reordered = AdmissionLog::open(domain()).expect("domain is valid");
    for tag in [2u8, 1, 0] {
        reordered.admit(&request(tag), CUTOFF).expect("admitted");
    }
    let reordered = reordered.seal(CUTOFF).expect("seal at the cutoff");
    assert_eq!(
        verify_receipt(&reordered, &receipt),
        Err(ReceiptDefect::Proof(ProofDefect::RootMismatch)),
        "the same records in a different order do not honour the old receipt"
    );
}

#[test]
fn a_receipt_with_a_tampered_root_is_refused() {
    let (log, cutoff) = sealed_log(domain(), 4);
    let receipt = log.receipt(2).expect("record is admitted");
    for byte in [0usize, 15, 31] {
        let mut tampered = cutoff;
        tampered.root[byte] ^= 0x01;
        assert_eq!(
            verify_receipt(&tampered, &receipt),
            Err(ReceiptDefect::Proof(ProofDefect::RootMismatch))
        );
    }
}

#[test]
fn a_receipt_with_a_tampered_leaf_count_is_refused() {
    let (log, cutoff) = sealed_log(domain(), 3);
    let receipt = log.receipt(1).expect("record is admitted");
    let tampered = CutoffRoot {
        leaf_count: 2,
        ..cutoff
    };
    assert_eq!(
        verify_receipt(&tampered, &receipt),
        Err(ReceiptDefect::Proof(ProofDefect::LeafCountMismatch {
            implied: 3,
            stated: 2
        }))
    );
}

#[test]
fn a_receipt_claiming_another_position_is_refused() {
    let (log, cutoff) = sealed_log(domain(), 4);
    for seq in 0..4u32 {
        let honest = log.receipt(seq).expect("record is admitted");
        for other in 0..4u32 {
            if other == seq {
                continue;
            }
            // The record is moved to another position while keeping the proof.
            let mut moved = honest.clone();
            moved.record.seq = other;
            assert!(
                matches!(
                    verify_receipt(&cutoff, &moved),
                    Err(ReceiptDefect::Proof(ProofDefect::RootMismatch))
                ),
                "record {seq} accepted at position {other}"
            );
            // The proof for another position is grafted onto this record.
            let mut grafted = honest.clone();
            grafted.proof = log
                .receipt(other)
                .expect("record is admitted")
                .proof
                .clone();
            assert!(
                matches!(
                    verify_receipt(&cutoff, &grafted),
                    Err(ReceiptDefect::Proof(ProofDefect::RootMismatch))
                ),
                "position {other}'s proof accepted for record {seq}"
            );
        }
    }
}

#[test]
fn a_receipt_with_a_tampered_sibling_is_refused() {
    let (log, cutoff) = sealed_log(domain(), 4);
    let receipt = log.receipt(1).expect("record is admitted");
    assert!(!receipt.proof.path.is_empty());
    for step in 0..receipt.proof.path.len() {
        for byte in [0usize, 31] {
            let mut tampered = receipt.clone();
            tampered.proof.path[step].hash[byte] ^= 0x01;
            assert_eq!(
                verify_receipt(&cutoff, &tampered),
                Err(ReceiptDefect::Proof(ProofDefect::RootMismatch))
            );
        }
        let mut flipped = receipt.clone();
        flipped.proof.path[step].side = match flipped.proof.path[step].side {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        };
        assert!(matches!(
            verify_receipt(&cutoff, &flipped),
            Err(ReceiptDefect::Proof(_) | ReceiptDefect::SequenceMismatch { .. })
        ));
    }
}

#[test]
fn a_receipt_with_a_tampered_record_field_is_refused() {
    let (log, cutoff) = sealed_log(domain(), 4);
    let receipt = log.receipt(2).expect("record is admitted");
    let mutations: Vec<Mutation> = vec![
        ("submitter", |r| r.record.submitter[0] ^= 0x01),
        ("payload commitment", |r| {
            r.record.payload_commitment[31] ^= 0x01
        }),
        ("arrival epoch", |r| r.record.arrival_epoch -= 1),
        ("nullifier", |r| r.record.nullifier[7] ^= 0x01),
    ];
    for (name, mutate) in mutations {
        let mut tampered = receipt.clone();
        mutate(&mut tampered);
        assert_eq!(
            verify_receipt(&cutoff, &tampered),
            Err(ReceiptDefect::Proof(ProofDefect::RootMismatch)),
            "case: {name}"
        );
    }
}

#[test]
fn a_record_that_violates_the_domain_is_refused_before_the_proof_is_read() {
    let (log, cutoff) = sealed_log(domain(), 4);
    let receipt = log.receipt(2).expect("record is admitted");
    let mutations: Vec<Mutation> = vec![
        ("payload length", |r| r.record.payload_len += 1),
        ("availability shares", |r| r.record.availability_shares += 1),
        ("arrival after the cutoff", |r| {
            r.record.arrival_epoch = CUTOFF + 1
        }),
        ("zero nullifier", |r| r.record.nullifier = [0u8; 32]),
        ("position past capacity", |r| {
            r.record.seq = DARK_FBA_CAPACITY
        }),
    ];
    for (name, mutate) in mutations {
        let mut tampered = receipt.clone();
        mutate(&mut tampered);
        assert_eq!(
            verify_receipt(&cutoff, &tampered),
            Err(ReceiptDefect::RecordViolatesDomain),
            "case: {name}"
        );
    }
}

#[test]
fn an_interior_node_proof_is_not_an_inclusion_receipt() {
    let (log, cutoff) = sealed_log(domain(), 4);
    let mut receipt = log.receipt(0).expect("record is admitted");
    receipt.proof.height = 1;
    assert_eq!(
        verify_receipt(&cutoff, &receipt),
        Err(ReceiptDefect::NotALeaf { height: 1 })
    );
}

#[test]
fn a_cutoff_root_admitting_more_records_than_its_capacity_is_refused() {
    let (log, cutoff) = sealed_log(domain(), 4);
    let receipt = log.receipt(0).expect("record is admitted");
    let malformed = CutoffRoot {
        leaf_count: u64::from(DARK_FBA_CAPACITY) + 1,
        ..cutoff
    };
    assert_eq!(
        verify_receipt(&malformed, &receipt),
        Err(ReceiptDefect::MalformedCutoffRoot)
    );
}

#[test]
fn a_post_cutoff_append_cannot_produce_a_receipt_against_the_cutoff_root() {
    let domain = harness_domain(16);
    let (mut log, cutoff) = sealed_log(domain, 5);
    let honest = receipts(&log, 5);

    for tag in 100u8..103 {
        log.adversarially_append_after_cutoff(&request(tag));
    }
    assert_eq!(log.records().len(), 8);
    assert_eq!(
        log.cutoff(),
        Some(cutoff),
        "the published root does not move"
    );

    // Every honest receipt still verifies: the log is append-only.
    for receipt in &honest {
        assert_eq!(verify_receipt(&cutoff, receipt), Ok(()));
    }

    // A record appended after the cutoff has a receipt against the *running*
    // root and none against the cutoff root.
    let extended = CutoffRoot {
        domain,
        leaf_count: 8,
        root: log.running_root(),
    };
    for seq in 5..8u32 {
        let late = log.running_receipt(seq).expect("record is in the log");
        assert_eq!(
            verify_receipt(&extended, &late),
            Ok(()),
            "the extended root does contain the late record"
        );
        assert!(
            matches!(
                verify_receipt(&cutoff, &late),
                Err(ReceiptDefect::Proof(ProofDefect::LeafCountMismatch { .. }))
            ),
            "the cutoff root must not admit the late record at seq {seq}"
        );
    }

    // And the extension is provably an extension, not a rewrite: the cutoff
    // root is exactly the five-leaf prefix of the extended log.
    let consistency = log.consistency_proof(5).expect("prefix is within the log");
    let derived = verify_consistency(&domain.digest(), &extended.root, 8, &consistency)
        .expect("consistency proof verifies");
    assert_eq!(derived, cutoff.root);
}

#[test]
fn a_receipt_taken_against_a_running_root_does_not_verify_against_the_cutoff_root() {
    let domain = harness_domain(16);
    let mut log = AdmissionLog::open(domain).expect("domain is valid");
    for tag in 0..3u8 {
        log.admit(&request(tag), CUTOFF).expect("admitted");
    }
    let early = log.running_receipt(0).expect("record is in the log");
    let early_root = CutoffRoot {
        domain,
        leaf_count: 3,
        root: log.running_root(),
    };
    assert_eq!(verify_receipt(&early_root, &early), Ok(()));

    for tag in 3..6u8 {
        log.admit(&request(tag), CUTOFF).expect("admitted");
    }
    let cutoff = log.seal(CUTOFF).expect("seal at the cutoff");
    assert!(
        matches!(
            verify_receipt(&cutoff, &early),
            Err(ReceiptDefect::Proof(ProofDefect::LeafCountMismatch { .. }))
        ),
        "a three-leaf proof must not be reusable against a six-leaf root"
    );
    // The holder simply reissues the receipt against the cutoff root.
    let reissued = log.receipt(0).expect("record is admitted");
    assert_eq!(verify_receipt(&cutoff, &reissued), Ok(()));
    assert_eq!(reissued.record, early.record);
}

#[test]
fn receipts_are_verified_without_the_log() {
    // The verifier is handed exactly two values and nothing else.
    let (log, cutoff) = sealed_log(harness_domain(16), 11);
    let carried: Vec<InclusionReceipt> = receipts(&log, 11);
    drop(log);
    for receipt in &carried {
        assert_eq!(verify_receipt(&cutoff, receipt), Ok(()));
    }
}

#[test]
fn a_padded_cutoff_root_proves_every_position_including_padding() {
    for real in 0..=DARK_FBA_CAPACITY {
        let domain = domain();
        let mut log = log_with(domain, real);
        let cutoff = log.seal_padded(CUTOFF).expect("seal at the cutoff");
        assert_eq!(cutoff.leaf_count, u64::from(DARK_FBA_CAPACITY));
        for seq in 0..DARK_FBA_CAPACITY {
            let receipt = log.receipt(seq).expect("every position is committed");
            assert_eq!(
                verify_receipt(&cutoff, &receipt),
                Ok(()),
                "real {real} seq {seq}"
            );
            assert_eq!(receipt.record.is_padding(&domain), seq >= real);
        }
    }
}

#[test]
fn padding_is_recognisable_which_is_the_limit_of_this_model() {
    // Occupancy is hidden from the leaf count and from nothing else. A relying
    // party can read it straight off the committed records, because this model
    // has no hiding payload commitment. Asserting it keeps the limit visible.
    let domain = domain();
    let mut log = log_with(domain, 1);
    let cutoff = log.seal_padded(CUTOFF).expect("seal at the cutoff");
    let occupied = (0..DARK_FBA_CAPACITY)
        .filter(|seq| {
            let receipt = log.receipt(*seq).expect("every position is committed");
            assert_eq!(verify_receipt(&cutoff, &receipt), Ok(()));
            !receipt.record.is_padding(&domain)
        })
        .count();
    assert_eq!(occupied, 1, "the record bytes still disclose occupancy");
}

#[test]
fn a_padded_power_of_two_capacity_gives_every_receipt_the_same_shape() {
    // The disclosure budget forbids variable proof sizes from carrying
    // information. When the capacity is a power of two and the log is padded to
    // it, the mountain range is a single perfect tree, so every receipt has the
    // same path length and no peaks: the receipt shape is a constant.
    assert!(DARK_FBA_CAPACITY.is_power_of_two());
    let mut log = log_with(domain(), 1);
    let cutoff = log.seal_padded(CUTOFF).expect("seal at the cutoff");
    let shapes: Vec<(usize, usize, usize)> = (0..DARK_FBA_CAPACITY)
        .map(|seq| {
            let receipt = log.receipt(seq).expect("every position is committed");
            assert_eq!(verify_receipt(&cutoff, &receipt), Ok(()));
            (
                receipt.proof.path.len(),
                receipt.proof.left_peaks.len(),
                receipt.proof.right_peaks.len(),
            )
        })
        .collect();
    assert_eq!(shapes, vec![(2, 0, 0); DARK_FBA_CAPACITY as usize]);

    // A capacity that is not a power of two does not have this property, which
    // is why it is a constraint on the relation and not an accident.
    let odd = LogDomain {
        capacity: 3,
        ..domain()
    };
    let mut odd_log = log_with(odd, 1);
    odd_log.seal_padded(CUTOFF).expect("seal at the cutoff");
    let odd_shapes: Vec<usize> = (0..3)
        .map(|seq| {
            odd_log
                .receipt(seq)
                .expect("every position is committed")
                .proof
                .path
                .len()
        })
        .collect();
    assert_eq!(odd_shapes, vec![1, 1, 0]);
}

#[test]
fn a_cutoff_root_with_an_invalid_domain_is_refused() {
    let (log, cutoff) = sealed_log(domain(), 2);
    let receipt = log.receipt(0).expect("record is admitted");
    let broken = CutoffRoot {
        domain: LogDomain {
            availability_threshold: 0,
            ..cutoff.domain
        },
        ..cutoff
    };
    assert_eq!(
        verify_receipt(&broken, &receipt),
        Err(ReceiptDefect::Domain(DomainDefect::ThresholdOutOfRange))
    );
}

#[test]
fn the_sequence_mismatch_defect_is_defence_in_depth() {
    // `ReceiptDefect::SequenceMismatch` cannot be produced by any witness,
    // because the canonical position is inside the leaf preimage: a record
    // whose `seq` disagrees with the position the proof determines has a
    // different leaf hash, so the root check fails first. The check stays as a
    // guard against a future encoding that drops `seq` from the preimage, and
    // this test records the argument rather than leaving a class that looks
    // reachable and is not.
    let (log, cutoff) = sealed_log(domain(), 4);
    for seq in 0..4u32 {
        for other in 0..4u32 {
            if seq == other {
                continue;
            }
            let mut moved = log.receipt(seq).expect("record is admitted");
            moved.record.seq = other;
            assert_eq!(
                verify_receipt(&cutoff, &moved),
                Err(ReceiptDefect::Proof(ProofDefect::RootMismatch)),
                "the leaf, not the receipt envelope, carries the position"
            );
        }
    }
}
