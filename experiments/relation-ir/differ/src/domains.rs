//! Enumerated differential domains over IR batches.
//!
//! Every domain is a total enumeration of a precisely stated finite set of
//! batches. Nothing is sampled and nothing is random. The construction reuses
//! the earlier differential's discipline: a mixed-radix slot alphabet for the
//! admissible book domains, and named perturbation subsets for the refusal
//! surface.

use degg_relation_ir::batch::{
    BatchInput, BoundaryStatements, OrderWitness, RequestedMode, Side, SlotInput,
};
use degg_relation_ir::lower::required_reservation;

/// Batch identifier of every enumerated batch.
pub const BATCH_ID: u64 = 7;
/// Market identifier of every enumerated batch.
pub const MARKET_ID: u64 = 9;
/// Cutoff of every enumerated batch.
pub const CUTOFF: u64 = 10;
/// Accepted-input root of every enumerated batch.
pub const ROOT: [u8; 32] = [0x5A; 32];

const TICKS: u64 = 4;
const SLOTS: usize = 4;
const PRICES: [u64; 4] = [1, 2, 3, 4];

/// Alphabet size for one slot at a given quantity ceiling: vacancy plus every
/// (side, tick, quantity) triple.
pub const fn alphabet(ceiling: u64) -> u64 {
    1 + 2 * TICKS * ceiling
}

/// An empty, admissible batch.
pub fn empty_batch() -> BatchInput {
    BatchInput {
        requested_mode: RequestedMode::Clear,
        batch_id: BATCH_ID,
        market_id: MARKET_ID,
        cutoff: CUTOFF,
        accepted_input_root: ROOT,
        boundary: BoundaryStatements::SATISFIED,
        slots: vec![SlotInput::Empty; SLOTS],
    }
}

/// Decode one slot letter. Letter 0 is vacancy; letter
/// `1 + ((side * TICKS) + tick) * ceiling + (q - 1)` is the corresponding
/// order with canonical admission fields and an exact-plus-surplus reservation.
pub fn decode_slot(letter: u64, index: usize, ceiling: u64, owner: u8, surplus: u64) -> SlotInput {
    if letter == 0 {
        return SlotInput::Empty;
    }
    let letter = letter - 1;
    let quantity = letter % ceiling + 1;
    let rest = letter / ceiling;
    let limit_tick = (rest % TICKS) as u8;
    let side = if rest / TICKS == 0 {
        Side::Buy
    } else {
        Side::Sell
    };
    let required = required_reservation(&PRICES, side, limit_tick, quantity)
        .expect("in-domain orders have bounded reservations");
    SlotInput::Occupied(OrderWitness {
        batch_id: BATCH_ID,
        market_id: MARKET_ID,
        owner,
        side,
        limit_tick,
        quantity,
        reserved: required + surplus,
        nullifier: index as u64 + 1,
        arrived_at: CUTOFF,
        authorized: true,
        eligible: true,
        included_under_root: true,
        custody_bound: true,
    })
}

/// Decode a whole book from a mixed-radix index.
pub fn decode_book(mut code: u64, ceiling: u64, owners: [u8; 4], surplus: [u64; 4]) -> BatchInput {
    let radix = alphabet(ceiling);
    let mut batch = empty_batch();
    for index in 0..SLOTS {
        let letter = code % radix;
        code /= radix;
        batch.slots[index] = decode_slot(letter, index, ceiling, owners[index], surplus[index]);
    }
    batch
}

/// One named perturbation. Batch-level perturbations ignore the slot argument;
/// a perturbation of a vacant slot is a no-op.
#[derive(Clone, Copy)]
pub struct Perturbation {
    /// Label used in divergence reports.
    pub label: &'static str,
    /// The edit, applied to the given slot index.
    pub apply: fn(&mut BatchInput, usize),
}

fn edit(batch: &mut BatchInput, index: usize, edit: impl FnOnce(&mut OrderWitness)) {
    if let SlotInput::Occupied(ref mut order) = batch.slots[index] {
        edit(order);
    }
}

macro_rules! perturbations {
    ($($label:literal => $body:expr),* $(,)?) => {
        [$(Perturbation { label: $label, apply: $body }),*]
    };
}

/// Every batch-level perturbation.
pub const BATCH_PERTURBATIONS: [Perturbation; 6] = perturbations![
    "boundary.log-not-final" => |b: &mut BatchInput, _| b.boundary.log_final = false,
    "boundary.root-unbound" => |b: &mut BatchInput, _| b.boundary.root_binds_slots = false,
    "boundary.equivocation" => |b: &mut BatchInput, _| b.boundary.no_conflicting_root = false,
    "boundary.unavailable" => |b: &mut BatchInput, _| b.boundary.payloads_available = false,
    "mode.shielded" => |b: &mut BatchInput, _| b.requested_mode = RequestedMode::ShieldedSingleExecutor,
    "mode.dark" => |b: &mut BatchInput, _| b.requested_mode = RequestedMode::DarkTarget,
];

