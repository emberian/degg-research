//! The modelled seal, and the fixed 64-byte order wire shape.
//!
//! Two different things are called sealing here, and they are different on
//! purpose.
//!
//! - A [`SealedPayload`] carries an order across the public boundary. Its
//!   bytes are the plaintext exclusive-ored with a SHA-256 tagged counter
//!   keystream, so the object a public verifier can hold genuinely does not
//!   contain the order fields. **This is a model of encryption, not a cipher.**
//!   It has no security argument, no reviewed construction, no authentication,
//!   and a repeated nonce under one key would be a two-time pad. It is here so
//!   that "the public cannot read the order" is a property of the data as well
//!   as of Rust's module system.
//! - A [`SealedLocalOutput`] carries one owner's local output back. It holds
//!   the structured value behind a private field and admits an
//!   [`OwnerDeliveryKey`] as the only opener, because decoding
//!   `degg-cbe/v1` bytes back into a value is a parser this packet has no
//!   reason to write. It is access control inside one process, and nothing
//!   more.
//!
//! Neither is confidentiality. The named executor holds the secret both are
//! derived from and reads every one of them; see
//! [`crate::SHIELDED_VISIBILITY_DISCLOSURE`].
//!
//! ## The compile-time half
//!
//! Both sealed types have private fields and no accessor that yields the
//! plaintext without a capability. A public-role function that reaches for an
//! order field does not compile, and the doctests on [`SealedPayload`] pin
//! that with `compile_fail` rather than leaving it to review.

use degg_inclusion_availability::hash::tagged;
use degg_relation_ir::batch::Side;
use degg_relation_ir::canon::Canonical;
use degg_relation_ir::lower::OwnerOutput;

use crate::roles::{ExecutorKey, OwnerDeliveryKey, SealingCapability};

/// Tag for the keystream blocks of the modelled payload seal.
pub const SEAL_TAG: &[u8] = b"degg/shielded-baseline/v0/seal";
/// Tag for the payload commitment the admission log records.
pub const PAYLOAD_COMMIT_TAG: &[u8] = b"degg/shielded-baseline/v0/payload-commitment";
/// Tag for the deterministic seal nonce.
pub const NONCE_TAG: &[u8] = b"degg/shielded-baseline/v0/nonce";
/// Tag for the batch-scoped log nullifier derived from an order's nullifier.
pub const LOG_NULLIFIER_TAG: &[u8] = b"degg/shielded-baseline/v0/log-nullifier";
/// Tag binding one delivered local output to one owner's delivery key.
pub const DELIVERY_BIND_TAG: &[u8] = b"degg/shielded-baseline/v0/delivery-bind";

/// The single admissible order wire length, in bytes.
///
/// Equal to [`degg_inclusion_availability::log::DARK_FBA_PAYLOAD_LEN`], which
/// the admission log enforces, which is what makes size a non-channel in the
/// disclosure budget of `DARK_RELATION_THREAT_MODEL.md`.
pub const WIRE_LEN: usize = 64;

/// Bytes of the canonical plaintext order before zero padding.
pub const PLAIN_BYTES: usize = 51;

/// One submitter's plaintext order.
///
/// The fields are exactly the private-to-owner ports of the relation module,
/// with the four statement ports left out: those are not submitter-chosen, and
/// [`crate::executor`] derives three of them from objects instead.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlainOrder {
    /// Batch identifier the order binds.
    pub batch_id: u64,
    /// Market identifier the order binds.
    pub market_id: u64,
    /// Owner index.
    pub owner: u8,
    /// Side of the book.
    pub side: Side,
    /// Limit tick index.
    pub limit_tick: u8,
    /// Quantity in base atoms.
    pub quantity: u64,
    /// Reserved amount in the spending asset.
    pub reserved: u64,
    /// Batch-scoped nullifier.
    pub nullifier: u64,
    /// Arrival time in the batch's external time domain.
    pub arrived_at: u64,
}

/// Why a sealed object could not be turned back into a value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SealDefect {
    /// The recovered bytes are not a canonical plaintext order.
    ///
    /// A wrong key produces this almost always and is not distinguished from
    /// it: the modelled seal is unauthenticated, so there is nothing to check
    /// a key against. The executor's assembly compares the payload commitment
    /// instead, which is the binding that matters.
    MalformedPlaintext,
    /// The delivery key does not open this local output.
    NotAuthorized,
}

