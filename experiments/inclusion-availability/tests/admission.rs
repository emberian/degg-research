//! Admission: what the log takes, in what order, and what the cutoff freezes.

mod common;

use common::{CUTOFF, domain, harness_domain, log_with, request, sealed_log};
/// Holder clock used by the fault cases; strictly between the fixture arrival
/// epoch and the cutoff.
const NOW: u64 = CUTOFF - 5;

use degg_inclusion_availability::log::{
    AdmissionLog, AdmissionRecord, AdmissionRefusal, AdmissionRequest,
    DARK_FBA_AVAILABILITY_SHARES, DARK_FBA_CAPACITY, DARK_FBA_PAYLOAD_LEN, DomainDefect, LogDomain,
    RECORD_BYTES, SealRefusal,
};

#[test]
fn a_canonical_request_is_admitted_at_the_next_position() {
    let domain = domain();
    let mut log = AdmissionLog::open(domain).expect("domain is valid");
    for expected in 0..DARK_FBA_CAPACITY {
        let ack = log
            .admit(&request(u8::try_from(expected).unwrap()), CUTOFF)
            .expect("canonical request is admitted");
        assert_eq!(ack.seq, expected);
        assert_eq!(ack.record.seq, expected);
        assert_eq!(ack.running_leaf_count, u64::from(expected) + 1);
        assert_eq!(ack.domain_digest, domain.digest());
        assert_eq!(ack.running_root, log.running_root());
    }
}

#[test]
fn every_admission_changes_the_running_root() {
    let domain = domain();
    let mut log = AdmissionLog::open(domain).expect("domain is valid");
    let mut roots = vec![log.running_root()];
    for tag in 0..DARK_FBA_CAPACITY {
        let ack = log
            .admit(&request(u8::try_from(tag).unwrap()), CUTOFF)
            .expect("canonical request is admitted");
        assert!(!roots.contains(&ack.running_root));
        roots.push(ack.running_root);
    }
}

#[test]
fn the_canonical_record_encoding_is_fixed_width_and_injective() {
    let domain = harness_domain(64);
    let log = log_with(domain, 8);
    let mut seen = Vec::new();
    for record in log.records() {
        let bytes = record.canonical_bytes();
        assert_eq!(bytes.len(), RECORD_BYTES);
        assert!(!seen.contains(&bytes), "record encoding repeated");
        seen.push(bytes);
    }
    let mut moved = log.records()[0];
    let original = moved.leaf(&domain.digest());
    moved.seq += 1;
    assert_ne!(
        moved.leaf(&domain.digest()),
        original,
        "the canonical position is inside the leaf"
    );
    assert_ne!(
        moved.leaf(&domain.digest()),
        moved.leaf(&harness_domain(65).digest()),
        "the log domain is inside the leaf"
    );
}

