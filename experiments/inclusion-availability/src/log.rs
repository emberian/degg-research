//! The append-only admission log: what is admitted, in what order, and what a
//! cutoff commits to.
//!
//! An admission record is an *envelope*, not an order. The log holder in the
//! Dark target must be able to sequence and commit inputs without reading them,
//! so the record carries a payload commitment, a fixed payload length, a
//! declared availability share count, an arrival epoch, and a batch-scoped
//! nullifier. Side, limit, quantity, and reservation are inside the payload and
//! never appear here. That is a modelling decision with teeth: the log cannot
//! price, filter, or reorder on order content, because it does not have it.
//!
//! The frozen check order in [`AdmissionLog::admit`] is part of the model.
//! `DARK_FBA_RELATION.md` §13.3 records that two conforming implementations of
//! the clearing relation publicly disagreed about which refusal class a
//! multiply-invalid witness reports, because no check order was fixed. This
//! module fixes one and pins it with a test rather than inheriting the same
//! gap.

use std::collections::BTreeMap;

use crate::hash::tagged;
use crate::mmr::{
    ConsistencyProof, Mmr, NodePosition, NodeProof, ProofDefect, leaf_hash, verify_node_proof,
};

/// Tag for the log domain digest.
pub const DOMAIN_TAG: &[u8] = b"degg/inclusion-availability/v0/domain";
/// Tag for the canonical record preimage.
pub const RECORD_TAG: &[u8] = b"degg/inclusion-availability/v0/record";
/// Tag for the deterministic padding record.
pub const PADDING_TAG: &[u8] = b"degg/inclusion-availability/v0/padding";

/// Relation identifier of the batch relation this log is modelled against.
pub const DARK_FBA_RELATION: &str = "dark-fba/n4-k4-q15/v0";
/// Padded slot capacity of that relation.
pub const DARK_FBA_CAPACITY: u32 = 4;
/// Fixed public wire shape, in bytes, from the disclosure budget.
pub const DARK_FBA_PAYLOAD_LEN: u32 = 64;
/// Declared availability shares per admitted payload.
pub const DARK_FBA_AVAILABILITY_SHARES: u8 = 4;
/// Shares required to reconstruct one admitted payload.
pub const DARK_FBA_AVAILABILITY_THRESHOLD: u8 = 3;

/// The frozen identity and shape of one admission log.
///
/// Every field is bound into the domain digest and therefore into every root
/// the log ever publishes. Changing any field produces a different commitment
/// space, not a configuration of the same one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LogDomain {
    /// Relation identifier and version.
    pub relation: &'static str,
    /// Batch identifier.
    pub batch: u64,
    /// Market identifier.
    pub market: u64,
    /// Cutoff, in a named external epoch domain.
    pub cutoff_epoch: u64,
    /// Maximum number of records the log admits.
    pub capacity: u32,
    /// The single admissible payload length.
    pub payload_len: u32,
    /// Availability shares each payload must declare.
    pub availability_shares: u8,
    /// Shares required to reconstruct a payload.
    pub availability_threshold: u8,
}

/// A domain that cannot describe a usable log.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DomainDefect {
    /// Capacity is zero.
    ZeroCapacity,
    /// The fixed payload length is zero.
    ZeroPayloadLength,
    /// The availability threshold is zero or exceeds the share count.
    ThresholdOutOfRange,
}

impl LogDomain {
    /// The frozen domain for one batch of the modelled Dark FBA relation.
    #[must_use]
    pub fn dark_fba_v0(batch: u64, market: u64, cutoff_epoch: u64) -> Self {
        Self {
            relation: DARK_FBA_RELATION,
            batch,
            market,
            cutoff_epoch,
            capacity: DARK_FBA_CAPACITY,
            payload_len: DARK_FBA_PAYLOAD_LEN,
            availability_shares: DARK_FBA_AVAILABILITY_SHARES,
            availability_threshold: DARK_FBA_AVAILABILITY_THRESHOLD,
        }
    }

    /// Check the domain against the bounds the model assumes.
    pub fn validate(&self) -> Result<(), DomainDefect> {
        if self.capacity == 0 {
            return Err(DomainDefect::ZeroCapacity);
        }
        if self.payload_len == 0 {
            return Err(DomainDefect::ZeroPayloadLength);
        }
        if self.availability_threshold == 0
            || self.availability_threshold > self.availability_shares
        {
            return Err(DomainDefect::ThresholdOutOfRange);
        }
        Ok(())
    }

