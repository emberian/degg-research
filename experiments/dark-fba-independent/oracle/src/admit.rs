//! Admission: every occupied slot must pass every rule or the whole batch
//! refuses. A malformed batch is never reinterpreted as no trade.
//!
//! Which rule wins when one witness violates several is not a free choice: it
//! is fixed by the frozen admission-check order of `DARK_FBA_RELATION.md`
//! section 4.1, because section 8 publishes the refusal class. [`screen`]
//! implements that order exactly, and its rule numbers below are the numbers
//! in that table.

use crate::book::{Batch, Direction, Mode, Order, Slot};
use crate::params::{OWNERS, QUANTITY_CEILING, QUANTITY_FLOOR, SLOTS, TICK_PRICES, TICKS};

/// One typed public refusal class.
///
/// The variants are declared in the frozen check order of section 4.1, so the
/// first variant a witness can reach is the class it is refused with.
///
/// The slot index carried by a per-slot class is diagnostic; the frozen public
/// boundary publishes only [`Refusal::class`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// Rule 1: `DarkTarget` was requested and no Dark backend exists.
    DarkTargetUnavailable,
    /// Rule 2: the admission log is not final.
    AdmissionLogNotFinal,
    /// Rule 3: the witness slots do not open to the accepted-input root.
    RootBindingAbsent,
    /// Rule 4: a conflicting finalized root exists for this relation/batch/market.
    RootEquivocation,
    /// Rule 5: an admitted payload is not recoverable by the availability threshold.
    PayloadUnavailable,
    /// Rule 6: a slot is bound to a different batch identifier.
    BatchBindingMismatch {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// Rule 7: a slot is bound to a different market identifier.
    MarketBindingMismatch {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// Rule 8: a slot names an owner outside `0..OWNERS`.
    OwnerOutOfDomain {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// Rule 9: a slot carries a quantity outside `QUANTITY_FLOOR..=QUANTITY_CEILING`.
    QuantityOutOfDomain {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// Rule 10: a slot names a tick outside `0..TICKS`.
    LimitOutOfDomain {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// Rule 11: a slot carries a zero nullifier.
    NullifierZero {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// Rule 12: two slots carry the same nullifier; `slot` is the later repeat.
    NullifierRepeated {
        /// Diagnostic slot index of the later repeat.
        slot: u8,
        /// Diagnostic slot index of the first use.
        first: u8,
    },
    /// Rule 13: a slot arrived after the cutoff.
    LateArrival {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// Rule 14: a slot's authorization statement is absent.
    Unauthorized {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// Rule 15: a slot's eligibility statement is absent.
    Ineligible {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// Rule 16: a slot's exact-inclusion statement is absent.
    InclusionAbsent {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// Rule 17: a slot's custody-binding statement is absent, so its reservation
    /// is not known to refer to distinct, non-double-counted custody.
    CustodyBindingAbsent {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// Rule 18: a slot's reservation does not cover its worst-case obligation.
    ReservationInsufficient {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// Post-admission: an accumulator could not represent an intermediate sum
    /// exactly. Unreachable for any batch this module admits.
    AccumulatorOverflow,
}

impl Refusal {
    /// The public class tag, with the diagnostic slot index dropped.
    ///
    /// The frozen leakage declaration forbids secret-bearing failure
    /// diagnostics, so only this tag may cross the public boundary. Section 4.1
    /// freezes which rule is reported, not this spelling of it.
    pub fn class(self) -> &'static str {
        match self {
            Self::DarkTargetUnavailable => "dark-target-unavailable",
            Self::AdmissionLogNotFinal => "admission-log-not-final",
            Self::RootBindingAbsent => "root-binding-absent",
            Self::RootEquivocation => "root-equivocation",
            Self::PayloadUnavailable => "payload-unavailable",
            Self::BatchBindingMismatch { .. } => "batch-binding-mismatch",
            Self::MarketBindingMismatch { .. } => "market-binding-mismatch",
            Self::OwnerOutOfDomain { .. } => "owner-out-of-domain",
            Self::QuantityOutOfDomain { .. } => "quantity-out-of-domain",
            Self::LimitOutOfDomain { .. } => "limit-out-of-domain",
            Self::NullifierZero { .. } => "nullifier-zero",
            Self::NullifierRepeated { .. } => "nullifier-repeated",
            Self::LateArrival { .. } => "late-arrival",
            Self::Unauthorized { .. } => "unauthorized",
            Self::Ineligible { .. } => "ineligible",
            Self::InclusionAbsent { .. } => "inclusion-absent",
            Self::CustodyBindingAbsent { .. } => "custody-binding-absent",
            Self::ReservationInsufficient { .. } => "reservation-insufficient",
            Self::AccumulatorOverflow => "accumulator-overflow",
        }
    }

    /// The rule number of section 4.1 this class reports, or `None` for a
    /// post-admission class that no admission rule can produce.
    ///
    /// Exposed so a test can assert that a batch violating several rules is
    /// refused with the lowest-numbered rule it violates.
    pub fn rule(self) -> Option<u8> {
        Some(match self {
            Self::DarkTargetUnavailable => 1,
            Self::AdmissionLogNotFinal => 2,
            Self::RootBindingAbsent => 3,
            Self::RootEquivocation => 4,
            Self::PayloadUnavailable => 5,
            Self::BatchBindingMismatch { .. } => 6,
            Self::MarketBindingMismatch { .. } => 7,
            Self::OwnerOutOfDomain { .. } => 8,
            Self::QuantityOutOfDomain { .. } => 9,
            Self::LimitOutOfDomain { .. } => 10,
            Self::NullifierZero { .. } => 11,
            Self::NullifierRepeated { .. } => 12,
            Self::LateArrival { .. } => 13,
            Self::Unauthorized { .. } => 14,
            Self::Ineligible { .. } => 15,
            Self::InclusionAbsent { .. } => 16,
            Self::CustodyBindingAbsent { .. } => 17,
            Self::ReservationInsufficient { .. } => 18,
            Self::AccumulatorOverflow => return None,
        })
    }
}

/// An occupied slot that passed every admission rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Admitted {
    /// Canonical rank: the slot's position in the frozen admission log.
    pub slot: u8,
    /// Owner index, known in domain.
    pub owner: u8,
    /// Side of the book.
    pub direction: Direction,
    /// Tick index, known in domain.
    pub limit_index: u8,
    /// Quantity in base atoms, known in domain.
    pub quantity: u32,
    /// Reserved amount in the spending asset.
    pub reserved: u64,
}

/// The admitted book: slot-indexed, so vacancy stays positional.
pub type AdmittedBook = [Option<Admitted>; SLOTS];

/// The worst-case obligation a slot must have reserved.
///
/// A buy reserves quote at its own limit price; a sell reserves base.
pub fn required_reservation(direction: Direction, limit_index: u8, quantity: u32) -> u64 {
    match direction {
        Direction::Buy => u64::from(quantity) * u64::from(TICK_PRICES[limit_index as usize]),
        Direction::Sell => u64::from(quantity),
    }
}

/// Apply rules 6 through 18 to one occupied slot, in frozen order.
///
/// `earlier` is the prefix of slots strictly before this one. Rule 12 is
/// evaluated against that prefix rather than in a later pass, so a repeated
/// nullifier is reported at the later slot and only after rules 6 through 11
/// have passed on it.
fn screen_order(
    slot: u8,
    order: &Order,
    batch: &Batch,
    earlier: &[Slot],
) -> Result<Admitted, Refusal> {
    // 6
    if order.batch != batch.batch {
        return Err(Refusal::BatchBindingMismatch { slot });
    }
    // 7
    if order.market != batch.market {
        return Err(Refusal::MarketBindingMismatch { slot });
    }
    // 8
    if order.owner >= OWNERS {
        return Err(Refusal::OwnerOutOfDomain { slot });
    }
    // 9
    if order.quantity < QUANTITY_FLOOR || order.quantity > QUANTITY_CEILING {
        return Err(Refusal::QuantityOutOfDomain { slot });
    }
    // 10. Rules 9 and 10 both precede rule 18, which indexes the tick grid.
    if usize::from(order.limit_index) >= TICKS {
        return Err(Refusal::LimitOutOfDomain { slot });
    }
    // 11
    if order.nullifier == 0 {
        return Err(Refusal::NullifierZero { slot });
    }
    // 12
    for (index, other) in earlier.iter().enumerate() {
        let Slot::Taken(other) = other else { continue };
        if other.nullifier == order.nullifier {
            return Err(Refusal::NullifierRepeated {
                slot,
                first: index as u8,
            });
        }
    }
    // 13
    if order.arrival > batch.cutoff {
        return Err(Refusal::LateArrival { slot });
    }
    // 14
    if !order.authorized {
        return Err(Refusal::Unauthorized { slot });
    }
    // 15
    if !order.eligible {
        return Err(Refusal::Ineligible { slot });
    }
    // 16
    if !order.included {
        return Err(Refusal::InclusionAbsent { slot });
    }
    // 17
    if !order.custody_bound {
        return Err(Refusal::CustodyBindingAbsent { slot });
    }
    // 18
    if order.reserved < required_reservation(order.direction, order.limit_index, order.quantity) {
        return Err(Refusal::ReservationInsufficient { slot });
    }
    Ok(Admitted {
        slot,
        owner: order.owner,
        direction: order.direction,
        limit_index: order.limit_index,
        quantity: order.quantity,
        reserved: order.reserved,
    })
}

/// Screen the whole batch in the frozen check order.
///
/// Rule 1 is the requested mode, rules 2 through 5 are the four boundary
/// statements, and rules 6 through 18 are per-slot, applied slot-major in
/// ascending slot order with the first failing rule of the first failing slot
/// deciding the class. `DARK_FBA_RELATION.md` section 4.1 freezes that order
/// for v0; it is not a local choice, because the class is a public output.
pub fn screen(batch: &Batch) -> Result<AdmittedBook, Refusal> {
    if batch.mode == Mode::DarkTarget {
        return Err(Refusal::DarkTargetUnavailable);
    }
    if !batch.boundary.log_final {
        return Err(Refusal::AdmissionLogNotFinal);
    }
    if !batch.boundary.root_binds_slots {
        return Err(Refusal::RootBindingAbsent);
    }
    if !batch.boundary.no_conflicting_root {
        return Err(Refusal::RootEquivocation);
    }
    if !batch.boundary.payloads_available {
        return Err(Refusal::PayloadUnavailable);
    }

    let mut admitted: AdmittedBook = [None; SLOTS];
    for (index, slot) in batch.slots.iter().enumerate() {
        if let Slot::Taken(order) = slot {
            admitted[index] = Some(screen_order(
                index as u8,
                order,
                batch,
                &batch.slots[..index],
            )?);
        }
    }

    Ok(admitted)
}
