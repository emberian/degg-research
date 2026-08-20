//! Off-hours thinness: the same window designs, read from a session that
//! carries a fraction of normal depth.
//!
//! # The question
//!
//! [`crate::table`] prices moving a bucket-grid print across a boundary on one
//! balanced pool at one depth. A settlement window that lands at three in the
//! morning is not read off that pool. It is read off whatever depth is standing
//! at three in the morning, which is less. This module asks, per window design:
//! **how much of its manipulation cost does a design keep when the session the
//! manipulation happens in is thin?**
//!
//! # The thin session
//!
//! A window of `window_seconds` is cut into `buckets` uniform buckets, exactly
//! as in [`crate::twap`]. The window *opens inside a thin session* of length
//! `session_seconds`; the normal session resumes when it ends. One price is
//! sampled per bucket, at the bucket's end, so bucket `i` is sampled inside the
//! thin session exactly when `(i + 1) * bucket_seconds <= session_seconds`, and
//!
//! ```text
//! thin_buckets = min(buckets, floor(session_seconds / bucket_seconds))
//! ```
//!
//! is the number of the window's samples the thin session carries. It is zero
//! when the grid's buckets are longer than the session --- a coarse grid over a
//! long window takes no sample inside a short thin session at all.
//!
//! # The depth ratio, exactly
//!
//! `depth_ratio = thin_depth / depth` is an exact rational, and the thin
//! session's pool holds `thin_depth` of *both* reserves against the normal
//! session's `depth` of both. Scaling both legs by the same factor is what
//! makes this a liquidity dial and not a price dial: the reserve-ratio print of
//! a balanced pool is one quote unit per base unit at every depth, so **thinning
//! the session does not move the print, only what it costs to move the print.**
//! The division must be exact; [`thin_depth`] refuses a ratio that would not
//! divide, rather than rounding a reserve.
//!
//! # The adversary
//!
//! The adversary of [`crate::twap`], confined to the thin session: it starts
//! flat, trades only against the thin session's pool, is exposed at the sample
//! instant of at most `hold` buckets, and must end flat. Only `thin_buckets` of
//! the window's samples are taken inside the session it trades in, so it is
//! exposed in
//!
//! ```text
//! hold_eff = min(hold, thin_buckets)
//! ```
//!
//! buckets, and moving the mean of all `buckets` samples by `boundary_bps`
//! while distorting only `hold_eff` of them requires moving each distorted
//! sample by `buckets * boundary_bps / hold_eff`. That is the whole interaction:
//! **thinning divides the cost of each basis point of displacement, and a
//! session that carries fewer of the window's samples multiplies the
//! displacement required.** Every bucket sampled outside the thin session
//! samples the undistorted print.
//!
//! # What this number is, and what it is not
//!
//! `cost` is the cost of *the stated off-hours schedule*: an attack mounted
//! from inside the thin session. It is **not** a minimum over all attacks. An
//! adversary that does not care when it acts can always attack the normal
//! session at `cost_v1`, so the cheapest attack on a design is
//! `min(cost, cost_v1)` and `cost_ratio_bps` above 10000 means only that **the
//! thin session offers no discount** --- never that the design is dearer to
//! attack than [`crate::table`] already reports.
//!
//! Named and not priced, each of which would lower the reported figure:
//! carrying inventory across the return of liquidity, so that a distortion
//! bought at thin depth stands into the normal session; splitting an attack
//! across both sessions; and every omission [`crate::pool::round_trip_to`]
//! already lists. Nothing here is market data, calibrated to any venue, or a
//! claim about any deployed pool, oracle, index, or contract.

use crate::pool::{BPS, Pool, PoolError, round_trip_cost_ceiling};
use crate::table::{self, DEPTHS, FEES, GRIDS, baseline_price, holds_for};
use crate::twap::{
    Grid, Schedule, TwapError, boundary_price, execute, required_bucket_price, twap_at_least,
};

/// Depth ratios of the thin session to a normal session, as exact rationals
/// `(numerator, denominator)`.
///
/// A full session, then a half, a quarter, a tenth, and a fiftieth of it. Every
/// denominator divides every depth in [`DEPTHS`], so every thin reserve in the
/// grid is an exact integer and no reserve is ever rounded.
pub const DEPTH_RATIOS: [(u128, u128); 5] = [(1, 1), (1, 2), (1, 4), (1, 10), (1, 50)];

