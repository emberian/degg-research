//! Shared fixture construction for the spec-derived test suite.
#![allow(dead_code)]

use degg_batch_oracle::admit::required_reservation;
use degg_batch_oracle::book::{Batch, Boundary, Direction, Mode, Order, Slot};
use degg_batch_oracle::params::SLOTS;

pub const BATCH_ID: u64 = 7;
pub const MARKET_ID: u64 = 11;
pub const CUTOFF: u64 = 1_000;

pub const ROOT: [u8; 32] = [0xA5; 32];

/// Slot description: owner, side, limit tick, quantity.
pub type SlotSpec = (u8, Direction, u8, u32);

/// A canonically valid occupied slot: exact reservation, on time, all
/// statements present, nullifier derived from the slot index.
pub fn order(index: usize, spec: SlotSpec) -> Order {
    let (owner, direction, limit_index, quantity) = spec;
    Order {
        batch: BATCH_ID,
        market: MARKET_ID,
        owner,
        direction,
        limit_index,
        quantity,
        reserved: required_reservation(direction, limit_index, quantity),
        nullifier: index as u64 + 1,
        arrival: CUTOFF,
        authorized: true,
        eligible: true,
        included: true,
        custody_bound: true,
    }
}

/// Build a batch from up to four occupied slot descriptions.
pub fn batch(specs: &[SlotSpec]) -> Batch {
    assert!(specs.len() <= SLOTS);
    let mut slots = [Slot::Vacant; SLOTS];
    for (index, spec) in specs.iter().enumerate() {
        slots[index] = Slot::Taken(order(index, *spec));
    }
    Batch {
        batch: BATCH_ID,
        market: MARKET_ID,
        cutoff: CUTOFF,
        accepted_input_root: ROOT,
        slots,
        boundary: Boundary::SATISFIED,
        mode: Mode::Clear,
    }
}

/// Mutate one occupied slot of an existing batch.
pub fn tweak(mut base: Batch, index: usize, edit: impl FnOnce(&mut Order)) -> Batch {
    let Slot::Taken(mut order) = base.slots[index] else {
        panic!("slot {index} is vacant");
    };
    edit(&mut order);
    base.slots[index] = Slot::Taken(order);
    base
}
