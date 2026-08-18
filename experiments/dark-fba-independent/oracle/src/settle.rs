//! Local outputs and the conservation audit.

use crate::admit::{Admitted, AdmittedBook, Refusal};
use crate::apportion::{Claim, largest_remainder};
use crate::book::Direction;
use crate::curve::Clearing;
use crate::params::{OWNERS, SLOTS};

/// One owner's local output at the selected public price.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct OwnerOutput {
    /// Base atoms received.
    pub bought_base: u32,
    /// Base atoms delivered.
    pub sold_base: u32,
    /// Signed base delta.
    pub base_delta: i64,
    /// Signed quote delta at the selected public price.
    pub quote_delta: i64,
    /// Base reservation returned to the owner.
    pub released_base_reservation: u64,
    /// Quote reservation returned to the owner.
    pub released_quote_reservation: u64,
}

/// The full settled result of one batch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settlement {
    /// Selected tick and volume, or the no-trade tag.
    pub clearing: Clearing,
    /// Base atoms filled, indexed by slot; vacant and ineligible slots are zero.
    pub fills: [u32; SLOTS],
    /// Local outputs, indexed by owner.
    pub owners: [OwnerOutput; OWNERS as usize],
}

/// A violated global settlement property.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Breach {
    /// A vacant slot was filled.
    FillOnVacantSlot {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// A fill exceeds its order quantity.
    FillAboveQuantity {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// A fill sits on an order the selected tick does not satisfy.
    FillViolatesLimit {
        /// Diagnostic slot index.
        slot: u8,
    },
    /// Bought base atoms do not sum to the public volume.
    BoughtVolumeMismatch,
    /// Sold base atoms do not sum to the public volume.
    SoldVolumeMismatch,
    /// Base deltas do not sum to zero.
    BaseNotConserved,
    /// Quote deltas do not sum to zero at zero fees.
    QuoteNotConserved,
    /// An owner spent more than the reservation backing its orders.
    ReservationExceeded {
        /// Diagnostic owner index.
        owner: u8,
    },
}

fn eligible(order: &Admitted, tick: u8) -> bool {
    match order.direction {
        Direction::Buy => order.limit_index >= tick,
        Direction::Sell => order.limit_index <= tick,
    }
}

fn side_claims(book: &AdmittedBook, direction: Direction, tick: u8) -> ([Claim; SLOTS], usize) {
    let mut claims = [Claim {
        rank: 0,
        quantity: 0,
    }; SLOTS];
    let mut count = 0usize;
    for order in book.iter().flatten() {
        if order.direction == direction && eligible(order, tick) {
            claims[count] = Claim {
                rank: order.slot,
                quantity: order.quantity,
            };
            count += 1;
        }
    }
    (claims, count)
}

fn side_slots(book: &AdmittedBook, direction: Direction, tick: u8) -> ([u8; SLOTS], usize) {
    let mut slots = [0u8; SLOTS];
    let mut count = 0usize;
    for order in book.iter().flatten() {
        if order.direction == direction && eligible(order, tick) {
            slots[count] = order.slot;
            count += 1;
        }
    }
    (slots, count)
}

/// Allocate the selected volume and compute every local output.
pub fn settle(book: &AdmittedBook, clearing: Clearing) -> Result<Settlement, Refusal> {
    let mut fills = [0u32; SLOTS];
    if let Clearing::Trade { tick, volume, .. } = clearing {
        for direction in [Direction::Buy, Direction::Sell] {
            let (claims, count) = side_claims(book, direction, tick);
            let (slots, _) = side_slots(book, direction, tick);
            let award =
                largest_remainder(&claims[..count], volume).ok_or(Refusal::AccumulatorOverflow)?;
            for index in 0..count {
                fills[usize::from(slots[index])] = award[index];
            }
        }
    }

    let price = match clearing {
        Clearing::NoTrade => 0,
        Clearing::Trade { price, .. } => price,
    };

    let mut owners = [OwnerOutput::default(); OWNERS as usize];
    for order in book.iter().flatten() {
        let fill = fills[usize::from(order.slot)];
        let owner = &mut owners[usize::from(order.owner)];
        // Bounded by 15 * 4 for one slot, so both widths are exact.
        let quote = fill * price;
        match order.direction {
            Direction::Buy => {
                owner.bought_base += fill;
                owner.base_delta += i64::from(fill);
                owner.quote_delta -= i64::from(quote);
                owner.released_quote_reservation += order.reserved - u64::from(quote);
            }
            Direction::Sell => {
                owner.sold_base += fill;
                owner.base_delta -= i64::from(fill);
                owner.quote_delta += i64::from(quote);
                owner.released_base_reservation += order.reserved - u64::from(fill);
            }
        }
    }

    Ok(Settlement {
        clearing,
        fills,
        owners,
    })
}

/// Check the global settlement properties the relation must preserve.
///
/// This is an independent re-derivation from the admitted book, not a replay of
/// [`settle`]; it is the falsifier for every allocation change.
pub fn audit(book: &AdmittedBook, settlement: &Settlement) -> Result<(), Breach> {
    let (tick, price, volume) = match settlement.clearing {
        Clearing::NoTrade => (None, 0u32, 0u32),
        Clearing::Trade {
            tick,
            price,
            volume,
        } => (Some(tick), price, volume),
    };

    let mut bought = 0u32;
    let mut sold = 0u32;
    for (slot, entry) in book.iter().enumerate() {
        let fill = settlement.fills[slot];
        let Some(order) = entry else {
            if fill != 0 {
                return Err(Breach::FillOnVacantSlot { slot: slot as u8 });
            }
            continue;
        };
        if fill > order.quantity {
            return Err(Breach::FillAboveQuantity { slot: slot as u8 });
        }
        if fill > 0 {
            match tick {
                None => return Err(Breach::FillViolatesLimit { slot: slot as u8 }),
                Some(tick) if !eligible(order, tick) => {
                    return Err(Breach::FillViolatesLimit { slot: slot as u8 });
                }
                Some(_) => {}
            }
        }
        match order.direction {
            Direction::Buy => bought += fill,
            Direction::Sell => sold += fill,
        }
    }

    if bought != volume {
        return Err(Breach::BoughtVolumeMismatch);
    }
    if sold != volume {
        return Err(Breach::SoldVolumeMismatch);
    }

    let base: i64 = settlement.owners.iter().map(|owner| owner.base_delta).sum();
    if base != 0 {
        return Err(Breach::BaseNotConserved);
    }
    let quote: i64 = settlement
        .owners
        .iter()
        .map(|owner| owner.quote_delta)
        .sum();
    if quote != 0 {
        return Err(Breach::QuoteNotConserved);
    }

    let mut reserved_quote = [0u64; OWNERS as usize];
    let mut reserved_base = [0u64; OWNERS as usize];
    for order in book.iter().flatten() {
        match order.direction {
            Direction::Buy => reserved_quote[usize::from(order.owner)] += order.reserved,
            Direction::Sell => reserved_base[usize::from(order.owner)] += order.reserved,
        }
    }
    for owner in 0..OWNERS as usize {
        let spent_quote = u64::from(settlement.owners[owner].bought_base) * u64::from(price);
        let spent_base = u64::from(settlement.owners[owner].sold_base);
        if spent_quote + settlement.owners[owner].released_quote_reservation
            != reserved_quote[owner]
            || spent_base + settlement.owners[owner].released_base_reservation
                != reserved_base[owner]
        {
            return Err(Breach::ReservationExceeded { owner: owner as u8 });
        }
    }

    Ok(())
}
