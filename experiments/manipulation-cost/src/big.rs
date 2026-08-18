//! A fixed-width unsigned integer used only for exact comparison.
//!
//! Nothing in this crate divides a [`Big`]. Every use is a cross-multiplied
//! comparison, so multiplication, addition, and ordering are the whole
//! interface. Every operation is checked: an operation that would not fit
//! returns `None` rather than wrapping, because a wrapped comparison would
//! silently change a reported cost.
//!
//! The width is 2048 bits. The largest quantity this crate forms is the common
//! denominator of a twelve-bucket time-weighted average over reserves bounded
//! by [`crate::pool::MAX_RESERVE`], which is below `10^140`.

use core::cmp::Ordering;

/// Number of 64-bit limbs, little-endian.
const LIMBS: usize = 32;

/// A 2048-bit unsigned integer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Big {
    limbs: [u64; LIMBS],
}

impl Big {
    /// Zero.
    pub const ZERO: Big = Big { limbs: [0; LIMBS] };

    /// Widen a `u128`.
    #[must_use]
    pub const fn from_u128(value: u128) -> Big {
        let mut limbs = [0u64; LIMBS];
        limbs[0] = value as u64;
        limbs[1] = (value >> 64) as u64;
        Big { limbs }
    }

    /// True when the value is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.limbs.iter().all(|limb| *limb == 0)
    }

    /// Add, returning `None` on overflow of the fixed width.
    #[must_use]
    pub fn checked_add(self, other: Big) -> Option<Big> {
        let mut out = [0u64; LIMBS];
        let mut carry: u128 = 0;
        for ((slot, left), right) in out.iter_mut().zip(self.limbs).zip(other.limbs) {
            let sum = u128::from(left) + u128::from(right) + carry;
            *slot = sum as u64;
            carry = sum >> 64;
        }
        if carry != 0 {
            return None;
        }
        Some(Big { limbs: out })
    }

    /// Multiply by a `u64`, returning `None` on overflow of the fixed width.
    #[must_use]
    fn checked_mul_u64(self, multiplier: u64) -> Option<Big> {
        let mut out = [0u64; LIMBS];
        let mut carry: u128 = 0;
        for (slot, limb) in out.iter_mut().zip(self.limbs) {
            let product = u128::from(limb) * u128::from(multiplier) + carry;
            *slot = product as u64;
            carry = product >> 64;
        }
        if carry != 0 {
            return None;
        }
        Some(Big { limbs: out })
    }

    /// Shift left by one limb, returning `None` when the top limb is occupied.
    #[must_use]
    fn checked_shl_limb(self) -> Option<Big> {
        if self.limbs[LIMBS - 1] != 0 {
            return None;
        }
        let mut out = [0u64; LIMBS];
        out[1..].copy_from_slice(&self.limbs[..LIMBS - 1]);
        Some(Big { limbs: out })
    }

    /// Multiply by a `u128`, returning `None` on overflow of the fixed width.
    #[must_use]
    pub fn checked_mul_u128(self, multiplier: u128) -> Option<Big> {
        let low = multiplier as u64;
        let high = (multiplier >> 64) as u64;
        let low_part = self.checked_mul_u64(low)?;
        if high == 0 {
            return Some(low_part);
        }
        let high_part = self.checked_mul_u64(high)?.checked_shl_limb()?;
        low_part.checked_add(high_part)
    }
}

impl Ord for Big {
    fn cmp(&self, other: &Big) -> Ordering {
        for (left, right) in self.limbs.iter().zip(other.limbs.iter()).rev() {
            match left.cmp(right) {
                Ordering::Equal => {}
                unequal => return unequal,
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Big {
    fn partial_cmp(&self, other: &Big) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
