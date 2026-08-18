//! Sanity properties the whole parameter grid must satisfy.
//!
//! Each of these is a statement about the model, and each was written down
//! before the table was read. Where the direction is the opposite of the
//! intuition the memo would reach for, the test says so and the finding stands.

use std::collections::BTreeMap;

use degg_manipulation_cost::table::{BOUNDARIES, DEPTHS, FEES, GRIDS, Row, grid_rows, holds_for};

fn rows() -> Vec<Row> {
    grid_rows().expect("grid")
}

fn find(
    rows: &[Row],
    depth: u128,
    fee: u128,
    window: u64,
    buckets: u32,
    hold: u32,
    boundary: u128,
) -> &Row {
    rows.iter()
        .find(|row| {
            row.depth == depth
                && row.fee_bps == fee
                && row.window_seconds == window
                && row.buckets == buckets
                && row.hold == hold
                && row.boundary_bps == boundary
        })
        .expect("row")
}

#[test]
fn the_grid_has_exactly_the_rows_it_claims() {
    let rows = rows();
    let expected = DEPTHS.len() * FEES.len() * GRIDS.len() * 3 * BOUNDARIES.len();
    assert_eq!(rows.len(), expected);
    assert_eq!(rows.len(), 1080);
    let mut seen = BTreeMap::new();
    for row in &rows {
        let key = (
            row.depth,
            row.fee_bps,
            row.window_seconds,
            row.buckets,
            row.hold,
            row.boundary_bps,
        );
        assert!(seen.insert(key, ()).is_none(), "duplicate row {key:?}");
    }
}

#[test]
fn every_row_crosses_its_boundary() {
    for row in rows() {
        assert!(row.crossed, "row failed to cross: {row:?}");
    }
}

#[test]
fn cost_is_zero_exactly_when_the_boundary_is_already_crossed() {
    for row in rows() {
        if row.boundary_bps == 0 {
            assert_eq!(row.contiguous, 0);
            assert_eq!(row.capital, 0);
            assert_eq!(row.naive_spot, 0);
            assert_eq!(row.naive_independent, 0);
            if let Some(spread) = row.spread {
                assert_eq!(spread, 0);
            }
        } else {
            assert!(row.contiguous > 0, "zero cost at a live boundary: {row:?}");
            assert!(row.capital > 0);
        }
    }
}

#[test]
fn cost_and_capital_rise_strictly_with_pool_depth() {
    let rows = rows();
    for fee in FEES {
        for (window, buckets) in GRIDS {
            for hold in holds_for(buckets) {
                for boundary in BOUNDARIES {
                    if boundary == 0 {
                        continue;
                    }
                    let mut previous: Option<&Row> = None;
                    for depth in DEPTHS {
                        let row = find(&rows, depth, fee, window, buckets, hold, boundary);
                        if let Some(before) = previous {
                            assert!(
                                row.contiguous > before.contiguous,
                                "cost did not rise with depth: {before:?} then {row:?}"
                            );
                            assert!(row.capital > before.capital);
                            assert!(row.ceiling > before.ceiling);
                        }
                        previous = Some(row);
                    }
                }
            }
        }
    }
}

#[test]
fn cost_and_capital_rise_strictly_with_boundary_distance() {
    let rows = rows();
    for depth in DEPTHS {
        for fee in FEES {
            for (window, buckets) in GRIDS {
                for hold in holds_for(buckets) {
                    let mut previous: Option<&Row> = None;
                    for boundary in BOUNDARIES {
                        let row = find(&rows, depth, fee, window, buckets, hold, boundary);
                        if let Some(before) = previous {
                            assert!(
                                row.contiguous > before.contiguous,
                                "cost did not rise with distance: {before:?} then {row:?}"
                            );
                            assert!(row.capital > before.capital);
                        }
                        previous = Some(row);
                    }
                }
            }
        }
    }
}

