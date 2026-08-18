//! Translation between the two oracles' input types, and the comparison of
//! their outputs.
//!
//! Nothing here evaluates anything. It only restates one batch in the other
//! crate's vocabulary and then compares two complete results field by field, so
//! that a divergence is always attributable to the oracles and never to the
//! harness.

use dark_fba_toy as toy;
use degg_batch_oracle as mine;

use mine::book::{Batch, Direction, Mode, Slot};

/// Restate one of my batches as the existing toy's batch.
///
/// Every witness field maps across one to one, including the custody-binding
/// statement (`Order::custody_bound` here, `ToyAdmissionWitness::
/// reservation_bound` there). Until 2026-08-18 the independent oracle did not
/// model that statement and the harness supplied it as present; it is now
/// modelled, mapped, and perturbed, so no admission statement is out of scope.
pub fn to_toy(batch: &Batch) -> toy::ToyBatch {
    let context = toy::PublicContext {
        batch_id: batch.batch,
        market_id: batch.market,
        cutoff: batch.cutoff,
        accepted_input_root: batch.accepted_input_root,
    };
    let mut slots = [toy::Slot::Empty; toy::SLOT_COUNT];
    for (index, slot) in batch.slots.iter().enumerate() {
        slots[index] = match slot {
            Slot::Vacant => toy::Slot::Empty,
            Slot::Taken(order) => toy::Slot::Order(toy::ToyOrder {
                batch_id: order.batch,
                market_id: order.market,
                owner: order.owner,
                side: match order.direction {
                    Direction::Buy => toy::Side::Buy,
                    Direction::Sell => toy::Side::Sell,
                },
                limit_tick: order.limit_index,
                quantity: u64::from(order.quantity),
                reserved: order.reserved,
                nullifier: order.nullifier,
                arrived_at: order.arrival,
                admission: toy::ToyAdmissionWitness {
                    authorized: order.authorized,
                    eligible: order.eligible,
                    included_under_root: order.included,
                    reservation_bound: order.custody_bound,
                },
            }),
        };
    }
    toy::ToyBatch {
        requested_mode: match batch.mode {
            Mode::Clear => toy::ExecutionMode::Clear,
            Mode::ShieldedSingleExecutor => toy::ExecutionMode::ShieldedSingleExecutor,
            Mode::DarkTarget => toy::ExecutionMode::DarkTarget,
        },
        context,
        boundary: toy::ToyBoundaryWitness {
            admission_log_finalized: batch.boundary.log_final,
            root_matches_slots: batch.boundary.root_binds_slots,
            non_equivocation_certificate_present: batch.boundary.no_conflicting_root,
            all_payloads_available_by_cutoff: batch.boundary.payloads_available,
        },
        slots,
    }
}

/// The refusal class the toy is expected to report for one of my classes.
///
/// This is a declared vocabulary map between two independently chosen
/// taxonomies, fixed before the differential ran.
pub fn expected_toy_refusal(refusal: mine::admit::Refusal, batch: &Batch) -> Option<toy::Refusal> {
    use mine::admit::Refusal as Mine;
    use toy::BoundaryRefusal as Boundary;
    use toy::OrderRefusal as Order;
    let order = |slot: u8, reason| Some(toy::Refusal::Order { slot, reason });
    match refusal {
        Mine::DarkTargetUnavailable => Some(toy::Refusal::DarkBackendAbsent),
        Mine::AdmissionLogNotFinal => Some(toy::Refusal::Boundary(Boundary::AdmissionLogNotFinal)),
        Mine::RootBindingAbsent => Some(toy::Refusal::Boundary(Boundary::RootNotBoundToSlots)),
        Mine::RootEquivocation => Some(toy::Refusal::Boundary(Boundary::NonEquivocationAbsent)),
        Mine::PayloadUnavailable => Some(toy::Refusal::Boundary(Boundary::PayloadUnavailable)),
        Mine::BatchBindingMismatch { slot } => order(slot, Order::WrongBatch),
        Mine::MarketBindingMismatch { slot } => order(slot, Order::WrongMarket),
        Mine::OwnerOutOfDomain { slot } => order(slot, Order::OwnerOutOfRange),
        Mine::LimitOutOfDomain { slot } => order(slot, Order::LimitOutOfRange),
        Mine::QuantityOutOfDomain { slot } => order(slot, Order::QuantityOutOfRange),
        Mine::LateArrival { slot } => order(slot, Order::ArrivedAfterCutoff),
        Mine::Unauthorized { slot } => order(slot, Order::Unauthorized),
        Mine::Ineligible { slot } => order(slot, Order::Ineligible),
        Mine::InclusionAbsent { slot } => order(slot, Order::MissingInclusion),
        Mine::CustodyBindingAbsent { slot } => order(slot, Order::ReservationNotBound),
        Mine::NullifierZero { slot } => order(slot, Order::ZeroNullifier),
        Mine::NullifierRepeated { slot, first } => {
            order(slot, Order::DuplicateNullifier { first_slot: first })
        }
        Mine::ReservationInsufficient { slot } => {
            let Slot::Taken(taken) = batch.slots[usize::from(slot)] else {
                return None;
            };
            order(
                slot,
                Order::InsufficientReservation {
                    required: mine::admit::required_reservation(
                        taken.direction,
                        taken.limit_index,
                        taken.quantity,
                    ),
                    supplied: taken.reserved,
                },
            )
        }
        Mine::AccumulatorOverflow => Some(toy::Refusal::ArithmeticOverflow),
    }
}