impl PlainOrder {
    /// The canonical fixed-width plaintext encoding, zero padded to
    /// [`WIRE_LEN`].
    #[must_use]
    pub fn encode(&self) -> [u8; WIRE_LEN] {
        let mut out = [0u8; WIRE_LEN];
        let mut at = 0usize;
        let mut put = |bytes: &[u8]| {
            out[at..at + bytes.len()].copy_from_slice(bytes);
            at += bytes.len();
        };
        put(&self.batch_id.to_be_bytes());
        put(&self.market_id.to_be_bytes());
        put(&[self.owner]);
        put(&[match self.side {
            Side::Buy => 0,
            Side::Sell => 1,
        }]);
        put(&[self.limit_tick]);
        put(&self.quantity.to_be_bytes());
        put(&self.reserved.to_be_bytes());
        put(&self.nullifier.to_be_bytes());
        put(&self.arrived_at.to_be_bytes());
        debug_assert_eq!(at, PLAIN_BYTES);
        out
    }

    /// Decode a canonical plaintext order.
    ///
    /// The padding must be exactly zero: a plaintext with anything else in it
    /// is not canonical, so the encoding stays injective and the wire shape
    /// carries no free bytes an executor could use as a side channel.
    pub fn decode(bytes: &[u8; WIRE_LEN]) -> Result<Self, SealDefect> {
        if bytes[PLAIN_BYTES..].iter().any(|byte| *byte != 0) {
            return Err(SealDefect::MalformedPlaintext);
        }
        let u64_at = |at: usize| {
            let mut word = [0u8; 8];
            word.copy_from_slice(&bytes[at..at + 8]);
            u64::from_be_bytes(word)
        };
        let side = match bytes[17] {
            0 => Side::Buy,
            1 => Side::Sell,
            _ => return Err(SealDefect::MalformedPlaintext),
        };
        Ok(Self {
            batch_id: u64_at(0),
            market_id: u64_at(8),
            owner: bytes[16],
            side,
            limit_tick: bytes[18],
            quantity: u64_at(19),
            reserved: u64_at(27),
            nullifier: u64_at(35),
            arrived_at: u64_at(43),
        })
    }

    /// The batch-scoped 32-byte nullifier this order occupies in the log.
    ///
    /// Derived, so that the log's uniqueness rule and the relation's rule 12
    /// are the same rule. The derivation is a public function of a
    /// caller-chosen `u64`; over a low-entropy nullifier choice it is
    /// trivially brute-forceable, exactly as `degg_relation_ir`'s receipts
    /// note about digests over low-entropy private data.
    #[must_use]
    pub fn log_nullifier(&self, domain_digest: &[u8; 32]) -> [u8; 32] {
        tagged(
            LOG_NULLIFIER_TAG,
            &[domain_digest, &self.nullifier.to_be_bytes()],
        )
    }

    /// The deterministic seal nonce for this order under one log domain.
    ///
    /// Nullifiers are unique within a batch and the domain digest differs
    /// between batches, so no nonce is reused under one secret. That is an
    /// argument about this model's own construction, not a cryptographic one.
    #[must_use]
    pub fn nonce(&self, domain_digest: &[u8; 32]) -> [u8; 32] {
        tagged(NONCE_TAG, &[domain_digest, &self.nullifier.to_be_bytes()])
    }
}

fn keystream(material: &[u8; 32], nonce: &[u8; 32], block: u8) -> [u8; 32] {
    tagged(SEAL_TAG, &[material, nonce, &[block]])
}

fn apply_keystream(material: &[u8; 32], nonce: &[u8; 32], bytes: &mut [u8; WIRE_LEN]) {
    for (block, chunk) in bytes.chunks_mut(32).enumerate() {
        let pad = keystream(material, nonce, u8::try_from(block).expect("two blocks"));
        for (byte, mask) in chunk.iter_mut().zip(pad) {
            *byte ^= mask;
        }
    }
}