#[test]
fn cost_rises_strictly_with_the_fee() {
    let rows = rows();
    for depth in DEPTHS {
        for (window, buckets) in GRIDS {
            for hold in holds_for(buckets) {
                for boundary in BOUNDARIES {
                    if boundary == 0 {
                        continue;
                    }
                    let mut previous: Option<&Row> = None;
                    for fee in FEES {
                        let row = find(&rows, depth, fee, window, buckets, hold, boundary);
                        if let Some(before) = previous {
                            assert!(
                                row.contiguous > before.contiguous,
                                "cost did not rise with fee: {before:?} then {row:?}"
                            );
                        }
                        previous = Some(row);
                    }
                }
            }
        }
    }
}

#[test]
fn requiring_a_longer_hold_makes_the_attack_cheaper_not_dearer() {
    // The direction that matters and the one a drafter would get wrong. Under a
    // model with no competing flow, holding costs nothing once established,
    // while every extra distorted bucket divides the amplification factor
    // n/k. So "the attacker must hold it for more of the window" *lowers* the
    // computed cost. What a longer hold really buys --- more time for arriving
    // flow to fight the position --- is exactly what this model does not price.
    let rows = rows();
    for depth in DEPTHS {
        for fee in FEES {
            for (window, buckets) in GRIDS {
                for boundary in BOUNDARIES {
                    if boundary == 0 {
                        continue;
                    }
                    let mut previous: Option<&Row> = None;
                    for hold in holds_for(buckets) {
                        let row = find(&rows, depth, fee, window, buckets, hold, boundary);
                        if let Some(before) = previous {
                            assert!(
                                row.contiguous < before.contiguous,
                                "cost did not fall with hold: {before:?} then {row:?}"
                            );
                            assert!(row.capital < before.capital);
                        }
                        previous = Some(row);
                    }
                }
            }
        }
    }
}

#[test]
fn the_window_length_does_not_enter_the_cost_at_all() {
    // Three window lengths, identical numbers. Under this model only the bucket
    // count and the hold count move the price, because seconds only matter
    // through flow that is not modelled. The window dial in a contract's terms
    // is therefore not priced here, and a memo must not claim it is.
    let rows = rows();
    for depth in DEPTHS {
        for fee in FEES {
            for buckets in [6u32, 12] {
                for hold in holds_for(buckets) {
                    for boundary in BOUNDARIES {
                        let reference = find(&rows, depth, fee, 300, buckets, hold, boundary);
                        for window in [900u64, 3600] {
                            let row = find(&rows, depth, fee, window, buckets, hold, boundary);
                            assert_eq!(row.contiguous, reference.contiguous);
                            assert_eq!(row.capital, reference.capital);
                            assert_eq!(row.spread, reference.spread);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn a_finer_grid_at_a_fixed_hold_count_raises_the_cost() {
    // Twelve buckets with one distorted needs twice the displacement six
    // buckets with one distorted needs. Sampling density is a real dial; the
    // window length is not.
    let rows = rows();
    for depth in DEPTHS {
        for fee in FEES {
            for boundary in BOUNDARIES {
                if boundary == 0 {
                    continue;
                }
                let coarse = find(&rows, depth, fee, 300, 6, 1, boundary);
                let fine = find(&rows, depth, fee, 300, 12, 1, boundary);
                assert!(
                    fine.contiguous > coarse.contiguous,
                    "finer grid was not dearer: {coarse:?} then {fine:?}"
                );
            }
        }
    }
}

#[test]
fn no_computed_cost_exceeds_the_single_round_trip_ceiling() {
    for row in rows() {
        assert!(
            row.contiguous <= row.ceiling,
            "contiguous cost broke the ceiling: {row:?}"
        );
        assert!(row.naive_spot <= row.ceiling);
        // The spread schedule is many round trips, so only its per-trip share
        // is bounded by the ceiling.
        if let Some(spread) = row.spread {
            assert!(spread <= row.ceiling * u128::from(row.hold));
        }
    }
}

#[test]
fn capital_dwarfs_cost_everywhere_in_the_grid() {
    // The headline shape of the table: what an attacker must *have* is orders
    // of magnitude larger than what the attack *burns*.
    for row in rows() {
        if row.boundary_bps == 0 {
            continue;
        }
        assert!(
            row.capital > row.contiguous,
            "capital did not exceed cost: {row:?}"
        );
    }
    let rows = rows();
    let deep = find(&rows, 10_000_000_000, 5, 3600, 12, 1, 10);
    assert!(deep.capital > 1_000 * deep.contiguous);
}
