//! A deterministic transcript of the whole model, rendered as bytes.
//!
//! The corpus under `vectors/` is this function's output. It is checked into
//! the repository and compared byte for byte by `tests/vectors.rs`, so any
//! change to a tag, an encoding, a commitment rule, a check order, or an abort
//! consequence shows up as a diff rather than as a quietly different system.

use crate::equivocation::{
    Conflict, EquivocationProof, HolderId, RootStatement, verify_equivocation,
};
use crate::hash::hex;
use crate::lifecycle::{AbortClass, BatchMachine, Entitlement, Phase, ReserveLedger, Timeouts};
use crate::log::{
    AdmissionLog, AdmissionRequest, CutoffRoot, DARK_FBA_AVAILABILITY_SHARES, DARK_FBA_PAYLOAD_LEN,
    LogDomain, verify_receipt,
};
use crate::mmr::verify_consistency;

const BATCH: u64 = 7;
const MARKET: u64 = 11;
const CUTOFF: u64 = 1_000;
const HOLDER: HolderId = HolderId([0x5a; 32]);
const RESULT: [u8; 32] = [0xc0; 32];
const AMOUNTS: [u64; 4] = [10, 200, 3_000, 40_000];
/// A relation's own public refusal-class code, opaque to this crate.
const REFUSAL_CODE: u32 = 17;

fn domain() -> LogDomain {
    LogDomain::dark_fba_v0(BATCH, MARKET, CUTOFF)
}

fn request(tag: u8) -> AdmissionRequest {
    AdmissionRequest {
        submitter: [tag ^ 0x11; 32],
        payload_commitment: [tag ^ 0x22; 32],
        payload_len: DARK_FBA_PAYLOAD_LEN,
        availability_shares: DARK_FBA_AVAILABILITY_SHARES,
        arrival_epoch: CUTOFF - 10,
        nullifier: [tag ^ 0x33; 32],
    }
}

fn sealed(tags: &[u8]) -> (AdmissionLog, CutoffRoot) {
    let domain = domain();
    let mut log = AdmissionLog::open(domain).expect("domain is valid");
    for tag in tags {
        log.admit(&request(*tag), CUTOFF).expect("admitted");
    }
    let cutoff = log.seal(CUTOFF).expect("seal at the cutoff");
    (log, cutoff)
}

/// Render the full transcript.
#[must_use]
pub fn render() -> String {
    let mut out = String::new();
    out.push_str(&format!("model {}\n", crate::MODEL));
    out.push_str(&format!("relation {}\n", domain().relation));
    out.push_str(&format!(
        "domain batch={BATCH} market={MARKET} cutoff={CUTOFF} capacity={} payload_len={} shares={} threshold={}\n",
        domain().capacity,
        domain().payload_len,
        domain().availability_shares,
        domain().availability_threshold,
    ));
    out.push_str(&format!("domain_digest {}\n", hex(&domain().digest())));
    out.push('\n');

    admission_section(&mut out);
    inclusion_section(&mut out);
    consistency_section(&mut out);
    equivocation_section(&mut out);
    lifecycle_section(&mut out);
    out
}

fn admission_section(out: &mut String) {
    out.push_str("[admission]\n");
    let domain = domain();
    let mut log = AdmissionLog::open(domain).expect("domain is valid");
    out.push_str(&format!("empty_root {}\n", hex(&log.running_root())));
    for tag in 0..4u8 {
        let ack = log.admit(&request(tag), CUTOFF).expect("admitted");
        out.push_str(&format!(
            "ack seq={} leaves={} leaf={} root={}\n",
            ack.seq,
            ack.running_leaf_count,
            hex(&ack.record.leaf(&domain.digest())),
            hex(&ack.running_root),
        ));
    }
    let refusal = log
        .admit(&request(9), CUTOFF)
        .expect_err("capacity is four");
    out.push_str(&format!("refusal {refusal:?}\n"));
    let cutoff = log.seal(CUTOFF).expect("seal at the cutoff");
    out.push_str(&format!(
        "cutoff leaves={} root={}\n\n",
        cutoff.leaf_count,
        hex(&cutoff.root)
    ));
}

