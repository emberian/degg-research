//! A synthetic constant-product pool and the exact cost of moving its price.
//!
//! # The pool
//!
//! Reserves `(x, y)` of a base and a quote asset, both in integer units of the
//! smallest denomination, and a fee in basis points charged on the input leg.
//! The **print** this crate cares about is the reserve ratio `p = y / x`,
//! carried exactly as a reduced rational by [`Price`]. That is the quantity a
//! constant-product venue exposes to a reader and the quantity an observation
//! accumulator would absorb; it is not the fee-adjusted marginal price a
//! taker faces.
//!
//! A swap uses the ordinary constant-product rule with the fee withheld from
//! the input and the whole input added to the reserve, with the output floored
//! to an integer:
//!
//! ```text
//! quote in d:  a = d * (BPS - f)   out_base  = floor(x * a / (y * BPS + a))
//! base in  e:  a = e * (BPS - f)   out_quote = floor(y * a / (x * BPS + a))
//! ```
//!
//! Flooring the output is the direction a real pool rounds and the direction
//! that never flatters the adversary, so every cost this crate reports is at
//! or above the continuous-model cost by less than one unit per leg.
//!
//! # Closed form for the price move
//!
//! Write `g = (BPS - f) / BPS` for the fee retention factor. Putting `d` quote
//! into a pool at `(x0, y0)` leaves
//!
//! ```text
//! x1 = x0 * y0 / (y0 + g*d)          y1 = y0 + d
//! p1 = y1 / x1 = (y0 + d) * (y0 + g*d) / (x0 * y0)
//! ```
//!
//! so the input needed to reach a target price `p1` solves the quadratic
//!
//! ```text
//! g*d^2 + (1 + g)*y0*d + y0^2 - p1*x0*y0 = 0
//! d = ( -(1 + g)*y0 + sqrt( (1 - g)^2 * y0^2 + 4*g*p1*x0*y0 ) ) / (2*g)
//! ```
//!
//! With `p1 = a / b` reduced and `F = BPS`, clearing denominators gives the
//! all-integer coefficients used by [`closed_form_quote_in`]:
//!
//! ```text
//! A = (F - f) * b
//! B = (2F - f) * b * y0
//! C = F * (b * y0^2 - a * x0 * y0)          (negative when the target is above spot)
//! d = ( -B + sqrt(B^2 - 4*A*C) ) / (2*A)
//! ```
//!
//! The discriminant is a perfect square only by accident, so the closed form
//! is not exact in the rationals. This crate therefore computes the price move
//! by exact integer bisection ([`Pool::min_quote_in_to_reach`]) and keeps the
//! closed form as an independently derived cross-check, tested to agree to
//! within one unit.
//!
//! # Closed form for the round-trip cost, and its ceiling
//!
//! Buying with `d` quote and immediately selling the whole base proceeds back
//! returns
//!
//! ```text
//! d' = g^2 * d * (y0 + d) / (y0 + g^2 * d)
//! ```
//!
//! so the net cost of one excursion and its reversal is
//!
//! ```text
//! c(d) = d - d' = (1 - g^2) * y0 * d / (y0 + g^2 * d)
//! ```
//!
//! which is zero at `f = 0`, approximately `2*f*d` for small `d`, and --- the
//! part worth staring at --- **bounded above for every `d`**:
//!
//! ```text
//! sup_d c(d) = (1 - g^2) * y0 / g^2 = y0 * (F^2 - (F - f)^2) / (F - f)^2
//! ```
//!
//! See [`round_trip_cost_ceiling`]. Under this recovery model the burn of an
//! arbitrarily large price excursion is bounded by a fixed fraction of the
//! pool's quote reserve, about 0.60% at 30 bps. The unbounded quantity is the
//! *capital* `d`, not the cost.
//!
//! # What is not modelled
//!
//! No competing order flow, no external arbitrageur, no inventory limit, no
//! gas or priority fee, no latency, no depth outside this one pool, no price
//! process. Every one of those raises the attacker's true cost, which is why
//! the numbers here are a lower bound and never a prediction.

