//! The time-weighted print over a frozen bucket grid, and the adversary's
//! cheapest schedule for moving it across a boundary.
//!
//! # The statistic
//!
//! A window of `window_seconds` is cut into `buckets` uniform buckets of equal
//! duration. Exactly one price is sampled per bucket, and the print is the
//! unweighted mean of the `n` samples, which is what an equal-duration bucket
//! grid makes of a time integral. This matches the shape of the observation
//! accumulator the program has already built --- one grid, one fixed duration
//! per bucket, an exact integral numerator retained rather than rounded --- and
//! deliberately not any deployed venue's oracle.
//!
//! # The adversary
//!
//! The adversary starts flat, trades only against the one synthetic pool, and
//! must end flat. It may hold a non-zero base position across the sample
//! instant of at most `k` of the `n` buckets; in every other bucket it is flat
//! at the sample instant. That is the operational reading of "hold the moved
//! price for `k` of `n` buckets": the constraint is on how many prints the
//! adversary is exposed under, not on how many trades it makes.
//!
//! Cost is quote paid in minus quote taken out, over the whole schedule, after
//! the terminal unwind. Capital is the running maximum of that difference,
//! which is a different and much larger number.
//!
//! # Two consequences worth naming before the numbers
//!
//! *Amplification.* Moving a mean of `n` samples by `D` while distorting only
//! `k` of them requires moving each distorted sample by `n*D/k`. Pricing the
//! boundary as a spot move --- the estimator [`naive_spot_cost`] --- therefore
//! understates the schedule by that factor.
//!
//! *The fee residual is a ratchet.* A round trip returns the pool to its
//! original base reserve with the fee residue sitting entirely on the quote
//! side, so the pool's price after an up-manipulation round trip is *above*
//! where it started. Repeated excursions start progressively closer to the
//! target. The residue always drifts the price in the direction the adversary
//! pushed.

use core::cmp::Ordering;

use crate::big::Big;
use crate::pool::{BPS, Pool, PoolError, Price, round_trip_to};

/// Every way the schedule model can refuse.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TwapError {
    /// A pool operation refused.
    Pool(PoolError),
    /// The bucket count was zero or did not divide the window.
    BadGrid,
    /// The hold count was zero or exceeded the bucket count.
    BadHold,
    /// A schedule did not have one entry per bucket.
    BadSchedule,
    /// An exact comparison exceeded the fixed big-integer width.
    ComparisonOverflow,
}

impl From<PoolError> for TwapError {
    fn from(error: PoolError) -> TwapError {
        TwapError::Pool(error)
    }
}

/// A frozen uniform sampling grid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Grid {
    window_seconds: u64,
    buckets: u32,
}

impl Grid {
    /// Construct a grid whose bucket count divides the window exactly.
    pub fn new(window_seconds: u64, buckets: u32) -> Result<Grid, TwapError> {
        if buckets == 0 || window_seconds == 0 {
            return Err(TwapError::BadGrid);
        }
        if !window_seconds.is_multiple_of(u64::from(buckets)) {
            return Err(TwapError::BadGrid);
        }
        Ok(Grid {
            window_seconds,
            buckets,
        })
    }

    /// Window length in seconds.
    #[must_use]
    pub const fn window_seconds(&self) -> u64 {
        self.window_seconds
    }

    /// Number of buckets in the window.
    #[must_use]
    pub const fn buckets(&self) -> u32 {
        self.buckets
    }

    /// Duration of one bucket in seconds.
    #[must_use]
    pub const fn bucket_seconds(&self) -> u64 {
        self.window_seconds / self.buckets as u64
    }
}

/// The price each distorted bucket must reach for the print to cross.
///
/// With a baseline `p0`, a boundary `p0 * (1 + D)` for `D = boundary_bps/BPS`,
/// `n` buckets and `k` distorted, the requirement is
/// `q >= p0 * (1 + n*D/k)`, returned exactly.
pub fn required_bucket_price(
    baseline: &Price,
    buckets: u32,
    hold: u32,
    boundary_bps: u128,
) -> Result<Price, TwapError> {
    if hold == 0 || hold > buckets {
        return Err(TwapError::BadHold);
    }
    let n = u128::from(buckets);
    let k = u128::from(hold);
    let numerator = k
        .checked_mul(BPS)
        .and_then(|value| value.checked_add(n.checked_mul(boundary_bps)?))
        .ok_or(TwapError::Pool(PoolError::Overflow))?;
    let denominator = k
        .checked_mul(BPS)
        .ok_or(TwapError::Pool(PoolError::Overflow))?;
    Ok(baseline.scaled(numerator, denominator)?)
}

/// The boundary price itself, `p0 * (1 + boundary_bps/BPS)`.
pub fn boundary_price(baseline: &Price, boundary_bps: u128) -> Result<Price, TwapError> {
    Ok(baseline.scaled(BPS + boundary_bps, BPS)?)
}

/// One adversary plan: for each bucket, whether the adversary is exposed at the
/// sample instant and, if so, the price it wants the pool to be at or above.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Schedule {
    entries: Vec<Option<Price>>,
}

