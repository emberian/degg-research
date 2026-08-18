//! Admission-check priority as a frozen IR field.
//!
//! Specification section 8 of `docs/research/DARK_FBA_RELATION.md` makes the
//! refusal class a public output, so the priority among simultaneously violated
//! rules is itself an observable. The 2026-08-18 differential proved that prose
//! alone underdetermined it; section 4.1 now freezes one total order. This
//! module makes that order a *data field* of the module, so a conforming
//! evaluator realizes whatever order the module carries and a different order
//! is a visibly different module with a different digest.

use crate::canon::{Canonical, Sink};

/// One batch-level boundary statement check.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundaryCheck {
    /// Rule 2: the admission log is final.
    LogFinal,
    /// Rule 3: the witness slots open exactly to the accepted-input root.
    RootBindsSlots,
    /// Rule 4: no conflicting finalized root exists.
    NoConflictingRoot,
    /// Rule 5: every admitted payload is available by the declared threshold.
    PayloadsAvailable,
}

impl BoundaryCheck {
    fn code(self) -> u32 {
        match self {
            Self::LogFinal => 0,
            Self::RootBindsSlots => 1,
            Self::NoConflictingRoot => 2,
            Self::PayloadsAvailable => 3,
        }
    }
}

/// One per-slot admission rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlotRule {
    /// The slot binds the batch identifier.
    BatchBinding,
    /// The slot binds the market identifier.
    MarketBinding,
    /// The owner index is in domain.
    OwnerInDomain,
    /// The quantity is in domain.
    QuantityInDomain,
    /// The limit tick is in domain.
    LimitInDomain,
    /// The nullifier is nonzero.
    NullifierNonzero,
    /// The nullifier differs from every strictly earlier occupied slot's.
    NullifierDistinctFromEarlierSlots,
    /// The arrival time is at or before the cutoff.
    ArrivedByCutoff,
    /// The authorization statement is present.
    Authorized,
    /// The eligibility statement is present.
    Eligible,
    /// The exact-inclusion statement is present.
    IncludedUnderRoot,
    /// The custody-binding statement is present.
    CustodyBound,
    /// The reservation covers the worst-case obligation. Requires
    /// [`SlotRule::QuantityInDomain`] and [`SlotRule::LimitInDomain`] to appear
    /// earlier in the same sequence; lowering refuses otherwise.
    ReservationCovers,
}

impl SlotRule {
    fn code(self) -> u32 {
        match self {
            Self::BatchBinding => 0,
            Self::MarketBinding => 1,
            Self::OwnerInDomain => 2,
            Self::QuantityInDomain => 3,
            Self::LimitInDomain => 4,
            Self::NullifierNonzero => 5,
            Self::NullifierDistinctFromEarlierSlots => 6,
            Self::ArrivedByCutoff => 7,
            Self::Authorized => 8,
            Self::Eligible => 9,
            Self::IncludedUnderRoot => 10,
            Self::CustodyBound => 11,
            Self::ReservationCovers => 12,
        }
    }
}

impl Canonical for SlotRule {
    fn tag(&self) -> &'static str {
        "ir/slot-rule"
    }
    fn body(&self, sink: &mut Sink) {
        sink.u32(self.code());
    }
}

/// One step of the admission policy, evaluated in sequence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdmissionStep {
    /// Rule 1: the requested execution mode must be executable; a `DarkTarget`
    /// request is refused before any witness is inspected.
    RequestedModeExecutable,
    /// One batch-level boundary statement.
    Boundary(BoundaryCheck),
    /// Per-slot rules: slots are visited in ascending slot index, rules in the
    /// listed order within a slot, and the first failing rule of the first
    /// failing slot decides the class. Empty slots are skipped.
    PerSlot(Vec<SlotRule>),
    /// A batch-wide pairwise duplicate-nullifier scan after the per-slot pass:
    /// ascending later slot, then ascending earlier slot. This step exists so
    /// the *rejected* alternative priority remains expressible; the frozen v0
    /// policy does not use it.
    NullifierSweep,
}