use core::cmp::Ordering;

use crate::big::Big;

/// Basis-point denominator.
pub const BPS: u128 = 10_000;

/// Largest reserve either side may hold, in integer units.
pub const MAX_RESERVE: u128 = 1_000_000_000_000;

/// Largest single-swap input this crate will search over, in integer units.
pub const MAX_SWAP_IN: u128 = 1_000_000_000_000_000;

/// Every way a pool operation can refuse.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PoolError {
    /// A reserve was zero.
    ZeroReserve,
    /// A reserve exceeded [`MAX_RESERVE`].
    ReserveTooLarge,
    /// The fee was not below [`BPS`].
    FeeOutOfRange,
    /// A denominator was zero.
    ZeroDenominator,
    /// A checked integer operation would not fit.
    Overflow,
    /// The target price is not reachable with an input at or below [`MAX_SWAP_IN`].
    PriceUnreachable,
    /// A round trip produced more quote than it consumed, which this model forbids.
    NegativeCost,
}

/// An exact price, carried as a reduced rational of quote units per base unit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Price {
    num: u128,
    den: u128,
}

impl Price {
    /// Construct a reduced price. The denominator must be non-zero.
    pub fn new(num: u128, den: u128) -> Result<Price, PoolError> {
        if den == 0 {
            return Err(PoolError::ZeroDenominator);
        }
        let divisor = gcd(num, den);
        let divisor = if divisor == 0 { 1 } else { divisor };
        Ok(Price {
            num: num / divisor,
            den: den / divisor,
        })
    }

    /// The reduced numerator.
    #[must_use]
    pub const fn num(&self) -> u128 {
        self.num
    }

    /// The reduced denominator.
    #[must_use]
    pub const fn den(&self) -> u128 {
        self.den
    }

    /// Exact comparison by cross-multiplication in [`Big`].
    #[must_use]
    pub fn compare(&self, other: &Price) -> Ordering {
        let left = Big::from_u128(self.num).checked_mul_u128(other.den);
        let right = Big::from_u128(other.num).checked_mul_u128(self.den);
        match (left, right) {
            (Some(left), Some(right)) => left.cmp(&right),
            _ => Ordering::Equal,
        }
    }

    /// Scale by an exact rational factor `mul_num / mul_den`.
    pub fn scaled(&self, mul_num: u128, mul_den: u128) -> Result<Price, PoolError> {
        let num = self.num.checked_mul(mul_num).ok_or(PoolError::Overflow)?;
        let den = self.den.checked_mul(mul_den).ok_or(PoolError::Overflow)?;
        Price::new(num, den)
    }

    /// Floor of the displacement of this price above `base`, in basis points.
    ///
    /// Returns zero when this price is at or below `base`.
    pub fn displacement_bps_above(&self, base: &Price) -> Result<u128, PoolError> {
        if self.compare(base) != Ordering::Greater {
            return Ok(0);
        }
        let left = self
            .num
            .checked_mul(base.den)
            .ok_or(PoolError::Overflow)?
            .checked_sub(base.num.checked_mul(self.den).ok_or(PoolError::Overflow)?)
            .ok_or(PoolError::Overflow)?;
        let scaled = left.checked_mul(BPS).ok_or(PoolError::Overflow)?;
        let right = base.num.checked_mul(self.den).ok_or(PoolError::Overflow)?;
        if right == 0 {
            return Err(PoolError::ZeroDenominator);
        }
        Ok(scaled / right)
    }
}

/// Greatest common divisor.
#[must_use]
pub fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// A synthetic constant-product pool.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pool {
    base: u128,
    quote: u128,
    fee_bps: u128,
}

