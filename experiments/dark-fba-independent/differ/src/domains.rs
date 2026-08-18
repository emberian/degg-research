//! Enumerated differential domains.
//!
//! Every domain is a total enumeration of a precisely stated finite set of
//! batches. Nothing is sampled and nothing is random.

use degg_batch_oracle::admit::required_reservation;
use degg_batch_oracle::book::{Batch, Boundary, Direction, Mode, Order, Slot};
use degg_batch_oracle::params::{QUANTITY_CEILING, SLOTS, TICKS};

pub const BATCH_ID: u64 = 7;
pub const MARKET_ID: u64 = 9;
pub const CUTOFF: u64 = 10;
pub const ROOT: [u8; 32] = [0x5A; 32];

/// Alphabet size for one slot at a given quantity ceiling: vacancy plus every
/// (side, tick, quantity) triple.
pub const fn alphabet(quantity_ceiling: u32) -> u32 {
    1 + 2 * TICKS as u32 * quantity_ceiling
}

/// An empty, admissible batch.
pub fn empty_batch() -> Batch {
    Batch {
        batch: BATCH_ID,
        market: MARKET_ID,
        cutoff: CUTOFF,
        accepted_input_root: ROOT,
        slots: [Slot::Vacant; SLOTS],
        boundary: Boundary::SATISFIED,
        mode: Mode::Clear,
    }
}

/// Decode one slot letter.
///
/// Letter 0 is vacancy; letter `1 + ((side * TICKS) + tick) * ceiling + (q - 1)`
/// is the corresponding order, given canonical admission fields.
pub fn decode_slot(letter: u32, index: usize, ceiling: u32, owner: u8, surplus: u64) -> Slot {
    if letter == 0 {
        return Slot::Vacant;
    }
    let letter = letter - 1;
    let quantity = letter % ceiling + 1;
    let rest = letter / ceiling;
    let limit_index = (rest % TICKS as u32) as u8;
    let direction = if rest / TICKS as u32 == 0 {
        Direction::Buy
    } else {
        Direction::Sell
    };
    Slot::Taken(Order {
        batch: BATCH_ID,
        market: MARKET_ID,
        owner,
        direction,
        limit_index,
        quantity,
        reserved: required_reservation(direction, limit_index, quantity) + surplus,
        nullifier: index as u64 + 1,
        arrival: CUTOFF,
        authorized: true,
        eligible: true,
        included: true,
        custody_bound: true,
    })
}

/// Decode a whole book from a mixed-radix index.
pub fn decode_book(
    mut code: u64,
    ceiling: u32,
    owners: [u8; SLOTS],
    surplus: [u64; SLOTS],
) -> Batch {
    let radix = u64::from(alphabet(ceiling));
    let mut batch = empty_batch();
    for index in 0..SLOTS {
        let letter = (code % radix) as u32;
        code /= radix;
        batch.slots[index] = decode_slot(letter, index, ceiling, owners[index], surplus[index]);
    }
    batch
}

/// The full quantity domain of the relation.
pub const FULL_CEILING: u32 = QUANTITY_CEILING;

/// One named perturbation of an admissible batch.
///
/// Batch-level perturbations ignore the slot argument.
#[derive(Clone, Copy)]
pub struct Perturbation {
    /// Human-readable label used in divergence reports.
    pub label: &'static str,
    /// The edit itself, applied to the given slot index.
    pub apply: fn(&mut Batch, usize),
}

fn edit(batch: &mut Batch, index: usize, edit: impl FnOnce(&mut Order)) {
    if let Slot::Taken(ref mut order) = batch.slots[index] {
        edit(order);
    }
}

macro_rules! perturbations {
    ($($label:literal => $body:expr),* $(,)?) => {
        [$(Perturbation { label: $label, apply: $body }),*]
    };
}

/// Every batch-level perturbation. The slot argument is ignored.
pub const BATCH_PERTURBATIONS: [Perturbation; 6] = perturbations![
    "boundary.log-not-final" => |b: &mut Batch, _| b.boundary.log_final = false,
    "boundary.root-unbound" => |b: &mut Batch, _| b.boundary.root_binds_slots = false,
    "boundary.equivocation" => |b: &mut Batch, _| b.boundary.no_conflicting_root = false,
    "boundary.unavailable" => |b: &mut Batch, _| b.boundary.payloads_available = false,
    "mode.shielded" => |b: &mut Batch, _| b.mode = Mode::ShieldedSingleExecutor,
    "mode.dark" => |b: &mut Batch, _| b.mode = Mode::DarkTarget,
];

/// Every per-slot perturbation. A perturbation of a vacant slot is a no-op.
pub const SLOT_PERTURBATIONS: [Perturbation; 19] = perturbations![
    "wrong-batch" => |b: &mut Batch, i| edit(b, i, |o| o.batch += 1),
    "wrong-market" => |b: &mut Batch, i| edit(b, i, |o| o.market += 1),
    "owner-4" => |b: &mut Batch, i| edit(b, i, |o| o.owner = 4),
    "owner-200" => |b: &mut Batch, i| edit(b, i, |o| o.owner = 200),
    "limit-4" => |b: &mut Batch, i| edit(b, i, |o| o.limit_index = 4),
    "limit-255" => |b: &mut Batch, i| edit(b, i, |o| o.limit_index = 255),
    "quantity-0" => |b: &mut Batch, i| edit(b, i, |o| o.quantity = 0),
    "quantity-16" => |b: &mut Batch, i| edit(b, i, |o| o.quantity = 16),
    "quantity-u32-max" => |b: &mut Batch, i| edit(b, i, |o| o.quantity = u32::MAX),
    "late-by-one" => |b: &mut Batch, i| edit(b, i, |o| o.arrival = CUTOFF + 1),
    "unauthorized" => |b: &mut Batch, i| edit(b, i, |o| o.authorized = false),
    "ineligible" => |b: &mut Batch, i| edit(b, i, |o| o.eligible = false),
    "not-included" => |b: &mut Batch, i| edit(b, i, |o| o.included = false),
    "custody-unbound" => |b: &mut Batch, i| edit(b, i, |o| o.custody_bound = false),
    "nullifier-zero" => |b: &mut Batch, i| edit(b, i, |o| o.nullifier = 0),
    "nullifier-dup-slot0" => |b: &mut Batch, i| edit(b, i, |o| o.nullifier = 1),
    "nullifier-dup-slot3" => |b: &mut Batch, i| edit(b, i, |o| o.nullifier = 4),
    "reserved-short-by-one" => |b: &mut Batch, i| edit(b, i, |o| o.reserved = o.reserved.saturating_sub(1)),
    "reserved-zero" => |b: &mut Batch, i| edit(b, i, |o| o.reserved = 0),
];