impl Schedule {
    /// Build a schedule from one entry per bucket.
    pub fn new(entries: Vec<Option<Price>>) -> Result<Schedule, TwapError> {
        if entries.is_empty() {
            return Err(TwapError::BadSchedule);
        }
        Ok(Schedule { entries })
    }

    /// Number of buckets.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Always false; present because `len` exists.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Number of buckets the adversary is exposed in.
    #[must_use]
    pub fn hold_count(&self) -> usize {
        self.entries.iter().filter(|entry| entry.is_some()).count()
    }

    /// The per-bucket entries.
    #[must_use]
    pub fn entries(&self) -> &[Option<Price>] {
        &self.entries
    }

    /// `k` distorted buckets at the head of the window, held as one excursion.
    pub fn contiguous(buckets: u32, hold: u32, target: &Price) -> Result<Schedule, TwapError> {
        if hold == 0 || hold > buckets {
            return Err(TwapError::BadHold);
        }
        let entries = (0..buckets)
            .map(|index| if index < hold { Some(*target) } else { None })
            .collect();
        Schedule::new(entries)
    }

    /// `k` distorted buckets with a flat bucket between each pair, which forces
    /// the adversary to unwind and re-establish the position `k` times.
    ///
    /// Returns `None` when the window is too short to separate them.
    pub fn spread(buckets: u32, hold: u32, target: &Price) -> Result<Option<Schedule>, TwapError> {
        if hold == 0 || hold > buckets {
            return Err(TwapError::BadHold);
        }
        if hold > 1 && 2 * hold - 1 > buckets {
            return Ok(None);
        }
        let entries = (0..buckets)
            .map(|index| {
                if index % 2 == 0 && index / 2 < hold {
                    Some(*target)
                } else {
                    None
                }
            })
            .collect();
        Ok(Some(Schedule::new(entries)?))
    }
}

/// What executing a schedule against a pool actually cost.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Execution {
    /// Quote paid into the pool over the whole schedule.
    pub quote_in: u128,
    /// Quote taken out of the pool over the whole schedule, terminal unwind included.
    pub quote_out: u128,
    /// `quote_in - quote_out`.
    pub net_cost: u128,
    /// Running maximum of quote deployed but not yet recovered.
    pub peak_capital: u128,
    /// Number of swaps the schedule required.
    pub swaps: u32,
    /// The price sampled in each bucket.
    pub samples: Vec<Price>,
    /// Pool state after the terminal unwind.
    pub pool_after: Pool,
}

/// Execute a schedule against a pool, exactly.
///
/// In a bucket the adversary is exposed in, it buys just enough quote-side
/// input to bring the pool to or above the bucket's target, or nothing if the
/// pool is already there. In a bucket it is not exposed in, it sells its entire
/// base inventory back before the sample is taken. After the last bucket it
/// unwinds whatever remains. The recovery model is the one stated on
/// [`round_trip_to`].
pub fn execute(pool: &Pool, schedule: &Schedule) -> Result<Execution, TwapError> {
    let mut working = *pool;
    let mut inventory: u128 = 0;
    let mut quote_in: u128 = 0;
    let mut quote_out: u128 = 0;
    let mut peak_capital: u128 = 0;
    let mut swaps: u32 = 0;
    let mut samples = Vec::with_capacity(schedule.len());

    for entry in schedule.entries() {
        match entry {
            Some(target) => {
                if working.price().compare(target) == Ordering::Less {
                    let input = working.min_quote_in_to_reach(target)?;
                    let output = working.swap_quote_in(input)?;
                    inventory = inventory
                        .checked_add(output)
                        .ok_or(TwapError::Pool(PoolError::Overflow))?;
                    quote_in = quote_in
                        .checked_add(input)
                        .ok_or(TwapError::Pool(PoolError::Overflow))?;
                    swaps += 1;
                    peak_capital = peak_capital.max(quote_in.saturating_sub(quote_out));
                }
            }
            None => {
                if inventory > 0 {
                    let output = working.swap_base_in(inventory)?;
                    quote_out = quote_out
                        .checked_add(output)
                        .ok_or(TwapError::Pool(PoolError::Overflow))?;
                    inventory = 0;
                    swaps += 1;
                }
            }
        }
        samples.push(working.price());
    }

    if inventory > 0 {
        let output = working.swap_base_in(inventory)?;
        quote_out = quote_out
            .checked_add(output)
            .ok_or(TwapError::Pool(PoolError::Overflow))?;
        swaps += 1;
    }

    let net_cost = quote_in
        .checked_sub(quote_out)
        .ok_or(TwapError::Pool(PoolError::NegativeCost))?;
    Ok(Execution {
        quote_in,
        quote_out,
        net_cost,
        peak_capital,
        swaps,
        samples,
        pool_after: working,
    })
}

