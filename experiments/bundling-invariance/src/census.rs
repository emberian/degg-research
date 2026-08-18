//! How severe a constraint invariance is, counted exactly.
//!
//! The candidate criteria are a handful of tests somebody might write down. This
//! module asks the wider question at the only bound where it can be answered by
//! enumeration: of **every** two-label criterion that reads only which outcomes
//! an object pays in, how many are bundling-invariant?
//!
//! A criterion of that kind is a function from subsets of the cells to one of
//! two labels, so there are `2^(2^n)` of them over `n` cells. Because payoffs
//! are nonnegative, the outcomes a bundle pays in are exactly the union of the
//! outcomes its parts pay in, so invariance for such a criterion is the
//! condition that each label's set of supports is closed under union. That is
//! checked here directly, over every pair, rather than argued.
//!
//! Bounds: `n` from 2 to 4. At `n = 5` there are `2^32` such criteria and this
//! module says nothing about them.

/// Smallest cell count in the census.
pub const MIN_CELLS: usize = 2;

/// Largest cell count in the census. `n = 5` needs `2^32` enumerations and is
/// out of bounds.
pub const MAX_CELLS: usize = 4;

/// One row of the census.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CensusRow {
    /// Cell count.
    pub cells: usize,
    /// Two-label support-reading criteria over this many cells.
    pub total: u64,
    /// How many of them are bundling-invariant.
    pub invariant: u64,
    /// How many of those separate at least two objects.
    pub invariant_discriminating: u64,
}

/// Count the invariant support-reading criteria over `cells` cells.
///
/// # Panics
///
/// Panics if `cells` is outside the census bounds.
#[must_use]
pub fn row(cells: usize) -> CensusRow {
    assert!(
        (MIN_CELLS..=MAX_CELLS).contains(&cells),
        "cell count outside the census bounds"
    );
    let supports = 1_usize << cells;
    let total = 1_u64 << supports;
    let mut invariant = 0;
    let mut invariant_discriminating = 0;
    for assignment in 0..total {
        let mut ok = true;
        'outer: for left in 0..supports {
            for right in left..supports {
                let left_label = (assignment >> left) & 1;
                let right_label = (assignment >> right) & 1;
                if left_label != right_label {
                    continue;
                }
                let union_label = (assignment >> (left | right)) & 1;
                if union_label != left_label {
                    ok = false;
                    break 'outer;
                }
            }
        }
        if ok {
            invariant += 1;
            let all = if supports == 64 {
                u64::MAX
            } else {
                (1_u64 << supports) - 1
            };
            if assignment != 0 && assignment != all {
                invariant_discriminating += 1;
            }
        }
    }
    CensusRow {
        cells,
        total,
        invariant,
        invariant_discriminating,
    }
}

/// Every census row, in cell order.
#[must_use]
pub fn census() -> Vec<CensusRow> {
    (MIN_CELLS..=MAX_CELLS).map(row).collect()
}
