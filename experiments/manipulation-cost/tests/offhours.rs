//! The off-hours corpus: byte-stable, anchored to `vectors/v1.txt`, and
//! monotone in the direction the model claims.
//!
//! Each property here was written down before the table was read. The two that
//! came out with a direction worth stating --- where a window design keeps its
//! cost under thinning, and where the retention law breaks --- say so in the
//! test rather than in prose alone.

use std::collections::BTreeMap;

use degg_manipulation_cost::pool::{BPS, round_trip_cost_ceiling};
use degg_manipulation_cost::session::{
    BOUNDARIES, DEPTH_RATIOS, HEADER_LINES, SESSION_SECONDS, SessionError, SessionRow, render_row,
    render_vectors_v2, session_rows, thin_buckets, thin_ceiling, thin_depth,
};
use degg_manipulation_cost::table::{self, DEPTHS, FEES, GRIDS, holds_for};
use degg_manipulation_cost::twap::Grid;
use degg_manipulation_cost::{MODEL, SESSION_MODEL};

const CORPUS: &str = include_str!("../vectors/v2-offhours.txt");
const CORPUS_V1: &str = include_str!("../vectors/v1.txt");

fn rows() -> Vec<SessionRow> {
    session_rows().expect("grid")
}

/// The identity of a row: everything before the first computed column.
type Key = (u128, u128, u64, u32, u32, u128, u128, u128, u64);

fn key(row: &SessionRow) -> Key {
    (
        row.depth,
        row.fee_bps,
        row.window_seconds,
        row.buckets,
        row.hold,
        row.boundary_bps,
        row.ratio_num,
        row.ratio_den,
        row.session_seconds,
    )
}

fn find(rows: &[SessionRow], wanted: Key) -> &SessionRow {
    rows.iter().find(|row| key(row) == wanted).expect("row")
}

/// The identity of a v1 row: depth, fee, window, buckets, hold, boundary.
type KeyV1 = (u128, u128, u64, u32, u32, u128);

/// The `capital` and `contiguous` columns of `vectors/v1.txt`, by parameters.
fn v1_table() -> BTreeMap<KeyV1, (u128, u128)> {
    let mut out = BTreeMap::new();
    for line in CORPUS_V1.lines() {
        if !line.starts_with("depth=") {
            continue;
        }
        let fields: BTreeMap<&str, &str> = line
            .split('|')
            .map(|field| field.split_once('=').expect("key=value"))
            .collect();
        let get = |name: &str| fields[name].parse::<u128>().expect("integer");
        out.insert(
            (
                get("depth"),
                get("fee_bps"),
                get("window_s") as u64,
                get("buckets") as u32,
                get("hold") as u32,
                get("boundary_bps"),
            ),
            (get("capital"), get("contiguous")),
        );
    }
    out
}

#[test]
fn the_rendered_corpus_matches_the_checked_in_file_byte_for_byte() {
    assert_eq!(render_vectors_v2().expect("render"), CORPUS);
}

#[test]
fn rendering_is_idempotent_across_runs() {
    assert_eq!(
        render_vectors_v2().expect("render"),
        render_vectors_v2().expect("render")
    );
}

#[test]
fn the_header_names_the_model_the_session_layer_and_the_depth_ratio_semantics() {
    let mut lines = CORPUS.lines();
    assert_eq!(lines.next(), Some("manipulation-cost-vectors-v2-offhours"));
    assert_eq!(lines.next(), Some(&format!("model={MODEL}")[..]));
    assert_eq!(
        lines.next(),
        Some(&format!("session-model={SESSION_MODEL}")[..])
    );
    assert!(CORPUS.contains("depth_ratio=thin_depth/depth|both-reserves-scaled-by-it"));
    assert!(CORPUS.contains("thin_buckets=min(buckets,floor(session_s/bucket_s))"));
    assert!(
        CORPUS.contains("adversary=confined-to-the-thin-session|hold_eff=min(hold,thin_buckets)")
    );
    assert!(
        CORPUS
            .contains("bound=cost-of-the-stated-off-hours-schedule|not-a-minimum-over-all-attacks")
    );
    assert!(CORPUS.contains("data=synthetic-only"));
    assert!(CORPUS.contains("omitted=attacking-the-normal-session-at-cost_v1"));
}