impl Pool {
    /// Construct a pool. Both reserves must be non-zero and at or below
    /// [`MAX_RESERVE`]; the fee must be below [`BPS`].
    pub fn new(base: u128, quote: u128, fee_bps: u128) -> Result<Pool, PoolError> {
        if base == 0 || quote == 0 {
            return Err(PoolError::ZeroReserve);
        }
        if base > MAX_RESERVE || quote > MAX_RESERVE {
            return Err(PoolError::ReserveTooLarge);
        }
        if fee_bps >= BPS {
            return Err(PoolError::FeeOutOfRange);
        }
        Ok(Pool {
            base,
            quote,
            fee_bps,
        })
    }

    /// The base reserve.
    #[must_use]
    pub const fn base(&self) -> u128 {
        self.base
    }

    /// The quote reserve.
    #[must_use]
    pub const fn quote(&self) -> u128 {
        self.quote
    }

    /// The fee in basis points.
    #[must_use]
    pub const fn fee_bps(&self) -> u128 {
        self.fee_bps
    }

    /// The reserve-ratio price, exactly.
    pub fn price(&self) -> Price {
        Price {
            num: self.quote,
            den: self.base,
        }
    }

    /// Base output for a quote input, without applying it.
    pub fn quote_in_output(&self, quote_in: u128) -> Result<u128, PoolError> {
        if quote_in == 0 {
            return Ok(0);
        }
        let effective = quote_in
            .checked_mul(BPS - self.fee_bps)
            .ok_or(PoolError::Overflow)?;
        let numerator = self
            .base
            .checked_mul(effective)
            .ok_or(PoolError::Overflow)?;
        let denominator = self
            .quote
            .checked_mul(BPS)
            .ok_or(PoolError::Overflow)?
            .checked_add(effective)
            .ok_or(PoolError::Overflow)?;
        Ok(numerator / denominator)
    }

    /// Quote output for a base input, without applying it.
    pub fn base_in_output(&self, base_in: u128) -> Result<u128, PoolError> {
        if base_in == 0 {
            return Ok(0);
        }
        let effective = base_in
            .checked_mul(BPS - self.fee_bps)
            .ok_or(PoolError::Overflow)?;
        let numerator = self
            .quote
            .checked_mul(effective)
            .ok_or(PoolError::Overflow)?;
        let denominator = self
            .base
            .checked_mul(BPS)
            .ok_or(PoolError::Overflow)?
            .checked_add(effective)
            .ok_or(PoolError::Overflow)?;
        Ok(numerator / denominator)
    }

    /// Apply a quote-in swap, returning the base output. Raises the price.
    pub fn swap_quote_in(&mut self, quote_in: u128) -> Result<u128, PoolError> {
        let out = self.quote_in_output(quote_in)?;
        if out >= self.base {
            return Err(PoolError::Overflow);
        }
        self.base -= out;
        self.quote = self
            .quote
            .checked_add(quote_in)
            .ok_or(PoolError::Overflow)?;
        Ok(out)
    }

    /// Apply a base-in swap, returning the quote output. Lowers the price.
    pub fn swap_base_in(&mut self, base_in: u128) -> Result<u128, PoolError> {
        let out = self.base_in_output(base_in)?;
        if out >= self.quote {
            return Err(PoolError::Overflow);
        }
        self.quote -= out;
        self.base = self.base.checked_add(base_in).ok_or(PoolError::Overflow)?;
        Ok(out)
    }

