//! One batch instance of the relation's input ports.
//!
//! Domain fields are stored unvalidated so that out-of-domain witnesses stay
//! representable and admission stays a real check. The four per-order
//! statements and the four boundary statements are booleans supplied by the
//! executor; they stand for external proofs that do not exist here.

use crate::canon::{Canonical, Sink};

/// Requested execution mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RequestedMode {
    /// The caller may inspect everything.
    Clear,
    /// One named executor process sees everything.
    ShieldedSingleExecutor,
    /// Refused: no Dark backend exists.
    DarkTarget,
}

impl RequestedMode {
    fn code(self) -> u32 {
        match self {
            Self::Clear => 0,
            Self::ShieldedSingleExecutor => 1,
            Self::DarkTarget => 2,
        }
    }
}

/// Which side of the book an order sits on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    /// Pays quote atoms, receives base atoms.
    Buy,
    /// Pays base atoms, receives quote atoms.
    Sell,
}

/// One occupied slot's witness.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OrderWitness {
    /// Batch identifier the order binds.
    pub batch_id: u64,
    /// Market identifier the order binds.
    pub market_id: u64,
    /// Owner index; admission requires it in domain.
    pub owner: u8,
    /// Side of the book.
    pub side: Side,
    /// Limit tick index; admission requires it in domain.
    pub limit_tick: u8,
    /// Quantity in base atoms; admission requires it in domain.
    pub quantity: u64,
    /// Reserved amount in the spending asset: quote for a buy, base for a sell.
    pub reserved: u64,
    /// Batch-scoped nullifier; admission requires nonzero and distinct.
    pub nullifier: u64,
    /// Arrival time in the batch's external time domain.
    pub arrived_at: u64,
    /// Authorization statement.
    pub authorized: bool,
    /// Eligibility statement.
    pub eligible: bool,
    /// Exact-inclusion statement for the accepted-input root.
    pub included_under_root: bool,
    /// Custody-binding statement.
    pub custody_bound: bool,
}

impl Canonical for OrderWitness {
    fn tag(&self) -> &'static str {
        "ir/order-witness"
    }
    fn body(&self, sink: &mut Sink) {
        sink.u64(self.batch_id);
        sink.u64(self.market_id);
        sink.u8(self.owner);
        sink.u32(match self.side {
            Side::Buy => 0,
            Side::Sell => 1,
        });
        sink.u8(self.limit_tick);
        sink.u64(self.quantity);
        sink.u64(self.reserved);
        sink.u64(self.nullifier);
        sink.u64(self.arrived_at);
        sink.bool(self.authorized);
        sink.bool(self.eligible);
        sink.bool(self.included_under_root);
        sink.bool(self.custody_bound);
    }
}

/// A padded slot: canonical emptiness with no latent fields, or one order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlotInput {
    /// Canonical empty encoding.
    Empty,
    /// Occupied slot.
    Occupied(OrderWitness),
}

impl Canonical for SlotInput {
    fn tag(&self) -> &'static str {
        "ir/slot"
    }
    fn body(&self, sink: &mut Sink) {
        match self {
            Self::Empty => sink.u8(0),
            Self::Occupied(order) => {
                sink.u8(1);
                sink.nested(order);
            }
        }
    }
}

/// The four executor-supplied boundary statements.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoundaryStatements {
    /// The admission log is final.
    pub log_final: bool,
    /// The witness slots open exactly to the accepted-input root.
    pub root_binds_slots: bool,
    /// No conflicting finalized root exists.
    pub no_conflicting_root: bool,
    /// Every admitted payload is available by the declared threshold.
    pub payloads_available: bool,
}

impl BoundaryStatements {
    /// All four statements hold.
    pub const SATISFIED: Self = Self {
        log_final: true,
        root_binds_slots: true,
        no_conflicting_root: true,
        payloads_available: true,
    };
}

impl Canonical for BoundaryStatements {
    fn tag(&self) -> &'static str {
        "ir/boundary-statements"
    }
    fn body(&self, sink: &mut Sink) {
        sink.bool(self.log_final);
        sink.bool(self.root_binds_slots);
        sink.bool(self.no_conflicting_root);
        sink.bool(self.payloads_available);
    }
}

/// One batch evaluation request: every input port, instantiated.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BatchInput {
    /// Requested execution mode.
    pub requested_mode: RequestedMode,
    /// Batch identifier.
    pub batch_id: u64,
    /// Market identifier.
    pub market_id: u64,
    /// Cutoff time.
    pub cutoff: u64,
    /// Opaque accepted-input root; preserved into the result, never verified.
    pub accepted_input_root: [u8; 32],
    /// Executor-supplied boundary statements.
    pub boundary: BoundaryStatements,
    /// The padded slots; index is the canonical residual rank. Length must
    /// equal the module's slot capacity or evaluation refuses as malformed.
    pub slots: Vec<SlotInput>,
}

impl Canonical for BatchInput {
    fn tag(&self) -> &'static str {
        "ir/batch-input"
    }
    fn body(&self, sink: &mut Sink) {
        sink.u32(self.requested_mode.code());
        sink.u64(self.batch_id);
        sink.u64(self.market_id);
        sink.u64(self.cutoff);
        sink.digest(&self.accepted_input_root);
        sink.nested(&self.boundary);
        sink.count(self.slots.len());
        for slot in &self.slots {
            sink.nested(slot);
        }
    }
}
