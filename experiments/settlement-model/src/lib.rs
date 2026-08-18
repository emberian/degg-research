//! The settlement relation for `dark-fba/n4-k4-q15/v0`, as a deterministic
//! offline model.
//!
//! **Private computation alone does not make settlement Dark.** This crate
//! models the relation `CLAUDE_HANDOFF.md` section 6 P1.7 asks for —
//! authorized custody, nullifiers, retries, conservation, and the leakage of
//! public and private transfer surfaces — as a *separate* relation joined to
//! the computation by exact identifiers, exactly as `ARCHITECTURE.md`
//! section 5 requires. Its sharpest content is a measurement: on a
//! transparent per-claim surface, [`surface::reconstruct`] rebuilds every
//! settled position's owner, side, exact fill, and exact reservation from
//! the public settlement transcript alone.
//!
//! ## What is composed
//!
//! This crate implements no admission, no clearing, and no receipts of its
//! own. It is a path dependency on the three landed packets:
//!
//! - [`degg_inclusion_availability`] supplies the cutoff root, inclusion
//!   receipts, the typed abort machine whose `Settled` and terminal-abort
//!   phases gate this relation, and the tagged hash;
//! - [`degg_shielded_baseline`] supplies the computation receipt, the
//!   per-position delivery commitment and its openings, and the end-to-end
//!   session the tests drive;
//! - [`degg_relation_ir`] supplies the frozen relation module, the public
//!   outcome, and the tick grid the re-derivation prices against.
//!
//! ## What this crate adds
//!
//! - [`custody`] — the custody ledger: `Reserved -> Obligated -> Settled |
//!   Refunded` per admission nullifier, a pre-funded two-asset pool, checked
//!   payouts, and the exact conservation invariant.
//! - [`authorize`] — settlement authorization: the conjunction of receipt,
//!   inclusion receipt, delivery opening, and custody consistency, with every
//!   delta re-derived rather than trusted, in a frozen check order.
//! - [`relation`] — the settlement book: obligation on observing one settled
//!   receipt, execution idempotent by settlement nullifier, and the
//!   terminal-abort refund gate.
//! - [`surface`] — the four transfer surfaces and the reconstruction
//!   measurement.
//! - [`transcript`] — the byte-stable corpus renderer.
//!
//! ## Claim boundary
//!
//! VERIFIED, at exactly the bounds the tests state: the custody state
//! machine, the frozen authorization order, idempotent execution, refund and
//! settlement conservation, order-independent pool solvency on honest runs,
//! and the surface measurements.
//!
//! REJECTED as claims, here and everywhere in this crate: that settlement
//! checks correctness (it checks binding, arithmetic consistency, and
//! custody; a wrong-but-consistent result settles and conserves, which
//! `tests/residual_inheritance.rs` demonstrates); that any surface here is
//! Dark (the Dark target refuses); that a modelled pool is a chain, a token,
//! or an account model; or that anything here has a cryptographic, economic,
//! or liveness property.
//!
//! Deliberately absent, each named rather than approximated: no chain, no
//! token standard, no signatures and therefore no attribution, no fees, no
//! multi-batch custody, no shielded-note construction, and no adjudication
//! rule that turns an omission verdict into funds.

#![deny(missing_docs)]

pub mod authorize;
pub mod custody;
pub mod harness;
pub mod relation;
pub mod surface;
pub mod transcript;

/// Identifier of the model this crate implements.
pub const MODEL: &str = "degg-settlement/v0";

/// The declared visibility of each settlement surface, in one sentence.
///
/// The counterpart of
/// [`degg_shielded_baseline::SHIELDED_VISIBILITY_DISCLOSURE`]: settlement
/// surfaces differ in who sees the account-and-amount graph, not in whether
/// the graph exists.
pub const SETTLEMENT_VISIBILITY_DISCLOSURE: &str = "per-claim public settlement publishes every account, position, side, fill, and \
     reservation; netted public settlement publishes exact participation and every owner's \
     net deltas; a named settlement agent sees every row; a Dark settlement surface does \
     not exist here and its target refuses";