impl AdmissionStep {
    fn code(&self) -> u32 {
        match self {
            Self::RequestedModeExecutable => 0,
            Self::Boundary(_) => 1,
            Self::PerSlot(_) => 2,
            Self::NullifierSweep => 3,
        }
    }
}

impl Canonical for AdmissionStep {
    fn tag(&self) -> &'static str {
        "ir/admission-step"
    }
    fn body(&self, sink: &mut Sink) {
        sink.u32(self.code());
        match self {
            Self::RequestedModeExecutable | Self::NullifierSweep => {}
            Self::Boundary(check) => sink.u32(check.code()),
            Self::PerSlot(rules) => {
                sink.count(rules.len());
                for rule in rules {
                    sink.nested(rule);
                }
            }
        }
    }
}

/// The whole admission policy: check priority as data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmissionPolicy {
    /// Steps in frozen evaluation order.
    pub steps: Vec<AdmissionStep>,
}

impl Canonical for AdmissionPolicy {
    fn tag(&self) -> &'static str {
        "ir/admission-policy"
    }
    fn body(&self, sink: &mut Sink) {
        sink.count(self.steps.len());
        for step in &self.steps {
            sink.nested(step);
        }
    }
}

/// The frozen v0 check order of specification section 4.1: the reference
/// oracle's observed order, which the published golden vectors pin.
pub fn frozen_v0_check_order() -> AdmissionPolicy {
    AdmissionPolicy {
        steps: vec![
            AdmissionStep::RequestedModeExecutable,
            AdmissionStep::Boundary(BoundaryCheck::LogFinal),
            AdmissionStep::Boundary(BoundaryCheck::RootBindsSlots),
            AdmissionStep::Boundary(BoundaryCheck::NoConflictingRoot),
            AdmissionStep::Boundary(BoundaryCheck::PayloadsAvailable),
            AdmissionStep::PerSlot(vec![
                SlotRule::BatchBinding,
                SlotRule::MarketBinding,
                SlotRule::OwnerInDomain,
                SlotRule::QuantityInDomain,
                SlotRule::LimitInDomain,
                SlotRule::NullifierNonzero,
                SlotRule::NullifierDistinctFromEarlierSlots,
                SlotRule::ArrivedByCutoff,
                SlotRule::Authorized,
                SlotRule::Eligible,
                SlotRule::IncludedUnderRoot,
                SlotRule::CustodyBound,
                SlotRule::ReservationCovers,
            ]),
        ],
    }
}

/// The priority the pre-freeze independent oracle used: limit before quantity,
/// per-slot statements before nullifier rules, and batch-wide nullifier
/// uniqueness as a final sweep.
///
/// NOT frozen and NOT part of v0. It exists to demonstrate that the check
/// order is a live semantic field: swapping it in changes the public refusal
/// class on the differential's minimal witnesses, which is exactly why the
/// field must be frozen data rather than prose.
pub fn rejected_alternative_check_order() -> AdmissionPolicy {
    AdmissionPolicy {
        steps: vec![
            AdmissionStep::RequestedModeExecutable,
            AdmissionStep::Boundary(BoundaryCheck::LogFinal),
            AdmissionStep::Boundary(BoundaryCheck::RootBindsSlots),
            AdmissionStep::Boundary(BoundaryCheck::NoConflictingRoot),
            AdmissionStep::Boundary(BoundaryCheck::PayloadsAvailable),
            AdmissionStep::PerSlot(vec![
                SlotRule::BatchBinding,
                SlotRule::MarketBinding,
                SlotRule::OwnerInDomain,
                SlotRule::LimitInDomain,
                SlotRule::QuantityInDomain,
                SlotRule::ArrivedByCutoff,
                SlotRule::Authorized,
                SlotRule::Eligible,
                SlotRule::IncludedUnderRoot,
                SlotRule::NullifierNonzero,
                SlotRule::CustodyBound,
                SlotRule::ReservationCovers,
            ]),
            AdmissionStep::NullifierSweep,
        ],
    }
}
