//! The computation receipt, and the per-position delivery commitment.
//!
//! `DARK_FBA_RELATION.md` section 10 asks that a backend bind the accepted
//! input set, the program, the result, and every local output to one another.
//! [`degg_relation_ir::receipt::ComputationReceipt`] binds module, input, and
//! outcome digests, but its input digest is a digest of the executor's own
//! assembled witness: an owner cannot check it, because checking it needs the
//! whole batch. This module adds the two bindings that make an owner's own
//! evidence bite.
//!
//! - A [`ShieldedReceipt`] additionally binds the **cutoff root** of
//!   [`degg_inclusion_availability::log::CutoffRoot`]. An owner holding an
//!   inclusion receipt knows which root its own record is committed under, so
//!   a result computed against any other admitted set is a mismatch the owner
//!   sees immediately, and
//!   [`degg_inclusion_availability::lifecycle::AbortClass::ResultUnbound`]
//!   already types the consequence.
//! - It binds a **delivery root**: a Merkle mountain range, reusing the
//!   inclusion lane's [`degg_inclusion_availability::mmr`], over exactly one
//!   entry per committed log position. The entry at position `k` is either the
//!   canonical no-local-output entry or the digest of the [`SlotOutcome`] that
//!   position produced. An owner opens position `k`, recomputes the slot
//!   outcome from its own plaintext and the public result, and compares.
//!
//! The leaf count of the delivery commitment is the cutoff root's leaf count,
//! which padded sealing makes a constant, so the delivery root discloses no
//! occupancy the cutoff root did not already disclose. With a power-of-two
//! capacity every opening has the same shape, exactly as
//! `INCLUSION_AVAILABILITY.md` section 3.2 records for inclusion receipts, and
//! for the same reason.
//!
//! Nothing here signs. A receipt is a canonical name for a claim, not
//! authority to make it.

use degg_inclusion_availability::hash::tagged;
use degg_inclusion_availability::log::CutoffRoot;
use degg_inclusion_availability::mmr::{
    Mmr, NodePosition, NodeProof, ProofDefect, leaf_hash, verify_node_proof,
};
use degg_relation_ir::batch::Side;
use degg_relation_ir::lower::ClearedTick;
use degg_relation_ir::receipt::ReceiptStatus;

use crate::seal::PlainOrder;

/// Tag for the receipt binding.
pub const RECEIPT_TAG: &[u8] = b"degg/shielded-baseline/v0/receipt";
/// Tag for one delivery-commitment entry preimage.
pub const ENTRY_TAG: &[u8] = b"degg/shielded-baseline/v0/delivery-entry";
/// Tag for the delivery commitment's own domain digest.
pub const DELIVERY_DOMAIN_TAG: &[u8] = b"degg/shielded-baseline/v0/delivery-domain";
/// Tag for one slot outcome's digest.
pub const SLOT_OUTCOME_TAG: &[u8] = b"degg/shielded-baseline/v0/slot-outcome";

/// The part of a cutoff root a receipt binds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CutoffBinding {
    /// Digest of the admission log's frozen domain.
    pub domain_digest: [u8; 32],
    /// Number of committed positions, padded to capacity.
    pub leaf_count: u64,
    /// The published cutoff root.
    pub root: [u8; 32],
}

impl CutoffBinding {
    /// The binding of one published cutoff root.
    #[must_use]
    pub fn of(cutoff: &CutoffRoot) -> Self {
        Self {
            domain_digest: cutoff.domain.digest(),
            leaf_count: cutoff.leaf_count,
            root: cutoff.root,
        }
    }

    /// Whether this binding names the same commitment as `cutoff`.
    #[must_use]
    pub fn matches(&self, cutoff: &CutoffRoot) -> bool {
        *self == Self::of(cutoff)
    }
}

/// One committed position's local effect, as both the executor and the owner
/// of that position can compute it.
///
/// [`SlotOutcome::derive`] is a pure function of the plaintext order, the
/// public cleared tick, the public price at that tick, and the fill. That is
/// what makes the delivery entry checkable by the owner: the executor has no
/// free parameter in it except the fill itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlotOutcome {
    /// Committed log position.
    pub seq: u32,
    /// Owner the position belongs to.
    pub owner: u8,
    /// Side of the book.
    pub side: Side,
    /// Base atoms filled at this position.
    pub fill: u64,
    /// Signed base delta of this position.
    pub base_delta: i64,
    /// Signed quote delta of this position at the public price.
    pub quote_delta: i64,
    /// Base reservation returned at this position.
    pub released_base: u64,
    /// Quote reservation returned at this position.
    pub released_quote: u64,
}