    /// The digest bound into every root this log publishes.
    #[must_use]
    pub fn digest(&self) -> [u8; 32] {
        let relation = self.relation.as_bytes();
        let relation_len = u32::try_from(relation.len()).expect("relation identifier is bounded");
        tagged(
            DOMAIN_TAG,
            &[
                &relation_len.to_be_bytes(),
                relation,
                &self.batch.to_be_bytes(),
                &self.market.to_be_bytes(),
                &self.cutoff_epoch.to_be_bytes(),
                &self.capacity.to_be_bytes(),
                &self.payload_len.to_be_bytes(),
                &[self.availability_shares, self.availability_threshold],
            ],
        )
    }
}

/// What a submitter hands the log holder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmissionRequest {
    /// Admission-credential commitment. Not an identity, and not authenticated here.
    pub submitter: [u8; 32],
    /// Commitment to the encrypted order payload.
    pub payload_commitment: [u8; 32],
    /// Payload length; must equal the domain's fixed wire shape.
    pub payload_len: u32,
    /// Declared availability shares; must equal the domain's share count.
    pub availability_shares: u8,
    /// Arrival epoch in the named external time domain.
    pub arrival_epoch: u64,
    /// Nonzero, batch-scoped nullifier.
    pub nullifier: [u8; 32],
}

/// One admitted record, exactly as committed.
///
/// The relation identifier, batch, market, cutoff, and capacity are *not*
/// repeated per record: they are in the domain digest, which is inside the
/// root, so a record cannot be lifted from one log into another.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmissionRecord {
    /// Canonical position in the log, assigned by the holder at admission.
    ///
    /// This is the residual-allocation rank of `DARK_FBA_RELATION.md` §5.
    pub seq: u32,
    /// Admission-credential commitment.
    pub submitter: [u8; 32],
    /// Commitment to the encrypted order payload.
    pub payload_commitment: [u8; 32],
    /// Payload length.
    pub payload_len: u32,
    /// Declared availability shares.
    pub availability_shares: u8,
    /// Arrival epoch. Epoch granularity only; the exact arrival tick is not committed.
    pub arrival_epoch: u64,
    /// Batch-scoped nullifier.
    pub nullifier: [u8; 32],
}

/// Byte length of a canonical record preimage.
pub const RECORD_BYTES: usize = 4 + 32 + 32 + 4 + 1 + 8 + 32;

impl AdmissionRecord {
    /// The canonical, fixed-width preimage of this record.
    ///
    /// Every field occupies a fixed number of bytes in a fixed order, so the
    /// encoding is injective and no length prefix is required.
    #[must_use]
    pub fn canonical_bytes(&self) -> [u8; RECORD_BYTES] {
        let mut out = [0u8; RECORD_BYTES];
        let mut at = 0usize;
        let mut put = |bytes: &[u8]| {
            out[at..at + bytes.len()].copy_from_slice(bytes);
            at += bytes.len();
        };
        put(&self.seq.to_be_bytes());
        put(&self.submitter);
        put(&self.payload_commitment);
        put(&self.payload_len.to_be_bytes());
        put(&[self.availability_shares]);
        put(&self.arrival_epoch.to_be_bytes());
        put(&self.nullifier);
        debug_assert_eq!(at, RECORD_BYTES);
        out
    }

    /// This record's leaf hash under one log domain.
    ///
    /// The domain digest is inside the leaf, not merely inside the root. Both
    /// bindings are needed for different reasons: the root's binding stops a
    /// root being replayed, and the leaf's binding stops a *record* being
    /// lifted into another log. Without the second, two batches that happened
    /// to admit byte-identical records in the same order would honour each
    /// other's receipts, which `tests/inclusion.rs` checks directly.
    #[must_use]
    pub fn leaf(&self, domain_digest: &[u8; 32]) -> [u8; 32] {
        leaf_hash(&tagged(
            RECORD_TAG,
            &[domain_digest, &self.canonical_bytes()],
        ))
    }

