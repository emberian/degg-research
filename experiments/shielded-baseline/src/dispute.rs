//! Positive, transferable objects: outcome equivocation and omission.
//!
//! `degg_inclusion_availability::equivocation` turns "the log holder told two
//! stories about one cutoff" into a content-addressed verdict. This module
//! does the same for the two contradictions a *computation* can be caught in,
//! with the same shape and the same limitation.
//!
//! - [`OutcomeEquivocationProof`]: two receipts, one cutoff root, two
//!   incompatible claims about what that admitted set evaluates to.
//! - [`OmissionProof`]: one settled receipt that committed no local output at
//!   a position whose committed record is provably not the deterministic
//!   padding record.
//!
//! The limitation is inherited verbatim from
//! `INCLUSION_AVAILABILITY.md` section 9.1: **nothing here signs**. A
//! [`crate::roles::ExecutorId`] is a label, a receipt binding is a public
//! function anyone can compute, and every verdict below therefore establishes
//! a contradiction between published objects rather than an attribution to a
//! party. Producing that attribution is a cryptographic dependency, not a
//! modelling one, and it is the first item on the promotion path.
//!
//! Silence still has no positive object. An executor that publishes nothing
//! produces no proof of anything, and only the timeout paths of
//! [`degg_inclusion_availability::lifecycle`] respond. They cannot distinguish
//! a censoring executor from a crashed one.

use degg_inclusion_availability::hash::tagged;
use degg_inclusion_availability::log::{
    CutoffRoot, InclusionReceipt, ReceiptDefect, verify_receipt,
};
use degg_relation_ir::receipt::ReceiptStatus;

use crate::receipt::{
    CutoffBinding, DeliveryEntry, DeliveryOpening, OpeningDefect, ShieldedReceipt, verify_opening,
};
use crate::roles::ExecutorId;

/// Tag for the outcome-equivocation verdict digest.
pub const OUTCOME_VERDICT_TAG: &[u8] = b"degg/shielded-baseline/v0/outcome-equivocation";
/// Tag for the omission verdict digest.
pub const OMISSION_VERDICT_TAG: &[u8] = b"degg/shielded-baseline/v0/omission";

/// The contradiction an outcome-equivocation proof exhibits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutcomeConflict {
    /// Two different public outcomes for one admitted set.
    Outcome,
    /// One public outcome, two different receipt statuses.
    Status,
    /// One public outcome, two different per-position delivery commitments.
    ///
    /// Economically distinct from [`OutcomeConflict::Outcome`]: the tick and
    /// volume agree while the allocation does not, which is exactly the
    /// residual-rank manipulation `DARK_FBA_RELATION.md` section 11 shows is
    /// invisible in the public result.
    Delivery,
    /// One public outcome and one delivery commitment, two different assembled
    /// witnesses. Assembly is a function of the committed set and the
    /// payloads, so two of them under one cutoff root is a contradiction.
    Input,
}

impl OutcomeConflict {
    /// A stable class name for transcripts and abort typing.
    #[must_use]
    pub fn class(&self) -> &'static str {
        match self {
            Self::Outcome => "conflicting-outcome",
            Self::Status => "conflicting-status",
            Self::Delivery => "conflicting-delivery-commitment",
            Self::Input => "conflicting-assembled-input",
        }
    }

    fn code(&self) -> u8 {
        match self {
            Self::Outcome => 0,
            Self::Status => 1,
            Self::Delivery => 2,
            Self::Input => 3,
        }
    }
}

/// A claim that one executor published two incompatible runs of one cutoff.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutcomeEquivocationProof {
    /// The first published receipt.
    pub left: ShieldedReceipt,
    /// The second published receipt.
    pub right: ShieldedReceipt,
}

/// A verified outcome equivocation, reduced to what a slashing rule would
/// consume. Content-addressed, so one equivocation always names one verdict.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutcomeEquivocationVerdict {
    /// The executor held responsible.
    pub executor: ExecutorId,
    /// The cutoff both receipts are bound to.
    pub cutoff: CutoffBinding,
    /// The conflict class.
    pub class: &'static str,
    /// The first receipt's binding.
    pub left_binding: [u8; 32],
    /// The second receipt's binding.
    pub right_binding: [u8; 32],
    /// Canonical identifier of this verdict.
    pub digest: [u8; 32],
}

/// Why an outcome-equivocation proof was rejected.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutcomeEquivocationDefect {
    /// A receipt's binding does not match its own contents.
    MalformedReceipt,
    /// The two receipts name different executors.
    DifferentExecutors,
    /// The two receipts are bound to different cutoff roots, so they describe
    /// different admitted sets and contradict nothing.
    DifferentCutoffs,
    /// The two receipts run different relation modules.
    DifferentModules,
    /// The two receipts agree on everything. There is no contradiction.
    ReceiptsAgree,
}

