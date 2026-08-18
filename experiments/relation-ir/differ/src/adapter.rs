//! Translation between the IR batch type and the two oracles' input types, and
//! the comparison of complete outputs.
//!
//! Nothing here evaluates anything: it restates one IR batch in each oracle's
//! vocabulary and compares three complete results field by field, so a
//! divergence is attributable to the implementations, never to the harness.
//!
//! One asymmetry, mirrored from the earlier differential: this harness always
//! constructs the oracles' custody-binding statement from the IR witness's own
//! `custody_bound` field, which the enumerated domains and perturbations
//! control explicitly.

use dark_fba_toy as toy;
use degg_batch_oracle as oracle;
use degg_relation_ir::batch::{BatchInput, RequestedMode, Side, SlotInput};
use degg_relation_ir::lower::{ClearedTick, Outcome, OwnerOutput};
use degg_relation_ir::refusal::{Refusal, RefusalClass};

/// Restate one IR batch as the reference toy's batch.
pub fn to_toy(batch: &BatchInput) -> toy::ToyBatch {
    let mut slots = [toy::Slot::Empty; toy::SLOT_COUNT];
    for (index, slot) in batch.slots.iter().enumerate() {
        slots[index] = match slot {
            SlotInput::Empty => toy::Slot::Empty,
            SlotInput::Occupied(order) => toy::Slot::Order(toy::ToyOrder {
                batch_id: order.batch_id,
                market_id: order.market_id,
                owner: order.owner,
                side: match order.side {
                    Side::Buy => toy::Side::Buy,
                    Side::Sell => toy::Side::Sell,
                },
                limit_tick: order.limit_tick,
                quantity: order.quantity,
                reserved: order.reserved,
                nullifier: order.nullifier,
                arrived_at: order.arrived_at,
                admission: toy::ToyAdmissionWitness {
                    authorized: order.authorized,
                    eligible: order.eligible,
                    included_under_root: order.included_under_root,
                    reservation_bound: order.custody_bound,
                },
            }),
        };
    }
    toy::ToyBatch {
        requested_mode: match batch.requested_mode {
            RequestedMode::Clear => toy::ExecutionMode::Clear,
            RequestedMode::ShieldedSingleExecutor => toy::ExecutionMode::ShieldedSingleExecutor,
            RequestedMode::DarkTarget => toy::ExecutionMode::DarkTarget,
        },
        context: toy::PublicContext {
            batch_id: batch.batch_id,
            market_id: batch.market_id,
            cutoff: batch.cutoff,
            accepted_input_root: batch.accepted_input_root,
        },
        boundary: toy::ToyBoundaryWitness {
            admission_log_finalized: batch.boundary.log_final,
            root_matches_slots: batch.boundary.root_binds_slots,
            non_equivocation_certificate_present: batch.boundary.no_conflicting_root,
            all_payloads_available_by_cutoff: batch.boundary.payloads_available,
        },
        slots,
    }
}

/// Restate one IR batch as the independent oracle's batch.
///
/// The oracle's quantity is `u32`; the enumerated domains stay inside that
/// width so the restatement is exact.
pub fn to_oracle(batch: &BatchInput) -> oracle::book::Batch {
    let mut slots = [oracle::book::Slot::Vacant; 4];
    for (index, slot) in batch.slots.iter().enumerate() {
        slots[index] = match slot {
            SlotInput::Empty => oracle::book::Slot::Vacant,
            SlotInput::Occupied(order) => oracle::book::Slot::Taken(oracle::book::Order {
                batch: order.batch_id,
                market: order.market_id,
                owner: order.owner,
                direction: match order.side {
                    Side::Buy => oracle::book::Direction::Buy,
                    Side::Sell => oracle::book::Direction::Sell,
                },
                limit_index: order.limit_tick,
                quantity: u32::try_from(order.quantity)
                    .expect("differential domains stay within u32 quantities"),
                reserved: order.reserved,
                nullifier: order.nullifier,
                arrival: order.arrived_at,
                authorized: order.authorized,
                eligible: order.eligible,
                included: order.included_under_root,
                custody_bound: order.custody_bound,
            }),
        };
    }
    oracle::book::Batch {
        batch: batch.batch_id,
        market: batch.market_id,
        cutoff: batch.cutoff,
        accepted_input_root: batch.accepted_input_root,
        slots,
        boundary: oracle::book::Boundary {
            log_final: batch.boundary.log_final,
            root_binds_slots: batch.boundary.root_binds_slots,
            no_conflicting_root: batch.boundary.no_conflicting_root,
            payloads_available: batch.boundary.payloads_available,
        },
        mode: match batch.requested_mode {
            RequestedMode::Clear => oracle::book::Mode::Clear,
            RequestedMode::ShieldedSingleExecutor => oracle::book::Mode::ShieldedSingleExecutor,
            RequestedMode::DarkTarget => oracle::book::Mode::DarkTarget,
        },
    }
}