#[test]
fn every_row_renders_on_one_line_with_the_declared_columns() {
    let column_line = CORPUS
        .lines()
        .find(|line| line.starts_with("columns="))
        .expect("column header");
    let columns = column_line
        .trim_start_matches("columns=")
        .split(',')
        .count();
    for row in rows() {
        let rendered = render_row(&row);
        assert!(!rendered.contains('\n'));
        let fields: Vec<&str> = rendered.split('|').collect();
        assert_eq!(fields.len(), columns);
        for field in fields {
            assert!(field.contains('='), "field without a key: {field}");
        }
    }
}

#[test]
fn the_corpus_carries_one_line_per_row_plus_its_header() {
    assert_eq!(CORPUS.lines().count(), HEADER_LINES + rows().len());
    assert!(
        CORPUS
            .lines()
            .nth(HEADER_LINES - 1)
            .expect("header")
            .starts_with("columns="),
        "the last header line is not the column line"
    );
    assert!(
        CORPUS
            .lines()
            .nth(HEADER_LINES)
            .expect("first row")
            .starts_with("depth=")
    );
}

#[test]
fn the_grid_has_exactly_the_rows_it_claims() {
    let rows = rows();
    let expected = DEPTHS.len()
        * FEES.len()
        * GRIDS.len()
        * 3
        * BOUNDARIES.len()
        * DEPTH_RATIOS.len()
        * SESSION_SECONDS.len();
    assert_eq!(rows.len(), expected);
    assert_eq!(rows.len(), 8640);
    let mut seen = BTreeMap::new();
    for row in &rows {
        assert!(seen.insert(key(row), ()).is_none(), "duplicate row");
    }
}

