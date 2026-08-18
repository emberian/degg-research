//! Named batch fixtures.
//!
//! The first three restate the reference oracle's published vector fixtures
//! (`experiments/dark-fba/vectors/v1.txt`), so the golden digests below name
//! the same semantic cases the anchor corpus pins. The last two are the
//! differential's minimal check-priority witnesses from
//! `docs/research/DARK_FBA_RELATION.md` section 13.

use crate::batch::{BatchInput, BoundaryStatements, OrderWitness, RequestedMode, Side, SlotInput};

fn order(
    context: (u64, u64, u64),
    owner: u8,
    side: Side,
    limit_tick: u8,
    quantity: u64,
    reserved: u64,
    nullifier: u64,
) -> SlotInput {
    let (batch_id, market_id, cutoff) = context;
    SlotInput::Occupied(OrderWitness {
        batch_id,
        market_id,
        owner,
        side,
        limit_tick,
        quantity,
        reserved,
        nullifier,
        arrived_at: cutoff,
        authorized: true,
        eligible: true,
        included_under_root: true,
        custody_bound: true,
    })
}

fn batch(root_byte: u8, mode: RequestedMode, slots: Vec<SlotInput>) -> BatchInput {
    BatchInput {
        requested_mode: mode,
        batch_id: 7,
        market_id: 9,
        cutoff: 10,
        accepted_input_root: [root_byte; 32],
        boundary: BoundaryStatements::SATISFIED,
        slots,
    }
}

const CONTEXT: (u64, u64, u64) = (7, 9, 10);

/// A balanced crossing that exercises pro-rata residual ties:
/// tick 2, volume 5, fills `[5, 0, 3, 2]`.
pub fn balanced_residual() -> BatchInput {
    batch(
        0x11,
        RequestedMode::ShieldedSingleExecutor,
        vec![
            order(CONTEXT, 0, Side::Buy, 2, 5, 15, 101),
            order(CONTEXT, 1, Side::Buy, 1, 3, 6, 102),
            order(CONTEXT, 2, Side::Sell, 0, 4, 4, 103),
            order(CONTEXT, 3, Side::Sell, 2, 4, 4, 104),
        ],
    )
}

/// A maximum-volume tie that must select the lowest tick: tick 1, volume 4.
pub fn price_tie_low() -> BatchInput {
    batch(
        0x22,
        RequestedMode::Clear,
        vec![
            order(CONTEXT, 0, Side::Buy, 2, 4, 12, 201),
            order(CONTEXT, 1, Side::Sell, 1, 4, 4, 202),
            SlotInput::Empty,
            SlotInput::Empty,
        ],
    )
}

/// A non-crossing book: a valid no-trade, not a refusal.
pub fn no_trade() -> BatchInput {
    batch(
        0x33,
        RequestedMode::Clear,
        vec![
            order(CONTEXT, 0, Side::Buy, 0, 2, 2, 301),
            order(CONTEXT, 1, Side::Sell, 3, 2, 2, 302),
            SlotInput::Empty,
            SlotInput::Empty,
        ],
    )
}

/// The tie book with a `DarkTarget` request: refused before any witness is
/// inspected.
pub fn dark_target_request() -> BatchInput {
    let mut fixture = price_tie_low();
    fixture.requested_mode = RequestedMode::DarkTarget;
    fixture
}

/// The tie book with slot 1 repeating slot 0's nullifier.
pub fn duplicate_nullifier() -> BatchInput {
    let mut fixture = price_tie_low();
    let SlotInput::Occupied(ref mut second) = fixture.slots[1] else {
        unreachable!("fixture slot 1 is occupied");
    };
    second.nullifier = 201;
    fixture
}

/// Section 13 minimal witness 1: one slot violating both the quantity and the
/// limit domain. The frozen order reports `quantity-out-of-domain`; the
/// rejected alternative order reports `limit-out-of-domain`.
pub fn witness_quantity_vs_limit() -> BatchInput {
    batch(
        0x44,
        RequestedMode::Clear,
        vec![
            order(CONTEXT, 0, Side::Buy, 4, 0, 0, 1),
            SlotInput::Empty,
            SlotInput::Empty,
            SlotInput::Empty,
        ],
    )
}

/// Section 13 minimal witness 2: slot 1 both repeats slot 0's nullifier and
/// under-reserves. The frozen order reports `nullifier-repeated`; the rejected
/// alternative order reports `reservation-insufficient`.
pub fn witness_nullifier_vs_reservation() -> BatchInput {
    batch(
        0x55,
        RequestedMode::Clear,
        vec![
            order(CONTEXT, 0, Side::Buy, 0, 1, 1, 1),
            order(CONTEXT, 0, Side::Buy, 0, 1, 0, 1),
            SlotInput::Empty,
            SlotInput::Empty,
        ],
    )
}