impl SlotOutcome {
    /// Derive the local effect of one filled position.
    ///
    /// Returns `None` when the fill is arithmetically impossible for the
    /// order: a fill above the order's quantity, or a reservation that cannot
    /// cover the obligation the fill implies. Both are refusals of the
    /// relation's own conservation audit, so a `None` here is the executor
    /// having published something the relation would never emit.
    #[must_use]
    pub fn derive(seq: u32, plain: &PlainOrder, price: u64, fill: u64) -> Option<Self> {
        if fill > plain.quantity {
            return None;
        }
        let signed_fill = i64::try_from(fill).ok()?;
        let quote = fill.checked_mul(price)?;
        let signed_quote = i64::try_from(quote).ok()?;
        let (base_delta, quote_delta, released_base, released_quote) = match plain.side {
            Side::Buy => (
                signed_fill,
                signed_quote.checked_neg()?,
                0,
                plain.reserved.checked_sub(quote)?,
            ),
            Side::Sell => (
                signed_fill.checked_neg()?,
                signed_quote,
                plain.reserved.checked_sub(fill)?,
                0,
            ),
        };
        Some(Self {
            seq,
            owner: plain.owner,
            side: plain.side,
            fill,
            base_delta,
            quote_delta,
            released_base,
            released_quote,
        })
    }

    /// This slot outcome's canonical digest.
    #[must_use]
    pub fn digest(&self) -> [u8; 32] {
        tagged(
            SLOT_OUTCOME_TAG,
            &[
                &self.seq.to_be_bytes(),
                &[self.owner],
                &[match self.side {
                    Side::Buy => 0,
                    Side::Sell => 1,
                }],
                &self.fill.to_be_bytes(),
                &self.base_delta.to_be_bytes(),
                &self.quote_delta.to_be_bytes(),
                &self.released_base.to_be_bytes(),
                &self.released_quote.to_be_bytes(),
            ],
        )
    }
}

/// What the executor commits at one log position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeliveryEntry {
    /// The position produced no local output: it is a padding position, or the
    /// batch was refused before any allocation existed.
    ///
    /// An owner whose inclusion receipt proves a *non-padding* record at this
    /// position, against a receipt whose status is settled, therefore holds a
    /// transferable omission object: see
    /// [`crate::owner::Finding::OmittedFromComputation`].
    NoLocalOutput,
    /// The position produced this local effect.
    Produced(SlotOutcome),
}

impl DeliveryEntry {
    /// The entry's leaf preimage digest at position `seq`.
    #[must_use]
    pub fn preimage(&self, seq: u32) -> [u8; 32] {
        match self {
            Self::NoLocalOutput => tagged(ENTRY_TAG, &[&seq.to_be_bytes(), &[0], &[0u8; 32]]),
            Self::Produced(outcome) => {
                tagged(ENTRY_TAG, &[&seq.to_be_bytes(), &[1], &outcome.digest()])
            }
        }
    }

    /// The entry's leaf hash at position `seq`.
    #[must_use]
    pub fn leaf(&self, seq: u32) -> [u8; 32] {
        leaf_hash(&self.preimage(seq))
    }
}

/// The delivery commitment's own domain digest.
///
/// Bound to the cutoff root and the module digest, so a delivery opening
/// cannot be lifted from one batch or one relation into another, exactly as
/// the admission log binds its records.
#[must_use]
pub fn delivery_domain(cutoff: &CutoffBinding, module_digest: &[u8; 32]) -> [u8; 32] {
    tagged(
        DELIVERY_DOMAIN_TAG,
        &[
            &cutoff.domain_digest,
            &cutoff.leaf_count.to_be_bytes(),
            &cutoff.root,
            module_digest,
        ],
    )
}

/// The executor-side delivery commitment over every committed position.
#[derive(Clone, Debug)]
pub struct DeliveryCommitment {
    domain: [u8; 32],
    entries: Vec<DeliveryEntry>,
    mmr: Mmr,
}

impl DeliveryCommitment {
    /// Commit one entry per position, in position order.
    #[must_use]
    pub fn build(
        cutoff: &CutoffBinding,
        module_digest: &[u8; 32],
        entries: Vec<DeliveryEntry>,
    ) -> Self {
        let domain = delivery_domain(cutoff, module_digest);
        let mut mmr = Mmr::new(domain);
        for (seq, entry) in entries.iter().enumerate() {
            let seq = u32::try_from(seq).expect("capacity is bounded by u32");
            mmr.append(entry.leaf(seq));
        }
        Self {
            domain,
            entries,
            mmr,
        }
    }

