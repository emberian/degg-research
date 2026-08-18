//! Aggregate curves and the maximum-volume / ties-low tick selection.

use crate::admit::{AdmittedBook, Refusal};
use crate::book::Direction;
use crate::params::{TICK_PRICES, TICKS};

/// The three aggregate curves of one batch, indexed by tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Curves {
    /// `Demand[k] = sum of admitted buy quantities with limit >= k`.
    pub demand: [u32; TICKS],
    /// `Supply[k] = sum of admitted sell quantities with limit <= k`.
    pub supply: [u32; TICKS],
    /// `Volume[k] = min(Demand[k], Supply[k])`.
    pub volume: [u32; TICKS],
}

/// The selected clearing outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clearing {
    /// Maximum volume is zero: the batch is a valid no-trade, not a refusal.
    NoTrade,
    /// The batch trades `volume` base atoms at `TICK_PRICES[tick]`.
    Trade {
        /// Selected tick index.
        tick: u8,
        /// Quote atoms per base atom at the selected tick.
        price: u32,
        /// Aggregate matched volume.
        volume: u32,
    },
}

/// Build the aggregate curves.
///
/// Sums use checked addition: an accumulator that cannot represent a partial
/// sum exactly is a refusal, never a modular wrap.
pub fn curves(book: &AdmittedBook) -> Result<Curves, Refusal> {
    let mut demand = [0u32; TICKS];
    let mut supply = [0u32; TICKS];
    for order in book.iter().flatten() {
        let limit = usize::from(order.limit_index);
        match order.direction {
            // A buy at limit L is willing to pay every tick k <= L.
            Direction::Buy => {
                for slot in demand.iter_mut().take(limit + 1) {
                    *slot = slot
                        .checked_add(order.quantity)
                        .ok_or(Refusal::AccumulatorOverflow)?;
                }
            }
            // A sell at limit L is willing to receive every tick k >= L.
            Direction::Sell => {
                for slot in supply.iter_mut().skip(limit) {
                    *slot = slot
                        .checked_add(order.quantity)
                        .ok_or(Refusal::AccumulatorOverflow)?;
                }
            }
        }
    }
    let mut volume = [0u32; TICKS];
    for k in 0..TICKS {
        volume[k] = demand[k].min(supply[k]);
    }
    Ok(Curves {
        demand,
        supply,
        volume,
    })
}

/// Select the clearing tick: maximum volume, ties to the lowest tick index.
///
/// The scan runs ascending with a strict `>` update, so the first tick reaching
/// the maximum wins without any secret-dependent sort.
pub fn select(curves: &Curves) -> Clearing {
    let mut best = 0usize;
    for k in 1..TICKS {
        if curves.volume[k] > curves.volume[best] {
            best = k;
        }
    }
    if curves.volume[best] == 0 {
        return Clearing::NoTrade;
    }
    Clearing::Trade {
        tick: best as u8,
        price: TICK_PRICES[best],
        volume: curves.volume[best],
    }
}
