//! The payoff object: an exhaustive partition of one reference variable, a
//! nonnegative integer payoff vector over its cells, and the economic facts a
//! classification criterion is permitted to read.
//!
//! Everything here is exact integer arithmetic. There is no price, no discount
//! rate, no probability, and no market data; a payoff object says only what it
//! pays in each state of the world and what its settlement amount is a function
//! of.

use std::fmt;

/// Smallest cell count in the enumerated corpus.
pub const MIN_CELLS: usize = 2;

/// Largest cell count in the enumerated corpus.
pub const MAX_CELLS: usize = 5;

/// Largest per-cell payout in the enumerated corpus.
pub const PAYOUT_CEILING: u64 = 3;

/// What a settlement amount is a function of.
///
/// This is the fact Question 8 turns on: whether the amount is a function of
/// the price or value of a security or index, or of something about an issuer
/// that is not a price.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReferenceKind {
    /// The price or value of one security.
    SecurityPrice,
    /// The value of a group or index of securities.
    IndexValue,
    /// A fact about an issuer that is not the price or value of a security.
    IssuerFact,
}

impl ReferenceKind {
    /// Every reference kind, in corpus order.
    pub const ALL: [Self; 3] = [Self::SecurityPrice, Self::IndexValue, Self::IssuerFact];

    /// Stable lower-case name used in the corpus file.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::SecurityPrice => "security_price",
            Self::IndexValue => "index_value",
            Self::IssuerFact => "issuer_fact",
        }
    }
}

impl fmt::Display for ReferenceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

/// The economic facts of a payoff object, other than its payoff vector.
///
/// A criterion may read any of these. Nothing else about the object exists.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Facts {
    /// What the settlement amount is a function of.
    pub reference: ReferenceKind,
    /// Whether the maximum payout the terms allow is locked against collateral
    /// before the claim exists.
    pub funded: bool,
    /// Whether the claim may be transferred before resolution.
    pub transferable: bool,
}

impl Facts {
    /// Build a fact profile.
    #[must_use]
    pub const fn new(reference: ReferenceKind, funded: bool, transferable: bool) -> Self {
        Self {
            reference,
            funded,
            transferable,
        }
    }

    /// Stable rendering used in the corpus file.
    #[must_use]
    pub fn label(self) -> String {
        format!(
            "reference={} funded={} transferable={}",
            self.reference, self.funded, self.transferable
        )
    }
}

/// A nonnegative integer payoff vector over the cells of one partition.
///
/// Entry `i` is the amount the object pays if cell `i` is the realized state.
/// The partition is exhaustive and non-overlapping by construction: exactly one
/// cell is realized, so exactly one entry is paid.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Payoff {
    amounts: Vec<u64>,
}

impl Payoff {
    /// Build a payoff vector from per-cell amounts.
    ///
    /// # Panics
    ///
    /// Panics if fewer than two cells are supplied; a one-cell partition has no
    /// contingency and no decomposition worth studying.
    #[must_use]
    pub fn new(amounts: Vec<u64>) -> Self {
        assert!(amounts.len() >= 2, "a partition needs at least two cells");
        Self { amounts }
    }

    /// The all-zero payoff over `cells` cells.
    #[must_use]
    pub fn zero(cells: usize) -> Self {
        Self::new(vec![0; cells])
    }

    /// The elementary claim paying one unit if cell `cell` is realized.
    ///
    /// # Panics
    ///
    /// Panics if `cell` is not a cell of the partition.
    #[must_use]
    pub fn unit(cells: usize, cell: usize) -> Self {
        assert!(cell < cells, "cell index outside the partition");
        let mut amounts = vec![0; cells];
        amounts[cell] = 1;
        Self::new(amounts)
    }

    /// `units` complete sets: one unit of every cell's claim.
    #[must_use]
    pub fn complete_set(cells: usize, units: u64) -> Self {
        Self::new(vec![units; cells])
    }