fn inclusion_section(out: &mut String) {
    out.push_str("[inclusion]\n");
    let (log, cutoff) = sealed(&[0, 1, 2, 3]);
    for seq in 0..4u32 {
        let receipt = log.receipt(seq).expect("record is admitted");
        let verdict = match verify_receipt(&cutoff, &receipt) {
            Ok(()) => "accept".to_string(),
            Err(defect) => format!("reject {defect:?}"),
        };
        out.push_str(&format!(
            "receipt seq={seq} path={} left_peaks={} right_peaks={} {verdict}\n",
            receipt.proof.path.len(),
            receipt.proof.left_peaks.len(),
            receipt.proof.right_peaks.len(),
        ));
    }
    // Padded sealing: the leaf count is the capacity whatever the occupancy.
    for real in 0..=4u8 {
        let domain = domain();
        let mut padded = AdmissionLog::open(domain).expect("domain is valid");
        for tag in 0..real {
            padded.admit(&request(tag), CUTOFF).expect("admitted");
        }
        let sealed_root = padded.seal_padded(CUTOFF).expect("seal at the cutoff");
        let occupied = (0..4u32)
            .filter(|seq| {
                let receipt = padded.receipt(*seq).expect("every position is committed");
                verify_receipt(&sealed_root, &receipt).expect("receipt verifies");
                !receipt.record.is_padding(&domain)
            })
            .count();
        out.push_str(&format!(
            "padded real={real} leaves={} occupied_from_bytes={occupied} root={}\n",
            sealed_root.leaf_count,
            hex(&sealed_root.root),
        ));
    }
    // A three-record log has two peaks, so its receipts exercise the splice.
    let (odd_log, odd_cutoff) = sealed(&[0, 1, 2]);
    for seq in 0..3u32 {
        let receipt = odd_log.receipt(seq).expect("record is admitted");
        let verdict = match verify_receipt(&odd_cutoff, &receipt) {
            Ok(()) => "accept".to_string(),
            Err(defect) => format!("reject {defect:?}"),
        };
        out.push_str(&format!(
            "receipt3 seq={seq} path={} left_peaks={} right_peaks={} {verdict}\n",
            receipt.proof.path.len(),
            receipt.proof.left_peaks.len(),
            receipt.proof.right_peaks.len(),
        ));
    }
    let mut tampered = log.receipt(1).expect("record is admitted");
    tampered.proof.path[0].hash[0] ^= 0x01;
    out.push_str(&format!(
        "tampered_sibling {:?}\n",
        verify_receipt(&cutoff, &tampered).expect_err("tampered receipt is refused")
    ));
    let mut moved = log.receipt(1).expect("record is admitted");
    moved.record.seq = 2;
    out.push_str(&format!(
        "moved_position {:?}\n",
        verify_receipt(&cutoff, &moved).expect_err("moved receipt is refused")
    ));
    let mut wrong_root = cutoff;
    wrong_root.root[0] ^= 0x01;
    out.push_str(&format!(
        "wrong_root {:?}\n\n",
        verify_receipt(&wrong_root, &log.receipt(0).expect("record is admitted"))
            .expect_err("wrong root is refused")
    ));
}

fn consistency_section(out: &mut String) {
    out.push_str("[consistency]\n");
    let (log, cutoff) = sealed(&[0, 1, 2, 3]);
    let digest = domain().digest();
    for prefix in 0..=4u64 {
        let proof = log.consistency_proof(prefix).expect("prefix is in the log");
        let derived = verify_consistency(&digest, &cutoff.root, cutoff.leaf_count, &proof)
            .expect("consistency proof verifies");
        out.push_str(&format!(
            "prefix leaves={prefix} peaks={} root={} matches_running={}\n",
            proof.prefix_peaks.len(),
            hex(&derived),
            derived == log.root_at(prefix),
        ));
    }
    out.push('\n');
}