    /// The delivery root a receipt binds.
    #[must_use]
    pub fn root(&self) -> [u8; 32] {
        self.mmr.root()
    }

    /// Number of committed positions.
    #[must_use]
    pub fn leaf_count(&self) -> u64 {
        self.mmr.leaf_count()
    }

    /// The delivery domain digest.
    #[must_use]
    pub fn domain(&self) -> [u8; 32] {
        self.domain
    }

    /// The entry at one position.
    #[must_use]
    pub fn entry(&self, seq: u32) -> Option<DeliveryEntry> {
        self.entries.get(usize::try_from(seq).ok()?).copied()
    }

    /// An opening of one position, for the owner that holds it.
    #[must_use]
    pub fn open(&self, seq: u32) -> Option<DeliveryOpening> {
        let entry = self.entry(seq)?;
        let proof = self.mmr.leaf_proof(u64::from(seq))?;
        Some(DeliveryOpening { seq, entry, proof })
    }
}

/// One position's entry with its membership proof under a delivery root.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeliveryOpening {
    /// The position claimed.
    pub seq: u32,
    /// The entry committed there.
    pub entry: DeliveryEntry,
    /// Its membership proof.
    pub proof: NodeProof,
}

/// Why a delivery opening failed to verify.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpeningDefect {
    /// The proof is for an interior node, not a leaf.
    NotALeaf {
        /// Height the proof claims.
        height: u32,
    },
    /// The proof places the entry at another position.
    PositionMismatch {
        /// Position the opening claims.
        claimed: u32,
        /// Position the proof determines.
        derived: u64,
    },
    /// The proof itself does not verify.
    Proof(ProofDefect),
}

/// Verify a delivery opening against a published receipt, and nothing else.
///
/// Like [`degg_inclusion_availability::log::verify_receipt`], this takes the
/// public commitment and the claim, and never consults the executor. The
/// verifier derives the position from the proof, so the executor cannot choose
/// which position an opening attests to.
pub fn verify_opening(
    receipt: &ShieldedReceipt,
    opening: &DeliveryOpening,
) -> Result<(), OpeningDefect> {
    if opening.proof.height != 0 {
        return Err(OpeningDefect::NotALeaf {
            height: opening.proof.height,
        });
    }
    let domain = delivery_domain(&receipt.cutoff, &receipt.module_digest);
    let position = verify_node_proof(
        &domain,
        &receipt.delivery_root,
        receipt.cutoff.leaf_count,
        &opening.entry.leaf(opening.seq),
        &opening.proof,
    )
    .map_err(OpeningDefect::Proof)?;
    if position
        != (NodePosition {
            height: 0,
            index: u64::from(opening.seq),
        })
    {
        return Err(OpeningDefect::PositionMismatch {
            claimed: opening.seq,
            derived: position.index,
        });
    }
    Ok(())
}

/// The computation receipt one Shielded run publishes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShieldedReceipt {
    /// The executor the receipt is attributed to. Not authenticated.
    pub executor: crate::roles::ExecutorId,
    /// Digest of the relation module: the relation identity that was run.
    pub module_digest: [u8; 32],
    /// The cutoff root the admitted set is committed under.
    pub cutoff: CutoffBinding,
    /// Digest of the assembled batch input.
    ///
    /// Executor-side evidence. An owner cannot check it without the whole
    /// witness, and a digest over a small witness domain is brute-forceable,
    /// so it names the evaluated object and proves nothing about it.
    pub input_digest: [u8; 32],
    /// Digest of the public outcome's canonical bytes.
    pub outcome_digest: [u8; 32],
    /// Root of the per-position delivery commitment.
    pub delivery_root: [u8; 32],
    /// Settled, or the public refusal class.
    pub status: ReceiptStatus,
    /// Canonical identifier of this receipt. Not a signature.
    pub binding: [u8; 32],
}

impl ShieldedReceipt {
    /// Build a well-formed receipt.
    #[must_use]
    pub fn new(
        executor: crate::roles::ExecutorId,
        module_digest: [u8; 32],
        cutoff: CutoffBinding,
        input_digest: [u8; 32],
        outcome_digest: [u8; 32],
        delivery_root: [u8; 32],
        status: ReceiptStatus,
    ) -> Self {
        let binding = Self::bind(
            &executor,
            &module_digest,
            &cutoff,
            &input_digest,
            &outcome_digest,
            &delivery_root,
            status,
        );
        Self {
            executor,
            module_digest,
            cutoff,
            input_digest,
            outcome_digest,
            delivery_root,
            status,
            binding,
        }
    }

