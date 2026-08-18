//! Independent oracle for the fixed-grid batch relation `dark-fba/n4-k4-q15/v0`.
//!
//! This crate is written from `relations/CLEARING_V0.md` and
//! `docs/research/DARK_FBA_RELATION.md` alone, as the second implementation of
//! a two-implementation differential experiment. It is dependency-free, uses
//! exact integer arithmetic only, and is deterministic.
//!
//! It provides no confidentiality of any kind. The evaluating process receives
//! every order and computes every local output, so it is Clear or
//! `ShieldedSingleExecutor` depending only on who may inspect the process. It
//! refuses [`Mode::DarkTarget`].
//!
//! Pipeline: [`admit::screen`] then [`curve::curves`] then [`curve::select`]
//! then [`settle::settle`], with [`settle::audit`] as the independent
//! conservation falsifier.

#![deny(missing_docs)]

pub mod admit;
pub mod apportion;
pub mod book;
pub mod curve;
pub mod params;
pub mod settle;

use admit::Refusal;
use book::{Batch, Mode};
use curve::Clearing;
use settle::Settlement;

/// The frozen public boundary of one batch evaluation.
///
/// Curves, occupancy, orders, allocations, owner deltas, reservations, and
/// nullifiers are deliberately absent from this struct: in the Dark target they
/// are not public outputs. Nothing here is enforced by cryptography.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PublicResult {
    /// Relation identifier and version.
    pub relation: &'static str,
    /// Batch identifier.
    pub batch: u64,
    /// Market identifier.
    pub market: u64,
    /// Accepted-input root, preserved verbatim.
    pub accepted_input_root: [u8; 32],
    /// Selected tick index, or `None` for the no-trade tag.
    pub tick: Option<u8>,
    /// Aggregate matched volume; zero exactly when no trade.
    pub volume: u32,
    /// Public refusal class, or `None` on success.
    pub refusal: Option<&'static str>,
}

/// The full evaluation of one batch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The batch is valid; a no-trade batch settles here with zero volume.
    Settled(Settlement),
    /// The batch is refused with one typed class. Never reinterpreted as no trade.
    Refused(Refusal),
}

/// Evaluate one batch end to end.
pub fn evaluate(batch: &Batch) -> Outcome {
    let outcome = || -> Result<Settlement, Refusal> {
        let book = admit::screen(batch)?;
        let curves = curve::curves(&book)?;
        let clearing = curve::select(&curves);
        settle::settle(&book, clearing)
    };
    match outcome() {
        Ok(settlement) => Outcome::Settled(settlement),
        Err(refusal) => Outcome::Refused(refusal),
    }
}

/// Project an outcome onto the frozen public boundary.
pub fn publish(batch: &Batch, outcome: &Outcome) -> PublicResult {
    let mut result = PublicResult {
        relation: params::RELATION,
        batch: batch.batch,
        market: batch.market,
        accepted_input_root: batch.accepted_input_root,
        tick: None,
        volume: 0,
        refusal: None,
    };
    match outcome {
        Outcome::Refused(refusal) => result.refusal = Some(refusal.class()),
        Outcome::Settled(settlement) => {
            if let Clearing::Trade { tick, volume, .. } = settlement.clearing {
                result.tick = Some(tick);
                result.volume = volume;
            }
        }
    }
    result
}

/// Whether a mode may execute at all.
///
/// `DarkTarget` is refused unconditionally: no Dark backend exists, and an
/// opaque root, fixed padding, or exact arithmetic does not create one.
pub fn mode_is_executable(mode: Mode) -> bool {
    match mode {
        Mode::Clear | Mode::ShieldedSingleExecutor => true,
        Mode::DarkTarget => false,
    }
}