fn equivocation_section(out: &mut String) {
    out.push_str("[equivocation]\n");
    let domain = domain();
    let (left_log, left) = sealed(&[0, 1, 2]);
    let (right_log, right) = sealed(&[0, 9, 2]);
    let (swapped_log, swapped) = sealed(&[1, 0, 2]);
    let left_statement = RootStatement::seal(HOLDER, &left);
    let right_statement = RootStatement::seal(HOLDER, &right);
    let swapped_statement = RootStatement::seal(HOLDER, &swapped);
    out.push_str(&format!("left_root {}\n", hex(&left.root)));
    out.push_str(&format!("right_root {}\n", hex(&right.root)));
    out.push_str(&format!("swapped_root {}\n", hex(&swapped.root)));

    let cases: Vec<(&str, EquivocationProof)> = vec![
        (
            "roots",
            EquivocationProof {
                domain,
                left: left_statement,
                right: right_statement,
                conflict: Conflict::Roots,
            },
        ),
        (
            "sequence",
            EquivocationProof {
                domain,
                left: left_statement,
                right: right_statement,
                conflict: Conflict::Sequence {
                    left: left_log.receipt(1).expect("record is admitted"),
                    right: right_log.receipt(1).expect("record is admitted"),
                },
            },
        ),
        (
            "position",
            EquivocationProof {
                domain,
                left: left_statement,
                right: swapped_statement,
                conflict: Conflict::Position {
                    left: left_log.receipt(1).expect("record is admitted"),
                    right: swapped_log.receipt(0).expect("record is admitted"),
                },
            },
        ),
        (
            "prefix",
            EquivocationProof {
                domain,
                left: RootStatement::ack(HOLDER, &domain, 2, left_log.root_at(2)),
                right: right_statement,
                conflict: Conflict::Prefix {
                    consistency: right_log
                        .consistency_proof(2)
                        .expect("prefix is in the log"),
                },
            },
        ),
    ];
    for (name, proof) in &cases {
        let verdict = verify_equivocation(proof).expect("proof verifies");
        out.push_str(&format!(
            "verdict {name} class={} digest={}\n",
            verdict.class,
            hex(&verdict.digest)
        ));
    }

    // The same shapes, offered by an honest single-root holder.
    let honest = RootStatement::seal(HOLDER, &left);
    let honest_cases: Vec<(&str, EquivocationProof)> = vec![
        (
            "honest-roots",
            EquivocationProof {
                domain,
                left: honest,
                right: honest,
                conflict: Conflict::Roots,
            },
        ),
        (
            "honest-prefix",
            EquivocationProof {
                domain,
                left: RootStatement::ack(HOLDER, &domain, 2, left_log.root_at(2)),
                right: honest,
                conflict: Conflict::Prefix {
                    consistency: left_log.consistency_proof(2).expect("prefix is in the log"),
                },
            },
        ),
    ];
    for (name, proof) in &honest_cases {
        let defect = verify_equivocation(proof).expect_err("honest operation is not equivocation");
        out.push_str(&format!("refused {name} {defect:?}\n"));
    }
    out.push('\n');
}

fn lifecycle_section(out: &mut String) {
    out.push_str("[lifecycle]\n");
    let timeouts = Timeouts::dark_fba_v0();
    out.push_str(&format!(
        "timeouts seal=+{} availability=+{} compute=+{} retries={}\n",
        timeouts.seal, timeouts.availability, timeouts.compute, timeouts.compute_retries
    ));
    for class in [
        AbortClass::CutoffRootWithheld,
        AbortClass::InputWithheld { seq: 0 },
        AbortClass::ComputeTimeout { attempts: 1 },
        AbortClass::ComputeExhausted { attempts: 2 },
        AbortClass::Equivocation {
            verdict_digest: [0u8; 32],
        },
        AbortClass::ResultUnbound,
        AbortClass::RelationRefused { class_code: 0 },
    ] {
        out.push_str(&format!(
            "abort {} retryable={} terminal={} consequence={:?}\n",
            class.class(),
            class.is_retryable(),
            class.is_terminal(),
            class.consequence(),
        ));
    }
    // The class code is the relation's own, carried verbatim and never read.
    out.push_str(&format!(
        "relation_refusal class_code={REFUSAL_CODE} phase={} consequence={:?}\n",
        Phase::Aborted(AbortClass::RelationRefused {
            class_code: REFUSAL_CODE
        })
        .name(),
        AbortClass::RelationRefused {
            class_code: REFUSAL_CODE
        }
        .consequence(),
    ));

    for (name, phase, ledger) in [
        settled_run(),
        withheld_root_run(),
        withheld_input_run(),
        exhausted_run(),
        unbound_result_run(),
        relation_refused_run(),
    ] {
        out.push_str(&format!(
            "run {name} phase={} escrowed={} refunded={} settled={} outstanding={} conserves={}\n",
            phase.name(),
            ledger.total_escrowed(),
            ledger.total_refunded(),
            ledger.total_settled(),
            ledger.total_outstanding(),
            ledger.conserves(),
        ));
    }
}

fn escrow(tags: &[u8]) -> ReserveLedger {
    let mut ledger = ReserveLedger::new();
    for (position, tag) in tags.iter().enumerate() {
        ledger.escrow(request(*tag).nullifier, AMOUNTS[position % AMOUNTS.len()]);
    }
    ledger
}