    fn status_code(status: ReceiptStatus) -> [u8; 5] {
        let mut out = [0u8; 5];
        match status {
            ReceiptStatus::Settled => out[0] = 0,
            ReceiptStatus::Refused(class) => {
                out[0] = 1;
                out[1..].copy_from_slice(&class.code().to_be_bytes());
            }
        }
        out
    }

    fn bind(
        executor: &crate::roles::ExecutorId,
        module_digest: &[u8; 32],
        cutoff: &CutoffBinding,
        input_digest: &[u8; 32],
        outcome_digest: &[u8; 32],
        delivery_root: &[u8; 32],
        status: ReceiptStatus,
    ) -> [u8; 32] {
        tagged(
            RECEIPT_TAG,
            &[
                &executor.0,
                module_digest,
                &cutoff.domain_digest,
                &cutoff.leaf_count.to_be_bytes(),
                &cutoff.root,
                input_digest,
                outcome_digest,
                delivery_root,
                &Self::status_code(status),
            ],
        )
    }

    /// Whether the binding matches the receipt's own contents.
    #[must_use]
    pub fn is_well_formed(&self) -> bool {
        self.binding
            == Self::bind(
                &self.executor,
                &self.module_digest,
                &self.cutoff,
                &self.input_digest,
                &self.outcome_digest,
                &self.delivery_root,
                self.status,
            )
    }
}

/// Why a published receipt was refused by the public verifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReceiptRejection {
    /// The binding does not match the receipt's own contents.
    Malformed,
    /// The receipt is not bound to the cutoff root the verifier observed.
    CutoffMismatch,
    /// The receipt does not run the relation module the verifier expects.
    ModuleMismatch,
    /// The receipt's outcome digest is not the digest of the published outcome.
    OutcomeMismatch,
    /// The published outcome is not the shape its own tick and volume imply.
    OutcomeShapeInvalid,
    /// A settled receipt carries a refusal outcome, or the reverse.
    StatusMismatch,
}

/// Everything the public role can check about one published run.
///
/// The list is deliberately short, and its shortness is the finding. A public
/// verifier can establish that a receipt is well formed, names the expected
/// relation, is bound to the observed cutoff root, and matches the outcome it
/// was published with. It cannot establish that the outcome is *correct*:
/// there is no proof object anywhere in this baseline, and nothing here
/// constrains the tick, the volume, or the allocation to the admitted orders.
pub fn public_check(
    receipt: &ShieldedReceipt,
    cutoff: &CutoffRoot,
    module_digest: &[u8; 32],
    outcome: &degg_relation_ir::lower::PublicOutcome,
) -> Result<(), ReceiptRejection> {
    use degg_relation_ir::canon::Canonical;
    use degg_relation_ir::lower::PublicOutcome;

    if !receipt.is_well_formed() {
        return Err(ReceiptRejection::Malformed);
    }
    if !receipt.cutoff.matches(cutoff) {
        return Err(ReceiptRejection::CutoffMismatch);
    }
    if receipt.module_digest != *module_digest {
        return Err(ReceiptRejection::ModuleMismatch);
    }
    if receipt.outcome_digest != outcome.digest() {
        return Err(ReceiptRejection::OutcomeMismatch);
    }
    match (outcome, receipt.status) {
        (PublicOutcome::Settled(result), ReceiptStatus::Settled) => {
            if result.accepted_input_root != cutoff.root {
                return Err(ReceiptRejection::CutoffMismatch);
            }
            let consistent = match result.tick {
                ClearedTick::NoTrade => result.volume == 0,
                ClearedTick::Tick(_) => result.volume > 0,
            };
            if !consistent {
                return Err(ReceiptRejection::OutcomeShapeInvalid);
            }
        }
        (PublicOutcome::Refused(refusal), ReceiptStatus::Refused(class)) => {
            if refusal.class != class {
                return Err(ReceiptRejection::StatusMismatch);
            }
            if refusal.accepted_input_root != cutoff.root {
                return Err(ReceiptRejection::CutoffMismatch);
            }
        }
        _ => return Err(ReceiptRejection::StatusMismatch),
    }
    Ok(())
}
