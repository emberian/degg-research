//! Shared fixtures for the inclusion and availability suite.
#![allow(dead_code)]

use degg_inclusion_availability::equivocation::{HolderId, RootStatement, StatementKind};
use degg_inclusion_availability::log::{
    AdmissionLog, AdmissionRequest, CutoffRoot, DARK_FBA_AVAILABILITY_SHARES, DARK_FBA_PAYLOAD_LEN,
    LogDomain,
};

/// Batch identifier used throughout the suite.
pub const BATCH: u64 = 7;
/// Market identifier used throughout the suite.
pub const MARKET: u64 = 11;
/// Cutoff epoch used throughout the suite.
pub const CUTOFF: u64 = 1_000;

/// A relation-shaped domain for the modelled Dark FBA batch.
#[must_use]
pub fn domain() -> LogDomain {
    LogDomain::dark_fba_v0(BATCH, MARKET, CUTOFF)
}

/// Relation identifier of the wider harness domain.
///
/// This is a *model harness* shape, not a relation: it exists so the proof
/// machinery can be exercised at leaf counts the four-slot relation cannot
/// reach. It is never presented as a version of `dark-fba/n4-k4-q15/v0`.
pub const HARNESS_RELATION: &str = "degg-inclusion-availability/model-harness/v0";

/// A domain with room for `capacity` records.
#[must_use]
pub fn harness_domain(capacity: u32) -> LogDomain {
    LogDomain {
        relation: HARNESS_RELATION,
        batch: BATCH,
        market: MARKET,
        cutoff_epoch: CUTOFF,
        capacity,
        payload_len: DARK_FBA_PAYLOAD_LEN,
        availability_shares: DARK_FBA_AVAILABILITY_SHARES,
        availability_threshold: 3,
    }
}

/// A canonical admission request distinguished only by `tag`.
#[must_use]
pub fn request(tag: u8) -> AdmissionRequest {
    AdmissionRequest {
        submitter: [tag ^ 0x11; 32],
        payload_commitment: [tag ^ 0x22; 32],
        payload_len: DARK_FBA_PAYLOAD_LEN,
        availability_shares: DARK_FBA_AVAILABILITY_SHARES,
        arrival_epoch: CUTOFF - 10,
        nullifier: [tag ^ 0x33; 32],
    }
}

/// An open log over `domain` holding `count` canonical records.
#[must_use]
pub fn log_with(domain: LogDomain, count: u32) -> AdmissionLog {
    let mut log = AdmissionLog::open(domain).expect("domain is valid");
    for tag in 0..count {
        let request = request(u8::try_from(tag).expect("fixture count is small"));
        log.admit(&request, domain.cutoff_epoch)
            .expect("canonical request is admitted");
    }
    log
}

/// A sealed log over `domain` holding `count` canonical records.
#[must_use]
pub fn sealed_log(domain: LogDomain, count: u32) -> (AdmissionLog, CutoffRoot) {
    let mut log = log_with(domain, count);
    let cutoff = log.seal(domain.cutoff_epoch).expect("seal at the cutoff");
    (log, cutoff)
}

/// The holder identity used throughout the suite.
#[must_use]
pub fn holder() -> HolderId {
    HolderId([0x5a; 32])
}

/// The holder's sealed statement about a cutoff root.
#[must_use]
pub fn seal_statement(cutoff: &CutoffRoot) -> RootStatement {
    RootStatement::seal(holder(), cutoff)
}

/// The holder's acknowledgement statement about a running root.
#[must_use]
pub fn ack_statement(domain: &LogDomain, leaf_count: u64, root: [u8; 32]) -> RootStatement {
    RootStatement::new(
        holder(),
        domain.digest(),
        StatementKind::Ack,
        leaf_count,
        root,
    )
}