    /// The deterministic padding record for one position of one log.
    ///
    /// Padding exists so that a cutoff root's leaf count is always the padded
    /// capacity and therefore discloses nothing about occupancy, which is what
    /// `DARK_RELATION_THREAT_MODEL.md` requires of the public surface. It is
    /// derived from the domain and the position alone, so two holders of the
    /// same admitted set produce the same padded root and no holder gets a
    /// free degree of freedom in the commitment.
    ///
    /// In this model a padding record is *recognisable*: [`Self::is_padding`]
    /// decides it from public data. Hiding occupancy for real would need a
    /// hiding payload commitment and an unlinkable nullifier, neither of which
    /// this crate has. What padding buys here is that the leaf count stops
    /// being an occupancy channel; the record bytes still are one.
    #[must_use]
    pub fn padding(domain_digest: &[u8; 32], seq: u32, domain: &LogDomain) -> Self {
        let derive =
            |label: u8| tagged(PADDING_TAG, &[domain_digest, &[label], &seq.to_be_bytes()]);
        Self {
            seq,
            submitter: derive(0),
            payload_commitment: derive(1),
            payload_len: domain.payload_len,
            availability_shares: domain.availability_shares,
            arrival_epoch: domain.cutoff_epoch,
            nullifier: derive(2),
        }
    }

    /// Whether this record is the padding record for its own position.
    #[must_use]
    pub fn is_padding(&self, domain: &LogDomain) -> bool {
        *self == Self::padding(&domain.digest(), self.seq, domain)
    }

    /// Whether the record satisfies the shape its domain fixes.
    #[must_use]
    pub fn conforms_to(&self, domain: &LogDomain) -> bool {
        self.payload_len == domain.payload_len
            && self.availability_shares == domain.availability_shares
            && self.arrival_epoch <= domain.cutoff_epoch
            && u64::from(self.seq) < u64::from(domain.capacity)
            && self.nullifier != [0u8; 32]
    }
}

/// One typed admission refusal.
///
/// The check order that selects among these is frozen in [`AdmissionLog::admit`]
/// and asserted by `tests/admission.rs`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdmissionRefusal {
    /// The cutoff root is already published; the log no longer accepts records.
    LogSealed,
    /// The holder's clock is already past the cutoff.
    CutoffPassed {
        /// Holder clock epoch.
        now_epoch: u64,
        /// Cutoff epoch.
        cutoff_epoch: u64,
    },
    /// The request arrived after the cutoff.
    LateArrival {
        /// Claimed arrival epoch.
        arrival_epoch: u64,
        /// Cutoff epoch.
        cutoff_epoch: u64,
    },
    /// The request claims an arrival the holder has not reached yet.
    ArrivalInFuture {
        /// Claimed arrival epoch.
        arrival_epoch: u64,
        /// Holder clock epoch.
        now_epoch: u64,
    },
    /// The payload is not the single admissible wire shape.
    NonCanonicalPayloadSize {
        /// Length offered.
        offered: u32,
        /// Length the domain fixes.
        required: u32,
    },
    /// The declared availability share count is not the domain's.
    AvailabilitySharesMismatch {
        /// Count offered.
        offered: u8,
        /// Count the domain fixes.
        required: u8,
    },
    /// The nullifier is zero.
    NullifierZero,
    /// The nullifier is one this log reserves for a padding record.
    ///
    /// Padding nullifiers are derived from public data, so a submitter can
    /// compute them. Without this check a submitter could claim a position's
    /// padding nullifier and break the batch-scoped uniqueness rule the
    /// relation depends on.
    NullifierReservedForPadding {
        /// Position whose padding record claims this nullifier.
        padding_seq: u32,
    },
    /// The nullifier is already admitted in this batch.
    NullifierRepeated {
        /// Sequence position of the earlier admission.
        first_seq: u32,
    },
    /// The log already holds `capacity` records.
    CapacityExhausted {
        /// The domain's capacity.
        capacity: u32,
    },
}

/// Why a cutoff seal was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SealRefusal {
    /// The holder's clock has not reached the cutoff.
    BeforeCutoff {
        /// Holder clock epoch.
        now_epoch: u64,
        /// Cutoff epoch.
        cutoff_epoch: u64,
    },
    /// A cutoff root is already published for this log.
    AlreadySealed,
}

