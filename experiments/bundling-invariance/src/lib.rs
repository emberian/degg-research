//! Bundling invariance, exhibited rather than argued.
//!
//! The definitions research memo on Question 8 of the joint definitions notice
//! takes the position that classification should be invariant under bundling and
//! unbundling: a set of cash-or-nothing claims covering every outcome of one
//! reference variable, without overlap, is a portfolio of such claims on that
//! variable, the complete set is economically identical to the collateral it was
//! issued against, and the two conversions are ordinary operations performed at
//! no cost. A criterion that answers differently for the bundle and for its
//! parts therefore contains a classification arbitrage exercisable for free.
//!
//! That is an argument. This crate turns it into a computation: a bounded corpus
//! of payoff objects, a small family of candidate classification criteria, and
//! an exhaustive check of every decomposition of every object under every
//! criterion, where each failure is emitted as a concrete witness — the object,
//! its decomposition, the two labels, and the zero-cost path between them.
//!
//! What is here:
//!
//! - [`payoff`] — the payoff object: an exhaustive partition, a nonnegative
//!   integer payoff vector over its cells, the economic facts a criterion may
//!   read, and the two costless operations (bundle, split) with their inverse
//!   relationship.
//! - [`market`] — the collateral ledger, so that "costless" is computed: claims
//!   are minted only as complete sets against collateral and burned only as
//!   complete sets, and the conservation identity that follows is checked
//!   exhaustively over operation sequences.
//! - [`criteria`] — a classification criterion as a decidable function from a
//!   payoff object's economic facts to a category label, with the candidate
//!   family: four payoff-shape tests, a prefunding test, the facts-based control
//!   the memo proposes, and a degenerate one-label control.
//! - [`corpus`] — the exhaustive enumeration and its bounds.
//! - [`invariance`] — the test, the witnesses, and the per-criterion counts.
//! - [`census`] — how many of *all* support-reading two-label criteria survive
//!   invariance, at the cell counts where that can be enumerated.
//! - [`transcript`] — the byte-stable corpus file under `vectors/`.
//!
//! ## Claim boundary
//!
//! VERIFIED, at exactly the bounds [`invariance::Bounds`] states: the payoff
//! arithmetic, the conservation identity of the collateral ledger, the label
//! each criterion assigns to each object in the corpus, the exhaustiveness of
//! the decomposition sweep, the witness counts, the minimized witnesses, and the
//! census counts. Those are deterministic offline measurements of this crate.
//!
//! PROPOSED: that bundling invariance is a test a classification criterion
//! should have to survive.
//!
//! Deliberately absent, each named rather than approximated: this is a synthetic
//! corpus over a toy model, so it exhibits the arbitrage structurally and
//! asserts nothing about any real rule's text, any real instrument, or any real
//! venue; the category labels are the criteria's own vocabulary and not
//! statutory categories, so no label here is a legal conclusion; there is no
//! price, probability, discount rate, fee, or market data, so nothing here
//! measures how profitable an arbitrage would be, only that its cost is zero;
//! there is no time, so nothing distinguishes a decomposition performed before
//! resolution from one performed after; and the corpus is bounded, so a
//! criterion recorded as invariant is invariant *on this corpus* and not
//! invariant as a theorem.

#![deny(missing_docs)]

pub mod census;
pub mod corpus;
pub mod criteria;
pub mod invariance;
pub mod market;
pub mod payoff;
pub mod transcript;

use std::sync::OnceLock;

/// Identifier of the model this crate implements.
pub const MODEL: &str = "degg-bundling-invariance/v0";

/// The memo this experiment was built for.
pub const MEMO: &str =
    "docs/regulatory/research-memos/definitions-q8-event-contracts-and-options-on-securities.md";

/// The sweep, computed once per process.
///
/// # Panics
///
/// Panics only if [`invariance::run`] does.
#[must_use]
pub fn report() -> &'static invariance::Report {
    static REPORT: OnceLock<invariance::Report> = OnceLock::new();
    REPORT.get_or_init(invariance::run)
}

/// The census, computed once per process.
#[must_use]
pub fn census_rows() -> &'static [census::CensusRow] {
    static ROWS: OnceLock<Vec<census::CensusRow>> = OnceLock::new();
    ROWS.get_or_init(census::census)
}