/// One admission fault, the class it must produce, and how to inject it.
type Case = (&'static str, AdmissionRefusal, fn(&mut AdmissionRequest));

fn faults() -> Vec<Case> {
    vec![
        (
            "late arrival",
            AdmissionRefusal::LateArrival {
                arrival_epoch: CUTOFF + 1,
                cutoff_epoch: CUTOFF,
            },
            |r: &mut AdmissionRequest| r.arrival_epoch = CUTOFF + 1,
        ),
        (
            "arrival in the future",
            AdmissionRefusal::ArrivalInFuture {
                arrival_epoch: CUTOFF - 1,
                now_epoch: NOW,
            },
            |r: &mut AdmissionRequest| r.arrival_epoch = CUTOFF - 1,
        ),
        (
            "non-canonical payload size",
            AdmissionRefusal::NonCanonicalPayloadSize {
                offered: DARK_FBA_PAYLOAD_LEN + 1,
                required: DARK_FBA_PAYLOAD_LEN,
            },
            |r: &mut AdmissionRequest| r.payload_len = DARK_FBA_PAYLOAD_LEN + 1,
        ),
        (
            "availability shares mismatch",
            AdmissionRefusal::AvailabilitySharesMismatch {
                offered: DARK_FBA_AVAILABILITY_SHARES - 1,
                required: DARK_FBA_AVAILABILITY_SHARES,
            },
            |r: &mut AdmissionRequest| r.availability_shares = DARK_FBA_AVAILABILITY_SHARES - 1,
        ),
        (
            "zero nullifier",
            AdmissionRefusal::NullifierZero,
            |r: &mut AdmissionRequest| r.nullifier = [0u8; 32],
        ),
    ]
}

#[test]
fn each_admission_fault_produces_its_own_class() {
    for (name, expected, inject) in faults() {
        let mut log = AdmissionLog::open(domain()).expect("domain is valid");
        let mut candidate = request(9);
        inject(&mut candidate);
        assert_eq!(log.admit(&candidate, NOW), Err(expected), "case: {name}");
    }
}

#[test]
fn the_frozen_check_order_holds_under_simultaneous_faults() {
    // Every prefix of the frozen order is injected at once; the earliest fault
    // must name the class. This is the gap `DARK_FBA_RELATION.md` §13.3 found
    // in the clearing relation, pinned here rather than left open.
    let cases = faults();
    for first in 0..cases.len() {
        let mut log = AdmissionLog::open(domain()).expect("domain is valid");
        let mut candidate = request(9);
        // Applied from the last fault backwards, so that when two faults
        // contend for one field the earlier check's injection is the one that
        // survives and the assertion tests the priority it claims.
        for (_, _, inject) in cases[first..].iter().rev() {
            inject(&mut candidate);
        }
        let (name, expected, _) = &cases[first];
        assert_eq!(
            log.admit(&candidate, NOW),
            Err(*expected),
            "with every fault from {name} onward present, {name} must be reported"
        );
    }
}

#[test]
fn the_holder_clock_outranks_every_request_level_fault() {
    let mut log = AdmissionLog::open(domain()).expect("domain is valid");
    let mut candidate = request(9);
    for (_, _, inject) in faults().iter().rev() {
        inject(&mut candidate);
    }
    assert_eq!(
        log.admit(&candidate, CUTOFF + 1),
        Err(AdmissionRefusal::CutoffPassed {
            now_epoch: CUTOFF + 1,
            cutoff_epoch: CUTOFF,
        })
    );
}

#[test]
fn a_seal_outranks_the_holder_clock() {
    let (mut log, _) = sealed_log(domain(), 1);
    let mut candidate = request(9);
    for (_, _, inject) in faults().iter().rev() {
        inject(&mut candidate);
    }
    assert_eq!(
        log.admit(&candidate, CUTOFF + 1),
        Err(AdmissionRefusal::LogSealed)
    );
}

#[test]
fn a_repeated_nullifier_names_the_first_position() {
    let mut log = AdmissionLog::open(domain()).expect("domain is valid");
    log.admit(&request(0), CUTOFF).expect("first admission");
    log.admit(&request(1), CUTOFF).expect("second admission");
    assert_eq!(
        log.admit(&request(0), CUTOFF),
        Err(AdmissionRefusal::NullifierRepeated { first_seq: 0 })
    );
}

#[test]
fn a_zero_nullifier_outranks_a_repeated_one() {
    let mut log = AdmissionLog::open(domain()).expect("domain is valid");
    let mut zeroed = request(0);
    zeroed.nullifier = [0u8; 32];
    log.admit(&request(0), CUTOFF).expect("first admission");
    assert_eq!(
        log.admit(&zeroed, CUTOFF),
        Err(AdmissionRefusal::NullifierZero)
    );
}

#[test]
fn capacity_is_exact_and_reported_last() {
    let domain = domain();
    let mut log = log_with(domain, DARK_FBA_CAPACITY);
    assert_eq!(log.records().len(), DARK_FBA_CAPACITY as usize);
    assert_eq!(
        log.admit(&request(200), CUTOFF),
        Err(AdmissionRefusal::CapacityExhausted {
            capacity: DARK_FBA_CAPACITY
        })
    );
    // A capacity-exhausted log still reports the nullifier fault first.
    assert_eq!(
        log.admit(&request(0), CUTOFF),
        Err(AdmissionRefusal::NullifierRepeated { first_seq: 0 })
    );
}

#[test]
fn every_admission_refusal_class_is_reachable() {
    let mut produced: Vec<&'static str> = Vec::new();
    let record = |produced: &mut Vec<&'static str>, refusal: AdmissionRefusal| {
        let name = match refusal {
            AdmissionRefusal::LogSealed => "LogSealed",
            AdmissionRefusal::CutoffPassed { .. } => "CutoffPassed",
            AdmissionRefusal::LateArrival { .. } => "LateArrival",
            AdmissionRefusal::ArrivalInFuture { .. } => "ArrivalInFuture",
            AdmissionRefusal::NonCanonicalPayloadSize { .. } => "NonCanonicalPayloadSize",
            AdmissionRefusal::AvailabilitySharesMismatch { .. } => "AvailabilitySharesMismatch",
            AdmissionRefusal::NullifierZero => "NullifierZero",
            AdmissionRefusal::NullifierReservedForPadding { .. } => "NullifierReservedForPadding",
            AdmissionRefusal::NullifierRepeated { .. } => "NullifierRepeated",
            AdmissionRefusal::CapacityExhausted { .. } => "CapacityExhausted",
        };
        if !produced.contains(&name) {
            produced.push(name);
        }
    };

    for (_, _, inject) in faults() {
        let mut log = AdmissionLog::open(domain()).expect("domain is valid");
        let mut candidate = request(9);
        inject(&mut candidate);
        record(
            &mut produced,
            log.admit(&candidate, NOW).expect_err("fault refuses"),
        );
    }
    let mut log = AdmissionLog::open(domain()).expect("domain is valid");
    record(
        &mut produced,
        log.admit(&request(9), CUTOFF + 1).expect_err("late clock"),
    );
    log.admit(&request(0), CUTOFF).expect("first admission");
    record(
        &mut produced,
        log.admit(&request(0), CUTOFF).expect_err("repeat"),
    );
    let mut full = log_with(domain(), DARK_FBA_CAPACITY);
    record(
        &mut produced,
        full.admit(&request(200), CUTOFF).expect_err("full"),
    );
    let (mut sealed, _) = sealed_log(domain(), 1);
    record(
        &mut produced,
        sealed.admit(&request(200), CUTOFF).expect_err("sealed"),
    );
    let mut fresh = AdmissionLog::open(domain()).expect("domain is valid");
    let mut reserved = request(200);
    reserved.nullifier = AdmissionRecord::padding(&domain().digest(), 0, &domain()).nullifier;
    record(
        &mut produced,
        fresh
            .admit(&reserved, CUTOFF)
            .expect_err("padding nullifier"),
    );

    produced.sort_unstable();
    assert_eq!(
        produced,
        vec![
            "ArrivalInFuture",
            "AvailabilitySharesMismatch",
            "CapacityExhausted",
            "CutoffPassed",
            "LateArrival",
            "LogSealed",
            "NonCanonicalPayloadSize",
            "NullifierRepeated",
            "NullifierReservedForPadding",
            "NullifierZero",
        ],
        "every admission refusal class must have a witness"
    );
}