/// Every per-slot perturbation. The quantity extreme stays within `u32` so the
/// independent oracle's width can represent every enumerated witness exactly.
pub const SLOT_PERTURBATIONS: [Perturbation; 19] = perturbations![
    "wrong-batch" => |b: &mut BatchInput, i| edit(b, i, |o| o.batch_id += 1),
    "wrong-market" => |b: &mut BatchInput, i| edit(b, i, |o| o.market_id += 1),
    "owner-4" => |b: &mut BatchInput, i| edit(b, i, |o| o.owner = 4),
    "owner-200" => |b: &mut BatchInput, i| edit(b, i, |o| o.owner = 200),
    "limit-4" => |b: &mut BatchInput, i| edit(b, i, |o| o.limit_tick = 4),
    "limit-255" => |b: &mut BatchInput, i| edit(b, i, |o| o.limit_tick = 255),
    "quantity-0" => |b: &mut BatchInput, i| edit(b, i, |o| o.quantity = 0),
    "quantity-16" => |b: &mut BatchInput, i| edit(b, i, |o| o.quantity = 16),
    "quantity-u32-max" => |b: &mut BatchInput, i| edit(b, i, |o| o.quantity = u64::from(u32::MAX)),
    "late-by-one" => |b: &mut BatchInput, i| edit(b, i, |o| o.arrived_at = CUTOFF + 1),
    "unauthorized" => |b: &mut BatchInput, i| edit(b, i, |o| o.authorized = false),
    "ineligible" => |b: &mut BatchInput, i| edit(b, i, |o| o.eligible = false),
    "not-included" => |b: &mut BatchInput, i| edit(b, i, |o| o.included_under_root = false),
    "custody-unbound" => |b: &mut BatchInput, i| edit(b, i, |o| o.custody_bound = false),
    "nullifier-zero" => |b: &mut BatchInput, i| edit(b, i, |o| o.nullifier = 0),
    "nullifier-dup-slot0" => |b: &mut BatchInput, i| edit(b, i, |o| o.nullifier = 1),
    "nullifier-dup-slot3" => |b: &mut BatchInput, i| edit(b, i, |o| o.nullifier = 4),
    "reserved-short-by-one" => |b: &mut BatchInput, i| edit(b, i, |o| o.reserved = o.reserved.saturating_sub(1)),
    "reserved-zero" => |b: &mut BatchInput, i| edit(b, i, |o| o.reserved = 0),
];

/// The slot-alphabet letter for one (side, tick, quantity) triple at the full
/// quantity ceiling of 15.
fn letter(side: Side, tick: u64, quantity: u64) -> u64 {
    let side = match side {
        Side::Buy => 0,
        Side::Sell => 1,
    };
    1 + (side * TICKS + tick) * 15 + (quantity - 1)
}

/// The six base books of the refusal domain.
pub fn base_books() -> [(&'static str, BatchInput); 6] {
    let full = {
        let mut batch = empty_batch();
        batch.slots = vec![
            decode_slot(letter(Side::Buy, 3, 7), 0, 15, 0, 0),
            decode_slot(letter(Side::Sell, 0, 9), 1, 15, 1, 0),
            decode_slot(letter(Side::Buy, 1, 15), 2, 15, 2, 0),
            decode_slot(letter(Side::Sell, 2, 1), 3, 15, 3, 0),
        ];
        batch
    };
    let single = {
        let mut batch = empty_batch();
        batch.slots[2] = decode_slot(letter(Side::Sell, 2, 5), 2, 15, 1, 0);
        batch
    };
    [
        ("empty", empty_batch()),
        (
            "balanced-residual",
            degg_relation_ir::fixtures::balanced_residual(),
        ),
        ("price-tie-low", degg_relation_ir::fixtures::price_tie_low()),
        ("no-trade", degg_relation_ir::fixtures::no_trade()),
        ("full-book", full),
        ("single-order", single),
    ]
}

/// One perturbation applied at one slot index.
pub type Application = (&'static str, fn(&mut BatchInput, usize), usize);

/// Every (perturbation, slot) application: the 6 batch-level ones once each,
/// and the 19 per-slot ones at each of the four slots.
pub fn applications() -> Vec<Application> {
    let mut all = Vec::new();
    for perturbation in BATCH_PERTURBATIONS {
        all.push((perturbation.label, perturbation.apply, 0));
    }
    for perturbation in SLOT_PERTURBATIONS {
        for slot in 0..SLOTS {
            all.push((perturbation.label, perturbation.apply, slot));
        }
    }
    all
}