/// Verify an outcome-equivocation proof.
///
/// Frozen conflict order, top to bottom: public outcome, receipt status,
/// delivery commitment, assembled input. The first field the two receipts
/// disagree about names the class, and later fields are not consulted. The
/// order is frozen for the same reason `DARK_FBA_RELATION.md` section 4.1
/// freezes its own: the class is an observable, so two implementations that
/// did not fix it could publish different classes for one pair of receipts.
pub fn verify_outcome_equivocation(
    proof: &OutcomeEquivocationProof,
) -> Result<OutcomeEquivocationVerdict, OutcomeEquivocationDefect> {
    let (left, right) = (&proof.left, &proof.right);
    for receipt in [left, right] {
        if !receipt.is_well_formed() {
            return Err(OutcomeEquivocationDefect::MalformedReceipt);
        }
    }
    if left.executor != right.executor {
        return Err(OutcomeEquivocationDefect::DifferentExecutors);
    }
    if left.cutoff != right.cutoff {
        return Err(OutcomeEquivocationDefect::DifferentCutoffs);
    }
    if left.module_digest != right.module_digest {
        return Err(OutcomeEquivocationDefect::DifferentModules);
    }
    let conflict = if left.outcome_digest != right.outcome_digest {
        OutcomeConflict::Outcome
    } else if left.status != right.status {
        OutcomeConflict::Status
    } else if left.delivery_root != right.delivery_root {
        OutcomeConflict::Delivery
    } else if left.input_digest != right.input_digest {
        OutcomeConflict::Input
    } else {
        return Err(OutcomeEquivocationDefect::ReceiptsAgree);
    };
    let digest = tagged(
        OUTCOME_VERDICT_TAG,
        &[
            &left.executor.0,
            &left.cutoff.root,
            &[conflict.code()],
            &left.binding,
            &right.binding,
        ],
    );
    Ok(OutcomeEquivocationVerdict {
        executor: left.executor,
        cutoff: left.cutoff,
        class: conflict.class(),
        left_binding: left.binding,
        right_binding: right.binding,
        digest,
    })
}

/// A claim that a settled run dropped one committed, non-padding position.
///
/// Every component is public or owner-held and independently checkable: the
/// inclusion receipt against the published cutoff root, the padding record
/// against the log domain, and the delivery opening against the published
/// receipt. The owner's plaintext order is *not* part of the object, so the
/// proof discloses no order content — only the owner's committed rank, which
/// `INCLUSION_AVAILABILITY.md` section 9.1 already records as the unpriced
/// privacy cost of any recourse in this family.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OmissionProof {
    /// The published cutoff root.
    pub cutoff: CutoffRoot,
    /// The published computation receipt.
    pub receipt: ShieldedReceipt,
    /// The inclusion receipt for the dropped position.
    pub inclusion: InclusionReceipt,
    /// The delivery opening at that position.
    pub opening: DeliveryOpening,
}

/// A verified omission, content-addressed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OmissionVerdict {
    /// The executor held responsible.
    pub executor: ExecutorId,
    /// The cutoff the run is bound to.
    pub cutoff: CutoffBinding,
    /// The dropped position.
    pub seq: u32,
    /// Canonical identifier of this verdict.
    pub digest: [u8; 32],
}

/// Why an omission proof was rejected.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OmissionDefect {
    /// The receipt's binding does not match its own contents.
    MalformedReceipt,
    /// The receipt is not bound to the supplied cutoff root.
    CutoffMismatch,
    /// The run did not settle, so no allocation was owed to any position.
    NotSettled,
    /// The inclusion receipt does not verify against the cutoff root.
    Inclusion(
        /// Why it failed.
        ReceiptDefect,
    ),
    /// The delivery opening does not verify against the receipt.
    Opening(
        /// Why it failed.
        OpeningDefect,
    ),
    /// The opening is at a different position from the inclusion receipt.
    PositionMismatch {
        /// Position the record claims.
        record: u32,
        /// Position the opening claims.
        opening: u32,
    },
    /// The committed record is the deterministic padding record for its own
    /// position, so committing no local output there is correct.
    PositionIsPadding {
        /// The position.
        seq: u32,
    },
    /// The run did commit a local output at that position.
    OutputWasCommitted {
        /// The position.
        seq: u32,
    },
}

/// Verify an omission proof against public objects alone.
pub fn verify_omission(proof: &OmissionProof) -> Result<OmissionVerdict, OmissionDefect> {
    if !proof.receipt.is_well_formed() {
        return Err(OmissionDefect::MalformedReceipt);
    }
    if !proof.receipt.cutoff.matches(&proof.cutoff) {
        return Err(OmissionDefect::CutoffMismatch);
    }
    if proof.receipt.status != ReceiptStatus::Settled {
        return Err(OmissionDefect::NotSettled);
    }
    verify_receipt(&proof.cutoff, &proof.inclusion).map_err(OmissionDefect::Inclusion)?;
    verify_opening(&proof.receipt, &proof.opening).map_err(OmissionDefect::Opening)?;
    if proof.inclusion.record.seq != proof.opening.seq {
        return Err(OmissionDefect::PositionMismatch {
            record: proof.inclusion.record.seq,
            opening: proof.opening.seq,
        });
    }
    let seq = proof.opening.seq;
    if proof.inclusion.record.is_padding(&proof.cutoff.domain) {
        return Err(OmissionDefect::PositionIsPadding { seq });
    }
    if !matches!(proof.opening.entry, DeliveryEntry::NoLocalOutput) {
        return Err(OmissionDefect::OutputWasCommitted { seq });
    }
    let digest = tagged(
        OMISSION_VERDICT_TAG,
        &[
            &proof.receipt.executor.0,
            &proof.receipt.cutoff.root,
            &seq.to_be_bytes(),
            &proof.receipt.binding,
        ],
    );
    Ok(OmissionVerdict {
        executor: proof.receipt.executor,
        cutoff: proof.receipt.cutoff,
        seq,
        digest,
    })
}
