//! Inclusion and availability for a batch relation, modelled offline and exactly.
//!
//! `CLAUDE_HANDOFF.md` §2 places `input inclusion + availability +
//! non-equivocation + typed abort` between the relation backends and the public
//! receipt, and `docs/ARCHITECTURE.md` §6 says plainly that darkness cannot
//! excuse hidden omission. `docs/research/DARK_FBA_RELATION.md` §5 states the
//! four properties a valid batch requires but implements none of them: its
//! offline toy exercises refusal booleans, and its accepted-input root `R` is
//! preserved verbatim without ever being computed or opened. This crate is the
//! missing mechanism, built so that no step is a boolean.
//!
//! What is here:
//!
//! - [`mmr`] — an append-only Merkle mountain range written from scratch, with
//!   inclusion proofs and prefix-consistency proofs whose verifiers derive the
//!   claimed position instead of trusting it.
//! - [`log`] — the admission log: a frozen domain, a frozen admission check
//!   order, per-admission acknowledgements carrying a running root, and a cutoff
//!   root that commits to the domain, the exact admitted multiset, and the exact
//!   admitted order.
//! - [`equivocation`] — four classes of contradiction a holder can be caught in,
//!   each reduced to a transferable, content-addressed verdict object.
//! - [`lifecycle`] — the typed abort machine: withholding, timeout, retry,
//!   result-binding failure, and exact refund conservation.
//! - [`hash`] — SHA-256 and a tagged hash, so the crate has no dependencies.
//! - [`transcript`] — the byte-stable corpus under `vectors/`.
//!
//! ## Claim boundary
//!
//! VERIFIED, at exactly the bounds the tests state: the commitment arithmetic,
//! the proof verifiers, the frozen admission check order, the equivocation
//! verdicts, the abort matrix, and refund conservation. Those are deterministic
//! offline measurements of this crate and nothing else.
//!
//! PROPOSED: that a cutoff root of this shape is the right thing to put in the
//! batch relation's accepted-input field.
//!
//! Deliberately absent, each named rather than approximated: no network and no
//! clock (epochs are caller-supplied integers); no signatures, so
//! [`equivocation::RootStatement`] attributes nothing and its binding is a
//! public function anyone can compute; no data-availability layer, so a share
//! count is a reported integer rather than erasure-coded bytes; no economics, so
//! a verdict object is not a slashing rule and no bond exists; no consensus, so
//! "published" means "handed to the verifier"; no encryption, so a payload
//! commitment commits to nothing this crate can open. A model of these
//! obligations is not an implementation of them.

#![deny(missing_docs)]

pub mod equivocation;
pub mod hash;
pub mod lifecycle;
pub mod log;
pub mod mmr;
pub mod transcript;

/// Identifier of the model this crate implements.
pub const MODEL: &str = "degg-inclusion-availability/v0";
