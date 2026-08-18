//! Equivocation: one holder, one cutoff, two incompatible stories.
//!
//! A cutoff root is only useful if there is exactly one of them per cutoff. A
//! holder that shows root `A` to one submitter and root `B` to another can
//! honour every inclusion receipt it ever issued while running two different
//! markets. This module makes that failure produce a **positive, transferable
//! object** rather than an accusation.
//!
//! Attribution is the part this model does not provide. A real system binds a
//! root to a holder with a signature, a consensus record, or a bonded
//! commitment. Here, [`RootStatement::binding`] is a *public function of its own
//! contents*: anyone can compute it, so it identifies a statement canonically
//! and provides no unforgeability whatsoever. Every result in this module is
//! therefore of the form "given two statements genuinely attributed to the same
//! holder, here is a checkable contradiction". Producing that attribution is
//! named as out of scope in the accompanying document.
//!
//! Three conflict classes are modelled, and they are genuinely different
//! failures:
//!
//! - [`Conflict::Roots`] — two sealed roots for one cutoff. The bare case.
//! - [`Conflict::Sequence`] — two receipts placing different records at the same
//!   canonical position. This is the economically loaded one: the position *is*
//!   the residual-allocation rank.
//! - [`Conflict::Position`] — one nullifier admitted at two different ranks.
//! - [`Conflict::Prefix`] — a pre-cutoff acknowledgement that the sealed log does
//!   not extend. This is a rollback: the holder took a record, acknowledged it,
//!   and then published a cutoff root built on a different history.
//!
//! What has no positive object here is *silence*. A holder that simply never
//! answers produces no proof of anything, and is handled by the timeout paths in
//! [`crate::lifecycle`] instead. That asymmetry is a real limit, not an
//! oversight.

use crate::hash::tagged;
use crate::log::{CutoffRoot, DomainDefect, InclusionReceipt, LogDomain, ReceiptDefect};
use crate::mmr::{ConsistencyProof, ProofDefect, verify_consistency};

/// Tag for the canonical statement binding.
pub const STATEMENT_TAG: &[u8] = b"degg/inclusion-availability/v0/statement";
/// Tag for the equivocation verdict digest.
pub const VERDICT_TAG: &[u8] = b"degg/inclusion-availability/v0/equivocation";

/// A holder identity. In this model an opaque label, not a public key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HolderId(pub [u8; 32]);

/// Which kind of root a statement is about.
///
/// The distinction matters: two *running* roots at different leaf counts are
/// ordinary honest behaviour, while two *sealed* roots for one cutoff are not.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatementKind {
    /// A pre-cutoff running root, as acknowledged to a submitter.
    Ack,
    /// The published cutoff root.
    Sealed,
}

impl StatementKind {
    fn code(self) -> u8 {
        match self {
            StatementKind::Ack => 0,
            StatementKind::Sealed => 1,
        }
    }
}

/// A holder's statement that a given root holds for a given log at a given size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RootStatement {
    /// The holder the statement is attributed to.
    pub holder: HolderId,
    /// Digest of the log domain the root belongs to.
    pub domain_digest: [u8; 32],
    /// Whether this is a running acknowledgement or the cutoff root.
    pub kind: StatementKind,
    /// Number of leaves the root is taken at.
    pub leaf_count: u64,
    /// The root itself.
    pub root: [u8; 32],
    /// Canonical identifier of this statement. Not a signature; see module docs.
    pub binding: [u8; 32],
}

impl RootStatement {
    /// Build a well-formed statement.
    #[must_use]
    pub fn new(
        holder: HolderId,
        domain_digest: [u8; 32],
        kind: StatementKind,
        leaf_count: u64,
        root: [u8; 32],
    ) -> Self {
        Self {
            holder,
            domain_digest,
            kind,
            leaf_count,
            root,
            binding: Self::bind(holder, &domain_digest, kind, leaf_count, &root),
        }
    }

    /// The statement a holder makes when it publishes a cutoff root.
    #[must_use]
    pub fn seal(holder: HolderId, cutoff: &CutoffRoot) -> Self {
        Self::new(
            holder,
            cutoff.domain.digest(),
            StatementKind::Sealed,
            cutoff.leaf_count,
            cutoff.root,
        )
    }

    /// The statement a holder makes when it acknowledges an admission.
    #[must_use]
    pub fn ack(
        holder: HolderId,
        domain: &LogDomain,
        running_leaf_count: u64,
        running_root: [u8; 32],
    ) -> Self {
        Self::new(
            holder,
            domain.digest(),
            StatementKind::Ack,
            running_leaf_count,
            running_root,
        )
    }