    /// Number of cells in the partition.
    #[must_use]
    pub fn cells(&self) -> usize {
        self.amounts.len()
    }

    /// The per-cell amounts.
    #[must_use]
    pub fn amounts(&self) -> &[u64] {
        &self.amounts
    }

    /// The amount paid if cell `cell` is realized.
    ///
    /// # Panics
    ///
    /// Panics if `cell` is not a cell of the partition.
    #[must_use]
    pub fn get(&self, cell: usize) -> u64 {
        self.amounts[cell]
    }

    /// Sum of the per-cell amounts. Not a payout; only a size measure used to
    /// order witnesses.
    #[must_use]
    pub fn total(&self) -> u64 {
        self.amounts.iter().sum()
    }

    /// Largest amount payable in any state.
    #[must_use]
    pub fn max_payout(&self) -> u64 {
        self.amounts.iter().copied().max().unwrap_or(0)
    }

    /// Smallest amount payable in any state, which is also the number of
    /// complete sets the position contains.
    #[must_use]
    pub fn min_payout(&self) -> u64 {
        self.amounts.iter().copied().min().unwrap_or(0)
    }

    /// Whether the object pays nothing in every state.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.amounts.iter().all(|amount| *amount == 0)
    }

    /// Whether the object pays the same amount in every state.
    #[must_use]
    pub fn is_constant(&self) -> bool {
        self.amounts.windows(2).all(|pair| pair[0] == pair[1])
    }

    /// The distinct amounts appearing in the vector, ascending.
    #[must_use]
    pub fn distinct_values(&self) -> Vec<u64> {
        let mut values = self.amounts.clone();
        values.sort_unstable();
        values.dedup();
        values
    }

    /// Bitmask of the cells in which the object pays something.
    #[must_use]
    pub fn support_mask(&self) -> u32 {
        let mut mask = 0;
        for (cell, amount) in self.amounts.iter().enumerate() {
            if *amount > 0 {
                mask |= 1 << cell;
            }
        }
        mask
    }

    /// Cells in which the object pays something, ascending.
    #[must_use]
    pub fn support(&self) -> Vec<usize> {
        (0..self.cells())
            .filter(|cell| self.amounts[*cell] > 0)
            .collect()
    }

    /// Cell-by-cell sum.
    ///
    /// # Errors
    ///
    /// Returns [`OpError::CellMismatch`] if the partitions differ.
    pub fn add(&self, other: &Self) -> Result<Self, OpError> {
        if self.cells() != other.cells() {
            return Err(OpError::CellMismatch {
                left: self.cells(),
                right: other.cells(),
            });
        }
        let amounts = self
            .amounts
            .iter()
            .zip(other.amounts.iter())
            .map(|(left, right)| left + right)
            .collect();
        Ok(Self::new(amounts))
    }

    /// Cell-by-cell difference.
    ///
    /// # Errors
    ///
    /// Returns [`OpError::CellMismatch`] if the partitions differ and
    /// [`OpError::PartExceedsWhole`] if `other` is not dominated cell by cell.
    pub fn sub(&self, other: &Self) -> Result<Self, OpError> {
        if self.cells() != other.cells() {
            return Err(OpError::CellMismatch {
                left: self.cells(),
                right: other.cells(),
            });
        }
        if !other.dominated_by(self) {
            return Err(OpError::PartExceedsWhole);
        }
        let amounts = self
            .amounts
            .iter()
            .zip(other.amounts.iter())
            .map(|(left, right)| left - right)
            .collect();
        Ok(Self::new(amounts))
    }

    /// Whether every entry of `self` is at most the matching entry of `other`.
    #[must_use]
    pub fn dominated_by(&self, other: &Self) -> bool {
        self.cells() == other.cells()
            && self
                .amounts
                .iter()
                .zip(other.amounts.iter())
                .all(|(left, right)| left <= right)
    }
}

impl fmt::Display for Payoff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[")?;
        for (cell, amount) in self.amounts.iter().enumerate() {
            if cell > 0 {
                f.write_str(",")?;
            }
            write!(f, "{amount}")?;
        }
        f.write_str("]")
    }
}