/// The toy refusal an IR refusal must correspond to: a declared one-to-one
/// vocabulary map, including diagnostics.
pub fn expected_toy_refusal(refusal: Refusal, batch: &BatchInput) -> Option<toy::Refusal> {
    use toy::BoundaryRefusal as Boundary;
    use toy::OrderRefusal as Order;
    let slot = refusal.slot;
    let order = |reason| {
        Some(toy::Refusal::Order {
            slot: slot.expect("per-slot class carries a slot"),
            reason,
        })
    };
    match refusal.class {
        RefusalClass::DarkTargetUnavailable => Some(toy::Refusal::DarkBackendAbsent),
        RefusalClass::AdmissionLogNotFinal => {
            Some(toy::Refusal::Boundary(Boundary::AdmissionLogNotFinal))
        }
        RefusalClass::RootBindingAbsent => {
            Some(toy::Refusal::Boundary(Boundary::RootNotBoundToSlots))
        }
        RefusalClass::RootEquivocation => {
            Some(toy::Refusal::Boundary(Boundary::NonEquivocationAbsent))
        }
        RefusalClass::PayloadUnavailable => {
            Some(toy::Refusal::Boundary(Boundary::PayloadUnavailable))
        }
        RefusalClass::BatchBindingMismatch => order(Order::WrongBatch),
        RefusalClass::MarketBindingMismatch => order(Order::WrongMarket),
        RefusalClass::OwnerOutOfDomain => order(Order::OwnerOutOfRange),
        RefusalClass::QuantityOutOfDomain => order(Order::QuantityOutOfRange),
        RefusalClass::LimitOutOfDomain => order(Order::LimitOutOfRange),
        RefusalClass::NullifierZero => order(Order::ZeroNullifier),
        RefusalClass::NullifierRepeated => order(Order::DuplicateNullifier {
            first_slot: refusal.first_slot.expect("repeat carries the first use"),
        }),
        RefusalClass::LateArrival => order(Order::ArrivedAfterCutoff),
        RefusalClass::Unauthorized => order(Order::Unauthorized),
        RefusalClass::Ineligible => order(Order::Ineligible),
        RefusalClass::InclusionAbsent => order(Order::MissingInclusion),
        RefusalClass::CustodyBindingAbsent => order(Order::ReservationNotBound),
        RefusalClass::ReservationInsufficient => {
            let SlotInput::Occupied(witness) =
                batch.slots[usize::from(slot.expect("per-slot class carries a slot"))]
            else {
                return None;
            };
            let required = degg_relation_ir::lower::required_reservation(
                &toy::PRICE_TICKS,
                witness.side,
                witness.limit_tick,
                witness.quantity,
            )?;
            order(Order::InsufficientReservation {
                required,
                supplied: witness.reserved,
            })
        }
        RefusalClass::AccumulatorOverflow => Some(toy::Refusal::ArithmeticOverflow),
        RefusalClass::InternalInvariant => Some(toy::Refusal::InternalInvariant),
        RefusalClass::MalformedEncoding => None,
    }
}

/// One normalized owner-local output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NormOwner {
    /// Base atoms received.
    pub bought: u64,
    /// Base atoms delivered.
    pub sold: u64,
    /// Signed base delta.
    pub base_delta: i64,
    /// Signed quote delta.
    pub quote_delta: i64,
    /// Base reservation returned.
    pub released_base: u64,
    /// Quote reservation returned.
    pub released_quote: u64,
}

/// One normalized settled result, complete enough that agreement covers every
/// public and every owner-local output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NormSettled {
    /// Selected tick index or `None` for no trade.
    pub tick: Option<u8>,
    /// Aggregate matched volume.
    pub volume: u64,
    /// Per-slot fills.
    pub fills: [u64; 4],
    /// Owner-local outputs, indexed by owner.
    pub owners: [NormOwner; 4],
}

fn norm_ir_owner(owner: &OwnerOutput) -> NormOwner {
    NormOwner {
        bought: owner.bought,
        sold: owner.sold,
        base_delta: owner.base_delta,
        quote_delta: owner.quote_delta,
        released_base: owner.released_base_reservation,
        released_quote: owner.released_quote_reservation,
    }
}

/// Normalize the IR outcome.
pub fn norm_ir(outcome: &Outcome) -> Result<NormSettled, Refusal> {
    match outcome {
        Outcome::Refused(refused) => Err(refused.refusal),
        Outcome::Settled(settled) => {
            let mut fills = [0u64; 4];
            fills.copy_from_slice(&settled.fills);
            let mut owners = [NormOwner {
                bought: 0,
                sold: 0,
                base_delta: 0,
                quote_delta: 0,
                released_base: 0,
                released_quote: 0,
            }; 4];
            for (norm, owner) in owners.iter_mut().zip(&settled.owners) {
                *norm = norm_ir_owner(owner);
            }
            Ok(NormSettled {
                tick: match settled.public.tick {
                    ClearedTick::NoTrade => None,
                    ClearedTick::Tick(tick) => Some(tick),
                },
                volume: settled.public.volume,
                fills,
                owners,
            })
        }
    }
}

