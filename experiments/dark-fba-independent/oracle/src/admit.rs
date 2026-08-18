//! Admission: every occupied slot must pass every rule or the whole batch
//! refuses. A malformed batch is never reinterpreted as no trade.

use crate::book::{Batch, Direction, Mode, Order, Slot};
use crate::params::{OWNERS, QUANTITY_CEILING, QUANTITY_FLOOR, SLOTS, TICK_PRICES, TICKS};

/// One typed public refusal class.
///
/// The slot index carried by a per-slot class is diagnostic; the frozen public
/// boundary publishes only [`Refusal::class`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// `DarkTarget` was requested and no Dark backend exists.
    DarkTargetUnavailable,
    /// The admission log is not final.
    AdmissionLogNotFinal,
    /// The witness slots do not open to the accepted-input root.
    RootBindingAbsent,
    /// A conflicting finalized root exists for this relation/batch/market.
    RootEquivocation,
    /// An admitted payload is not recoverable by the availability threshold.
    PayloadUnavailable,
    /// A slot is bound to a different batch identifier.
    BatchBindingMismatch {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// A slot is bound to a different market identifier.
    MarketBindingMismatch {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// A slot names an owner outside `0..OWNERS`.
    OwnerOutOfDomain {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// A slot names a tick outside `0..TICKS`.
    LimitOutOfDomain {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// A slot carries a quantity outside `QUANTITY_FLOOR..=QUANTITY_CEILING`.
    QuantityOutOfDomain {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// A slot arrived after the cutoff.
    LateArrival {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// A slot's authorization statement is absent.
    Unauthorized {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// A slot's eligibility statement is absent.
    Ineligible {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// A slot's exact-inclusion statement is absent.
    InclusionAbsent {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// A slot carries a zero nullifier.
    NullifierZero {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// Two slots carry the same nullifier; `slot` is the later repeat.
    NullifierRepeated {
        /// Diagnostic slot index of the later repeat.
        slot: u8,
        /// Diagnostic slot index of the first use.
        first: u8,
    },
    /// A slot's reservation does not cover its worst-case obligation.
    ReservationInsufficient {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// An accumulator could not represent an intermediate sum exactly.
    AccumulatorOverflow,
}

impl Refusal {
    /// The public class tag, with the diagnostic slot index dropped.
    ///
    /// The frozen leakage declaration forbids secret-bearing failure
    /// diagnostics, so only this tag may cross the public boundary.
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
            Self::LimitOutOfDomain { .. } => "limit-out-of-domain",
            Self::QuantityOutOfDomain { .. } => "quantity-out-of-domain",
            Self::LateArrival { .. } => "late-arrival",
            Self::Unauthorized { .. } => "unauthorized",
            Self::Ineligible { .. } => "ineligible",
            Self::InclusionAbsent { .. } => "inclusion-absent",
            Self::NullifierZero { .. } => "nullifier-zero",
            Self::NullifierRepeated { .. } => "nullifier-repeated",
            Self::ReservationInsufficient { .. } => "reservation-insufficient",
            Self::AccumulatorOverflow => "accumulator-overflow",
        }
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

fn screen_order(slot: u8, order: &Order, batch: &Batch) -> Result<Admitted, Refusal> {
    if order.batch != batch.batch {
        return Err(Refusal::BatchBindingMismatch { slot });
    }
    if order.market != batch.market {
        return Err(Refusal::MarketBindingMismatch { slot });
    }
    if order.owner >= OWNERS {
        return Err(Refusal::OwnerOutOfDomain { slot });
    }
    if usize::from(order.limit_index) >= TICKS {
        return Err(Refusal::LimitOutOfDomain { slot });
    }
    if order.quantity < QUANTITY_FLOOR || order.quantity > QUANTITY_CEILING {
        return Err(Refusal::QuantityOutOfDomain { slot });
    }
    if order.arrival > batch.cutoff {
        return Err(Refusal::LateArrival { slot });
    }
    if !order.authorized {
        return Err(Refusal::Unauthorized { slot });
    }
    if !order.eligible {
        return Err(Refusal::Ineligible { slot });
    }
    if !order.included {
        return Err(Refusal::InclusionAbsent { slot });
    }
    if order.nullifier == 0 {
        return Err(Refusal::NullifierZero { slot });
    }
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

/// Screen the whole batch.
///
/// Check priority is: requested mode, then the four boundary statements, then
/// per-slot rules in slot order, then batch-scoped nullifier uniqueness. The
/// specification fixes which books refuse, not which class wins when a witness
/// violates several rules at once; this priority is therefore a local choice.
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
            admitted[index] = Some(screen_order(index as u8, order, batch)?);
        }
    }

    for (index, slot) in batch.slots.iter().enumerate() {
        let Slot::Taken(order) = slot else { continue };
        for (earlier, other) in batch.slots.iter().enumerate().take(index) {
            let Slot::Taken(other) = other else { continue };
            if other.nullifier == order.nullifier {
                return Err(Refusal::NullifierRepeated {
                    slot: index as u8,
                    first: earlier as u8,
                });
            }
        }
    }

    Ok(admitted)
}