/// The receipt a submitter keeps before the cutoff.
///
/// It commits to the log *as it stood* at that moment. Combined with a
/// consistency proof against the cutoff root, it is what turns a silent
/// rollback into a verifiable append-only violation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmissionAck {
    /// Digest of the log domain.
    pub domain_digest: [u8; 32],
    /// Position assigned to the record.
    pub seq: u32,
    /// The record as committed.
    pub record: AdmissionRecord,
    /// Leaf count immediately after this admission.
    pub running_leaf_count: u64,
    /// Root immediately after this admission.
    pub running_root: [u8; 32],
}

/// The published cutoff commitment.
///
/// `root` commits to the domain, the exact number of admitted records, and
/// their exact order. It is not opaque: every claim about the admitted set is
/// checkable against it with a proof object from this crate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CutoffRoot {
    /// The frozen log domain.
    pub domain: LogDomain,
    /// Number of admitted records at the cutoff.
    pub leaf_count: u64,
    /// The cutoff root.
    pub root: [u8; 32],
}

/// A per-record membership proof against a cutoff root.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InclusionReceipt {
    /// The admitted record.
    pub record: AdmissionRecord,
    /// Its membership proof.
    pub proof: NodeProof,
}

/// Why an inclusion receipt failed to verify.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReceiptDefect {
    /// The cutoff root's domain is invalid.
    Domain(DomainDefect),
    /// The published root is not the root its own domain and leaf count imply.
    MalformedCutoffRoot,
    /// The record violates the shape its domain fixes.
    RecordViolatesDomain,
    /// The proof is for an interior node, not a leaf.
    NotALeaf {
        /// Height the proof claims.
        height: u32,
    },
    /// The proof places the record at a position other than its own `seq`.
    SequenceMismatch {
        /// Sequence the record claims.
        claimed: u32,
        /// Index the proof determines.
        derived: u64,
    },
    /// The proof itself does not verify.
    Proof(ProofDefect),
}

/// Verify an inclusion receipt against a cutoff root, with no access to the log.
///
/// This function is the whole point of the packet: it takes a root, a record,
/// and a proof, and nothing else. The log holder is not consulted, cannot
/// participate, and cannot make a false receipt verify without a hash
/// collision.
pub fn verify_receipt(
    cutoff: &CutoffRoot,
    receipt: &InclusionReceipt,
) -> Result<(), ReceiptDefect> {
    cutoff.domain.validate().map_err(ReceiptDefect::Domain)?;
    let domain_digest = cutoff.domain.digest();
    if cutoff.leaf_count > u64::from(cutoff.domain.capacity) {
        return Err(ReceiptDefect::MalformedCutoffRoot);
    }
    if !receipt.record.conforms_to(&cutoff.domain) {
        return Err(ReceiptDefect::RecordViolatesDomain);
    }
    if receipt.proof.height != 0 {
        return Err(ReceiptDefect::NotALeaf {
            height: receipt.proof.height,
        });
    }
    let position = verify_node_proof(
        &domain_digest,
        &cutoff.root,
        cutoff.leaf_count,
        &receipt.record.leaf(&domain_digest),
        &receipt.proof,
    )
    .map_err(ReceiptDefect::Proof)?;
    if position
        != (NodePosition {
            height: 0,
            index: u64::from(receipt.record.seq),
        })
    {
        return Err(ReceiptDefect::SequenceMismatch {
            claimed: receipt.record.seq,
            derived: position.index,
        });
    }
    Ok(())
}

/// An append-only admission log held by one party for one cutoff.
#[derive(Clone, Debug)]
pub struct AdmissionLog {
    domain: LogDomain,
    mmr: Mmr,
    records: Vec<AdmissionRecord>,
    nullifiers: BTreeMap<[u8; 32], u32>,
    sealed: Option<CutoffRoot>,
}

impl AdmissionLog {
    /// Open a log for `domain`.
    pub fn open(domain: LogDomain) -> Result<Self, DomainDefect> {
        domain.validate()?;
        Ok(Self {
            mmr: Mmr::new(domain.digest()),
            domain,
            records: Vec::new(),
            nullifiers: BTreeMap::new(),
            sealed: None,
        })
    }

    /// The log's frozen domain.
    #[must_use]
    pub fn domain(&self) -> LogDomain {
        self.domain
    }

    /// Records admitted so far, in canonical order.
    #[must_use]
    pub fn records(&self) -> &[AdmissionRecord] {
        &self.records
    }

    /// The current running root, whether or not the log is sealed.
    #[must_use]
    pub fn running_root(&self) -> [u8; 32] {
        self.mmr.root()
    }