#[test]
fn a_seal_before_the_cutoff_is_refused() {
    let mut log = log_with(domain(), 2);
    assert_eq!(
        log.seal(CUTOFF - 1),
        Err(SealRefusal::BeforeCutoff {
            now_epoch: CUTOFF - 1,
            cutoff_epoch: CUTOFF,
        })
    );
    assert!(log.cutoff().is_none());
}

#[test]
fn a_log_seals_once() {
    let (mut log, cutoff) = sealed_log(domain(), 3);
    assert_eq!(log.seal(CUTOFF + 10), Err(SealRefusal::AlreadySealed));
    assert_eq!(log.cutoff(), Some(cutoff));
}

#[test]
fn the_cutoff_root_commits_the_exact_count_and_order() {
    let domain = domain();
    let (_, forward) = sealed_log(domain, 3);
    assert_eq!(forward.leaf_count, 3);

    let mut reversed = AdmissionLog::open(domain).expect("domain is valid");
    for tag in (0..3u8).rev() {
        reversed.admit(&request(tag), CUTOFF).expect("admitted");
    }
    let reversed = reversed.seal(CUTOFF).expect("seal at the cutoff");
    assert_eq!(reversed.leaf_count, forward.leaf_count);
    assert_ne!(
        reversed.root, forward.root,
        "the same multiset in a different order must not share a cutoff root"
    );

    let (_, shorter) = sealed_log(domain, 2);
    assert_ne!(shorter.root, forward.root);
    assert_ne!(shorter.leaf_count, forward.leaf_count);
}

#[test]
fn the_cutoff_root_binds_the_batch_the_market_and_the_cutoff() {
    let base = domain();
    let (_, reference) = sealed_log(base, 3);
    for altered in [
        LogDomain {
            batch: base.batch + 1,
            ..base
        },
        LogDomain {
            market: base.market + 1,
            ..base
        },
        LogDomain {
            cutoff_epoch: base.cutoff_epoch + 1,
            ..base
        },
        LogDomain {
            capacity: base.capacity + 1,
            ..base
        },
        LogDomain {
            payload_len: base.payload_len,
            availability_threshold: base.availability_threshold - 1,
            ..base
        },
    ] {
        let (_, other) = sealed_log(altered, 3);
        assert_ne!(
            other.root, reference.root,
            "a changed domain field must move the cutoff root"
        );
    }
}