/// How two complete results differ, if they differ at all.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Divergence {
    /// One oracle settled the batch and the other refused it.
    AcceptanceDisagreement,
    /// Both settled, but the selected tick or no-trade tag differs.
    ClearingTick,
    /// Both settled at the same tick, but the public volume differs.
    Volume,
    /// Both settled with the same public result, but an allocation differs.
    Allocation,
    /// Both settled with the same allocation, but an owner-local output differs.
    OwnerOutput,
    /// Both settled, but the executed-mode label differs.
    ExecutedMode,
    /// Both settled, but a preserved public field differs.
    PublicBinding,
    /// Both refused, but not with corresponding classes.
    RefusalClass,
}

/// Per-slot fills as recovered from the toy's owner-local outputs.
///
/// The toy exposes no public accessor for its fill vector, so the harness
/// reassembles it from `order_fills`, which is the same information an owner
/// would receive.
fn toy_fills(execution: &toy::ToyExecution) -> [u64; toy::SLOT_COUNT] {
    let mut fills = [0u64; toy::SLOT_COUNT];
    for owner in 0..toy::OWNER_COUNT as u8 {
        let Some(output) = execution.owner_output(owner) else {
            continue;
        };
        for fill in output.order_fills.iter().flatten() {
            fills[usize::from(fill.slot)] = fill.quantity;
        }
    }
    fills
}

/// Compare two complete outcomes for the same batch.
pub fn compare(
    batch: &Batch,
    ours: &mine::Outcome,
    theirs: &Result<toy::ToyExecution, toy::Refusal>,
) -> Option<Divergence> {
    let (settlement, execution) = match (ours, theirs) {
        (mine::Outcome::Refused(our_class), Err(their_class)) => {
            return match expected_toy_refusal(*our_class, batch) {
                Some(expected) if expected == *their_class => None,
                _ => Some(Divergence::RefusalClass),
            };
        }
        (mine::Outcome::Settled(settlement), Ok(execution)) => (settlement, execution),
        _ => return Some(Divergence::AcceptanceDisagreement),
    };

    let public = execution.public_result();
    let (our_tick, our_volume) = match settlement.clearing {
        mine::curve::Clearing::NoTrade => (toy::PublicTick::NoTrade, 0u64),
        mine::curve::Clearing::Trade { tick, volume, .. } => {
            (toy::PublicTick::Tick(tick), u64::from(volume))
        }
    };
    if our_tick != public.tick {
        return Some(Divergence::ClearingTick);
    }
    if our_volume != public.volume {
        return Some(Divergence::Volume);
    }
    if public.relation_id != mine::params::RELATION
        || public.batch_id != batch.batch
        || public.market_id != batch.market
        || public.accepted_input_root != batch.accepted_input_root
    {
        return Some(Divergence::PublicBinding);
    }

    let expected_mode = match batch.mode {
        Mode::Clear => toy::ExecutedMode::Clear,
        Mode::ShieldedSingleExecutor => toy::ExecutedMode::ShieldedSingleExecutor,
        Mode::DarkTarget => return Some(Divergence::ExecutedMode),
    };
    if execution.executed_mode() != expected_mode {
        return Some(Divergence::ExecutedMode);
    }

    let fills = toy_fills(execution);
    for (slot, fill) in fills.iter().enumerate() {
        if u64::from(settlement.fills[slot]) != *fill {
            return Some(Divergence::Allocation);
        }
    }

    for owner in 0..toy::OWNER_COUNT {
        let Some(output) = execution.owner_output(owner as u8) else {
            return Some(Divergence::OwnerOutput);
        };
        let ours = settlement.owners[owner];
        if u64::from(ours.bought_base) != output.bought
            || u64::from(ours.sold_base) != output.sold
            || ours.base_delta != output.base_delta
            || ours.quote_delta != output.quote_delta
            || ours.released_base_reservation != output.released_base_reservation
            || ours.released_quote_reservation != output.released_quote_reservation
        {
            return Some(Divergence::OwnerOutput);
        }
    }

    None
}