    fn bind(
        holder: HolderId,
        domain_digest: &[u8; 32],
        kind: StatementKind,
        leaf_count: u64,
        root: &[u8; 32],
    ) -> [u8; 32] {
        tagged(
            STATEMENT_TAG,
            &[
                &holder.0,
                domain_digest,
                &[kind.code()],
                &leaf_count.to_be_bytes(),
                root,
            ],
        )
    }

    /// Whether the binding matches the statement's own contents.
    #[must_use]
    pub fn is_well_formed(&self) -> bool {
        self.binding
            == Self::bind(
                self.holder,
                &self.domain_digest,
                self.kind,
                self.leaf_count,
                &self.root,
            )
    }
}

/// The contradiction an equivocation proof exhibits.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Conflict {
    /// Two sealed roots for the same cutoff, and nothing more.
    Roots,
    /// Two different records at the same canonical position.
    Sequence {
        /// Receipt under the left statement's root.
        left: InclusionReceipt,
        /// Receipt under the right statement's root.
        right: InclusionReceipt,
    },
    /// One nullifier admitted at two different canonical positions.
    Position {
        /// Receipt under the left statement's root.
        left: InclusionReceipt,
        /// Receipt under the right statement's root.
        right: InclusionReceipt,
    },
    /// The sealed log does not extend an acknowledged prefix.
    Prefix {
        /// Proof that the sealed log's prefix at the acknowledged size is a
        /// different root.
        consistency: ConsistencyProof,
    },
}

impl Conflict {
    /// A stable class name for transcripts and abort typing.
    #[must_use]
    pub fn class(&self) -> &'static str {
        match self {
            Conflict::Roots => "conflicting-sealed-roots",
            Conflict::Sequence { .. } => "conflicting-record-at-sequence",
            Conflict::Position { .. } => "nullifier-at-two-positions",
            Conflict::Prefix { .. } => "acknowledged-prefix-abandoned",
        }
    }

    fn code(&self) -> u8 {
        match self {
            Conflict::Roots => 0,
            Conflict::Sequence { .. } => 1,
            Conflict::Position { .. } => 2,
            Conflict::Prefix { .. } => 3,
        }
    }
}

/// A transferable claim that one holder equivocated about one cutoff.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EquivocationProof {
    /// The log domain both statements are about.
    pub domain: LogDomain,
    /// The first attributed statement.
    pub left: RootStatement,
    /// The second attributed statement.
    pub right: RootStatement,
    /// The checkable contradiction between them.
    pub conflict: Conflict,
}

/// A verified equivocation, reduced to what a slashing rule would consume.
///
/// It is content-addressed: [`EquivocationVerdict::digest`] is a function of the
/// holder, the domain, the conflict class, and both statement bindings, so the
/// same equivocation always names the same verdict and two different
/// equivocations never collide by construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EquivocationVerdict {
    /// The holder held responsible.
    pub holder: HolderId,
    /// Digest of the log domain.
    pub domain_digest: [u8; 32],
    /// Conflict class name.
    pub class: &'static str,
    /// The first root the holder is bound to.
    pub left_root: [u8; 32],
    /// The second root the holder is bound to.
    pub right_root: [u8; 32],
    /// Canonical identifier of this verdict.
    pub digest: [u8; 32],
}

/// Why an equivocation proof was rejected.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EquivocationDefect {
    /// The log domain is invalid.
    Domain(DomainDefect),
    /// A statement is not about this domain.
    DomainMismatch,
    /// A statement's binding does not match its contents.
    MalformedStatement,
    /// The two statements name different holders.
    DifferentHolders,
    /// The two roots are the same, so nothing is contradicted.
    RootsIdentical,
    /// A statement that must be a sealed cutoff root is not one.
    NotSealed,
    /// A statement that must be a pre-cutoff acknowledgement is not one.
    NotAck,
    /// A receipt did not verify against its own statement's root.
    Receipt(ReceiptDefect),
    /// The two receipts do not actually sit at the same position.
    NotSameSequence {
        /// Left receipt's sequence.
        left: u32,
        /// Right receipt's sequence.
        right: u32,
    },
    /// The two receipts carry the same record, so there is no conflict.
    RecordsIdentical,
    /// The two receipts do not carry the same nullifier.
    NotSameNullifier,
    /// The two receipts place the nullifier at the same position.
    SamePosition {
        /// The shared sequence.
        seq: u32,
    },
    /// The consistency proof does not verify against the sealed root.
    Consistency(ProofDefect),
    /// The consistency proof is not taken at the acknowledged size.
    PrefixSizeMismatch {
        /// Size the acknowledgement claims.
        acknowledged: u64,
        /// Size the consistency proof is taken at.
        proved: u64,
    },
    /// The sealed log does extend the acknowledged prefix, so there is no conflict.
    PrefixAgrees,
}