/// Thin-session lengths in seconds.
///
/// Two lengths, chosen so that crossing them with the six bucket grids of
/// [`GRIDS`] produces every distinct `(buckets, thin_buckets)` pair those grids
/// can produce: a window that lies wholly inside the thin session, four
/// straddles, and a window whose buckets are longer than the session, which
/// takes no sample inside it. A third length adds a label, not a pair, because
/// under this model seconds enter only through `thin_buckets`.
pub const SESSION_SECONDS: [u64; 2] = [150, 600];

/// Boundary distances above the baseline print, in basis points.
///
/// The zero boundary of [`crate::table::BOUNDARIES`] is omitted here: it is
/// already crossed, so both the off-hours cost and the reference cost are zero
/// and the ratio this table exists to report would be `0/0`.
pub const BOUNDARIES: [u128; 4] = [10, 50, 200, 1000];

/// Every way the session model can refuse.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionError {
    /// A schedule or pool operation refused.
    Twap(TwapError),
    /// The depth ratio was not a ratio.
    BadRatio,
    /// The depth ratio would not divide the depth exactly.
    InexactThinDepth,
    /// The reference cost was zero, so no retention ratio exists.
    ZeroReference,
}

impl From<TwapError> for SessionError {
    fn from(error: TwapError) -> SessionError {
        SessionError::Twap(error)
    }
}

impl From<PoolError> for SessionError {
    fn from(error: PoolError) -> SessionError {
        SessionError::Twap(TwapError::Pool(error))
    }
}

/// How many of the window's samples the thin session carries.
///
/// Samples are taken at bucket ends and the thin session occupies the head of
/// the window, so this is `min(buckets, floor(session_seconds / bucket_seconds))`.
#[must_use]
pub fn thin_buckets(grid: Grid, session_seconds: u64) -> u32 {
    let covered = session_seconds / grid.bucket_seconds();
    let capped = covered.min(u64::from(grid.buckets()));
    // `capped` is at most `grid.buckets()`, which is a `u32`.
    capped as u32
}

/// The thin session's reserve, `depth * numerator / denominator`, exactly.
///
/// Refuses rather than rounds: a reserve that is not an exact integer multiple
/// of the ratio is a reserve this model will not invent.
pub fn thin_depth(depth: u128, numerator: u128, denominator: u128) -> Result<u128, SessionError> {
    if denominator == 0 || numerator == 0 || numerator > denominator {
        return Err(SessionError::BadRatio);
    }
    let scaled = depth
        .checked_mul(numerator)
        .ok_or(SessionError::Twap(TwapError::Pool(PoolError::Overflow)))?;
    if !scaled.is_multiple_of(denominator) {
        return Err(SessionError::InexactThinDepth);
    }
    Ok(scaled / denominator)
}

/// One synthetic configuration and what the off-hours attack on it costs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionRow {
    /// Normal-session quote reserve, equal to the base reserve.
    pub depth: u128,
    /// Pool fee in basis points.
    pub fee_bps: u128,
    /// Window length in seconds.
    pub window_seconds: u64,
    /// Buckets in the window.
    pub buckets: u32,
    /// Buckets the adversary may be exposed in.
    pub hold: u32,
    /// Boundary distance above the baseline print, in basis points.
    pub boundary_bps: u128,
    /// Numerator of the thin session's depth ratio.
    pub ratio_num: u128,
    /// Denominator of the thin session's depth ratio.
    pub ratio_den: u128,
    /// Length of the thin session in seconds.
    pub session_seconds: u64,
    /// Window samples taken inside the thin session.
    pub thin_buckets: u32,
    /// The thin session's reserve, both legs.
    pub thin_depth: u128,
    /// Buckets the confined adversary is actually exposed in.
    pub hold_eff: u32,
    /// Displacement each distorted sample needs, in basis points.
    pub bucket_bps: Option<u128>,
    /// Peak quote deployed inside the thin session.
    pub capital: Option<u128>,
    /// Cost of the off-hours schedule.
    pub cost: Option<u128>,
    /// Cost as a fraction of the thin session's own reserve, in basis points.
    pub cost_thin_bps: Option<u128>,
    /// Cost of the same design attacked at full depth: the `contiguous` column
    /// of the corresponding row of `vectors/v1.txt`.
    pub cost_v1: u128,
    /// `10000 * cost / cost_v1`: what fraction of the daylight cost the
    /// off-hours attack pays.
    pub cost_ratio_bps: Option<u128>,
    /// Whether the off-hours attack undercuts the daylight attack at all.
    pub offhours_cheaper: bool,
    /// Whether the realized print of the off-hours schedule crossed the boundary.
    pub crossed: Option<bool>,
}