/// One sealed order payload, as it crosses the public boundary.
///
/// The plaintext is not in this object; the fields are private; and there is
/// no accessor that yields an order field without an [`ExecutorKey`].
///
/// A public-role function cannot read the bytes:
///
/// ```compile_fail,E0616
/// fn public_audit(sealed: &degg_shielded_baseline::seal::SealedPayload) -> u8 {
///     sealed.ciphertext[0]
/// }
/// ```
///
/// and cannot open the payload without the executor's key:
///
/// ```compile_fail,E0061
/// fn public_audit(sealed: &degg_shielded_baseline::seal::SealedPayload) {
///     let _ = sealed.open();
/// }
/// ```
///
/// The executor, holding the key, reads everything:
///
/// ```
/// use degg_relation_ir::batch::Side;
/// use degg_shielded_baseline::roles::ExecutorKey;
/// use degg_shielded_baseline::seal::{PlainOrder, SealedPayload};
///
/// let (_, key) = ExecutorKey::commission("named-executor", &[9u8; 32]);
/// let order = PlainOrder {
///     batch_id: 7,
///     market_id: 9,
///     owner: 0,
///     side: Side::Buy,
///     limit_tick: 2,
///     quantity: 5,
///     reserved: 15,
///     nullifier: 101,
///     arrived_at: 10,
/// };
/// let sealed = SealedPayload::seal(&key.grant_sealing(), [3u8; 32], &order);
/// assert_eq!(sealed.open(&key).expect("opens"), order);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SealedPayload {
    nonce: [u8; 32],
    ciphertext: [u8; WIRE_LEN],
}

impl SealedPayload {
    /// Seal one plaintext order to the executor.
    #[must_use]
    pub fn seal(capability: &SealingCapability, nonce: [u8; 32], plain: &PlainOrder) -> Self {
        let mut ciphertext = plain.encode();
        apply_keystream(capability.material(), &nonce, &mut ciphertext);
        Self { nonce, ciphertext }
    }

    /// The commitment the admission log records for this payload.
    #[must_use]
    pub fn commitment(&self) -> [u8; 32] {
        tagged(PAYLOAD_COMMIT_TAG, &[&self.nonce, &self.ciphertext])
    }

    /// The public wire form: the nonce and the sealed bytes.
    ///
    /// This is what a public verifier may legitimately hold. The plaintext is
    /// not recoverable from it without the executor's secret, which is a
    /// property of this model's construction and not a cryptographic claim.
    #[must_use]
    pub fn wire(&self) -> ([u8; 32], [u8; WIRE_LEN]) {
        (self.nonce, self.ciphertext)
    }

    /// The fixed wire length the admission log requires.
    #[must_use]
    pub fn wire_len(&self) -> u32 {
        u32::try_from(WIRE_LEN).expect("wire length is bounded")
    }

    /// Open the payload. Only the executor can call this.
    pub fn open(&self, key: &ExecutorKey) -> Result<PlainOrder, SealDefect> {
        let mut plaintext = self.ciphertext;
        apply_keystream(key.material(), &self.nonce, &mut plaintext);
        PlainOrder::decode(&plaintext)
    }

    /// Replace the sealed bytes with another payload's, keeping this nonce.
    ///
    /// A deliberate adversary handle for the substitution tests, not an
    /// operation the honest interface offers.
    #[must_use]
    pub fn adversarially_replace(&self, other: &Self) -> Self {
        Self {
            nonce: self.nonce,
            ciphertext: other.ciphertext,
        }
    }
}

/// One owner's local output, sealed for delivery to that owner.
///
/// The output is behind a private field and only an [`OwnerDeliveryKey`] bound
/// to it opens it. A public-role function cannot read the output:
///
/// ```compile_fail,E0616
/// fn public_audit(sealed: &degg_shielded_baseline::seal::SealedLocalOutput) -> u8 {
///     sealed.output.owner
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SealedLocalOutput {
    owner: u8,
    bound: [u8; 32],
    output: OwnerOutput,
}

impl SealedLocalOutput {
    /// Seal one owner's local output for delivery.
    #[must_use]
    pub fn seal(key: &OwnerDeliveryKey, output: OwnerOutput) -> Self {
        let bound = tagged(DELIVERY_BIND_TAG, &[key.material(), &output.digest()]);
        Self {
            owner: key.owner(),
            bound,
            output,
        }
    }

    /// The owner this delivery is addressed to.
    #[must_use]
    pub fn owner(&self) -> u8 {
        self.owner
    }

    /// Digest of the sealed local output's canonical bytes.
    ///
    /// Public, because the delivery receipt binds it. A digest over a small
    /// domain of owner outputs is brute-forceable and hides nothing; it names
    /// the object, and that is all it is for.
    #[must_use]
    pub fn output_digest(&self) -> [u8; 32] {
        self.output.digest()
    }

    /// Open the delivered local output.
    pub fn open(&self, key: &OwnerDeliveryKey) -> Result<OwnerOutput, SealDefect> {
        if key.owner() != self.owner {
            return Err(SealDefect::NotAuthorized);
        }
        let expected = tagged(DELIVERY_BIND_TAG, &[key.material(), &self.output.digest()]);
        if expected != self.bound {
            return Err(SealDefect::NotAuthorized);
        }
        Ok(self.output.clone())
    }
}