    /// Smallest integer quote input whose post-swap price is at least `target`.
    ///
    /// Zero when the pool is already at or above `target`. The post-swap price
    /// is strictly increasing in the input, so bisection is exact.
    pub fn min_quote_in_to_reach(&self, target: &Price) -> Result<u128, PoolError> {
        if self.price().compare(target) != Ordering::Less {
            return Ok(0);
        }
        let mut high: u128 = 1;
        loop {
            if high > MAX_SWAP_IN {
                return Err(PoolError::PriceUnreachable);
            }
            let mut probe = *self;
            probe.swap_quote_in(high)?;
            if probe.price().compare(target) != Ordering::Less {
                break;
            }
            high = high.checked_mul(2).ok_or(PoolError::Overflow)?;
        }
        let mut low: u128 = 0;
        while high - low > 1 {
            let mid = low + (high - low) / 2;
            let mut probe = *self;
            probe.swap_quote_in(mid)?;
            if probe.price().compare(target) == Ordering::Less {
                low = mid;
            } else {
                high = mid;
            }
        }
        Ok(high)
    }

    /// Smallest integer base input whose post-swap price is at most `target`.
    ///
    /// Zero when the pool is already at or below `target`.
    pub fn min_base_in_to_fall_to(&self, target: &Price) -> Result<u128, PoolError> {
        if self.price().compare(target) != Ordering::Greater {
            return Ok(0);
        }
        let mut high: u128 = 1;
        loop {
            if high > MAX_SWAP_IN {
                return Err(PoolError::PriceUnreachable);
            }
            let mut probe = *self;
            probe.swap_base_in(high)?;
            if probe.price().compare(target) != Ordering::Greater {
                break;
            }
            high = high.checked_mul(2).ok_or(PoolError::Overflow)?;
        }
        let mut low: u128 = 0;
        while high - low > 1 {
            let mid = low + (high - low) / 2;
            let mut probe = *self;
            probe.swap_base_in(mid)?;
            if probe.price().compare(target) == Ordering::Greater {
                low = mid;
            } else {
                high = mid;
            }
        }
        Ok(high)
    }
}

/// One excursion and its immediate reversal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RoundTrip {
    /// Quote paid into the pool.
    pub quote_in: u128,
    /// Quote received back on the reversal.
    pub quote_out: u128,
    /// Base bought and then returned.
    pub base_bought: u128,
    /// `quote_in - quote_out`: the whole cost of the excursion under this model.
    pub net_cost: u128,
    /// Price observed at the top of the excursion.
    pub peak_price: Price,
    /// Pool state after the reversal.
    pub pool_after: Pool,
}

/// Push the price to `target`, observe, and reverse the whole position.
///
/// The recovery model is stated once, here: the adversary sells back exactly
/// the base it bought, into the same pool, with no other flow in between, and
/// keeps everything the pool returns. It therefore recovers its capital less
/// two fee legs and the price impact of its own reversal. No arbitrageur takes
/// a share, no one front-runs the reversal, and no one trades against the
/// distorted price while it stands. Each of those omissions can only raise a
/// real attacker's cost, so [`RoundTrip::net_cost`] is a lower bound.
pub fn round_trip_to(pool: &Pool, target: &Price) -> Result<RoundTrip, PoolError> {
    let mut working = *pool;
    let quote_in = working.min_quote_in_to_reach(target)?;
    let base_bought = working.swap_quote_in(quote_in)?;
    let peak_price = working.price();
    let quote_out = working.swap_base_in(base_bought)?;
    let net_cost = quote_in
        .checked_sub(quote_out)
        .ok_or(PoolError::NegativeCost)?;
    Ok(RoundTrip {
        quote_in,
        quote_out,
        base_bought,
        net_cost,
        peak_price,
        pool_after: working,
    })
}

/// The continuous-model round-trip cost `(1 - g^2) * y0 * d / (y0 + g^2 * d)`,
/// floored to an integer.
pub fn closed_form_round_trip_cost(
    quote_reserve: u128,
    fee_bps: u128,
    quote_in: u128,
) -> Result<u128, PoolError> {
    if fee_bps >= BPS {
        return Err(PoolError::FeeOutOfRange);
    }
    let retained = BPS - fee_bps;
    let retained_sq = retained * retained;
    let full_sq = BPS * BPS;
    let numerator = quote_reserve
        .checked_mul(quote_in)
        .ok_or(PoolError::Overflow)?
        .checked_mul(full_sq - retained_sq)
        .ok_or(PoolError::Overflow)?;
    let denominator = quote_reserve
        .checked_mul(full_sq)
        .ok_or(PoolError::Overflow)?
        .checked_add(
            retained_sq
                .checked_mul(quote_in)
                .ok_or(PoolError::Overflow)?,
        )
        .ok_or(PoolError::Overflow)?;
    if denominator == 0 {
        return Err(PoolError::ZeroDenominator);
    }
    Ok(numerator / denominator)
}

