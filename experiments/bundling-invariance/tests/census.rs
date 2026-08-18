//! The census: how severe a constraint bundling invariance is, and that the
//! count means what it says.

use degg_bundling_invariance::census::{CensusRow, MAX_CELLS, MIN_CELLS, row};
use degg_bundling_invariance::census_rows;
use degg_bundling_invariance::corpus::{binary_decompositions, vectors_for};

fn label(assignment: u64, payoff: &degg_bundling_invariance::payoff::Payoff) -> u64 {
    (assignment >> payoff.support_mask()) & 1
}

/// Invariance checked the long way: over real payoff vectors and real
/// decompositions, with no appeal to the union argument.
fn invariant_over_the_corpus(assignment: u64, cells: usize) -> bool {
    for whole in vectors_for(cells) {
        let whole_label = label(assignment, &whole);
        for (first, second) in binary_decompositions(&whole) {
            let first_label = label(assignment, &first);
            if first_label == label(assignment, &second) && whole_label != first_label {
                return false;
            }
        }
    }
    true
}

#[test]
fn the_census_counts_match_a_direct_sweep_over_payoff_vectors() {
    for cells in MIN_CELLS..=3 {
        let supports = 1_u64 << cells;
        let total = 1_u64 << supports;
        let direct = (0..total)
            .filter(|assignment| invariant_over_the_corpus(*assignment, cells))
            .count() as u64;
        assert_eq!(row(cells).invariant, direct, "cells={cells}");
    }
}

#[test]
fn the_census_is_the_one_recorded_in_the_corpus_file() {
    assert_eq!(
        census_rows(),
        [
            CensusRow {
                cells: 2,
                total: 16,
                invariant: 12,
                invariant_discriminating: 10,
            },
            CensusRow {
                cells: 3,
                total: 256,
                invariant: 52,
                invariant_discriminating: 50,
            },
            CensusRow {
                cells: 4,
                total: 65_536,
                invariant: 300,
                invariant_discriminating: 298,
            },
        ]
    );
}

#[test]
fn invariance_gets_harder_to_satisfy_as_the_partition_grows() {
    let rows = census_rows();
    assert_eq!(rows.len(), MAX_CELLS - MIN_CELLS + 1);
    for entry in rows {
        // The two constant criteria are always invariant and always useless.
        assert_eq!(entry.invariant_discriminating + 2, entry.invariant);
    }
    for pair in rows.windows(2) {
        let (wider, narrower) = (pair[1], pair[0]);
        assert!(
            wider.invariant * narrower.total < narrower.invariant * wider.total,
            "the surviving fraction did not fall from {} cells to {} cells",
            narrower.cells,
            wider.cells
        );
    }
    let widest = rows.last().expect("the census has rows");
    assert!(
        widest.invariant * 200 < widest.total,
        "fewer than one in two hundred should survive at {} cells: {} of {}",
        widest.cells,
        widest.invariant,
        widest.total
    );
}
