//! What it costs to move a venue-read time-weighted print across a decision
//! boundary, computed exactly, on synthetic pools only.
//!
//! `docs/regulatory/research-memos/definitions-q15-reference-integrity.md`
//! states a position and then gates it: the cost of moving a settlement
//! statistic for the length of its observation window "is a computable number,"
//! and "no number should appear in a filing before it exists." This crate is
//! the experiment that makes the number exist. It inserts the number nowhere;
//! whether any of it may enter a filing remains the memo's gate and the
//! author's decision.
//!
//! ## What is computed
//!
//! - [`pool`] --- a synthetic constant-product pool with an integer fee, and
//!   the exact integer cost of moving its reserve-ratio price to a target. The
//!   closed forms are derived in that module's documentation; the computation
//!   is exact-integer bisection, cross-checked against the closed form.
//! - [`twap`] --- a uniform bucket grid over a window with one sample per
//!   bucket, an adversary that must be flat in all but `k` of the `n` buckets,
//!   the exact cost of a schedule, and exhaustive enumeration of the cheapest
//!   schedule at small bucket counts.
//! - [`table`] --- the deterministic parameter sweep over depth, fee, window
//!   length, bucket count, hold count, and boundary distance.
//! - [`big`] --- a fixed-width unsigned integer used only for exact comparison.
//!
//! ## Claim boundary
//!
//! VERIFIED, at exactly the bounds the tests state: the arithmetic of this
//! model. Every reported number is an exact integer computed from stated
//! synthetic parameters, reproducible byte-for-byte.
//!
//! PROPOSED: that a number of this shape belongs in a surveillance picture at
//! all.
//!
//! **A lower bound, not a prediction, and not about any market.** The recovery
//! model assumed throughout --- stated in full on [`pool::round_trip_to`] --- is
//! that the adversary sells its whole position back into the same pool with no
//! other flow in between, recovering everything but two fee legs and its own
//! reversal impact. There is no competing order flow, no external arbitrageur
//! taking a share of the distorted price, no inventory or borrowing limit, no
//! gas or priority fee, no latency, no second venue, and no price process.
//! Every omission moves a real attacker's cost upward. There is no market data
//! here of any kind, no calibration to any venue, and no claim about any
//! deployed pool, oracle, or contract.
//!
//! Deliberately absent and named rather than approximated: concentrated
//! liquidity, multi-hop routing, sandwich and backrun competition, block
//! building and inclusion, the attacker's payoff on the derivative, detection
//! probability, and the cost of *holding* a price against arriving flow --- the
//! last of which is exactly why the hold count `k` behaves the way the
//! monotonicity tests report.

#![deny(missing_docs)]

pub mod big;
pub mod pool;
pub mod table;
pub mod twap;

/// Identifier of the model this crate implements.
pub const MODEL: &str = "degg-manipulation-cost/v0";
