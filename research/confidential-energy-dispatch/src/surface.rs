//! Frozen public leakage projection.
//!
//! The frame has one byte length and field layout for every outcome. It omits
//! provider identities, occupancy, buses, costs, capacities, ramps, outages,
//! dispatch, credits, exact failure details, internal counters, and wall-clock
//! timing. A fixed frame is only a transcript-shape invariant. This Clear Rust
//! implementation has data-dependent runtime and memory access and therefore
//! does not realize the proposed Dark surface by itself.

use crate::canonical::{domain_commitment, relation_commitment};
use crate::model::DispatchRequest;

/// Exact frame length for all outcomes.
pub const PUBLIC_FRAME_LEN: usize = 176;

/// Coarse public result. Detailed witness defects remain private.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PublicStatus {
    /// The public domain itself is malformed.
    MalformedPublic = 1,
    /// Admission or payload availability is not complete.
    InputUnavailable = 2,
    /// A private input or its commitment is invalid.
    WitnessRefused = 3,
    /// Valid admitted inputs have no feasible plan.
    Infeasible = 4,
    /// Checked arithmetic or an internal invariant refused.
    ArithmeticRefused = 5,
    /// A feasible canonical minimum-cost plan was found.
    Settled = 6,
}

/// Fixed public output. The domain itself is public through the request; its
/// commitment here prevents replay or substitution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)] // These four named public claims are the frozen leakage.
pub struct PublicOutcome {
    /// Relation identity.
    pub relation: [u8; 32],
    /// Public-domain commitment.
    pub domain: [u8; 32],
    /// Accepted fixed-size input commitment.
    pub inputs: [u8; 32],
    /// Commitment to the canonical plan, or zero on refusal.
    pub plan: [u8; 32],
    /// Commitment to padded authorized local outputs, or zero on refusal.
    pub deliveries: [u8; 32],
    /// Coarse status.
    pub status: PublicStatus,
    /// True exactly for a settled plan satisfying nodal balances.
    pub demand_satisfied: bool,
    /// True exactly for a settled plan satisfying line constraints.
    pub line_satisfied: bool,
    /// True exactly for a settled plan satisfying the reserve policy.
    pub reserve_satisfied: bool,
    /// True exactly for a settled exactly conserving settlement.
    pub settlement_conserves: bool,
}

impl PublicOutcome {
    /// Refusal frame with no plan or delivery commitment.
    #[must_use]
    pub fn refused(request: &DispatchRequest, status: PublicStatus) -> Self {
        debug_assert_ne!(status, PublicStatus::Settled);
        Self {
            relation: relation_commitment(),
            domain: domain_commitment(&request.domain),
            inputs: request.accepted_inputs,
            plan: [0; 32],
            deliveries: [0; 32],
            status,
            demand_satisfied: false,
            line_satisfied: false,
            reserve_satisfied: false,
            settlement_conserves: false,
        }
    }

    /// Canonical fixed-size public bytes.
    #[must_use]
    pub fn frame(&self) -> [u8; PUBLIC_FRAME_LEN] {
        let mut out = [0u8; PUBLIC_FRAME_LEN];
        out[..8].copy_from_slice(b"DEGGEDV0");
        out[8] = self.status as u8;
        out[9] = u8::from(self.demand_satisfied);
        out[10] = u8::from(self.line_satisfied);
        out[11] = u8::from(self.reserve_satisfied);
        out[12] = u8::from(self.settlement_conserves);
        out[16..48].copy_from_slice(&self.relation);
        out[48..80].copy_from_slice(&self.domain);
        out[80..112].copy_from_slice(&self.inputs);
        out[112..144].copy_from_slice(&self.plan);
        out[144..176].copy_from_slice(&self.deliveries);
        out
    }
}
