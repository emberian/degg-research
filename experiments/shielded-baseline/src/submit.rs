//! The submitter role.
//!
//! A submitter turns a plaintext order into three things that leave its hands:
//! a [`crate::seal::SealedPayload`] for the executor, an
//! [`degg_inclusion_availability::log::AdmissionRequest`] carrying only
//! commitments, lengths, and an epoch, and an escrow amount for the reserve
//! ledger. It keeps the plaintext, the seal, the admission acknowledgement,
//! and later the inclusion receipt: that keepsake set is exactly the evidence
//! [`crate::owner`] audits with.
//!
//! Note which fields the admission request does *not* carry: side, limit,
//! quantity, reservation, and owner index are all inside the payload. The log
//! holder cannot price, filter, or reorder on them because it does not have
//! them, which is the property `INCLUSION_AVAILABILITY.md` section 3 calls a
//! modelling decision with teeth. In this baseline the log holder and the
//! executor are the same party and the executor *does* have the payload key,
//! so the property buys nothing here; it is preserved because splitting the
//! two roles is the first rung of the promotion path.

use degg_inclusion_availability::log::{AdmissionRequest, LogDomain};

use crate::roles::{CredentialRegistry, SealingCapability};
use crate::seal::{PlainOrder, SealedPayload};

/// Everything one submitter holds about one order.
///
/// An owner-local object. It contains the plaintext, so it never crosses the
/// public boundary; the object that does is [`Submission::sealed`], whose
/// fields are private.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Submission {
    /// The submitter's plaintext order.
    pub plain: PlainOrder,
    /// The sealed payload handed to the executor.
    pub sealed: SealedPayload,
    /// The admission request handed to the log holder.
    pub request: AdmissionRequest,
    /// The amount escrowed under this order's log nullifier.
    pub escrow: u64,
}

/// Why a submission could not be prepared.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubmitDefect {
    /// The claimed owner has no enrolled admission credential.
    OwnerNotEnrolled {
        /// The owner index claimed.
        owner: u8,
    },
    /// The order binds a batch or market other than the log domain's.
    ///
    /// The relation refuses this too, at rules 6 and 7. Refusing it here as
    /// well is a submitter-side sanity check, not a second authority: a
    /// submitter that skips this check simply gets the relation's public
    /// refusal instead.
    DomainMismatch,
}

/// Prepare one submission against one log domain.
///
/// The nonce and the 32-byte log nullifier are both derived deterministically
/// from the domain digest and the order's own nullifier, so the whole packet
/// reproduces byte for byte offline and the log's uniqueness rule and the
/// relation's rule 12 become the same rule.
///
/// The envelope's `arrival_epoch` is the epoch the holder would observe, so it
/// is clamped to the cutoff: an envelope past the cutoff is refused at
/// admission and never reaches the relation. The payload's own `arrived_at` is
/// a separate claim the sealed bytes carry, and only the relation checks it, at
/// rule 13. The two are different facts about one order and only the first is
/// committed before the cutoff.
pub fn prepare(
    capability: &SealingCapability,
    registry: &CredentialRegistry,
    domain: &LogDomain,
    plain: PlainOrder,
) -> Result<Submission, SubmitDefect> {
    if plain.batch_id != domain.batch || plain.market_id != domain.market {
        return Err(SubmitDefect::DomainMismatch);
    }
    let submitter = registry
        .commitment(plain.owner)
        .ok_or(SubmitDefect::OwnerNotEnrolled { owner: plain.owner })?;
    let domain_digest = domain.digest();
    let sealed = SealedPayload::seal(capability, plain.nonce(&domain_digest), &plain);
    let request = AdmissionRequest {
        submitter,
        payload_commitment: sealed.commitment(),
        payload_len: sealed.wire_len(),
        availability_shares: domain.availability_shares,
        arrival_epoch: plain.arrived_at.min(domain.cutoff_epoch),
        nullifier: plain.log_nullifier(&domain_digest),
    };
    Ok(Submission {
        plain,
        sealed,
        request,
        escrow: plain.reserved,
    })
}