/// Compute one row.
pub fn session_row(
    depth: u128,
    fee_bps: u128,
    grid: Grid,
    hold: u32,
    boundary_bps: u128,
    ratio: (u128, u128),
    session_seconds: u64,
) -> Result<SessionRow, SessionError> {
    let buckets = grid.buckets();
    let thin_count = thin_buckets(grid, session_seconds);
    let thin = thin_depth(depth, ratio.0, ratio.1)?;
    let reference = table::row(depth, fee_bps, grid, hold, boundary_bps)?.contiguous;
    if reference == 0 {
        return Err(SessionError::ZeroReference);
    }

    let mut out = SessionRow {
        depth,
        fee_bps,
        window_seconds: grid.window_seconds(),
        buckets,
        hold,
        boundary_bps,
        ratio_num: ratio.0,
        ratio_den: ratio.1,
        session_seconds,
        thin_buckets: thin_count,
        thin_depth: thin,
        hold_eff: 0,
        bucket_bps: None,
        capital: None,
        cost: None,
        cost_thin_bps: None,
        cost_v1: reference,
        cost_ratio_bps: None,
        offhours_cheaper: false,
        crossed: None,
    };
    if thin_count == 0 {
        // The grid takes no sample inside the session. There is no off-hours
        // attack on this design at all, which is a result and not a gap.
        return Ok(out);
    }

    let hold_eff = hold.min(thin_count);
    let baseline = baseline_price()?;
    let target = required_bucket_price(&baseline, buckets, hold_eff, boundary_bps)?;
    let pool = Pool::new(thin, thin, fee_bps)?;
    let schedule = Schedule::contiguous(thin_count, hold_eff, &target)?;
    let executed = execute(&pool, &schedule)?;

    // The window's remaining samples are taken after the thin session has
    // ended, on a pool the confined adversary never touches.
    let mut samples = executed.samples.clone();
    for _ in thin_count..buckets {
        samples.push(baseline);
    }
    let boundary = boundary_price(&baseline, boundary_bps)?;
    let crossed = twap_at_least(&samples, &boundary)?;

    out.hold_eff = hold_eff;
    out.bucket_bps = Some(boundary_bps * u128::from(buckets) / u128::from(hold_eff));
    out.capital = Some(executed.peak_capital);
    out.cost = Some(executed.net_cost);
    out.cost_thin_bps = Some(executed.net_cost.saturating_mul(BPS) / thin);
    out.cost_ratio_bps = Some(executed.net_cost.saturating_mul(BPS) / reference);
    out.offhours_cheaper = executed.net_cost < reference;
    out.crossed = Some(crossed);
    Ok(out)
}

/// The flat upper bound on any round trip against the thin session's pool.
///
/// Not a column of the corpus: it is a function of `thin_depth` and `fee_bps`
/// alone, and the tests hold every reported cost under it.
pub fn thin_ceiling(row: &SessionRow) -> Result<u128, SessionError> {
    Ok(round_trip_cost_ceiling(row.thin_depth, row.fee_bps)?)
}