#[test]
fn the_hand_computed_row_is_the_row_the_table_holds() {
    // Depth 10_000_000, fee 5 bps, a 300 s window in 6 buckets, hold 1,
    // boundary 10 bps, depth ratio 1/2, thin session 150 s. Every step below is
    // integer arithmetic done outside the crate and then checked against it.
    //
    // Grid. bucket_s = 300 / 6 = 50, and floor(150 / 50) = 3, so 3 of the 6
    // samples are taken inside the thin session: thin_buckets = 3. The design
    // exposes the adversary in hold = 1 bucket, so hold_eff = min(1, 3) = 1.
    //
    // Depth. 10_000_000 * 1 / 2 = 5_000_000 of each leg, exactly.
    //
    // Displacement. buckets * boundary / hold_eff = 6 * 10 / 1 = 60 bps, so the
    // one distorted sample must reach p0 * 10060/10000 = 503/500.
    let all = rows();
    let row = find(&all, (10_000_000, 5, 300, 6, 1, 10, 1, 2, 150));
    assert_eq!(row.thin_buckets, 3);
    assert_eq!(row.thin_depth, 5_000_000);
    assert_eq!(row.hold_eff, 1);
    assert_eq!(row.bucket_bps, Some(60));

    // The excursion. Quote in d = 14_982 against reserves (5_000_000,
    // 5_000_000) at 5 bps: the fee-adjusted input is
    //   a = 14_982 * 9_995 = 149_745_090,
    // the base output is
    //   floor(5_000_000 * 149_745_090 / (5_000_000 * 10_000 + 149_745_090))
    //     = floor(748_725_450_000_000 / 50_149_745_090) = 14_929,
    // and the pool lands at (4_985_071, 5_014_982).
    let quote_in: u128 = 14_982;
    let effective = quote_in * (BPS - 5);
    assert_eq!(effective, 149_745_090);
    let base_out = 5_000_000 * effective / (5_000_000 * BPS + effective);
    assert_eq!(base_out, 14_929);
    let base_after = 5_000_000 - base_out;
    let quote_after = 5_000_000 + quote_in;
    assert_eq!((base_after, quote_after), (4_985_071, 5_014_982));
    // It reaches the target: 500 * 5_014_982 >= 503 * 4_985_071.
    assert!(500 * quote_after >= 503 * base_after);
    assert_eq!(500 * quote_after, 2_507_491_000);
    assert_eq!(503 * base_after, 2_507_490_713);
    // And one unit less does not: at d = 14_981 the pool lands at
    // (4_985_072, 5_014_981) and 2_507_490_500 < 2_507_491_216.
    let short = 14_981 * (BPS - 5);
    let short_out = 5_000_000 * short / (5_000_000 * BPS + short);
    assert_eq!(short_out, 14_928);
    assert!(500 * (5_000_000 + 14_981) < 503 * (5_000_000 - short_out));

    // The reversal. Selling all 14_929 base back:
    //   a = 14_929 * 9_995 = 149_215_355,
    //   out = floor(5_014_982 * 149_215_355 / (4_985_071 * 10_000 + 149_215_355))
    //       = floor(748_312_319_448_610 / 49_999_925_355) = 14_966.
    let unwind = base_out * (BPS - 5);
    assert_eq!(unwind, 149_215_355);
    let quote_out = quote_after * unwind / (base_after * BPS + unwind);
    assert_eq!(quote_out, 14_966);

    // Cost is what went in less what came back, and capital is the whole
    // excursion, because the adversary is never further out than that.
    assert_eq!(row.cost, Some(quote_in - quote_out));
    assert_eq!(row.cost, Some(16));
    assert_eq!(row.capital, Some(14_982));
    // 10_000 * 16 / 5_000_000 floors to zero: the burn is under a basis point
    // of the thin session's own reserve.
    assert_eq!(row.cost_thin_bps, Some(0));
    // The same design attacked at full depth costs 31, from vectors/v1.txt.
    assert_eq!(row.cost_v1, 31);
    assert_eq!(v1_table()[&(10_000_000, 5, 300, 6, 1, 10)].1, 31);
    // 10_000 * 16 / 31 = 5161.29..., floored.
    assert_eq!(row.cost_ratio_bps, Some(5_161));
    assert!(row.offhours_cheaper);
    assert_eq!(row.crossed, Some(true));
}

#[test]
fn cost_never_rises_as_the_session_thins() {
    // The direction the memo would reach for, and here it holds: for a fixed
    // design and a fixed thin session, taking depth out of that session cannot
    // make the attack dearer. The ratios are ordered coarsest first, so this
    // walks each design down the thinning grid.
    let rows = rows();
    let mut ties = 0;
    for depth in DEPTHS {
        for fee in FEES {
            for (window, buckets) in GRIDS {
                for hold in holds_for(buckets) {
                    for boundary in BOUNDARIES {
                        for session in SESSION_SECONDS {
                            let mut previous: Option<u128> = None;
                            for (num, den) in DEPTH_RATIOS {
                                let row = find(
                                    &rows,
                                    (
                                        depth, fee, window, buckets, hold, boundary, num, den,
                                        session,
                                    ),
                                );
                                let Some(cost) = row.cost else { continue };
                                if let Some(before) = previous {
                                    assert!(
                                        cost <= before,
                                        "cost rose as the session thinned: {row:?}"
                                    );
                                    if cost == before {
                                        ties += 1;
                                    }
                                }
                                previous = Some(cost);
                            }
                        }
                    }
                }
            }
        }
    }
    // The only steps that do not fall are four rows whose cost is already one
    // integer unit at a thin reserve of 200_000, where the flooring the README
    // warns about is the whole quantity.
    assert_eq!(ties, 4);
    for row in rows {
        if row.cost == Some(1) && row.thin_depth == 200_000 {
            assert_eq!(row.depth, 10_000_000);
            assert_eq!(row.fee_bps, 5);
        }
    }
}