/// Exact test that the mean of `samples` is at or above `boundary`.
///
/// Cross-multiplied over the common denominator; no division and no rounding.
pub fn twap_at_least(samples: &[Price], boundary: &Price) -> Result<bool, TwapError> {
    if samples.is_empty() {
        return Err(TwapError::BadSchedule);
    }
    let mut left = Big::ZERO;
    for (index, sample) in samples.iter().enumerate() {
        let mut term = Big::from_u128(sample.num())
            .checked_mul_u128(boundary.den())
            .ok_or(TwapError::ComparisonOverflow)?;
        for (other, denominator) in samples.iter().enumerate() {
            if other != index {
                term = term
                    .checked_mul_u128(denominator.den())
                    .ok_or(TwapError::ComparisonOverflow)?;
            }
        }
        left = left
            .checked_add(term)
            .ok_or(TwapError::ComparisonOverflow)?;
    }
    let mut right = Big::from_u128(boundary.num())
        .checked_mul_u128(samples.len() as u128)
        .ok_or(TwapError::ComparisonOverflow)?;
    for sample in samples {
        right = right
            .checked_mul_u128(sample.den())
            .ok_or(TwapError::ComparisonOverflow)?;
    }
    Ok(left >= right)
}

/// The estimator that prices the boundary as a spot move and forgets the grid.
///
/// One round trip that carries the spot across the boundary. It answers a
/// different question than the schedule does, and it is the understatement the
/// falsifier tests pin down.
pub fn naive_spot_cost(pool: &Pool, boundary_bps: u128) -> Result<u128, TwapError> {
    let baseline = pool.price();
    let target = boundary_price(&baseline, boundary_bps)?;
    Ok(round_trip_to(pool, &target)?.net_cost)
}

/// The estimator that prices each distorted bucket independently from the
/// pristine pool and adds them up.
///
/// It gets the amplification right and the pool-state carry wrong: it charges
/// every excursion as if the pool had never been touched.
pub fn naive_independent_cost(
    pool: &Pool,
    buckets: u32,
    hold: u32,
    boundary_bps: u128,
) -> Result<u128, TwapError> {
    let baseline = pool.price();
    let target = required_bucket_price(&baseline, buckets, hold, boundary_bps)?;
    let single = round_trip_to(pool, &target)?.net_cost;
    single
        .checked_mul(u128::from(hold))
        .ok_or(TwapError::Pool(PoolError::Overflow))
}

/// The cheapest schedule found by exhaustive enumeration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Candidate {
    /// Bit `i` set means the adversary is exposed in bucket `i`.
    pub mask: u32,
    /// Chosen level index per exposed bucket, in bucket order.
    pub levels: Vec<usize>,
    /// Cost of the winning schedule.
    pub net_cost: u128,
    /// Capital of the winning schedule.
    pub peak_capital: u128,
}

/// Enumerate every exposure pattern of exactly `hold` buckets crossed with
/// every assignment of `level_bps` to those buckets, and return the cheapest
/// schedule whose realized print crosses the boundary.
///
/// Exhaustive and deterministic: ties break on the smaller mask, then on the
/// lexicographically smaller level vector. Intended for small `buckets`; the
/// search is `C(n, k) * levels^k` schedules.
pub fn exhaustive_min_cost(
    pool: &Pool,
    buckets: u32,
    hold: u32,
    boundary_bps: u128,
    level_bps: &[u128],
) -> Result<Option<Candidate>, TwapError> {
    if hold == 0 || hold > buckets {
        return Err(TwapError::BadHold);
    }
    if level_bps.is_empty() {
        return Err(TwapError::BadSchedule);
    }
    let baseline = pool.price();
    let boundary = boundary_price(&baseline, boundary_bps)?;
    let mut targets = Vec::with_capacity(level_bps.len());
    for level in level_bps {
        targets.push(baseline.scaled(BPS + level, BPS)?);
    }

    let mut best: Option<Candidate> = None;
    let hold_usize = hold as usize;
    for mask in 0u32..(1u32 << buckets) {
        if mask.count_ones() != hold {
            continue;
        }
        let mut levels = vec![0usize; hold_usize];
        loop {
            let mut entries = Vec::with_capacity(buckets as usize);
            let mut cursor = 0usize;
            for index in 0..buckets {
                if mask & (1 << index) != 0 {
                    entries.push(Some(targets[levels[cursor]]));
                    cursor += 1;
                } else {
                    entries.push(None);
                }
            }
            let schedule = Schedule::new(entries)?;
            if let Ok(execution) = execute(pool, &schedule)
                && twap_at_least(&execution.samples, &boundary)?
            {
                let candidate = Candidate {
                    mask,
                    levels: levels.clone(),
                    net_cost: execution.net_cost,
                    peak_capital: execution.peak_capital,
                };
                let replace = match &best {
                    None => true,
                    Some(current) => {
                        (candidate.net_cost, candidate.mask, &candidate.levels)
                            < (current.net_cost, current.mask, &current.levels)
                    }
                };
                if replace {
                    best = Some(candidate);
                }
            }
            let mut position = 0usize;
            loop {
                if position == hold_usize {
                    break;
                }
                levels[position] += 1;
                if levels[position] < level_bps.len() {
                    break;
                }
                levels[position] = 0;
                position += 1;
            }
            if position == hold_usize {
                break;
            }
        }
    }
    Ok(best)
}
