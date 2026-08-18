//! Receipt domains, receipt shapes, and the two receipts the Clear lowering
//! actually emits.
//!
//! Receipt domains are distinct preimage domains: digests from different
//! domains must never be compared. In this crate a receipt is plain data with
//! a canonical encoding. Nothing signs it, nothing verifies it, and a digest
//! over low-entropy private data is trivially brute-forceable, so no receipt
//! here hides anything or proves anything.

use crate::canon::{Canonical, Sink};
use crate::refusal::RefusalClass;
use crate::ty::PortDecl;

/// The distinct receipt domains of the IR design.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReceiptDomain {
    /// Source/admission receipt: inclusion of an accepted input.
    SourceAdmission,
    /// Computation receipt: one evaluation of the relation.
    Computation,
    /// Proof-verification receipt.
    ProofVerification,
    /// Output-delivery receipt: one owner-local output handed over.
    OutputDelivery,
    /// Settlement receipt.
    Settlement,
    /// Disclosure receipt.
    Disclosure,
    /// Retention/destruction receipt.
    Retention,
}

impl ReceiptDomain {
    fn code(self) -> u32 {
        match self {
            Self::SourceAdmission => 0,
            Self::Computation => 1,
            Self::ProofVerification => 2,
            Self::OutputDelivery => 3,
            Self::Settlement => 4,
            Self::Disclosure => 5,
            Self::Retention => 6,
        }
    }
}

impl Canonical for ReceiptDomain {
    fn tag(&self) -> &'static str {
        "ir/receipt-domain"
    }
    fn body(&self, sink: &mut Sink) {
        sink.u32(self.code());
    }
}

/// Whether the Clear lowering emits instances of a shape, or the shape is a
/// declared obligation with no emitting machinery in this repository.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeStatus {
    /// The Clear evaluator emits instances of this shape.
    EmittedByClearLowering,
    /// Declared only: no admission log, proof system, settlement adapter,
    /// disclosure process, or retention machinery exists here.
    DeclaredOnly,
}

impl ShapeStatus {
    fn code(self) -> u32 {
        match self {
            Self::EmittedByClearLowering => 0,
            Self::DeclaredOnly => 1,
        }
    }
}

impl Canonical for ShapeStatus {
    fn tag(&self) -> &'static str {
        "ir/shape-status"
    }
    fn body(&self, sink: &mut Sink) {
        sink.u32(self.code());
    }
}

/// One frozen receipt shape of a relation module.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReceiptShape {
    /// The preimage domain.
    pub domain: ReceiptDomain,
    /// Whether anything here emits it.
    pub status: ShapeStatus,
    /// The receipt's fields, in canonical order.
    pub fields: Vec<PortDecl>,
}

impl Canonical for ReceiptShape {
    fn tag(&self) -> &'static str {
        "ir/receipt-shape"
    }
    fn body(&self, sink: &mut Sink) {
        sink.nested(&self.domain);
        sink.nested(&self.status);
        sink.count(self.fields.len());
        for field in &self.fields {
            sink.nested(field);
        }
    }
}

/// The status a computation receipt reports.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReceiptStatus {
    /// The batch settled (a valid no-trade settles).
    Settled,
    /// The batch was refused with the named public class.
    Refused(RefusalClass),
}

impl Canonical for ReceiptStatus {
    fn tag(&self) -> &'static str {
        "ir/receipt-status"
    }
    fn body(&self, sink: &mut Sink) {
        match self {
            Self::Settled => sink.u32(0),
            Self::Refused(class) => {
                sink.u32(1);
                sink.nested(class);
            }
        }
    }
}

/// The computation receipt the Clear evaluator emits for every evaluation,
/// settled or refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComputationReceipt {
    /// Digest of the relation module's canonical bytes.
    pub module_digest: [u8; 32],
    /// Digest of the batch input's canonical bytes.
    pub input_digest: [u8; 32],
    /// Digest of the public outcome's canonical bytes.
    pub outcome_digest: [u8; 32],
    /// Settled, or the public refusal class.
    pub status: ReceiptStatus,
}

impl Canonical for ComputationReceipt {
    fn tag(&self) -> &'static str {
        "ir/receipt/computation"
    }
    fn body(&self, sink: &mut Sink) {
        sink.digest(&self.module_digest);
        sink.digest(&self.input_digest);
        sink.digest(&self.outcome_digest);
        sink.nested(&self.status);
    }
}

/// The per-owner output-delivery receipt the Clear evaluator emits for a
/// settled batch. In the Clear lowering the one executing process holds every
/// one of these; there is no access control and no delivery mechanism.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutputDeliveryReceipt {
    /// The owner the output belongs to.
    pub owner: u8,
    /// Digest of the relation module's canonical bytes.
    pub module_digest: [u8; 32],
    /// Digest of the batch input's canonical bytes.
    pub input_digest: [u8; 32],
    /// Digest of the owner-local output's canonical bytes.
    pub output_digest: [u8; 32],
}

impl Canonical for OutputDeliveryReceipt {
    fn tag(&self) -> &'static str {
        "ir/receipt/output-delivery"
    }
    fn body(&self, sink: &mut Sink) {
        sink.u8(self.owner);
        sink.digest(&self.module_digest);
        sink.digest(&self.input_digest);
        sink.digest(&self.output_digest);
    }
}