/// Every row of the parameter grid, in a fixed order.
pub fn session_rows() -> Result<Vec<SessionRow>, SessionError> {
    let mut rows = Vec::new();
    for depth in DEPTHS {
        for fee_bps in FEES {
            for (window_seconds, buckets) in GRIDS {
                let grid = Grid::new(window_seconds, buckets)?;
                for hold in holds_for(buckets) {
                    for boundary_bps in BOUNDARIES {
                        for ratio in DEPTH_RATIOS {
                            for session_seconds in SESSION_SECONDS {
                                rows.push(session_row(
                                    depth,
                                    fee_bps,
                                    grid,
                                    hold,
                                    boundary_bps,
                                    ratio,
                                    session_seconds,
                                )?);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(rows)
}

/// Render an optional integer, with a dash where the model has no number.
fn optional(value: Option<u128>) -> String {
    match value {
        Some(value) => value.to_string(),
        None => "-".to_string(),
    }
}

/// Render one row in the `key=value|...` corpus format.
#[must_use]
pub fn render_row(row: &SessionRow) -> String {
    let crossed = match row.crossed {
        Some(true) => "yes",
        Some(false) => "no",
        None => "-",
    };
    format!(
        "depth={}|fee_bps={}|window_s={}|buckets={}|hold={}|boundary_bps={}|depth_ratio={}/{}|session_s={}|thin_buckets={}|thin_depth={}|hold_eff={}|bucket_bps={}|capital={}|cost={}|cost_thin_bps={}|cost_v1={}|cost_ratio_bps={}|offhours_cheaper={}|crossed={}",
        row.depth,
        row.fee_bps,
        row.window_seconds,
        row.buckets,
        row.hold,
        row.boundary_bps,
        row.ratio_num,
        row.ratio_den,
        row.session_seconds,
        row.thin_buckets,
        row.thin_depth,
        row.hold_eff,
        optional(row.bucket_bps),
        optional(row.capital),
        optional(row.cost),
        optional(row.cost_thin_bps),
        row.cost_v1,
        optional(row.cost_ratio_bps),
        if row.offhours_cheaper { "yes" } else { "no" },
        crossed,
    )
}

/// The whole byte-stable off-hours corpus, header included, newline terminated.
pub fn render_vectors_v2() -> Result<String, SessionError> {
    let mut out = String::new();
    out.push_str("manipulation-cost-vectors-v2-offhours\n");
    out.push_str(&format!("model={}\n", crate::MODEL));
    out.push_str(&format!("session-model={}\n", crate::SESSION_MODEL));
    out.push_str(
        "pool=constant-product|print=reserve-ratio|reserves=balanced|arithmetic=exact-integer\n",
    );
    out.push_str(
        "recovery=self-reversal-into-the-same-pool|external-flow=none|gas=none|latency=none\n",
    );
    out.push_str(
        "bound=cost-of-the-stated-off-hours-schedule|not-a-minimum-over-all-attacks|data=synthetic-only\n",
    );
    out.push_str(
        "question=what-fraction-of-a-window-design-manipulation-cost-survives-when-the-manipulable-session-is-thin\n",
    );
    out.push_str(
        "depth_ratio=thin_depth/depth|both-reserves-scaled-by-it|print-unchanged-by-thinning|division-must-be-exact\n",
    );
    out.push_str(
        "session=thin-session-opens-the-window|sample-instant=bucket-end|bucket_s=window_s/buckets\n",
    );
    out.push_str("thin_buckets=min(buckets,floor(session_s/bucket_s))\n");
    out.push_str(
        "adversary=confined-to-the-thin-session|hold_eff=min(hold,thin_buckets)|flat-elsewhere|samples-outside-the-session-are-undistorted\n",
    );
    out.push_str(
        "bucket_bps=buckets*boundary_bps/hold_eff|target=p0*(1+bucket_bps/10000)|p0=1-quote-unit-per-base-unit\n",
    );
    out.push_str("cost_thin_bps=10000*cost/thin_depth\n");
    out.push_str(
        "cost_v1=contiguous-column-of-the-same-parameters-in-vectors/v1.txt|the-same-design-attacked-at-full-depth\n",
    );
    out.push_str(
        "cost_ratio_bps=10000*cost/cost_v1|above-10000-means-the-thin-session-offers-no-discount-not-that-the-design-is-dearer-to-attack\n",
    );
    out.push_str(
        "omitted=attacking-the-normal-session-at-cost_v1|carrying-inventory-across-the-return-of-liquidity|external-flow-of-any-kind\n",
    );
    out.push_str(
        "no-sample-inside-the-session=thin_buckets-0|no-off-hours-attack-exists|every-computed-column-is-a-dash\n",
    );
    out.push_str(
        "boundary_bps=0-omitted-here|both-costs-are-zero-there-and-the-ratio-would-be-0/0\n",
    );
    out.push_str(
        "grid=depth(4)xfee(3)x(window_s,buckets)(6)xhold(3)xboundary_bps(4)xdepth_ratio(5)xsession_s(2)\n",
    );
    out.push_str(
        "columns=depth,fee_bps,window_s,buckets,hold,boundary_bps,depth_ratio,session_s,thin_buckets,thin_depth,hold_eff,bucket_bps,capital,cost,cost_thin_bps,cost_v1,cost_ratio_bps,offhours_cheaper,crossed\n",
    );
    for row in session_rows()? {
        out.push_str(&render_row(&row));
        out.push('\n');
    }
    Ok(out)
}

/// Number of header lines the corpus carries before its first row.
pub const HEADER_LINES: usize = 20;