#[test]
fn the_reference_column_is_the_v1_row_with_the_same_parameters() {
    // The cross-version anchor, read out of the checked-in v1 file rather than
    // recomputed: every row's cost_v1 is the contiguous cost v1 published for
    // the identical design.
    let v1 = v1_table();
    for row in rows() {
        let reference = v1[&(
            row.depth,
            row.fee_bps,
            row.window_seconds,
            row.buckets,
            row.hold,
            row.boundary_bps,
        )];
        assert_eq!(row.cost_v1, reference.1, "reference mismatch: {row:?}");
    }
}

#[test]
fn a_full_depth_session_that_carries_the_whole_exposure_reproduces_v1_exactly() {
    // The boundary case that ties the two corpora together. At depth ratio 1/1
    // with the thin session carrying at least the design's whole exposure, the
    // off-hours attack *is* the v1 attack: same pool, same schedule, same
    // numbers, and a retention ratio of exactly 10_000.
    let v1 = v1_table();
    let mut checked = 0;
    for row in rows() {
        if (row.ratio_num, row.ratio_den) != (1, 1) || row.hold_eff != row.hold {
            continue;
        }
        let (capital, contiguous) = v1[&(
            row.depth,
            row.fee_bps,
            row.window_seconds,
            row.buckets,
            row.hold,
            row.boundary_bps,
        )];
        assert_eq!(row.thin_depth, row.depth);
        assert_eq!(row.cost, Some(contiguous), "cost differs from v1: {row:?}");
        assert_eq!(
            row.capital,
            Some(capital),
            "capital differs from v1: {row:?}"
        );
        assert_eq!(row.cost_ratio_bps, Some(BPS));
        assert!(!row.offhours_cheaper);
        checked += 1;
    }
    assert_eq!(checked, 864);
}

#[test]
fn every_row_is_the_v1_model_at_the_thin_depth_and_the_effective_hold() {
    // The session layer adds no arithmetic. It chooses a depth and an effective
    // hold count, and the answer is the v1 model evaluated there --- which is
    // why this table needs no second derivation and no second cross-check.
    for row in rows() {
        let Some(cost) = row.cost else { continue };
        let grid = Grid::new(row.window_seconds, row.buckets).expect("grid");
        let equivalent = table::row(
            row.thin_depth,
            row.fee_bps,
            grid,
            row.hold_eff,
            row.boundary_bps,
        )
        .expect("v1 row");
        assert_eq!(cost, equivalent.contiguous, "not the v1 model: {row:?}");
        assert_eq!(row.capital, Some(equivalent.capital));
        assert_eq!(row.bucket_bps, Some(equivalent.bucket_bps));
    }
}

#[test]
fn a_grid_whose_buckets_outlast_the_session_takes_no_sample_inside_it() {
    // Not a gap in the table: a result. A 3600 s window sampled every 300 s or
    // every 600 s takes no sample at all inside a 150 s thin session, so no
    // attack mounted from inside that session touches the print.
    let mut dashes = 0;
    for row in rows() {
        let inside = row.window_seconds == 3600 && row.session_seconds == 150;
        assert_eq!(row.thin_buckets == 0, inside, "thin bucket count: {row:?}");
        if row.thin_buckets == 0 {
            assert_eq!(row.cost, None);
            assert_eq!(row.capital, None);
            assert_eq!(row.bucket_bps, None);
            assert_eq!(row.cost_ratio_bps, None);
            assert_eq!(row.crossed, None);
            assert_eq!(row.hold_eff, 0);
            assert!(!row.offhours_cheaper);
            assert!(render_row(&row).contains("cost=-"));
            dashes += 1;
        }
    }
    assert_eq!(dashes, 1440);
    // The rule that produces it, stated on its own.
    let coarse = Grid::new(3600, 6).expect("grid");
    assert_eq!(thin_buckets(coarse, 150), 0);
    assert_eq!(thin_buckets(coarse, 600), 1);
    assert_eq!(thin_buckets(coarse, 3600), 6);
    assert_eq!(thin_buckets(coarse, 100_000), 6);
}