    /// The published cutoff root, if the log is sealed.
    #[must_use]
    pub fn cutoff(&self) -> Option<CutoffRoot> {
        self.sealed
    }

    /// Admit one request at holder clock `now_epoch`.
    ///
    /// Frozen check order, top to bottom: seal state, holder clock against the
    /// cutoff, arrival against the cutoff, arrival against the holder clock,
    /// payload shape, availability shares, nullifier nonzero, nullifier not
    /// reserved for padding, nullifier freshness, capacity. The first failing
    /// check names the refusal; later checks are not consulted.
    ///
    /// The order is not arbitrary. Testing arrival against the *cutoff* before
    /// arrival against the *holder clock* is what keeps both classes reachable:
    /// the holder clock never passes the cutoff without the earlier check
    /// firing, so under the opposite order a late arrival could only ever be
    /// reported as a future arrival, and `LateArrival` would be a class no
    /// witness can produce. `tests/admission.rs` pins the resulting order and
    /// separately asserts that every class is reachable.
    pub fn admit(
        &mut self,
        request: &AdmissionRequest,
        now_epoch: u64,
    ) -> Result<AdmissionAck, AdmissionRefusal> {
        if self.sealed.is_some() {
            return Err(AdmissionRefusal::LogSealed);
        }
        if now_epoch > self.domain.cutoff_epoch {
            return Err(AdmissionRefusal::CutoffPassed {
                now_epoch,
                cutoff_epoch: self.domain.cutoff_epoch,
            });
        }
        if request.arrival_epoch > self.domain.cutoff_epoch {
            return Err(AdmissionRefusal::LateArrival {
                arrival_epoch: request.arrival_epoch,
                cutoff_epoch: self.domain.cutoff_epoch,
            });
        }
        if request.arrival_epoch > now_epoch {
            return Err(AdmissionRefusal::ArrivalInFuture {
                arrival_epoch: request.arrival_epoch,
                now_epoch,
            });
        }
        if request.payload_len != self.domain.payload_len {
            return Err(AdmissionRefusal::NonCanonicalPayloadSize {
                offered: request.payload_len,
                required: self.domain.payload_len,
            });
        }
        if request.availability_shares != self.domain.availability_shares {
            return Err(AdmissionRefusal::AvailabilitySharesMismatch {
                offered: request.availability_shares,
                required: self.domain.availability_shares,
            });
        }
        if request.nullifier == [0u8; 32] {
            return Err(AdmissionRefusal::NullifierZero);
        }
        if let Some(padding_seq) = self.padding_position_of(&request.nullifier) {
            return Err(AdmissionRefusal::NullifierReservedForPadding { padding_seq });
        }
        if let Some(first_seq) = self.nullifiers.get(&request.nullifier) {
            return Err(AdmissionRefusal::NullifierRepeated {
                first_seq: *first_seq,
            });
        }
        let seq = u32::try_from(self.records.len()).expect("capacity is bounded by u32");
        if seq >= self.domain.capacity {
            return Err(AdmissionRefusal::CapacityExhausted {
                capacity: self.domain.capacity,
            });
        }

        let record = AdmissionRecord {
            seq,
            submitter: request.submitter,
            payload_commitment: request.payload_commitment,
            payload_len: request.payload_len,
            availability_shares: request.availability_shares,
            arrival_epoch: request.arrival_epoch,
            nullifier: request.nullifier,
        };
        self.mmr.append(record.leaf(&self.domain.digest()));
        self.records.push(record);
        self.nullifiers.insert(record.nullifier, seq);

        Ok(AdmissionAck {
            domain_digest: self.domain.digest(),
            seq,
            record,
            running_leaf_count: self.mmr.leaf_count(),
            running_root: self.mmr.root(),
        })
    }

    /// The position whose padding record claims `nullifier`, if any.
    #[must_use]
    pub fn padding_position_of(&self, nullifier: &[u8; 32]) -> Option<u32> {
        let digest = self.domain.digest();
        (0..self.domain.capacity).find(|seq| {
            AdmissionRecord::padding(&digest, *seq, &self.domain).nullifier == *nullifier
        })
    }

