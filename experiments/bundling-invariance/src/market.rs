//! The collateral ledger, so that "costless" is a computed fact rather than an
//! assertion.
//!
//! One market has one exhaustive partition, one collateral pool, and a set of
//! positions. Claims enter existence in exactly one way — a deposit mints one
//! unit of every cell's claim against one unit of collateral — and leave in
//! exactly one way — a recombination burns one unit of every cell's claim and
//! releases one unit of collateral. Transfers, splits, and bundles move claims
//! between positions and touch collateral not at all.
//!
//! The conservation identity the whole experiment leans on falls straight out of
//! that: the claims outstanding in a cell are the same number in every cell, and
//! that number is the collateral locked. Bundling and unbundling therefore cost
//! nothing at the market level, which is what makes a criterion that answers
//! differently for a bundle and for its parts an arbitrage rather than a
//! curiosity.

use crate::payoff::Payoff;
use std::fmt;

/// A market: one partition, one collateral pool, and its positions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Market {
    cells: usize,
    positions: Vec<Payoff>,
    collateral_locked: u64,
}

/// One operation a holder or the market can perform.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Op {
    /// Lock `units` of collateral and mint `units` complete sets into `holder`.
    Deposit {
        /// Position receiving the complete sets.
        holder: usize,
        /// Number of complete sets.
        units: u64,
    },
    /// Burn `units` complete sets from `holder` and release `units` collateral.
    Recombine {
        /// Position surrendering the complete sets.
        holder: usize,
        /// Number of complete sets.
        units: u64,
    },
    /// Move `units` of the claim on `cell` from one position to another.
    Transfer {
        /// Position paying the claims.
        from: usize,
        /// Position receiving the claims.
        to: usize,
        /// Cell whose claim moves.
        cell: usize,
        /// Number of claims.
        units: u64,
    },
    /// Move `take` out of `holder` into a new position.
    Split {
        /// Position being divided.
        holder: usize,
        /// The part moved into the new position.
        take: Payoff,
    },
    /// Move everything in `from` into `to`.
    Bundle {
        /// Position absorbing the claims.
        to: usize,
        /// Position being emptied.
        from: usize,
    },
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Deposit { holder, units } => write!(f, "deposit holder={holder} units={units}"),
            Self::Recombine { holder, units } => {
                write!(f, "recombine holder={holder} units={units}")
            }
            Self::Transfer {
                from,
                to,
                cell,
                units,
            } => write!(f, "transfer from={from} to={to} cell={cell} units={units}"),
            Self::Split { holder, take } => write!(f, "split holder={holder} take={take}"),
            Self::Bundle { to, from } => write!(f, "bundle to={to} from={from}"),
        }
    }
}

/// Why an operation was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarketError {
    /// The position index does not exist.
    NoSuchPosition,
    /// The cell index is outside the partition.
    NoSuchCell,
    /// The position does not hold what the operation would move.
    Insufficient,
    /// The operands are over different partitions.
    CellMismatch,
}

impl fmt::Display for MarketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSuchPosition => f.write_str("no such position"),
            Self::NoSuchCell => f.write_str("no such cell"),
            Self::Insufficient => f.write_str("the position does not hold that much"),
            Self::CellMismatch => f.write_str("partition mismatch"),
        }
    }
}

impl std::error::Error for MarketError {}

impl Market {
    /// Open a market over `cells` cells with `holders` empty positions.
    ///
    /// # Panics
    ///
    /// Panics if fewer than two cells are requested.
    #[must_use]
    pub fn open(cells: usize, holders: usize) -> Self {
        assert!(cells >= 2, "a partition needs at least two cells");
        Self {
            cells,
            positions: (0..holders).map(|_| Payoff::zero(cells)).collect(),
            collateral_locked: 0,
        }
    }

    /// Number of cells in the partition.
    #[must_use]
    pub const fn cells(&self) -> usize {
        self.cells
    }

    /// The positions, in creation order.
    #[must_use]
    pub fn positions(&self) -> &[Payoff] {
        &self.positions
    }