#[test]
fn the_rows_quoted_in_the_readme_are_the_rows_the_table_holds() {
    // README.md states the finding with these two families and these counts. If
    // the model changes, this fails before the README goes stale.
    let all = rows();
    let retention = |window: u64, session: u64, den: u128| {
        find(
            &all,
            (1_000_000_000, 30, window, 12, 12, 10, 1, den, session),
        )
        .cost_ratio_bps
    };
    // A window wholly inside the thin session: cost falls with depth.
    for (den, expected) in [(1, 10_000), (2, 5_003), (4, 2_501), (10, 1_004), (50, 203)] {
        assert_eq!(retention(300, 600, den), Some(expected));
    }
    // The same design sampling that session in 2 of its 12 buckets: sixfold
    // amplification, and the whole daylight cost kept down to a quarter depth.
    for (den, expected) in [
        (1, 59_776),
        (2, 29_889),
        (4, 14_946),
        (10, 5_980),
        (50, 1_200),
    ] {
        assert_eq!(retention(3600, 600, den), Some(expected));
    }
    assert_eq!(retention(3600, 150, 1), None);
    let sixfold = find(&all, (1_000_000_000, 30, 3600, 12, 12, 10, 1, 1, 600));
    assert_eq!(sixfold.thin_buckets, 2);
    assert_eq!(sixfold.hold_eff, 2);
    assert_eq!(sixfold.capital, Some(3_000_014));
    assert_eq!(sixfold.cost, Some(17_921));

    // The clawback erodes as the boundary widens, because cost is concave in
    // displacement.
    for (boundary, expected) in [(10, 59_776), (50, 58_903), (200, 55_923), (1000, 45_035)] {
        let row = find(&all, (1_000_000_000, 30, 900, 6, 6, boundary, 1, 1, 150));
        assert_eq!(row.hold_eff, 1);
        assert_eq!(row.cost_ratio_bps, Some(expected));
    }

    let computed = all.iter().filter(|row| row.cost.is_some()).count();
    let dearer = all
        .iter()
        .filter(|row| row.cost.is_some() && !row.offhours_cheaper)
        .count();
    assert_eq!(computed, 7_200);
    assert_eq!(dearer, 2_028);
}

#[test]
fn every_off_hours_schedule_crosses_its_boundary() {
    for row in rows() {
        if row.cost.is_some() {
            assert_eq!(row.crossed, Some(true), "failed to cross: {row:?}");
        }
    }
}

#[test]
fn no_off_hours_cost_exceeds_the_thin_sessions_own_ceiling() {
    // The whole budget of an attack confined to the thin session is bounded by
    // a fixed fraction of that session's quote reserve, so thinning the session
    // thins the ceiling with it.
    for row in rows() {
        let Some(cost) = row.cost else { continue };
        let ceiling = thin_ceiling(&row).expect("ceiling");
        assert!(cost <= ceiling, "cost broke the thin ceiling: {row:?}");
        assert_eq!(
            ceiling,
            round_trip_cost_ceiling(row.thin_depth, row.fee_bps).expect("ceiling")
        );
    }
}

#[test]
fn seconds_enter_only_through_the_bucket_count() {
    // The v2 refinement of v1's finding that window length does not enter the
    // cost. It enters now --- but only by deciding how many of the window's
    // samples fall inside the session. Two rows agreeing on (buckets,
    // thin_buckets) and everything else agree on every computed column, whatever
    // their seconds say.
    let mut classes: BTreeMap<_, Vec<SessionRow>> = BTreeMap::new();
    for row in rows() {
        classes
            .entry((
                row.buckets,
                row.thin_buckets,
                row.hold,
                row.boundary_bps,
                row.depth,
                row.fee_bps,
                row.ratio_num,
                row.ratio_den,
            ))
            .or_default()
            .push(row);
    }
    let mut collapsed = 0;
    for members in classes.values() {
        let first = &members[0];
        for other in &members[1..] {
            assert_eq!(first.cost, other.cost);
            assert_eq!(first.capital, other.capital);
            assert_eq!(first.cost_ratio_bps, other.cost_ratio_bps);
            assert_eq!(first.hold_eff, other.hold_eff);
            if first.window_seconds != other.window_seconds {
                collapsed += 1;
            }
        }
    }
    assert!(collapsed > 0, "no class spanned two window lengths");
}

