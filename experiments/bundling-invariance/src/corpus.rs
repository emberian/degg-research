//! The bounded corpus, enumerated exhaustively.
//!
//! Bounds, stated once and enforced by the constants in [`crate::payoff`]:
//!
//! - cell counts 2 through 5;
//! - per-cell payouts 0 through 3;
//! - every combination of the twelve fact profiles.
//!
//! Within those bounds nothing is sampled. Every payoff vector is enumerated,
//! and every decomposition of every vector in the two decomposition families is
//! checked. Outside them nothing is claimed.

use crate::payoff::{MAX_CELLS, MIN_CELLS, PAYOUT_CEILING, Payoff};

/// Base of the positional encoding used to index payoff vectors.
pub const BASE: usize = (PAYOUT_CEILING + 1) as usize;

/// Every payoff vector over `cells` cells with entries at most the ceiling, in
/// index order.
///
/// The index of a vector is `sum(amount[i] * BASE^i)`, so `vectors_for(n)[k]`
/// has index `k`.
///
/// # Panics
///
/// Panics if `cells` is outside the corpus bounds.
#[must_use]
pub fn vectors_for(cells: usize) -> Vec<Payoff> {
    assert!(
        (MIN_CELLS..=MAX_CELLS).contains(&cells),
        "cell count outside the corpus bounds"
    );
    let count = BASE.pow(u32::try_from(cells).expect("cell count fits in u32"));
    (0..count)
        .map(|index| {
            let mut rest = index;
            let mut amounts = Vec::with_capacity(cells);
            for _ in 0..cells {
                amounts.push(u64::try_from(rest % BASE).expect("digit fits in u64"));
                rest /= BASE;
            }
            Payoff::new(amounts)
        })
        .collect()
}

/// Index of a payoff vector in [`vectors_for`].
///
/// # Panics
///
/// Panics if any entry exceeds the corpus ceiling.
#[must_use]
pub fn index_of(payoff: &Payoff) -> usize {
    let mut index = 0;
    let mut scale = 1;
    for amount in payoff.amounts() {
        assert!(*amount <= PAYOUT_CEILING, "amount above the corpus ceiling");
        index += usize::try_from(*amount).expect("amount fits in usize") * scale;
        scale *= BASE;
    }
    index
}

/// Number of payoff vectors in the corpus, across every cell count.
#[must_use]
pub fn vector_count() -> usize {
    (MIN_CELLS..=MAX_CELLS)
        .map(|cells| BASE.pow(u32::try_from(cells).expect("cell count fits in u32")))
        .sum()
}

/// Every unordered binary decomposition of `whole`, as `(part, remainder)` with
/// `part` no greater than `remainder` in vector order.
///
/// The trivial decomposition into the zero vector and the whole is included; it
/// can never be a violation, and excluding it would be a special case nobody
/// asked for.
#[must_use]
pub fn binary_decompositions(whole: &Payoff) -> Vec<(Payoff, Payoff)> {
    let cells = whole.cells();
    let mut parts = Vec::new();
    let mut take = vec![0_u64; cells];
    loop {
        let first = Payoff::new(take.clone());
        let second = whole.sub(&first).expect("take is dominated by the whole");
        if first <= second {
            parts.push((first, second));
        }
        let mut cell = 0;
        loop {
            if cell == cells {
                return parts;
            }
            if take[cell] < whole.get(cell) {
                take[cell] += 1;
                break;
            }
            take[cell] = 0;
            cell += 1;
        }
    }
}
