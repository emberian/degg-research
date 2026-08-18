//! The Shielded single-executor baseline for `dark-fba/n4-k4-q15/v0`.
//!
//! **The named executor sees everything.** It holds the sealing secret, opens
//! every admitted order payload, learns every owner's side, limit, quantity,
//! reservation, and nullifier, computes every owner-local output, and could
//! publish all of it at any moment. That is what `docs/PRIVACY_MODES.md` means
//! by Shielded, it is stated here rather than hidden, and no privacy,
//! confidentiality, noninterference, or leakage property is claimed, measured,
//! or approximated anywhere in this crate. See
//! [`SHIELDED_VISIBILITY_DISCLOSURE`].
//!
//! What this packet studies is the *other* half: given that the executor is
//! trusted with confidentiality by definition, exactly how much of
//! **correctness and inclusion** stops being a matter of trust, and exactly how
//! much does not. Every mechanism below exists to move one obligation from
//! "the executor asserts it" to "an object a third party or an owner can
//! check", and the residue is named rather than rounded off.
//!
//! ## What is composed
//!
//! This crate implements nothing that the two landed packets already
//! implement. It is a *path dependency* on both:
//!
//! - [`degg_inclusion_availability`] supplies the append-only admission log,
//!   the Merkle mountain range, padded cutoff sealing, inclusion receipts,
//!   root equivocation verdicts, the typed abort machine, and the reserve
//!   ledger. This crate reuses its hash, its `Mmr`, its `AdmissionLog`, its
//!   `BatchMachine`, and its `ReserveLedger` directly; it reimplements none of
//!   them.
//! - [`degg_relation_ir`] supplies the relation as data
//!   (`dark_fba_n4_k4_q15_v0`), its canonical byte encoding, the frozen
//!   admission-check order of `DARK_FBA_RELATION.md` section 4.1, and the
//!   Clear evaluator that interprets the module. This crate calls
//!   `lower(module, LoweringTarget::Clear)` and evaluates through it; it
//!   reimplements no clearing, allocation, or admission logic.
//!
//! The Shielded lowering in [`degg_relation_ir::lower`] still refuses, and
//! that refusal is correct and untouched: this crate does not add a lowering
//! target. A Shielded run here is the module's Clear evaluator executed by one
//! named process, exactly as `LoweringRefusal::ShieldedBackendAbsent` says, and
//! the mode is requested per batch through
//! [`degg_relation_ir::batch::RequestedMode::ShieldedSingleExecutor`].
//!
//! ## What this crate adds
//!
//! - [`roles`] — the four roles, named: submitters, one named executor (who is
//!   also the admission-log holder in this baseline), the public verifier, and
//!   owners receiving local outputs.
//! - [`seal`] — the modelled seal. A [`seal::SealedPayload`] has private
//!   fields and is opened only with an [`roles::ExecutorKey`], so a
//!   public-role function that tries to read an order field is a compile
//!   error, not a review comment.
//! - [`submit`] — submitters turn a plaintext order into a sealed payload, an
//!   admission request carrying only commitments, and an escrow entry.
//! - [`executor`] — the executor assembles the batch *from the committed log
//!   order*, derives the four boundary statements and three of the four
//!   per-slot statements from real objects instead of asserting booleans, runs
//!   the Clear evaluator, and publishes a receipt and a per-position delivery
//!   commitment.
//! - [`receipt`] — the computation receipt binding module digest, cutoff root,
//!   outcome digest, and delivery root; and the per-position delivery entries.
//! - [`owner`] — the owner-side check battery: what an owner holding an
//!   inclusion receipt can actually detect, stated as typed findings.
//! - [`dispute`] — outcome equivocation: two receipts, one cutoff, one
//!   content-addressed verdict.
//! - [`scenario`] — the end-to-end honest run, used by the tests and by the
//!   corpus binary.
//! - [`differential`] — the Shielded run against the Clear lowering over
//!   enumerated book domains, with an explicit statement of what sharing an
//!   evaluator does and does not make that comparison worth.
//!
//! ## Claim boundary
//!
//! VERIFIED, at exactly the bounds the tests state: the assembly path, the
//! derived boundary and per-slot statements, the receipt bindings, the
//! delivery commitment and its openings, the owner check battery, the omission
//! and substitution detections, the outcome-equivocation verdict, refund
//! conservation on the reused abort paths, and the bounded differential
//! against the Clear lowering.
//!
//! PROPOSED: that this is the right shape for a Shielded baseline, and that
//! the promotion path in `docs/research/SHIELDED_BASELINE.md` leads anywhere.
//!
//! REJECTED as claims, here and everywhere in this crate: that a modelled seal
//! is encryption, that a role type is an access-control mechanism outside this
//! process, that an owner check battery is soundness, that a detection is a
//! prevention, or that anything here is Dark.
//!
//! Deliberately absent, each named rather than approximated: no network, no
//! clock, no signatures and therefore no attribution for any verdict, no
//! consensus, no data-availability layer, no cipher with a security argument,
//! no proof system, no settlement, and no economics.

#![deny(missing_docs)]

pub mod differential;
pub mod dispute;
pub mod executor;
pub mod owner;
pub mod receipt;
pub mod roles;
pub mod scenario;
pub mod seal;
pub mod submit;
pub mod transcript;

/// Identifier of the model this crate implements.
pub const MODEL: &str = "degg-shielded-baseline/v0";

/// The declared visibility of the Shielded single-executor baseline.
///
/// The counterpart of [`degg_relation_ir::lower::CLEAR_VISIBILITY_DISCLOSURE`].
/// Under Clear, anyone may inspect the executing process. Under Shielded, one
/// *named* party may, and that party learns exactly as much as the Clear
/// process does. The difference between the two modes is who is allowed to
/// look, not what is visible to whoever looks.
pub const SHIELDED_VISIBILITY_DISCLOSURE: &str = "shielded single executor: one named executor learns every private order field of every \
     admitted order and every owner-local output; confidentiality against that executor is \
     assumed, never enforced, and never proved";