/// A payoff object: everything a classification criterion is allowed to see.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PayoffObject {
    payoff: Payoff,
    facts: Facts,
}

impl PayoffObject {
    /// Build a payoff object.
    #[must_use]
    pub const fn new(payoff: Payoff, facts: Facts) -> Self {
        Self { payoff, facts }
    }

    /// The payoff vector.
    #[must_use]
    pub const fn payoff(&self) -> &Payoff {
        &self.payoff
    }

    /// The economic facts.
    #[must_use]
    pub const fn facts(&self) -> Facts {
        self.facts
    }
}

impl fmt::Display for PayoffObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.payoff, self.facts.label())
    }
}

/// Why a bundling or unbundling operation was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpError {
    /// The operands are over partitions of different sizes.
    CellMismatch {
        /// Cell count of the left operand.
        left: usize,
        /// Cell count of the right operand.
        right: usize,
    },
    /// The operands carry different economic facts, so neither operation is one
    /// a holder could perform on a single reference variable.
    FactsMismatch,
    /// A bundle of no parts has no partition and no facts.
    EmptyBundle,
    /// The proposed part is not dominated by the whole cell by cell.
    PartExceedsWhole,
}

impl fmt::Display for OpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CellMismatch { left, right } => {
                write!(f, "partition mismatch: {left} cells against {right} cells")
            }
            Self::FactsMismatch => f.write_str("the parts carry different economic facts"),
            Self::EmptyBundle => f.write_str("a bundle needs at least one part"),
            Self::PartExceedsWhole => f.write_str("the part is not dominated by the whole"),
        }
    }
}

impl std::error::Error for OpError {}

/// Bundle: hold the parts together and add their payoffs cell by cell.
///
/// This is one of the two costless operations. It moves no collateral, creates
/// no claim, and destroys none; it records that one holder holds all the parts.
///
/// # Errors
///
/// Returns [`OpError::EmptyBundle`], [`OpError::CellMismatch`], or
/// [`OpError::FactsMismatch`] if the parts cannot be held as one position.
pub fn bundle(parts: &[PayoffObject]) -> Result<PayoffObject, OpError> {
    let Some(first) = parts.first() else {
        return Err(OpError::EmptyBundle);
    };
    let facts = first.facts();
    let mut total = Payoff::zero(first.payoff().cells());
    for part in parts {
        if part.facts() != facts {
            return Err(OpError::FactsMismatch);
        }
        total = total.add(part.payoff())?;
    }
    Ok(PayoffObject::new(total, facts))
}

/// Split: divide one position into two, the first taking `take`.
///
/// The other costless operation, and the exact inverse of [`bundle`]. The claims
/// outstanding in every cell are unchanged, so the collateral the terms require
/// is unchanged.
///
/// # Errors
///
/// Returns [`OpError::CellMismatch`] or [`OpError::PartExceedsWhole`] if `take`
/// is not a part of `whole`.
pub fn split(whole: &PayoffObject, take: &Payoff) -> Result<(PayoffObject, PayoffObject), OpError> {
    let remainder = whole.payoff().sub(take)?;
    Ok((
        PayoffObject::new(take.clone(), whole.facts()),
        PayoffObject::new(remainder, whole.facts()),
    ))
}

/// Unbundle to elementary claims: `v[i]` copies of the claim on cell `i`.
///
/// The complete-set decomposition is the special case where every entry is
/// equal; there the parts are exactly one claim per cell per unit of collateral.
#[must_use]
pub fn unbundle_elementary(object: &PayoffObject) -> Vec<PayoffObject> {
    let payoff = object.payoff();
    let mut parts = Vec::new();
    for cell in 0..payoff.cells() {
        for _ in 0..payoff.get(cell) {
            parts.push(PayoffObject::new(
                Payoff::unit(payoff.cells(), cell),
                object.facts(),
            ));
        }
    }
    parts
}