/// Verify an equivocation proof.
///
/// Every check is structural: two attributed statements, a genuine
/// disagreement, and a contradiction that a third party can recompute from the
/// bytes in the object.
pub fn verify_equivocation(
    proof: &EquivocationProof,
) -> Result<EquivocationVerdict, EquivocationDefect> {
    proof
        .domain
        .validate()
        .map_err(EquivocationDefect::Domain)?;
    let domain_digest = proof.domain.digest();
    for statement in [&proof.left, &proof.right] {
        if !statement.is_well_formed() {
            return Err(EquivocationDefect::MalformedStatement);
        }
        if statement.domain_digest != domain_digest {
            return Err(EquivocationDefect::DomainMismatch);
        }
    }
    if proof.left.holder != proof.right.holder {
        return Err(EquivocationDefect::DifferentHolders);
    }
    if proof.left.root == proof.right.root {
        return Err(EquivocationDefect::RootsIdentical);
    }

    match &proof.conflict {
        Conflict::Roots => {
            require_sealed(&proof.left)?;
            require_sealed(&proof.right)?;
        }
        Conflict::Sequence { left, right } => {
            let (left_cutoff, right_cutoff) = sealed_pair(proof)?;
            crate::log::verify_receipt(&left_cutoff, left).map_err(EquivocationDefect::Receipt)?;
            crate::log::verify_receipt(&right_cutoff, right)
                .map_err(EquivocationDefect::Receipt)?;
            if left.record.seq != right.record.seq {
                return Err(EquivocationDefect::NotSameSequence {
                    left: left.record.seq,
                    right: right.record.seq,
                });
            }
            if left.record == right.record {
                return Err(EquivocationDefect::RecordsIdentical);
            }
        }
        Conflict::Position { left, right } => {
            let (left_cutoff, right_cutoff) = sealed_pair(proof)?;
            crate::log::verify_receipt(&left_cutoff, left).map_err(EquivocationDefect::Receipt)?;
            crate::log::verify_receipt(&right_cutoff, right)
                .map_err(EquivocationDefect::Receipt)?;
            if left.record.nullifier != right.record.nullifier {
                return Err(EquivocationDefect::NotSameNullifier);
            }
            if left.record.seq == right.record.seq {
                return Err(EquivocationDefect::SamePosition {
                    seq: left.record.seq,
                });
            }
        }
        Conflict::Prefix { consistency } => {
            if proof.left.kind != StatementKind::Ack {
                return Err(EquivocationDefect::NotAck);
            }
            require_sealed(&proof.right)?;
            if consistency.prefix_leaf_count != proof.left.leaf_count {
                return Err(EquivocationDefect::PrefixSizeMismatch {
                    acknowledged: proof.left.leaf_count,
                    proved: consistency.prefix_leaf_count,
                });
            }
            let derived = verify_consistency(
                &domain_digest,
                &proof.right.root,
                proof.right.leaf_count,
                consistency,
            )
            .map_err(EquivocationDefect::Consistency)?;
            if derived == proof.left.root {
                return Err(EquivocationDefect::PrefixAgrees);
            }
        }
    }

    let class = proof.conflict.class();
    let digest = tagged(
        VERDICT_TAG,
        &[
            &proof.left.holder.0,
            &domain_digest,
            &[proof.conflict.code()],
            &proof.left.binding,
            &proof.right.binding,
        ],
    );
    Ok(EquivocationVerdict {
        holder: proof.left.holder,
        domain_digest,
        class,
        left_root: proof.left.root,
        right_root: proof.right.root,
        digest,
    })
}

fn require_sealed(statement: &RootStatement) -> Result<(), EquivocationDefect> {
    if statement.kind == StatementKind::Sealed {
        Ok(())
    } else {
        Err(EquivocationDefect::NotSealed)
    }
}

fn sealed_pair(proof: &EquivocationProof) -> Result<(CutoffRoot, CutoffRoot), EquivocationDefect> {
    require_sealed(&proof.left)?;
    require_sealed(&proof.right)?;
    Ok((
        CutoffRoot {
            domain: proof.domain,
            leaf_count: proof.left.leaf_count,
            root: proof.left.root,
        },
        CutoffRoot {
            domain: proof.domain,
            leaf_count: proof.right.leaf_count,
            root: proof.right.root,
        },
    ))
}
