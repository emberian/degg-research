//! The parameter grid, one row per synthetic configuration, and its byte-stable
//! rendering.
//!
//! Every pool in the grid is balanced --- equal base and quote reserves, so the
//! baseline print is exactly one quote unit per base unit --- which makes
//! "depth" one number and every displacement purely relative. Reserves are in
//! abstract integer units; nothing here is denominated in any currency and
//! nothing here is calibrated to any venue.

use crate::pool::{Pool, Price, round_trip_cost_ceiling};
use crate::twap::{
    Grid, Schedule, TwapError, boundary_price, execute, naive_independent_cost, naive_spot_cost,
    required_bucket_price, twap_at_least,
};

/// Quote-reserve depths in the grid, in integer units.
pub const DEPTHS: [u128; 4] = [10_000_000, 100_000_000, 1_000_000_000, 10_000_000_000];

/// Fee levels in the grid, in basis points.
pub const FEES: [u128; 3] = [5, 30, 100];

/// Window length in seconds crossed with bucket count.
pub const GRIDS: [(u64, u32); 6] = [
    (300, 6),
    (300, 12),
    (900, 6),
    (900, 12),
    (3600, 6),
    (3600, 12),
];

/// Boundary distances above the baseline print, in basis points.
pub const BOUNDARIES: [u128; 5] = [0, 10, 50, 200, 1000];

/// One synthetic configuration and every number this experiment computes for it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Row {
    /// Quote reserve, equal to the base reserve.
    pub depth: u128,
    /// Pool fee in basis points.
    pub fee_bps: u128,
    /// Window length in seconds.
    pub window_seconds: u64,
    /// Buckets in the window.
    pub buckets: u32,
    /// Buckets the adversary is exposed in.
    pub hold: u32,
    /// Boundary distance above the baseline print, in basis points.
    pub boundary_bps: u128,
    /// Required displacement of each distorted bucket, in basis points.
    pub bucket_bps: u128,
    /// Peak quote deployed by the contiguous schedule.
    pub capital: u128,
    /// Peak capital as a fraction of depth, in basis points, floored.
    pub capital_bps: u128,
    /// Cost of the contiguous schedule: one excursion held across `hold` buckets.
    pub contiguous: u128,
    /// Contiguous cost as a fraction of depth, in basis points, floored.
    pub cost_bps: u128,
    /// Cost of the spread schedule, when the window can separate the buckets.
    pub spread: Option<u128>,
    /// The spot-move estimator, which ignores the grid.
    pub naive_spot: u128,
    /// The per-bucket-independent estimator, which ignores the pool-state carry.
    pub naive_independent: u128,
    /// Flat upper bound on any round-trip cost against this pool.
    pub ceiling: u128,
    /// Whether the realized print of the contiguous schedule crossed the boundary.
    pub crossed: bool,
}

/// Compute one row.
pub fn row(
    depth: u128,
    fee_bps: u128,
    grid: Grid,
    hold: u32,
    boundary_bps: u128,
) -> Result<Row, TwapError> {
    let pool = Pool::new(depth, depth, fee_bps)?;
    let baseline = pool.price();
    let buckets = grid.buckets();
    let target = required_bucket_price(&baseline, buckets, hold, boundary_bps)?;
    let boundary = boundary_price(&baseline, boundary_bps)?;

    let contiguous_schedule = Schedule::contiguous(buckets, hold, &target)?;
    let contiguous = execute(&pool, &contiguous_schedule)?;
    let crossed = twap_at_least(&contiguous.samples, &boundary)?;

    let spread = match Schedule::spread(buckets, hold, &target)? {
        Some(schedule) => Some(execute(&pool, &schedule)?.net_cost),
        None => None,
    };

    let bucket_bps = boundary_bps * u128::from(buckets) / u128::from(hold);
    let capital_bps = contiguous.peak_capital.saturating_mul(crate::pool::BPS) / depth;
    let cost_bps = contiguous.net_cost.saturating_mul(crate::pool::BPS) / depth;

    Ok(Row {
        depth,
        fee_bps,
        window_seconds: grid.window_seconds(),
        buckets,
        hold,
        boundary_bps,
        bucket_bps,
        capital: contiguous.peak_capital,
        capital_bps,
        contiguous: contiguous.net_cost,
        cost_bps,
        spread,
        naive_spot: naive_spot_cost(&pool, boundary_bps)?,
        naive_independent: naive_independent_cost(&pool, buckets, hold, boundary_bps)?,
        ceiling: round_trip_cost_ceiling(depth, fee_bps)?,
        crossed,
    })
}

/// The hold counts the grid uses for a given bucket count: one, half, all.
#[must_use]
pub fn holds_for(buckets: u32) -> [u32; 3] {
    [1, buckets / 2, buckets]
}

/// Every row of the parameter grid, in a fixed order.
pub fn grid_rows() -> Result<Vec<Row>, TwapError> {
    let mut rows = Vec::new();
    for depth in DEPTHS {
        for fee_bps in FEES {
            for (window_seconds, buckets) in GRIDS {
                let grid = Grid::new(window_seconds, buckets)?;
                for hold in holds_for(buckets) {
                    for boundary_bps in BOUNDARIES {
                        rows.push(row(depth, fee_bps, grid, hold, boundary_bps)?);
                    }
                }
            }
        }
    }
    Ok(rows)
}

/// Render one row in the `key=value|...` corpus format.
#[must_use]
pub fn render_row(row: &Row) -> String {
    let spread = match row.spread {
        Some(value) => value.to_string(),
        None => "-".to_string(),
    };
    format!(
        "depth={}|fee_bps={}|window_s={}|buckets={}|hold={}|boundary_bps={}|bucket_bps={}|capital={}|capital_bps={}|contiguous={}|cost_bps={}|spread={}|naive_spot={}|naive_indep={}|ceiling={}|crossed={}",
        row.depth,
        row.fee_bps,
        row.window_seconds,
        row.buckets,
        row.hold,
        row.boundary_bps,
        row.bucket_bps,
        row.capital,
        row.capital_bps,
        row.contiguous,
        row.cost_bps,
        spread,
        row.naive_spot,
        row.naive_independent,
        row.ceiling,
        if row.crossed { "yes" } else { "no" },
    )
}

/// The whole byte-stable corpus, header included, newline terminated.
pub fn render_vectors_v1() -> Result<String, TwapError> {
    let mut out = String::new();
    out.push_str("manipulation-cost-vectors-v1\n");
    out.push_str("model=degg-manipulation-cost/v0\n");
    out.push_str(
        "pool=constant-product|print=reserve-ratio|reserves=balanced|arithmetic=exact-integer\n",
    );
    out.push_str(
        "recovery=self-reversal-into-the-same-pool|external-flow=none|gas=none|latency=none\n",
    );
    out.push_str("bound=lower-bound-under-stated-assumptions|data=synthetic-only\n");
    out.push_str(
        "columns=depth,fee_bps,window_s,buckets,hold,boundary_bps,bucket_bps,capital,capital_bps,contiguous,cost_bps,spread,naive_spot,naive_indep,ceiling,crossed\n",
    );
    for row in grid_rows()? {
        out.push_str(&render_row(&row));
        out.push('\n');
    }
    Ok(out)
}

/// The baseline print of every pool in the grid: one quote unit per base unit.
pub fn baseline_price() -> Result<Price, TwapError> {
    Ok(Price::new(1, 1)?)
}
