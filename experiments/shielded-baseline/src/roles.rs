//! The four roles, named.
//!
//! `docs/PRIVACY_MODES.md` says a Shielded claim must name who may learn, the
//! collusion threshold, whether learning is technical or merely prohibited,
//! what happens after compromise, whether past ciphertexts become readable,
//! who can censor, and how rotation works. This module answers the first
//! question with types and answers the rest, honestly, with absences:
//!
//! - **who may learn:** exactly one party, the holder of [`ExecutorKey`];
//! - **collusion threshold:** one. There is no committee, no threshold, and no
//!   second party whose cooperation is required for anything;
//! - **technical or procedural:** procedural. The seal in [`crate::seal`] is a
//!   modelled keystream, and the role separation is Rust module visibility
//!   inside one process. Neither survives contact with a real deployment;
//! - **after compromise:** everything. One key opens every payload of every
//!   batch it was used on;
//! - **past payloads:** readable. There is no forward secrecy, no rotation
//!   schedule, and no rotation mechanism;
//! - **censorship:** the executor is also the admission-log holder in this
//!   baseline, so it can refuse admission, refuse to seal, and refuse to
//!   compute. Only the timeout paths of
//!   [`degg_inclusion_availability::lifecycle`] respond, and they cannot
//!   distinguish a censoring executor from a crashed one.
//!
//! That the executor and the log holder are the same party is a *choice* of
//! this baseline, and the strongest single reason the promotion path in
//! `docs/research/SHIELDED_BASELINE.md` starts by splitting them.

use degg_inclusion_availability::equivocation::HolderId;
use degg_inclusion_availability::hash::tagged;

/// Tag for the executor's public identity.
pub const EXECUTOR_TAG: &[u8] = b"degg/shielded-baseline/v0/executor";
/// Tag for the executor's sealing secret.
pub const SECRET_TAG: &[u8] = b"degg/shielded-baseline/v0/secret";
/// Tag for one owner's admission-credential commitment.
pub const CREDENTIAL_TAG: &[u8] = b"degg/shielded-baseline/v0/credential";
/// Tag for one owner's local-output delivery key.
pub const DELIVERY_KEY_TAG: &[u8] = b"degg/shielded-baseline/v0/delivery-key";

/// The named executor's public identity.
///
/// It is a label, not a public key: nothing in this crate signs, and every
/// verdict object built against an identity therefore establishes a
/// contradiction rather than an attribution. This is the same limit
/// `INCLUSION_AVAILABILITY.md` section 9.1 records for
/// [`degg_inclusion_availability::equivocation::RootStatement`], inherited
/// deliberately rather than papered over.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExecutorId(
    /// The public identity bytes.
    pub [u8; 32],
);

impl ExecutorId {
    /// The identity of the executor named `label`.
    #[must_use]
    pub fn named(label: &str) -> Self {
        Self(tagged(EXECUTOR_TAG, &[label.as_bytes()]))
    }

    /// This executor viewed as the admission-log holder.
    ///
    /// In this baseline the two are the same party, so a root equivocation
    /// verdict from the inclusion lane and an outcome equivocation verdict
    /// from [`crate::dispute`] name the same actor.
    #[must_use]
    pub fn as_log_holder(&self) -> HolderId {
        HolderId(self.0)
    }
}

/// The executor's secret. Whoever holds this learns every private order field.
///
/// There is no public constructor from raw bytes and no accessor that returns
/// the secret, so the capability cannot be widened by accident inside this
/// crate. That is a discipline about this process, not a security mechanism:
/// a real deployment's confidentiality rests on encryption and key custody,
/// neither of which exists here.
#[derive(Clone, Debug)]
pub struct ExecutorKey {
    material: [u8; 32],
}

/// The capability a submitter needs to seal an order to the executor.
///
/// The modelled seal is **symmetric**: this capability wraps the same secret
/// [`ExecutorKey`] does. Every submitter can therefore open every other
/// submitter's payload, which a real Shielded backend fixes with public-key or
/// threshold encryption. It is named here because it changes nothing about the
/// boundary this packet actually enforces, which is between the
/// executor/owner side and the public verifier.
#[derive(Clone, Debug)]
pub struct SealingCapability {
    material: [u8; 32],
}