#[test]
fn an_empty_batch_still_has_a_distinct_cutoff_root() {
    let (_, empty) = sealed_log(domain(), 0);
    let (_, one) = sealed_log(domain(), 1);
    assert_eq!(empty.leaf_count, 0);
    assert_ne!(empty.root, one.root);
    assert_ne!(empty.root, [0u8; 32]);
}

#[test]
fn an_invalid_domain_cannot_open_a_log() {
    let base = domain();
    for (expected, altered) in [
        (
            DomainDefect::ZeroCapacity,
            LogDomain {
                capacity: 0,
                ..base
            },
        ),
        (
            DomainDefect::ZeroPayloadLength,
            LogDomain {
                payload_len: 0,
                ..base
            },
        ),
        (
            DomainDefect::ThresholdOutOfRange,
            LogDomain {
                availability_threshold: 0,
                ..base
            },
        ),
        (
            DomainDefect::ThresholdOutOfRange,
            LogDomain {
                availability_threshold: base.availability_shares + 1,
                ..base
            },
        ),
    ] {
        assert_eq!(AdmissionLog::open(altered).err(), Some(expected));
    }
}

#[test]
fn a_padded_seal_always_commits_exactly_capacity_leaves() {
    for count in 0..=DARK_FBA_CAPACITY {
        let mut log = log_with(domain(), count);
        let cutoff = log.seal_padded(CUTOFF).expect("seal at the cutoff");
        assert_eq!(
            cutoff.leaf_count,
            u64::from(DARK_FBA_CAPACITY),
            "occupancy {count} must not be readable from the leaf count"
        );
        assert_eq!(log.records().len(), DARK_FBA_CAPACITY as usize);
    }
}

#[test]
fn padding_is_deterministic_and_position_bound() {
    let domain = domain();
    let mut left = log_with(domain, 2);
    let mut right = log_with(domain, 2);
    assert_eq!(
        left.seal_padded(CUTOFF).expect("seal").root,
        right.seal_padded(CUTOFF).expect("seal").root,
        "two holders of one admitted set must produce one padded root"
    );

    let digest = domain.digest();
    for seq in 0..DARK_FBA_CAPACITY {
        let record = AdmissionRecord::padding(&digest, seq, &domain);
        assert!(record.is_padding(&domain));
        assert!(record.conforms_to(&domain));
        let moved = AdmissionRecord {
            seq: seq + 1,
            ..record
        };
        assert!(
            !moved.is_padding(&domain) || seq + 1 >= DARK_FBA_CAPACITY,
            "a padding record is bound to its own position"
        );
    }
}

#[test]
fn a_padded_root_differs_from_the_unpadded_root_of_the_same_records() {
    let mut padded = log_with(domain(), 2);
    let mut plain = log_with(domain(), 2);
    assert_ne!(
        padded.seal_padded(CUTOFF).expect("seal").root,
        plain.seal(CUTOFF).expect("seal").root
    );
}

#[test]
fn a_padding_nullifier_cannot_be_claimed_by_a_submitter() {
    let domain = domain();
    let mut log = AdmissionLog::open(domain).expect("domain is valid");
    let digest = domain.digest();
    for padding_seq in 0..DARK_FBA_CAPACITY {
        let mut candidate = request(200);
        candidate.nullifier = AdmissionRecord::padding(&digest, padding_seq, &domain).nullifier;
        assert_eq!(
            log.admit(&candidate, CUTOFF),
            Err(AdmissionRefusal::NullifierReservedForPadding { padding_seq })
        );
    }
    assert!(log.records().is_empty());
}

#[test]
fn a_padded_log_keeps_every_nullifier_distinct() {
    let mut log = log_with(domain(), 2);
    log.seal_padded(CUTOFF).expect("seal at the cutoff");
    let mut nullifiers: Vec<[u8; 32]> = log.records().iter().map(|r| r.nullifier).collect();
    nullifiers.sort_unstable();
    let count = nullifiers.len();
    nullifiers.dedup();
    assert_eq!(nullifiers.len(), count);
}

#[test]
fn a_padded_seal_obeys_the_same_seal_rules() {
    let mut early = log_with(domain(), 1);
    assert_eq!(
        early.seal_padded(CUTOFF - 1),
        Err(SealRefusal::BeforeCutoff {
            now_epoch: CUTOFF - 1,
            cutoff_epoch: CUTOFF,
        })
    );
    let mut twice = log_with(domain(), 1);
    twice.seal_padded(CUTOFF).expect("first seal");
    assert_eq!(twice.seal_padded(CUTOFF), Err(SealRefusal::AlreadySealed));
}
