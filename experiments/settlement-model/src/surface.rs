//! Transfer surfaces: what settling the same batch reveals on each of them.
//!
//! The computation's frozen leakage table (`DARK_FBA_RELATION.md` section 8)
//! hides side, quantity, occupancy, and individual fills, and marks the
//! settlement graph "outside v0; cannot be claimed hidden". This module makes
//! that last row concrete: the same executed settlements are projected onto
//! four surfaces, and what each projection mechanically contains is measured
//! by `tests/surface.rs` rather than asserted.
//!
//! - [`SettlementSurface::PublicPerClaim`] — every deposit, claim, and refund
//!   is a public account-and-amount event, as on a transparent chain.
//!   [`reconstruct`] then rebuilds each settled position's owner, side, exact
//!   fill, and exact reservation from the projection alone: the settlement
//!   layer retroactively publishes what the computation hid.
//! - [`SettlementSurface::PublicNetted`] — one net flow per account. The flows
//!   are exactly the owner deltas the computation marked owner-local, and the
//!   account list is exact participation.
//! - [`SettlementSurface::ShieldedAgent`] — a named agent executes custody
//!   and the public sees conservation totals only. Shielded, by definition:
//!   the agent sees every row.
//! - [`SettlementSurface::DarkTarget`] — refuses. No shielded-note or
//!   confidential-asset construction exists in this repository, and a refusal
//!   is the only honest projection.

use degg_relation_ir::batch::Side;

use crate::custody::{Asset, Balances, CustodyLedger, DepositRow, RefundRow};
use crate::relation::{ClaimRow, SettlementBook};

/// The transfer surface a composed system settles on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettlementSurface {
    /// Per-claim public transfers: account, position, and amounts all public.
    PublicPerClaim,
    /// Netted public transfers: one net flow per account.
    PublicNetted,
    /// A named settlement agent executes custody; the public sees totals.
    ShieldedAgent,
    /// The Dark settlement target. Refuses: no mechanism exists here.
    DarkTarget,
}

/// Why a surface refuses to produce a projection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceRefusal {
    /// No Dark settlement mechanism exists in this repository. Shielded
    /// notes, unlinkable claims, and value-conservation proofs are named
    /// absences, and labelling a public surface Dark would be the exact
    /// forbidden claim `PRIVACY_MODES.md` section 6 lists.
    DarkSettlementAbsent,
}

/// The named agent of the Shielded surface.
pub const SETTLEMENT_AGENT: &str = "degg-named-settlement-agent/v0";

/// One account's net flow on the netted surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NetFlow {
    /// The account.
    pub account: u8,
    /// Net base atoms: credits minus deposits.
    pub base: i64,
    /// Net quote atoms: credits minus deposits.
    pub quote: i64,
}

/// What one surface makes public.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PublicProjection {
    /// The per-claim surface: every custody event, with account, position,
    /// asset, and amount.
    PerClaim {
        /// The deposit rows.
        deposits: Vec<DepositRow>,
        /// The claim rows.
        claims: Vec<ClaimRow>,
        /// The refund rows.
        refunds: Vec<RefundRow>,
    },
    /// The netted surface: one signed flow per account that touched custody.
    Netted {
        /// The flows, in account order.
        flows: Vec<NetFlow>,
    },
    /// The Shielded-agent surface: conservation totals and the agent's name.
    Aggregate {
        /// The named agent that sees every row.
        agent: &'static str,
        /// Total credited through claims.
        credited: Balances,
        /// Total returned through refunds.
        refunded: Balances,
    },
}

fn flow(amount: u64) -> i64 {
    i64::try_from(amount).expect("model amounts are bounded far below i64::MAX")
}

/// Project the custody ledger and settlement book onto one surface.
pub fn project(
    custody: &CustodyLedger,
    book: &SettlementBook,
    surface: SettlementSurface,
) -> Result<PublicProjection, SurfaceRefusal> {
    match surface {
        SettlementSurface::PublicPerClaim => Ok(PublicProjection::PerClaim {
            deposits: custody.deposits().to_vec(),
            claims: book.claims().to_vec(),
            refunds: custody.refunds().to_vec(),
        }),
        SettlementSurface::PublicNetted => {
            let mut flows: std::collections::BTreeMap<u8, (i64, i64)> =
                std::collections::BTreeMap::new();
            for deposit in custody.deposits() {
                let entry = flows.entry(deposit.account).or_default();
                match deposit.asset {
                    Asset::Base => entry.0 -= flow(deposit.amount),
                    Asset::Quote => entry.1 -= flow(deposit.amount),
                }
            }
            for claim in book.claims() {
                let entry = flows.entry(claim.account).or_default();
                entry.0 += flow(claim.credited_base);
                entry.1 += flow(claim.credited_quote);
            }
            for refund in custody.refunds() {
                let entry = flows.entry(refund.account).or_default();
                match refund.asset {
                    Asset::Base => entry.0 += flow(refund.amount),
                    Asset::Quote => entry.1 += flow(refund.amount),
                }
            }
            Ok(PublicProjection::Netted {
                flows: flows
                    .into_iter()
                    .map(|(account, (base, quote))| NetFlow {
                        account,
                        base,
                        quote,
                    })
                    .collect(),
            })
        }
        SettlementSurface::ShieldedAgent => {
            let mut credited = Balances::zero();
            let mut refunded = Balances::zero();
            for claim in book.claims() {
                credited.base += claim.credited_base;
                credited.quote += claim.credited_quote;
            }
            for refund in custody.refunds() {
                match refund.asset {
                    Asset::Base => refunded.base += refund.amount,
                    Asset::Quote => refunded.quote += refund.amount,
                }
            }
            Ok(PublicProjection::Aggregate {
                agent: SETTLEMENT_AGENT,
                credited,
                refunded,
            })
        }
        SettlementSurface::DarkTarget => Err(SurfaceRefusal::DarkSettlementAbsent),
    }
}