/// Normalize the toy outcome. The toy exposes no fill vector, so it is
/// reassembled from the owner-local `order_fills`, the same information an
/// owner receives.
pub fn norm_toy(execution: &toy::ToyExecution) -> NormSettled {
    let mut fills = [0u64; 4];
    let mut owners = [NormOwner {
        bought: 0,
        sold: 0,
        base_delta: 0,
        quote_delta: 0,
        released_base: 0,
        released_quote: 0,
    }; 4];
    for owner_index in 0..4u8 {
        let owner = execution
            .owner_output(owner_index)
            .expect("owners 0..4 exist");
        owners[usize::from(owner_index)] = NormOwner {
            bought: owner.bought,
            sold: owner.sold,
            base_delta: owner.base_delta,
            quote_delta: owner.quote_delta,
            released_base: owner.released_base_reservation,
            released_quote: owner.released_quote_reservation,
        };
        for fill in owner.order_fills.iter().flatten() {
            fills[usize::from(fill.slot)] = fill.quantity;
        }
    }
    NormSettled {
        tick: match execution.public_result().tick {
            toy::PublicTick::NoTrade => None,
            toy::PublicTick::Tick(tick) => Some(tick),
        },
        volume: execution.public_result().volume,
        fills,
        owners,
    }
}

/// Normalize the independent oracle's outcome.
pub fn norm_oracle(settlement: &oracle::settle::Settlement) -> NormSettled {
    let mut fills = [0u64; 4];
    for (norm, fill) in fills.iter_mut().zip(settlement.fills) {
        *norm = u64::from(fill);
    }
    let mut owners = [NormOwner {
        bought: 0,
        sold: 0,
        base_delta: 0,
        quote_delta: 0,
        released_base: 0,
        released_quote: 0,
    }; 4];
    for (norm, owner) in owners.iter_mut().zip(&settlement.owners) {
        *norm = NormOwner {
            bought: u64::from(owner.bought_base),
            sold: u64::from(owner.sold_base),
            base_delta: owner.base_delta,
            quote_delta: owner.quote_delta,
            released_base: owner.released_base_reservation,
            released_quote: owner.released_quote_reservation,
        };
    }
    NormSettled {
        tick: match settlement.clearing {
            oracle::curve::Clearing::NoTrade => None,
            oracle::curve::Clearing::Trade { tick, .. } => Some(tick),
        },
        volume: match settlement.clearing {
            oracle::curve::Clearing::NoTrade => 0,
            oracle::curve::Clearing::Trade { volume, .. } => u64::from(volume),
        },
        fills,
        owners,
    }
}

/// Compare one batch's IR outcome against both oracles. `Some` is a divergence
/// description; `None` is complete agreement.
pub fn compare(batch: &BatchInput, ir: &Outcome) -> Option<String> {
    let toy_result = toy::evaluate(&to_toy(batch));
    let oracle_outcome = oracle::evaluate(&to_oracle(batch));

    match (norm_ir(ir), toy_result) {
        (Ok(mine), Ok(toy_execution)) => {
            let theirs = norm_toy(&toy_execution);
            if mine != theirs {
                return Some(format!("toy settled mismatch: {mine:?} vs {theirs:?}"));
            }
        }
        (Err(refusal), Err(toy_refusal)) => {
            let expected = expected_toy_refusal(refusal, batch);
            if expected != Some(toy_refusal) {
                return Some(format!(
                    "toy refusal mismatch: {refusal:?} maps to {expected:?}, toy said {toy_refusal:?}"
                ));
            }
        }
        (mine, theirs) => {
            return Some(format!("toy verdict mismatch: {mine:?} vs {theirs:?}"));
        }
    }

    match (norm_ir(ir), oracle_outcome) {
        (Ok(mine), oracle::Outcome::Settled(settlement)) => {
            let theirs = norm_oracle(&settlement);
            if mine != theirs {
                return Some(format!("oracle settled mismatch: {mine:?} vs {theirs:?}"));
            }
        }
        (Err(refusal), oracle::Outcome::Refused(oracle_refusal)) => {
            if refusal.class.tag() != oracle_refusal.class() {
                return Some(format!(
                    "oracle refusal class mismatch: {} vs {}",
                    refusal.class.tag(),
                    oracle_refusal.class()
                ));
            }
        }
        (mine, theirs) => {
            return Some(format!("oracle verdict mismatch: {mine:?} vs {theirs:?}"));
        }
    }
    None
}
