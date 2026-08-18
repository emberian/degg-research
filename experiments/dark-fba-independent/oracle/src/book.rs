//! Batch inputs: the padded slot array, the boundary statements supplied by the
//! executor, and the requested execution mode.

use crate::params::SLOTS;

/// Which side of the book an order sits on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    /// Pays quote atoms, receives base atoms.
    Buy,
    /// Pays base atoms, receives quote atoms.
    Sell,
}

/// One occupied slot's private witness.
///
/// Domain fields (`owner`, `limit_index`, `quantity`) are stored unvalidated so
/// that out-of-domain witnesses stay representable and admission stays a real
/// check rather than a type-level assumption.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Order {
    /// Batch the order is bound to.
    pub batch: u64,
    /// Market the order is bound to.
    pub market: u64,
    /// Owner index; admission requires `< OWNERS`.
    pub owner: u8,
    /// Side of the book.
    pub direction: Direction,
    /// Tick index; admission requires `< TICKS`.
    pub limit_index: u8,
    /// Quantity in base atoms; admission requires `QUANTITY_FLOOR..=QUANTITY_CEILING`.
    pub quantity: u32,
    /// Reserved amount in the spending asset (quote for buys, base for sells).
    pub reserved: u64,
    /// Batch-scoped nullifier; admission requires nonzero and pairwise distinct.
    pub nullifier: u64,
    /// Arrival time in the batch's external time domain.
    pub arrival: u64,
    /// External authorization statement.
    pub authorized: bool,
    /// External eligibility statement.
    pub eligible: bool,
    /// Exact-inclusion statement for the accepted-input root.
    pub included: bool,
    /// Custody-binding statement: the reserved amount refers to distinct,
    /// non-double-counted custody. `DARK_FBA_RELATION.md` section 4 places this
    /// obligation on the external admission relation; like the three statements
    /// above it is a boolean witness here, not a proof, and this oracle cannot
    /// bind it to a ledger.
    pub custody_bound: bool,
}

/// A padded slot: either canonical emptiness or one order.
///
/// `Vacant` carries no latent fields, so an empty slot cannot hold value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Slot {
    /// Canonical empty encoding.
    #[default]
    Vacant,
    /// Occupied slot.
    Taken(Order),
}

/// The four externally supplied statements about the admission phase.
///
/// These are booleans, not proofs. The relation refuses unless all four hold.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Boundary {
    /// The admission log is final for this batch.
    pub log_final: bool,
    /// The witness slots open exactly to the accepted-input root.
    pub root_binds_slots: bool,
    /// No conflicting finalized root exists for the same relation/batch/market.
    pub no_conflicting_root: bool,
    /// Every admitted payload is recoverable before computation begins.
    pub payloads_available: bool,
}

impl Boundary {
    /// All four statements hold.
    pub const SATISFIED: Self = Self {
        log_final: true,
        root_binds_slots: true,
        no_conflicting_root: true,
        payloads_available: true,
    };
}

/// Requested execution mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// Caller may inspect the whole fixture.
    Clear,
    /// One named executor process sees the whole fixture and all local outputs.
    ShieldedSingleExecutor,
    /// Refused: no Dark backend exists.
    DarkTarget,
}

/// One batch evaluation request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Batch {
    /// Batch identifier.
    pub batch: u64,
    /// Market identifier.
    pub market: u64,
    /// Cutoff time in the batch's external time domain.
    pub cutoff: u64,
    /// Opaque accepted-input root, preserved into the result and never verified.
    pub accepted_input_root: [u8; 32],
    /// The padded slot array; index is the canonical residual rank.
    pub slots: [Slot; SLOTS],
    /// Executor-supplied boundary statements.
    pub boundary: Boundary,
    /// Requested execution mode.
    pub mode: Mode,
}