/// The capability one owner needs to open its own delivered local output.
#[derive(Clone, Debug)]
pub struct OwnerDeliveryKey {
    owner: u8,
    material: [u8; 32],
}

impl ExecutorKey {
    /// Commission a named executor from a seed.
    ///
    /// Deterministic: the same label and seed always produce the same identity
    /// and secret, because every experiment in this repository is
    /// reproducible offline.
    #[must_use]
    pub fn commission(label: &str, seed: &[u8; 32]) -> (ExecutorId, Self) {
        let id = ExecutorId::named(label);
        let material = tagged(SECRET_TAG, &[&id.0, seed]);
        (id, Self { material })
    }

    /// Grant a submitter the capability to seal to this executor.
    #[must_use]
    pub fn grant_sealing(&self) -> SealingCapability {
        SealingCapability {
            material: self.material,
        }
    }

    /// Grant one owner the capability to open its own delivered local output.
    #[must_use]
    pub fn grant_delivery(&self, owner: u8) -> OwnerDeliveryKey {
        OwnerDeliveryKey {
            owner,
            material: tagged(DELIVERY_KEY_TAG, &[&self.material, &[owner]]),
        }
    }

    pub(crate) fn material(&self) -> &[u8; 32] {
        &self.material
    }
}

impl SealingCapability {
    pub(crate) fn material(&self) -> &[u8; 32] {
        &self.material
    }
}

impl OwnerDeliveryKey {
    /// The owner this key belongs to.
    #[must_use]
    pub fn owner(&self) -> u8 {
        self.owner
    }

    pub(crate) fn material(&self) -> &[u8; 32] {
        &self.material
    }
}

/// The enrolment table binding one admission credential to one owner index.
///
/// The admission log commits a `submitter` field it calls "not an identity";
/// this registry is what turns that commitment into a checkable statement that
/// the plaintext's claimed owner is the party that was admitted. It is a
/// modelled enrolment: there is no issuance protocol, no revocation, no
/// unlinkability, and no credential cryptography of any kind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CredentialRegistry {
    commitments: Vec<[u8; 32]>,
}

impl CredentialRegistry {
    /// Enrol `owners` owners deterministically from a seed.
    #[must_use]
    pub fn enrol(owners: u8, seed: &[u8; 32]) -> Self {
        Self {
            commitments: (0..owners)
                .map(|owner| tagged(CREDENTIAL_TAG, &[seed, &[owner]]))
                .collect(),
        }
    }

    /// Number of enrolled owners.
    #[must_use]
    pub fn owners(&self) -> u8 {
        u8::try_from(self.commitments.len()).unwrap_or(u8::MAX)
    }

    /// The credential commitment of one owner.
    #[must_use]
    pub fn commitment(&self, owner: u8) -> Option<[u8; 32]> {
        self.commitments.get(usize::from(owner)).copied()
    }

    /// The owner an enrolled commitment belongs to.
    #[must_use]
    pub fn owner_of(&self, commitment: &[u8; 32]) -> Option<u8> {
        self.commitments
            .iter()
            .position(|candidate| candidate == commitment)
            .and_then(|index| u8::try_from(index).ok())
    }
}

/// The public verifier role: a transcript reader with no private capability.
///
/// The type is empty on purpose. Every method it has takes only objects that
/// cross the public boundary, and there is no method anywhere in this crate
/// that turns a `PublicVerifier` into an [`ExecutorKey`], an
/// [`OwnerDeliveryKey`], or a plaintext order. What the public role can check
/// is in [`crate::receipt`] and [`crate::dispute`]; what it cannot read is
/// enforced by the private fields of [`crate::seal::SealedPayload`] and
/// [`crate::seal::SealedLocalOutput`], and pinned by compile-fail doctests
/// there.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PublicVerifier;