    /// Collateral locked in the pool.
    #[must_use]
    pub const fn collateral_locked(&self) -> u64 {
        self.collateral_locked
    }

    /// Claims outstanding, cell by cell, across every position.
    #[must_use]
    pub fn outstanding(&self) -> Payoff {
        let mut total = Payoff::zero(self.cells);
        for position in &self.positions {
            total = total.add(position).expect("positions share the partition");
        }
        total
    }

    /// The largest amount the market can be called on to pay in any one state.
    #[must_use]
    pub fn backing_required(&self) -> u64 {
        self.outstanding().max_payout()
    }

    /// The conservation identity: claims outstanding are level across cells and
    /// equal to the collateral locked.
    ///
    /// Every operation in [`Op`] preserves this, and the exhaustive test over
    /// operation sequences says so.
    #[must_use]
    pub fn conserved(&self) -> bool {
        let outstanding = self.outstanding();
        outstanding.is_constant() && outstanding.get(0) == self.collateral_locked
    }

    /// Apply one operation.
    ///
    /// # Errors
    ///
    /// Returns a [`MarketError`] if the operation names something that does not
    /// exist or moves more than a position holds.
    pub fn apply(&mut self, op: &Op) -> Result<(), MarketError> {
        match op {
            Op::Deposit { holder, units } => {
                let position = self
                    .positions
                    .get_mut(*holder)
                    .ok_or(MarketError::NoSuchPosition)?;
                *position = position
                    .add(&Payoff::complete_set(self.cells, *units))
                    .map_err(|_| MarketError::CellMismatch)?;
                self.collateral_locked += units;
                Ok(())
            }
            Op::Recombine { holder, units } => {
                let position = self
                    .positions
                    .get_mut(*holder)
                    .ok_or(MarketError::NoSuchPosition)?;
                let complete = Payoff::complete_set(self.cells, *units);
                *position = position
                    .sub(&complete)
                    .map_err(|_| MarketError::Insufficient)?;
                self.collateral_locked -= units;
                Ok(())
            }
            Op::Transfer {
                from,
                to,
                cell,
                units,
            } => {
                if *from >= self.positions.len() || *to >= self.positions.len() {
                    return Err(MarketError::NoSuchPosition);
                }
                if *cell >= self.cells {
                    return Err(MarketError::NoSuchCell);
                }
                if self.positions[*from].get(*cell) < *units {
                    return Err(MarketError::Insufficient);
                }
                if from == to {
                    return Ok(());
                }
                let mut amounts = vec![0_u64; self.cells];
                amounts[*cell] = *units;
                let moved = Payoff::new(amounts);
                self.positions[*from] = self.positions[*from]
                    .sub(&moved)
                    .map_err(|_| MarketError::Insufficient)?;
                self.positions[*to] = self.positions[*to]
                    .add(&moved)
                    .map_err(|_| MarketError::CellMismatch)?;
                Ok(())
            }
            Op::Split { holder, take } => {
                if *holder >= self.positions.len() {
                    return Err(MarketError::NoSuchPosition);
                }
                if take.cells() != self.cells {
                    return Err(MarketError::CellMismatch);
                }
                self.positions[*holder] = self.positions[*holder]
                    .sub(take)
                    .map_err(|_| MarketError::Insufficient)?;
                self.positions.push(take.clone());
                Ok(())
            }
            Op::Bundle { to, from } => {
                if *to >= self.positions.len() || *from >= self.positions.len() {
                    return Err(MarketError::NoSuchPosition);
                }
                if to == from {
                    return Ok(());
                }
                let moved = self.positions[*from].clone();
                self.positions[*from] = Payoff::zero(self.cells);
                self.positions[*to] = self.positions[*to]
                    .add(&moved)
                    .map_err(|_| MarketError::CellMismatch)?;
                Ok(())
            }
        }
    }

    /// Whether the operation moves collateral. Only deposit and recombination do.
    #[must_use]
    pub const fn moves_collateral(op: &Op) -> bool {
        matches!(op, Op::Deposit { .. } | Op::Recombine { .. })
    }
}