#[test]
fn retention_follows_the_depth_ratio_times_the_amplification_and_where_it_does_not() {
    // The law the table exists to report: what an off-hours attack pays,
    // relative to the same attack in daylight, is the session's depth ratio
    // multiplied by the amplification the session forces on it, hold/hold_eff.
    //
    // It is a law with two named error terms, and both directions are recorded
    // here rather than smoothed:
    //
    //   * downward, by up to 15022 bps, where cost is concave in displacement
    //     --- a sixfold amplification returns sixfold at a 10 bp boundary and
    //     only four and a half fold at 1000 bps, because the burn saturates;
    //   * upward, by up to 3000 bps, where the costs involved are single
    //     integers and the flooring is the whole quantity.
    let mut lowest: i128 = 0;
    let mut highest: i128 = 0;
    for row in rows() {
        let Some(ratio) = row.cost_ratio_bps else {
            continue;
        };
        let law =
            BPS * row.ratio_num * u128::from(row.hold) / (row.ratio_den * u128::from(row.hold_eff));
        let deviation = ratio as i128 - law as i128;
        lowest = lowest.min(deviation);
        highest = highest.max(deviation);
    }
    assert_eq!(lowest, -15_022);
    assert_eq!(highest, 3_000);
}

#[test]
fn no_window_design_keeps_its_daylight_cost_once_the_session_holds_a_tenth() {
    // The headline. A design "keeps its cost" when an attack from inside the
    // thin session is no cheaper than the same attack in daylight, which needs
    // the amplification the session forces to outrun the depth it removed. The
    // widest amplification any design in this grid forces is sixfold, so the
    // keeping stops between a quarter and a tenth.
    let mut best: BTreeMap<(u128, u128), u128> = BTreeMap::new();
    let mut kept: BTreeMap<(u128, u128), usize> = BTreeMap::new();
    for row in rows() {
        let Some(ratio) = row.cost_ratio_bps else {
            continue;
        };
        let slot = (row.ratio_num, row.ratio_den);
        let entry = best.entry(slot).or_default();
        *entry = (*entry).max(ratio);
        if ratio >= BPS {
            *kept.entry(slot).or_default() += 1;
        }
    }
    assert_eq!(best[&(1, 1)], 62_000);
    assert_eq!(best[&(1, 2)], 32_000);
    assert_eq!(best[&(1, 4)], 18_000);
    assert_eq!(best[&(1, 10)], 8_000);
    assert_eq!(best[&(1, 50)], 4_000);
    assert!(kept[&(1, 2)] > 0);
    assert!(kept[&(1, 4)] > 0);
    assert_eq!(kept.get(&(1, 10)), None);
    assert_eq!(kept.get(&(1, 50)), None);
}

#[test]
fn a_ratio_that_would_not_divide_the_depth_is_refused_rather_than_rounded() {
    assert_eq!(thin_depth(10_000_000, 1, 50), Ok(200_000));
    assert_eq!(thin_depth(999, 1, 50), Err(SessionError::InexactThinDepth));
    assert_eq!(thin_depth(1_000, 0, 50), Err(SessionError::BadRatio));
    assert_eq!(thin_depth(1_000, 1, 0), Err(SessionError::BadRatio));
    assert_eq!(thin_depth(1_000, 3, 2), Err(SessionError::BadRatio));
    for depth in DEPTHS {
        for (num, den) in DEPTH_RATIOS {
            assert!(thin_depth(depth, num, den).is_ok());
        }
    }
}
