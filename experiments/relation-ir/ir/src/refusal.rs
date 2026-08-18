//! The frozen refusal taxonomy of `dark-fba/n4-k4-q15/v0`.
//!
//! Class identities, positions, and this crate's tag spellings follow the
//! check order frozen in `docs/research/DARK_FBA_RELATION.md` section 4.1.
//! Only the class tag is a public output; every other field of a refusal is an
//! executor-visible diagnostic, and the frozen leakage declaration forbids
//! publishing secret-bearing diagnostics.

use crate::canon::{Canonical, Sink};

/// One public refusal class.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RefusalClass {
    /// Rule 1: `DarkTarget` was requested and no Dark backend exists.
    DarkTargetUnavailable,
    /// Rule 2: the admission log is not final.
    AdmissionLogNotFinal,
    /// Rule 3: the witness slots do not open to the accepted-input root.
    RootBindingAbsent,
    /// Rule 4: a conflicting finalized root exists.
    RootEquivocation,
    /// Rule 5: an admitted payload is not available by the declared threshold.
    PayloadUnavailable,
    /// Rule 6: a slot is bound to a different batch identifier.
    BatchBindingMismatch,
    /// Rule 7: a slot is bound to a different market identifier.
    MarketBindingMismatch,
    /// Rule 8: a slot names an owner outside the owner domain.
    OwnerOutOfDomain,
    /// Rule 9: a slot carries a quantity outside the quantity domain.
    QuantityOutOfDomain,
    /// Rule 10: a slot names a tick outside the tick domain.
    LimitOutOfDomain,
    /// Rule 11: a slot carries a zero nullifier.
    NullifierZero,
    /// Rule 12: a slot repeats a strictly earlier occupied slot's nullifier.
    NullifierRepeated,
    /// Rule 13: a slot arrived after the cutoff.
    LateArrival,
    /// Rule 14: a slot's authorization statement is absent.
    Unauthorized,
    /// Rule 15: a slot's eligibility statement is absent.
    Ineligible,
    /// Rule 16: a slot's exact-inclusion statement is absent.
    InclusionAbsent,
    /// Rule 17: a slot's custody-binding statement is absent.
    CustodyBindingAbsent,
    /// Rule 18: a slot's reservation does not cover its worst-case obligation.
    ReservationInsufficient,
    /// Tier 3: an accumulator could not represent an intermediate sum exactly.
    AccumulatorOverflow,
    /// Tier 3: the evaluator's own conservation audit failed. Refusal instead
    /// of emitting a result that fails its stated invariants.
    InternalInvariant,
    /// Structural: the input does not fit the module's frozen shape (for
    /// example a slot vector of the wrong length). Not reachable from a
    /// well-typed instance; outside the section 4.1 order.
    MalformedEncoding,
}

/// Every class in canonical (code) order.
pub const REFUSAL_CLASSES: [RefusalClass; 21] = [
    RefusalClass::DarkTargetUnavailable,
    RefusalClass::AdmissionLogNotFinal,
    RefusalClass::RootBindingAbsent,
    RefusalClass::RootEquivocation,
    RefusalClass::PayloadUnavailable,
    RefusalClass::BatchBindingMismatch,
    RefusalClass::MarketBindingMismatch,
    RefusalClass::OwnerOutOfDomain,
    RefusalClass::QuantityOutOfDomain,
    RefusalClass::LimitOutOfDomain,
    RefusalClass::NullifierZero,
    RefusalClass::NullifierRepeated,
    RefusalClass::LateArrival,
    RefusalClass::Unauthorized,
    RefusalClass::Ineligible,
    RefusalClass::InclusionAbsent,
    RefusalClass::CustodyBindingAbsent,
    RefusalClass::ReservationInsufficient,
    RefusalClass::AccumulatorOverflow,
    RefusalClass::InternalInvariant,
    RefusalClass::MalformedEncoding,
];

impl RefusalClass {
    /// Canonical numeric code: the class's index in [`REFUSAL_CLASSES`].
    pub fn code(self) -> u32 {
        REFUSAL_CLASSES
            .iter()
            .position(|class| *class == self)
            .expect("every class is listed") as u32
    }

    /// The frozen public tag, shared vocabulary of specification section 4.1.
    pub fn tag(self) -> &'static str {
        match self {
            Self::DarkTargetUnavailable => "dark-target-unavailable",
            Self::AdmissionLogNotFinal => "admission-log-not-final",
            Self::RootBindingAbsent => "root-binding-absent",
            Self::RootEquivocation => "root-equivocation",
            Self::PayloadUnavailable => "payload-unavailable",
            Self::BatchBindingMismatch => "batch-binding-mismatch",
            Self::MarketBindingMismatch => "market-binding-mismatch",
            Self::OwnerOutOfDomain => "owner-out-of-domain",
            Self::QuantityOutOfDomain => "quantity-out-of-domain",
            Self::LimitOutOfDomain => "limit-out-of-domain",
            Self::NullifierZero => "nullifier-zero",
            Self::NullifierRepeated => "nullifier-repeated",
            Self::LateArrival => "late-arrival",
            Self::Unauthorized => "unauthorized",
            Self::Ineligible => "ineligible",
            Self::InclusionAbsent => "inclusion-absent",
            Self::CustodyBindingAbsent => "custody-binding-absent",
            Self::ReservationInsufficient => "reservation-insufficient",
            Self::AccumulatorOverflow => "accumulator-overflow",
            Self::InternalInvariant => "internal-invariant",
            Self::MalformedEncoding => "malformed-encoding",
        }
    }
}

impl Canonical for RefusalClass {
    fn tag(&self) -> &'static str {
        "ir/refusal-class"
    }
    fn body(&self, sink: &mut Sink) {
        sink.u32(self.code());
        sink.str(RefusalClass::tag(*self));
    }
}

/// One refusal with executor-visible diagnostics.
///
/// The public boundary carries [`RefusalClass::tag`] only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refusal {
    /// The public class.
    pub class: RefusalClass,
    /// Diagnostic: the slot that failed, for per-slot classes.
    pub slot: Option<u8>,
    /// Diagnostic: the earliest prior use, for `nullifier-repeated`.
    pub first_slot: Option<u8>,
}

impl Refusal {
    /// A batch-level refusal with no slot diagnostic.
    pub fn batch(class: RefusalClass) -> Self {
        Self {
            class,
            slot: None,
            first_slot: None,
        }
    }

    /// A per-slot refusal.
    pub fn at_slot(class: RefusalClass, slot: u8) -> Self {
        Self {
            class,
            slot: Some(slot),
            first_slot: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_are_dense_and_stable() {
        for (index, class) in REFUSAL_CLASSES.iter().enumerate() {
            assert_eq!(class.code() as usize, index);
        }
        assert_eq!(RefusalClass::DarkTargetUnavailable.code(), 0);
        assert_eq!(RefusalClass::ReservationInsufficient.code(), 17);
        assert_eq!(RefusalClass::MalformedEncoding.code(), 20);
    }

    #[test]
    fn tags_are_unique() {
        for a in REFUSAL_CLASSES {
            for b in REFUSAL_CLASSES {
                if a != b {
                    assert_ne!(a.tag(), b.tag());
                }
            }
        }
    }
}