fn watching(cutoff: CutoffRoot) -> BatchMachine {
    let mut machine = BatchMachine::new(domain(), Timeouts::dark_fba_v0());
    machine
        .observe_cutoff(cutoff, CUTOFF)
        .expect("the root is published on time");
    for seq in 0..u32::try_from(cutoff.leaf_count).expect("capacity is small") {
        machine
            .report_availability(seq, DARK_FBA_AVAILABILITY_SHARES)
            .expect("report is admissible");
    }
    machine
}

fn settled_run() -> (&'static str, Phase, ReserveLedger) {
    let (log, cutoff) = sealed(&[0, 1, 2, 3]);
    let mut ledger = escrow(&[0, 1, 2, 3]);
    let mut machine = watching(cutoff);
    machine.begin_compute(CUTOFF + 1).expect("inputs present");
    let phase = machine
        .deliver_result(cutoff.root, RESULT, CUTOFF + 3)
        .expect("delivery is admissible");
    for seq in 0..4u32 {
        let receipt = log.receipt(seq).expect("record is admitted");
        machine
            .release_to_settlement(&mut ledger, receipt.record.nullifier)
            .expect("settled batch releases reservations");
    }
    ("settled", phase, ledger)
}

fn withheld_root_run() -> (&'static str, Phase, ReserveLedger) {
    let mut ledger = escrow(&[0, 1, 2, 3]);
    let mut machine = BatchMachine::new(domain(), Timeouts::dark_fba_v0());
    let phase = machine.tick(CUTOFF + 2);
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
    ("cutoff-root-withheld", phase, ledger)
}

fn withheld_input_run() -> (&'static str, Phase, ReserveLedger) {
    let (log, cutoff) = sealed(&[0, 1, 2, 3]);
    let mut ledger = escrow(&[0, 1, 2, 3]);
    let mut machine = watching(cutoff);
    machine
        .report_availability(2, 2)
        .expect("report is admissible");
    let phase = machine.tick(CUTOFF + 3);
    refund_all(&log, &machine, &mut ledger);
    ("input-withheld", phase, ledger)
}

fn exhausted_run() -> (&'static str, Phase, ReserveLedger) {
    let (log, cutoff) = sealed(&[0, 1, 2, 3]);
    let mut ledger = escrow(&[0, 1, 2, 3]);
    let mut machine = watching(cutoff);
    machine.begin_compute(CUTOFF + 1).expect("inputs present");
    machine.tick(CUTOFF + 5);
    machine.resume(CUTOFF + 5).expect("retry");
    let phase = machine.tick(CUTOFF + 9);
    refund_all(&log, &machine, &mut ledger);
    ("compute-exhausted", phase, ledger)
}

fn unbound_result_run() -> (&'static str, Phase, ReserveLedger) {
    let (log, cutoff) = sealed(&[0, 1, 2, 3]);
    let mut ledger = escrow(&[0, 1, 2, 3]);
    let mut machine = watching(cutoff);
    machine.begin_compute(CUTOFF + 1).expect("inputs present");
    let mut other = cutoff.root;
    other[31] ^= 0x01;
    let phase = machine
        .deliver_result(other, RESULT, CUTOFF + 3)
        .expect("delivery is admissible");
    refund_all(&log, &machine, &mut ledger);
    ("result-unbound", phase, ledger)
}

fn relation_refused_run() -> (&'static str, Phase, ReserveLedger) {
    let (log, cutoff) = sealed(&[0, 1, 2, 3]);
    let mut ledger = escrow(&[0, 1, 2, 3]);
    let mut machine = watching(cutoff);
    machine.begin_compute(CUTOFF + 1).expect("inputs present");
    let phase = machine
        .deliver_refusal(cutoff.root, REFUSAL_CODE, CUTOFF + 3)
        .expect("delivery is admissible");
    refund_all(&log, &machine, &mut ledger);
    ("relation-refused", phase, ledger)
}

fn refund_all(log: &AdmissionLog, machine: &BatchMachine, ledger: &mut ReserveLedger) {
    for seq in 0..4u32 {
        let receipt = log.receipt(seq).expect("record is admitted");
        machine
            .claim_refund(ledger, &Entitlement::Included(&receipt))
            .expect("an admitted record is refundable");
    }
}