/// `sup_d c(d) = y0 * (F^2 - (F - f)^2) / (F - f)^2`, floored.
///
/// No round trip on this pool, at any target price whatsoever, costs more than
/// this. It is the flat ceiling on the whole manipulation budget under the
/// recovery model of [`round_trip_to`].
pub fn round_trip_cost_ceiling(quote_reserve: u128, fee_bps: u128) -> Result<u128, PoolError> {
    if fee_bps >= BPS {
        return Err(PoolError::FeeOutOfRange);
    }
    let retained = BPS - fee_bps;
    let retained_sq = retained * retained;
    let full_sq = BPS * BPS;
    let numerator = quote_reserve
        .checked_mul(full_sq - retained_sq)
        .ok_or(PoolError::Overflow)?;
    Ok(numerator / retained_sq)
}

/// The closed-form quote input rounded up, for cross-checking bisection.
///
/// Solves `A*d^2 + B*d + C = 0` with the integer coefficients derived in the
/// module documentation, using an exact integer square root. Restricted to
/// parameters whose discriminant fits in a `u128`; it exists to check
/// [`Pool::min_quote_in_to_reach`], not to replace it.
pub fn closed_form_quote_in(pool: &Pool, target: &Price) -> Result<u128, PoolError> {
    let base = pool.base();
    let quote = pool.quote();
    let fee = pool.fee_bps();
    if pool.price().compare(target) != Ordering::Less {
        return Ok(0);
    }
    let a_num = target.num();
    let b_den = target.den();
    let coefficient_a = (BPS - fee).checked_mul(b_den).ok_or(PoolError::Overflow)?;
    let coefficient_b = (2 * BPS - fee)
        .checked_mul(b_den)
        .ok_or(PoolError::Overflow)?
        .checked_mul(quote)
        .ok_or(PoolError::Overflow)?;
    // C is negative exactly when the target is above spot, which the guard above ensures.
    let deficit_right = a_num
        .checked_mul(base)
        .ok_or(PoolError::Overflow)?
        .checked_mul(quote)
        .ok_or(PoolError::Overflow)?;
    let deficit_left = b_den
        .checked_mul(quote)
        .ok_or(PoolError::Overflow)?
        .checked_mul(quote)
        .ok_or(PoolError::Overflow)?;
    let magnitude_c = BPS
        .checked_mul(
            deficit_right
                .checked_sub(deficit_left)
                .ok_or(PoolError::Overflow)?,
        )
        .ok_or(PoolError::Overflow)?;
    let discriminant = coefficient_b
        .checked_mul(coefficient_b)
        .ok_or(PoolError::Overflow)?
        .checked_add(
            4u128
                .checked_mul(coefficient_a)
                .ok_or(PoolError::Overflow)?
                .checked_mul(magnitude_c)
                .ok_or(PoolError::Overflow)?,
        )
        .ok_or(PoolError::Overflow)?;
    let root = discriminant.isqrt();
    let exact_root = root * root == discriminant;
    let numerator = root.saturating_sub(coefficient_b);
    let denominator = 2u128
        .checked_mul(coefficient_a)
        .ok_or(PoolError::Overflow)?;
    let floored = numerator / denominator;
    if exact_root && numerator % denominator == 0 {
        Ok(floored)
    } else {
        Ok(floored + 1)
    }
}