    /// Publish the cutoff root, padded to capacity, at holder clock `now_epoch`.
    ///
    /// This is the sealing rule the disclosure budget requires: every batch of
    /// the same relation commits to exactly `capacity` leaves, so the published
    /// leaf count is a constant and not a participation count. The unpadded
    /// [`Self::seal`] remains available because the difference between the two
    /// is exactly the occupancy channel the tests measure.
    pub fn seal_padded(&mut self, now_epoch: u64) -> Result<CutoffRoot, SealRefusal> {
        if self.sealed.is_some() {
            return Err(SealRefusal::AlreadySealed);
        }
        if now_epoch < self.domain.cutoff_epoch {
            return Err(SealRefusal::BeforeCutoff {
                now_epoch,
                cutoff_epoch: self.domain.cutoff_epoch,
            });
        }
        let digest = self.domain.digest();
        while self.records.len() < self.domain.capacity as usize {
            let seq = u32::try_from(self.records.len()).expect("capacity is bounded by u32");
            let record = AdmissionRecord::padding(&digest, seq, &self.domain);
            self.mmr.append(record.leaf(&digest));
            self.records.push(record);
            self.nullifiers.insert(record.nullifier, seq);
        }
        self.seal(now_epoch)
    }

    /// Publish the cutoff root at holder clock `now_epoch`, without padding.
    pub fn seal(&mut self, now_epoch: u64) -> Result<CutoffRoot, SealRefusal> {
        if self.sealed.is_some() {
            return Err(SealRefusal::AlreadySealed);
        }
        if now_epoch < self.domain.cutoff_epoch {
            return Err(SealRefusal::BeforeCutoff {
                now_epoch,
                cutoff_epoch: self.domain.cutoff_epoch,
            });
        }
        let cutoff = CutoffRoot {
            domain: self.domain,
            leaf_count: self.mmr.leaf_count(),
            root: self.mmr.root(),
        };
        self.sealed = Some(cutoff);
        Ok(cutoff)
    }

    /// An inclusion receipt for `seq` against the cutoff root.
    ///
    /// Returns `None` before the cutoff: there is no committed set to prove
    /// membership in yet, and an incremental ack is the only pre-cutoff object.
    #[must_use]
    pub fn receipt(&self, seq: u32) -> Option<InclusionReceipt> {
        let cutoff = self.sealed?;
        let record = *self.records.get(usize::try_from(seq).ok()?)?;
        let proof = self.mmr.leaf_proof_at(u64::from(seq), cutoff.leaf_count)?;
        Some(InclusionReceipt { record, proof })
    }

    /// A receipt for `seq` against the log's *current* running root.
    ///
    /// Used by the model to build the post-cutoff-append adversarial case: a
    /// receipt that verifies against an extended root and must not verify
    /// against the cutoff root.
    #[must_use]
    pub fn running_receipt(&self, seq: u32) -> Option<InclusionReceipt> {
        let record = *self.records.get(usize::try_from(seq).ok()?)?;
        let proof = self.mmr.leaf_proof(u64::from(seq))?;
        Some(InclusionReceipt { record, proof })
    }

    /// A consistency proof from `prefix_leaf_count` leaves to the current root.
    #[must_use]
    pub fn consistency_proof(&self, prefix_leaf_count: u64) -> Option<ConsistencyProof> {
        self.mmr.consistency_proof(prefix_leaf_count)
    }

    /// The running root the log had after `leaf_count` admissions.
    #[must_use]
    pub fn root_at(&self, leaf_count: u64) -> [u8; 32] {
        self.mmr.root_at(leaf_count)
    }

    /// Append past the cutoff, ignoring the seal.
    ///
    /// This is a deliberate adversary handle, not an operation the honest
    /// interface offers. It exists so the tests can build a holder that keeps
    /// growing a log whose cutoff root is already published, and check that the
    /// resulting receipts do not verify against that root.
    pub fn adversarially_append_after_cutoff(&mut self, request: &AdmissionRequest) -> u32 {
        let seq = u32::try_from(self.records.len()).expect("capacity is bounded by u32");
        let record = AdmissionRecord {
            seq,
            submitter: request.submitter,
            payload_commitment: request.payload_commitment,
            payload_len: request.payload_len,
            availability_shares: request.availability_shares,
            arrival_epoch: request.arrival_epoch,
            nullifier: request.nullifier,
        };
        self.mmr.append(record.leaf(&self.domain.digest()));
        self.records.push(record);
        self.nullifiers.entry(record.nullifier).or_insert(seq);
        seq
    }
}
