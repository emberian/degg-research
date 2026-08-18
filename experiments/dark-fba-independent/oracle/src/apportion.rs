//! Exact largest-remainder apportionment.
//!
//! For side total `T` and target `V`:
//!
//! ```text
//! base_i = floor(q_i * V / T)
//! rem_i  = (q_i * V) mod T
//! ```
//!
//! The `V - sum base_i` residual atoms go one each to the largest remainders,
//! with ties broken by the earliest canonical rank. Each side sums to exactly
//! `V`; no residual atom becomes dust.

use crate::params::SLOTS;

/// One participant in an apportionment: canonical rank and eligible quantity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Claim {
    /// Canonical rank; lower wins equal remainders.
    pub rank: u8,
    /// Eligible quantity in base atoms.
    pub quantity: u32,
}

/// Apportion `target` atoms over `claims`, writing one award per claim.
///
/// Returns `None` when `total == 0` while `target > 0`, which the clearing
/// relation never produces because `target = min(Demand, Supply) <= total`.
pub fn largest_remainder(claims: &[Claim], target: u32) -> Option<[u32; SLOTS]> {
    let mut award = [0u32; SLOTS];
    if target == 0 {
        return Some(award);
    }
    let total: u64 = claims.iter().map(|claim| u64::from(claim.quantity)).sum();
    if total == 0 {
        return None;
    }

    let mut remainder = [0u64; SLOTS];
    let mut assigned: u64 = 0;
    for (index, claim) in claims.iter().enumerate() {
        let weighted = u64::from(claim.quantity) * u64::from(target);
        award[index] = (weighted / total) as u32;
        remainder[index] = weighted % total;
        assigned += u64::from(award[index]);
    }

    // Residual is strictly below the claim count, so one atom per claim suffices.
    let mut residual = u64::from(target).checked_sub(assigned)?;
    while residual > 0 {
        let mut best: Option<usize> = None;
        for index in 0..claims.len() {
            if remainder[index] == 0 {
                continue;
            }
            let better = match best {
                None => true,
                Some(current) => {
                    remainder[index] > remainder[current]
                        || (remainder[index] == remainder[current]
                            && claims[index].rank < claims[current].rank)
                }
            };
            if better {
                best = Some(index);
            }
        }
        let winner = best?;
        award[winner] += 1;
        remainder[winner] = 0;
        residual -= 1;
    }
    Some(award)
}