/// Short stable tag for one of the toy's refusal classes.
///
/// The tags are the shared vocabulary of `DARK_FBA_RELATION.md` section 4.1:
/// each is the rule identity, not either implementation's own spelling.
pub fn toy_class(refusal: &toy::Refusal) -> &'static str {
    use toy::BoundaryRefusal as Boundary;
    use toy::OrderRefusal as Order;
    match refusal {
        toy::Refusal::DarkBackendAbsent => "dark-target-unavailable",
        toy::Refusal::Boundary(Boundary::AdmissionLogNotFinal) => "admission-log-not-final",
        toy::Refusal::Boundary(Boundary::RootNotBoundToSlots) => "root-binding-absent",
        toy::Refusal::Boundary(Boundary::NonEquivocationAbsent) => "root-equivocation",
        toy::Refusal::Boundary(Boundary::PayloadUnavailable) => "payload-unavailable",
        toy::Refusal::Order { reason, .. } => match reason {
            Order::WrongBatch => "batch-binding-mismatch",
            Order::WrongMarket => "market-binding-mismatch",
            Order::OwnerOutOfRange => "owner-out-of-domain",
            Order::QuantityOutOfRange => "quantity-out-of-domain",
            Order::LimitOutOfRange => "limit-out-of-domain",
            Order::ZeroNullifier => "nullifier-zero",
            Order::DuplicateNullifier { .. } => "nullifier-repeated",
            Order::ArrivedAfterCutoff => "late-arrival",
            Order::Unauthorized => "unauthorized",
            Order::Ineligible => "ineligible",
            Order::MissingInclusion => "inclusion-absent",
            Order::ReservationNotBound => "custody-binding-absent",
            Order::InsufficientReservation { .. } => "reservation-insufficient",
        },
        toy::Refusal::ArithmeticOverflow => "accumulator-overflow",
        toy::Refusal::InternalInvariant => "internal-invariant",
    }
}

/// The slot a refusal points at, if any.
pub fn toy_slot(refusal: &toy::Refusal) -> Option<u8> {
    match refusal {
        toy::Refusal::Order { slot, .. } => Some(*slot),
        _ => None,
    }
}

/// Every admission rule the batch actually violates, checked independently and
/// exhaustively rather than first-failure.
///
/// This is a third, deliberately naive re-derivation of the admission
/// predicate, used only to judge whether a refusal-class disagreement is a
/// disagreement about check priority (both reported rules really are violated)
/// or a genuine one (an oracle reports a rule the witness does not violate).
pub fn violations(batch: &Batch) -> Vec<(String, Option<u8>)> {
    use mine::params::{OWNERS, QUANTITY_CEILING, QUANTITY_FLOOR, TICKS};
    let mut found = Vec::new();
    if batch.mode == Mode::DarkTarget {
        found.push(("dark-target-unavailable".to_owned(), None));
    }
    if !batch.boundary.log_final {
        found.push(("admission-log-not-final".to_owned(), None));
    }
    if !batch.boundary.root_binds_slots {
        found.push(("root-binding-absent".to_owned(), None));
    }
    if !batch.boundary.no_conflicting_root {
        found.push(("root-equivocation".to_owned(), None));
    }
    if !batch.boundary.payloads_available {
        found.push(("payload-unavailable".to_owned(), None));
    }
    for (index, slot) in batch.slots.iter().enumerate() {
        let Slot::Taken(order) = slot else { continue };
        let at = Some(index as u8);
        let mut note = |tag: &str| found.push((tag.to_owned(), at));
        if order.batch != batch.batch {
            note("batch-binding-mismatch");
        }
        if order.market != batch.market {
            note("market-binding-mismatch");
        }
        if order.owner >= OWNERS {
            note("owner-out-of-domain");
        }
        if usize::from(order.limit_index) >= TICKS {
            note("limit-out-of-domain");
        }
        if order.quantity < QUANTITY_FLOOR || order.quantity > QUANTITY_CEILING {
            note("quantity-out-of-domain");
        }
        if order.arrival > batch.cutoff {
            note("late-arrival");
        }
        if !order.authorized {
            note("unauthorized");
        }
        if !order.eligible {
            note("ineligible");
        }
        if !order.included {
            note("inclusion-absent");
        }
        if !order.custody_bound {
            note("custody-binding-absent");
        }
        if order.nullifier == 0 {
            note("nullifier-zero");
        }
        if usize::from(order.limit_index) < TICKS {
            let required = mine::admit::required_reservation(
                order.direction,
                order.limit_index,
                order.quantity,
            );
            if order.reserved < required {
                note("reservation-insufficient");
            }
        }
        for (earlier, other) in batch.slots.iter().enumerate().take(index) {
            let Slot::Taken(other) = other else { continue };
            if other.nullifier == order.nullifier {
                found.push(("nullifier-repeated".to_owned(), at));
                let _ = earlier;
                break;
            }
        }
    }
    found
}