/// One settled position, rebuilt from the per-claim projection alone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferredPosition {
    /// The claiming account.
    pub account: u8,
    /// The claimed position: the admission rank, disclosed by the claim.
    pub seq: u32,
    /// The side, disclosed by the deposit's asset.
    pub side: Side,
    /// The exact fill.
    pub fill: u64,
    /// The exact reservation, rebuilt from deposit and claim.
    pub budget: u64,
    /// For a sell: the reservation upper-bounds the quantity, and equals it
    /// when the reservation was the exact worst case.
    pub sell_quantity_bound: Option<u64>,
    /// For a buy: every `(limit tick, quantity)` pair whose worst-case
    /// obligation is exactly the budget.
    pub buy_candidates: Vec<(u8, u64)>,
}

/// Why the per-claim reconstruction could not run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReconstructionDefect {
    /// The account has no deposit row to join the claim against.
    MissingDeposit {
        /// The account.
        account: u8,
    },
    /// The account has more than one deposit row; this reconstruction only
    /// implements the one-position-per-account join. Amounts still leak; the
    /// join is merely harder.
    AmbiguousDeposits {
        /// The account.
        account: u8,
    },
    /// The claim and deposit rows are not consistent with any position.
    Inconsistent {
        /// The account.
        account: u8,
    },
}

/// Rebuild every settled position from the per-claim projection and the
/// public result alone.
///
/// The inputs are exactly what the public holds on a transparent settlement
/// surface: deposit rows, claim rows, the public clearing price (zero on
/// no-trade), and the frozen tick grid. Nothing owner-local and nothing
/// executor-local is consulted. That this function works is the packet's
/// central measurement.
pub fn reconstruct(
    deposits: &[DepositRow],
    claims: &[ClaimRow],
    price: u64,
    tick_prices: &[u64],
) -> Result<Vec<InferredPosition>, ReconstructionDefect> {
    let mut inferred = Vec::with_capacity(claims.len());
    for claim in claims {
        let mut rows = deposits.iter().filter(|row| row.account == claim.account);
        let deposit = rows.next().ok_or(ReconstructionDefect::MissingDeposit {
            account: claim.account,
        })?;
        if rows.next().is_some() {
            return Err(ReconstructionDefect::AmbiguousDeposits {
                account: claim.account,
            });
        }
        let budget = deposit.amount;
        let (side, fill) = match deposit.asset {
            Asset::Quote => (Side::Buy, claim.credited_base),
            Asset::Base => {
                if price == 0 {
                    (Side::Sell, 0)
                } else {
                    if claim.credited_quote % price != 0 {
                        return Err(ReconstructionDefect::Inconsistent {
                            account: claim.account,
                        });
                    }
                    (Side::Sell, claim.credited_quote / price)
                }
            }
        };
        let consistent = match side {
            Side::Buy => fill
                .checked_mul(price)
                .and_then(|spend| budget.checked_sub(spend))
                .is_some_and(|released| released == claim.credited_quote),
            Side::Sell => budget
                .checked_sub(fill)
                .is_some_and(|released| released == claim.credited_base),
        };
        if !consistent {
            return Err(ReconstructionDefect::Inconsistent {
                account: claim.account,
            });
        }
        let sell_quantity_bound = match side {
            Side::Sell => Some(budget),
            Side::Buy => None,
        };
        let buy_candidates = match side {
            Side::Buy => tick_prices
                .iter()
                .enumerate()
                .filter_map(|(tick, tick_price)| {
                    if *tick_price == 0 || budget % *tick_price != 0 {
                        return None;
                    }
                    let quantity = budget / *tick_price;
                    ((1..=15).contains(&quantity) && quantity >= fill)
                        .then_some((u8::try_from(tick).expect("the grid is bounded"), quantity))
                })
                .collect(),
            Side::Sell => Vec::new(),
        };
        inferred.push(InferredPosition {
            account: claim.account,
            seq: claim.seq,
            side,
            fill,
            budget,
            sell_quantity_bound,
            buy_candidates,
        });
    }
    Ok(inferred)
}
