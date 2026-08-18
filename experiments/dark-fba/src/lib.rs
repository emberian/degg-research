//! Offline semantic toy for `dark-fba/n4-k4-q15/v0`.
//!
//! This crate implements only bounded clear arithmetic. The
//! `ShieldedSingleExecutor` label records that the one process executing this
//! function sees the complete batch. `DarkTarget` is deliberately refused.
//! Boolean admission/binding/availability fields are toy witnesses, not proofs.

#![forbid(unsafe_code)]

use core::fmt;

pub const RELATION_ID: &str = "dark-fba/n4-k4-q15/v0";
pub const OWNER_COUNT: usize = 4;
pub const SLOT_COUNT: usize = 4;
pub const TICK_COUNT: usize = 4;
pub const Q_MAX: u64 = 15;
pub const PRICE_TICKS: [u64; TICK_COUNT] = [1, 2, 3, 4];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutionMode {
    Clear,
    ShieldedSingleExecutor,
    DarkTarget,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutedMode {
    Clear,
    ShieldedSingleExecutor,
}

impl ExecutedMode {
    fn label(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::ShieldedSingleExecutor => "shielded-single-executor",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToyAdmissionWitness {
    pub authorized: bool,
    pub eligible: bool,
    pub included_under_root: bool,
    pub reservation_bound: bool,
}

impl ToyAdmissionWitness {
    pub const PASS: Self = Self {
        authorized: true,
        eligible: true,
        included_under_root: true,
        reservation_bound: true,
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToyOrder {
    pub batch_id: u64,
    pub market_id: u64,
    pub owner: u8,
    pub side: Side,
    pub limit_tick: u8,
    pub quantity: u64,
    /// Quote atoms for a buy; base atoms for a sell.
    pub reserved: u64,
    pub nullifier: u64,
    pub arrived_at: u64,
    pub admission: ToyAdmissionWitness,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slot {
    Empty,
    Order(ToyOrder),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PublicContext {
    pub batch_id: u64,
    pub market_id: u64,
    pub cutoff: u64,
    /// Opaque external label in this toy. It is not computed or opened here.
    pub accepted_input_root: [u8; 32],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToyBoundaryWitness {
    pub admission_log_finalized: bool,
    pub root_matches_slots: bool,
    pub non_equivocation_certificate_present: bool,
    pub all_payloads_available_by_cutoff: bool,
}

impl ToyBoundaryWitness {
    pub const PASS: Self = Self {
        admission_log_finalized: true,
        root_matches_slots: true,
        non_equivocation_certificate_present: true,
        all_payloads_available_by_cutoff: true,
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToyBatch {
    pub requested_mode: ExecutionMode,
    pub context: PublicContext,
    pub boundary: ToyBoundaryWitness,
    pub slots: [Slot; SLOT_COUNT],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PublicTick {
    NoTrade,
    Tick(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PublicResult {
    pub relation_id: &'static str,
    pub batch_id: u64,
    pub market_id: u64,
    pub accepted_input_root: [u8; 32],
    pub tick: PublicTick,
    pub volume: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocalFill {
    pub slot: u8,
    pub quantity: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OwnerOutput {
    pub owner: u8,
    pub bought: u64,
    pub sold: u64,
    pub base_delta: i64,
    pub quote_delta: i64,
    pub released_base_reservation: u64,
    pub released_quote_reservation: u64,
    pub order_fills: [Option<LocalFill>; SLOT_COUNT],
}

impl OwnerOutput {
    fn empty(owner: usize) -> Self {
        Self {
            owner: owner as u8,
            bought: 0,
            sold: 0,
            base_delta: 0,
            quote_delta: 0,
            released_base_reservation: 0,
            released_quote_reservation: 0,
            order_fills: [None; SLOT_COUNT],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToyExecution {
    executed_mode: ExecutedMode,
    public: PublicResult,
    fills: [u64; SLOT_COUNT],
    owners: [OwnerOutput; OWNER_COUNT],
}

impl ToyExecution {
    pub const fn executed_mode(&self) -> ExecutedMode {
        self.executed_mode
    }

    pub const fn public_result(&self) -> &PublicResult {
        &self.public
    }

    /// Models owner-local delivery, but supplies no authentication or access
    /// control. The caller is still the one Shielded executor.
    pub fn owner_output(&self, owner: u8) -> Option<&OwnerOutput> {
        self.owners.get(usize::from(owner))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundaryRefusal {
    AdmissionLogNotFinal,
    RootNotBoundToSlots,
    NonEquivocationAbsent,
    PayloadUnavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrderRefusal {
    WrongBatch,
    WrongMarket,
    OwnerOutOfRange,
    QuantityOutOfRange,
    LimitOutOfRange,
    ZeroNullifier,
    DuplicateNullifier { first_slot: u8 },
    ArrivedAfterCutoff,
    Unauthorized,
    Ineligible,
    MissingInclusion,
    ReservationNotBound,
    InsufficientReservation { required: u64, supplied: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    DarkBackendAbsent,
    Boundary(BoundaryRefusal),
    Order { slot: u8, reason: OrderRefusal },
    ArithmeticOverflow,
    InternalInvariant,
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DarkBackendAbsent => f.write_str("dark-backend-absent"),
            Self::Boundary(reason) => write!(f, "boundary:{reason:?}"),
            Self::Order { slot, reason } => write!(f, "order[{slot}]:{reason:?}"),
            Self::ArithmeticOverflow => f.write_str("arithmetic-overflow"),
            Self::InternalInvariant => f.write_str("internal-invariant"),
        }
    }
}

impl std::error::Error for Refusal {}

#[derive(Clone, Copy)]
struct Curves {
    demand: [u64; TICK_COUNT],
    supply: [u64; TICK_COUNT],
}

pub fn evaluate(batch: &ToyBatch) -> Result<ToyExecution, Refusal> {
    let executed_mode = match batch.requested_mode {
        ExecutionMode::Clear => ExecutedMode::Clear,
        ExecutionMode::ShieldedSingleExecutor => ExecutedMode::ShieldedSingleExecutor,
        ExecutionMode::DarkTarget => return Err(Refusal::DarkBackendAbsent),
    };

    validate_boundary(batch.boundary)?;
    validate_orders(batch)?;
    let curves = aggregate_curves(&batch.slots)?;
    let (tick, volume) = select_tick(curves)?;
    let fills = allocate(&batch.slots, tick, volume)?;
    let owners = owner_outputs(&batch.slots, tick, fills)?;
    verify_result(&batch.slots, tick, volume, fills, &owners)?;

    Ok(ToyExecution {
        executed_mode,
        public: PublicResult {
            relation_id: RELATION_ID,
            batch_id: batch.context.batch_id,
            market_id: batch.context.market_id,
            accepted_input_root: batch.context.accepted_input_root,
            tick,
            volume,
        },
        fills,
        owners,
    })
}

fn validate_boundary(boundary: ToyBoundaryWitness) -> Result<(), Refusal> {
    if !boundary.admission_log_finalized {
        return Err(Refusal::Boundary(BoundaryRefusal::AdmissionLogNotFinal));
    }
    if !boundary.root_matches_slots {
        return Err(Refusal::Boundary(BoundaryRefusal::RootNotBoundToSlots));
    }
    if !boundary.non_equivocation_certificate_present {
        return Err(Refusal::Boundary(BoundaryRefusal::NonEquivocationAbsent));
    }
    if !boundary.all_payloads_available_by_cutoff {
        return Err(Refusal::Boundary(BoundaryRefusal::PayloadUnavailable));
    }
    Ok(())
}

fn validate_orders(batch: &ToyBatch) -> Result<(), Refusal> {
    let mut seen: [Option<(u64, u8)>; SLOT_COUNT] = [None; SLOT_COUNT];
    let mut seen_len = 0usize;

    for (slot_index, slot) in batch.slots.iter().enumerate() {
        let Slot::Order(order) = slot else {
            continue;
        };
        let slot = slot_index as u8;
        let refuse = |reason| Refusal::Order { slot, reason };

        if order.batch_id != batch.context.batch_id {
            return Err(refuse(OrderRefusal::WrongBatch));
        }
        if order.market_id != batch.context.market_id {
            return Err(refuse(OrderRefusal::WrongMarket));
        }
        if usize::from(order.owner) >= OWNER_COUNT {
            return Err(refuse(OrderRefusal::OwnerOutOfRange));
        }
        if order.quantity == 0 || order.quantity > Q_MAX {
            return Err(refuse(OrderRefusal::QuantityOutOfRange));
        }
        if usize::from(order.limit_tick) >= TICK_COUNT {
            return Err(refuse(OrderRefusal::LimitOutOfRange));
        }
        if order.nullifier == 0 {
            return Err(refuse(OrderRefusal::ZeroNullifier));
        }
        if let Some((_, first_slot)) = seen[..seen_len]
            .iter()
            .flatten()
            .find(|(nullifier, _)| *nullifier == order.nullifier)
        {
            return Err(refuse(OrderRefusal::DuplicateNullifier {
                first_slot: *first_slot,
            }));
        }
        seen[seen_len] = Some((order.nullifier, slot));
        seen_len += 1;
        if order.arrived_at > batch.context.cutoff {
            return Err(refuse(OrderRefusal::ArrivedAfterCutoff));
        }
        if !order.admission.authorized {
            return Err(refuse(OrderRefusal::Unauthorized));
        }
        if !order.admission.eligible {
            return Err(refuse(OrderRefusal::Ineligible));
        }
        if !order.admission.included_under_root {
            return Err(refuse(OrderRefusal::MissingInclusion));
        }
        if !order.admission.reservation_bound {
            return Err(refuse(OrderRefusal::ReservationNotBound));
        }

        let required = match order.side {
            Side::Buy => order
                .quantity
                .checked_mul(PRICE_TICKS[usize::from(order.limit_tick)])
                .ok_or(Refusal::ArithmeticOverflow)?,
            Side::Sell => order.quantity,
        };
        if order.reserved < required {
            return Err(refuse(OrderRefusal::InsufficientReservation {
                required,
                supplied: order.reserved,
            }));
        }
    }
    Ok(())
}

fn aggregate_curves(slots: &[Slot; SLOT_COUNT]) -> Result<Curves, Refusal> {
    let mut curves = Curves {
        demand: [0; TICK_COUNT],
        supply: [0; TICK_COUNT],
    };
    for slot in slots {
        let Slot::Order(order) = slot else {
            continue;
        };
        for tick in 0..TICK_COUNT {
            let contributes = match order.side {
                Side::Buy => usize::from(order.limit_tick) >= tick,
                Side::Sell => usize::from(order.limit_tick) <= tick,
            };
            if contributes {
                let target = match order.side {
                    Side::Buy => &mut curves.demand[tick],
                    Side::Sell => &mut curves.supply[tick],
                };
                *target = target
                    .checked_add(order.quantity)
                    .ok_or(Refusal::ArithmeticOverflow)?;
            }
        }
    }
    Ok(curves)
}

fn select_tick(curves: Curves) -> Result<(PublicTick, u64), Refusal> {
    let mut best_tick = 0usize;
    let mut best_volume = 0u64;
    for tick in 0..TICK_COUNT {
        let volume = curves.demand[tick].min(curves.supply[tick]);
        if volume > best_volume {
            best_tick = tick;
            best_volume = volume;
        }
    }
    if best_volume == 0 {
        Ok((PublicTick::NoTrade, 0))
    } else {
        Ok((PublicTick::Tick(best_tick as u8), best_volume))
    }
}

fn eligible(order: ToyOrder, side: Side, tick: u8) -> bool {
    order.side == side
        && match side {
            Side::Buy => order.limit_tick >= tick,
            Side::Sell => order.limit_tick <= tick,
        }
}

fn allocate(
    slots: &[Slot; SLOT_COUNT],
    tick: PublicTick,
    target: u64,
) -> Result<[u64; SLOT_COUNT], Refusal> {
    if tick == PublicTick::NoTrade {
        return Ok([0; SLOT_COUNT]);
    }
    let PublicTick::Tick(tick) = tick else {
        return Err(Refusal::InternalInvariant);
    };
    let buys = ration_side(slots, Side::Buy, tick, target)?;
    let sells = ration_side(slots, Side::Sell, tick, target)?;
    let mut fills = [0u64; SLOT_COUNT];
    for index in 0..SLOT_COUNT {
        fills[index] = buys[index]
            .checked_add(sells[index])
            .ok_or(Refusal::ArithmeticOverflow)?;
    }
    Ok(fills)
}

fn ration_side(
    slots: &[Slot; SLOT_COUNT],
    side: Side,
    tick: u8,
    target: u64,
) -> Result<[u64; SLOT_COUNT], Refusal> {
    let mut total = 0u64;
    for slot in slots {
        if let Slot::Order(order) = slot
            && eligible(*order, side, tick)
        {
            total = total
                .checked_add(order.quantity)
                .ok_or(Refusal::ArithmeticOverflow)?;
        }
    }
    if target > total || (target > 0 && total == 0) {
        return Err(Refusal::InternalInvariant);
    }

    let mut fills = [0u64; SLOT_COUNT];
    if target == 0 {
        return Ok(fills);
    }
    if target == total {
        for (index, slot) in slots.iter().enumerate() {
            if let Slot::Order(order) = slot
                && eligible(*order, side, tick)
            {
                fills[index] = order.quantity;
            }
        }
        return Ok(fills);
    }

    let mut remainders = [0u64; SLOT_COUNT];
    let mut base_sum = 0u64;
    for (index, slot) in slots.iter().enumerate() {
        if let Slot::Order(order) = slot
            && eligible(*order, side, tick)
        {
            let product = order
                .quantity
                .checked_mul(target)
                .ok_or(Refusal::ArithmeticOverflow)?;
            fills[index] = product / total;
            remainders[index] = product % total;
            base_sum = base_sum
                .checked_add(fills[index])
                .ok_or(Refusal::ArithmeticOverflow)?;
        }
    }

    let residual = target
        .checked_sub(base_sum)
        .ok_or(Refusal::InternalInvariant)?;
    let mut received_bonus = [false; SLOT_COUNT];
    for _ in 0..residual {
        let mut best: Option<usize> = None;
        for (index, slot) in slots.iter().enumerate() {
            let Slot::Order(order) = slot else {
                continue;
            };
            if !eligible(*order, side, tick) || received_bonus[index] {
                continue;
            }
            if best.is_none_or(|current| remainders[index] > remainders[current]) {
                best = Some(index);
            }
        }
        let best = best.ok_or(Refusal::InternalInvariant)?;
        fills[best] = fills[best]
            .checked_add(1)
            .ok_or(Refusal::ArithmeticOverflow)?;
        received_bonus[best] = true;
    }
    Ok(fills)
}

fn owner_outputs(
    slots: &[Slot; SLOT_COUNT],
    tick: PublicTick,
    fills: [u64; SLOT_COUNT],
) -> Result<[OwnerOutput; OWNER_COUNT], Refusal> {
    let mut owners = core::array::from_fn(OwnerOutput::empty);
    let price = match tick {
        PublicTick::NoTrade => 0,
        PublicTick::Tick(tick) => PRICE_TICKS[usize::from(tick)],
    };

    for (slot_index, slot) in slots.iter().enumerate() {
        let Slot::Order(order) = slot else {
            continue;
        };
        let fill = fills[slot_index];
        let owner = &mut owners[usize::from(order.owner)];
        owner.order_fills[slot_index] = Some(LocalFill {
            slot: slot_index as u8,
            quantity: fill,
        });
        let fill_i64 = i64::try_from(fill).map_err(|_| Refusal::ArithmeticOverflow)?;
        let quote = fill.checked_mul(price).ok_or(Refusal::ArithmeticOverflow)?;
        let quote_i64 = i64::try_from(quote).map_err(|_| Refusal::ArithmeticOverflow)?;
        match order.side {
            Side::Buy => {
                owner.bought = owner
                    .bought
                    .checked_add(fill)
                    .ok_or(Refusal::ArithmeticOverflow)?;
                owner.base_delta = owner
                    .base_delta
                    .checked_add(fill_i64)
                    .ok_or(Refusal::ArithmeticOverflow)?;
                owner.quote_delta = owner
                    .quote_delta
                    .checked_sub(quote_i64)
                    .ok_or(Refusal::ArithmeticOverflow)?;
                owner.released_quote_reservation = owner
                    .released_quote_reservation
                    .checked_add(
                        order
                            .reserved
                            .checked_sub(quote)
                            .ok_or(Refusal::InternalInvariant)?,
                    )
                    .ok_or(Refusal::ArithmeticOverflow)?;
            }
            Side::Sell => {
                owner.sold = owner
                    .sold
                    .checked_add(fill)
                    .ok_or(Refusal::ArithmeticOverflow)?;
                owner.base_delta = owner
                    .base_delta
                    .checked_sub(fill_i64)
                    .ok_or(Refusal::ArithmeticOverflow)?;
                owner.quote_delta = owner
                    .quote_delta
                    .checked_add(quote_i64)
                    .ok_or(Refusal::ArithmeticOverflow)?;
                owner.released_base_reservation = owner
                    .released_base_reservation
                    .checked_add(
                        order
                            .reserved
                            .checked_sub(fill)
                            .ok_or(Refusal::InternalInvariant)?,
                    )
                    .ok_or(Refusal::ArithmeticOverflow)?;
            }
        }
    }
    Ok(owners)
}

fn verify_result(
    slots: &[Slot; SLOT_COUNT],
    tick: PublicTick,
    volume: u64,
    fills: [u64; SLOT_COUNT],
    owners: &[OwnerOutput; OWNER_COUNT],
) -> Result<(), Refusal> {
    let mut bought = 0u64;
    let mut sold = 0u64;
    for (index, slot) in slots.iter().enumerate() {
        let fill = fills[index];
        let Some(order) = (match slot {
            Slot::Empty => None,
            Slot::Order(order) => Some(order),
        }) else {
            if fill != 0 {
                return Err(Refusal::InternalInvariant);
            }
            continue;
        };
        if fill > order.quantity {
            return Err(Refusal::InternalInvariant);
        }
        let active = match tick {
            PublicTick::NoTrade => false,
            PublicTick::Tick(tick) => eligible(*order, order.side, tick),
        };
        if !active && fill != 0 {
            return Err(Refusal::InternalInvariant);
        }
        match order.side {
            Side::Buy => {
                bought = bought
                    .checked_add(fill)
                    .ok_or(Refusal::ArithmeticOverflow)?;
            }
            Side::Sell => {
                sold = sold.checked_add(fill).ok_or(Refusal::ArithmeticOverflow)?;
            }
        }
    }
    if bought != volume || sold != volume {
        return Err(Refusal::InternalInvariant);
    }

    let base_sum = owners.iter().try_fold(0i64, |sum, owner| {
        sum.checked_add(owner.base_delta)
            .ok_or(Refusal::ArithmeticOverflow)
    })?;
    let quote_sum = owners.iter().try_fold(0i64, |sum, owner| {
        sum.checked_add(owner.quote_delta)
            .ok_or(Refusal::ArithmeticOverflow)
    })?;
    if base_sum != 0 || quote_sum != 0 {
        return Err(Refusal::InternalInvariant);
    }
    Ok(())
}

fn order(
    context: PublicContext,
    owner: u8,
    side: Side,
    limit_tick: u8,
    quantity: u64,
    reserved: u64,
    nullifier: u64,
) -> Slot {
    Slot::Order(ToyOrder {
        batch_id: context.batch_id,
        market_id: context.market_id,
        owner,
        side,
        limit_tick,
        quantity,
        reserved,
        nullifier,
        arrived_at: context.cutoff,
        admission: ToyAdmissionWitness::PASS,
    })
}

fn context(root_byte: u8) -> PublicContext {
    PublicContext {
        batch_id: 7,
        market_id: 9,
        cutoff: 10,
        accepted_input_root: [root_byte; 32],
    }
}

fn balanced_fixture() -> ToyBatch {
    let context = context(0x11);
    ToyBatch {
        requested_mode: ExecutionMode::ShieldedSingleExecutor,
        context,
        boundary: ToyBoundaryWitness::PASS,
        slots: [
            order(context, 0, Side::Buy, 2, 5, 15, 101),
            order(context, 1, Side::Buy, 1, 3, 6, 102),
            order(context, 2, Side::Sell, 0, 4, 4, 103),
            order(context, 3, Side::Sell, 2, 4, 4, 104),
        ],
    }
}

fn tie_fixture() -> ToyBatch {
    let context = context(0x22);
    ToyBatch {
        requested_mode: ExecutionMode::Clear,
        context,
        boundary: ToyBoundaryWitness::PASS,
        slots: [
            order(context, 0, Side::Buy, 2, 4, 12, 201),
            order(context, 1, Side::Sell, 1, 4, 4, 202),
            Slot::Empty,
            Slot::Empty,
        ],
    }
}

fn no_trade_fixture() -> ToyBatch {
    let context = context(0x33);
    ToyBatch {
        requested_mode: ExecutionMode::Clear,
        context,
        boundary: ToyBoundaryWitness::PASS,
        slots: [
            order(context, 0, Side::Buy, 0, 2, 2, 301),
            order(context, 1, Side::Sell, 3, 2, 2, 302),
            Slot::Empty,
            Slot::Empty,
        ],
    }
}

fn render_ok(name: &str, execution: &ToyExecution) -> String {
    let tick = match execution.public.tick {
        PublicTick::NoTrade => "none".to_owned(),
        PublicTick::Tick(tick) => tick.to_string(),
    };
    let fills = execution
        .fills
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(",");
    let owners = execution
        .owners
        .iter()
        .map(|owner| {
            format!(
                "o{}:b{}:s{}:db{}:dq{}:rb{}:rq{}",
                owner.owner,
                owner.bought,
                owner.sold,
                owner.base_delta,
                owner.quote_delta,
                owner.released_base_reservation,
                owner.released_quote_reservation
            )
        })
        .collect::<Vec<_>>()
        .join(";");
    let root = execution
        .public
        .accepted_input_root
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!(
        "case={name}|status=ok|executed={}|tick={tick}|volume={}|fills={fills}|owners={owners}|root={root}\n",
        execution.executed_mode.label(),
        execution.public.volume
    )
}

fn render_refused(name: &str, result: Result<ToyExecution, Refusal>) -> String {
    match result {
        Ok(execution) => render_ok(name, &execution),
        Err(error) => format!("case={name}|status=refused|error={error}\n"),
    }
}

/// Stable, synthetic, human-readable vectors. This is not a canonical protocol
/// encoding and must never be used as one.
pub fn render_vectors_v1() -> String {
    let mut output = String::from("dark-fba-vectors-v1\n");
    output.push_str(&render_refused(
        "balanced-residual",
        evaluate(&balanced_fixture()),
    ));
    output.push_str(&render_refused("price-tie-low", evaluate(&tie_fixture())));
    output.push_str(&render_refused("no-trade", evaluate(&no_trade_fixture())));

    let mut late = tie_fixture();
    let Slot::Order(ref mut late_order) = late.slots[0] else {
        unreachable!();
    };
    late_order.arrived_at = late.context.cutoff + 1;
    output.push_str(&render_refused("late-order", evaluate(&late)));

    let mut unavailable = tie_fixture();
    unavailable.boundary.all_payloads_available_by_cutoff = false;
    output.push_str(&render_refused(
        "payload-unavailable",
        evaluate(&unavailable),
    ));

    let mut duplicate = tie_fixture();
    let Slot::Order(first) = duplicate.slots[0] else {
        unreachable!();
    };
    let Slot::Order(ref mut second) = duplicate.slots[1] else {
        unreachable!();
    };
    second.nullifier = first.nullifier;
    output.push_str(&render_refused("duplicate-nullifier", evaluate(&duplicate)));

    let mut under_reserved = tie_fixture();
    let Slot::Order(ref mut buy) = under_reserved.slots[0] else {
        unreachable!();
    };
    buy.reserved -= 1;
    output.push_str(&render_refused("under-reserved", evaluate(&under_reserved)));

    let mut dark = tie_fixture();
    dark.requested_mode = ExecutionMode::DarkTarget;
    output.push_str(&render_refused("dark-target", evaluate(&dark)));
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vectors_are_byte_stable() {
        assert_eq!(render_vectors_v1(), include_str!("../vectors/v1.txt"));
    }

    #[test]
    fn balanced_fixture_uses_exact_residual_rank() {
        let execution = evaluate(&balanced_fixture()).expect("valid fixture");
        assert_eq!(execution.public.tick, PublicTick::Tick(2));
        assert_eq!(execution.public.volume, 5);
        assert_eq!(execution.fills, [5, 0, 3, 2]);
        assert_eq!(execution.owners[0].base_delta, 5);
        assert_eq!(execution.owners[0].quote_delta, -15);
        assert_eq!(execution.owners[2].released_base_reservation, 1);
        assert_eq!(execution.owners[3].released_base_reservation, 2);
    }

    #[test]
    fn maximum_volume_tie_chooses_lowest_tick() {
        let execution = evaluate(&tie_fixture()).expect("valid fixture");
        assert_eq!(execution.public.tick, PublicTick::Tick(1));
        assert_eq!(execution.public.volume, 4);
        assert_eq!(execution.fills, [4, 4, 0, 0]);
    }

    #[test]
    fn non_crossing_book_is_valid_no_trade() {
        let execution = evaluate(&no_trade_fixture()).expect("valid fixture");
        assert_eq!(execution.public.tick, PublicTick::NoTrade);
        assert_eq!(execution.public.volume, 0);
        assert_eq!(execution.fills, [0; SLOT_COUNT]);
        assert_eq!(execution.owners[0].released_quote_reservation, 2);
        assert_eq!(execution.owners[1].released_base_reservation, 2);
    }

    #[test]
    fn dark_target_is_refused_before_private_validation() {
        let mut valid = tie_fixture();
        valid.requested_mode = ExecutionMode::DarkTarget;
        let mut invalid = valid;
        let Slot::Order(ref mut order) = invalid.slots[0] else {
            unreachable!();
        };
        order.quantity = 0;
        assert_eq!(evaluate(&valid), Err(Refusal::DarkBackendAbsent));
        assert_eq!(evaluate(&invalid), Err(Refusal::DarkBackendAbsent));
    }

    #[test]
    fn availability_failure_is_not_no_trade() {
        let mut batch = no_trade_fixture();
        batch.boundary.all_payloads_available_by_cutoff = false;
        assert_eq!(
            evaluate(&batch),
            Err(Refusal::Boundary(BoundaryRefusal::PayloadUnavailable))
        );
    }

    #[test]
    fn duplicate_nullifier_reports_the_actual_first_slot() {
        let context = context(0x44);
        let batch = ToyBatch {
            requested_mode: ExecutionMode::Clear,
            context,
            boundary: ToyBoundaryWitness::PASS,
            slots: [
                Slot::Empty,
                order(context, 0, Side::Buy, 1, 1, 2, 601),
                Slot::Empty,
                order(context, 1, Side::Sell, 1, 1, 1, 601),
            ],
        };
        assert_eq!(
            evaluate(&batch),
            Err(Refusal::Order {
                slot: 3,
                reason: OrderRefusal::DuplicateNullifier { first_slot: 1 },
            })
        );
    }

    #[test]
    fn canonical_slot_rank_is_economically_material() {
        let original = balanced_fixture();
        let first = evaluate(&original).expect("original");
        let mut swapped = original;
        swapped.slots.swap(2, 3);
        let second = evaluate(&swapped).expect("swapped");
        assert_eq!(first.public.tick, second.public.tick);
        assert_eq!(first.public.volume, second.public.volume);
        assert_eq!(first.owners[2].sold, 3);
        assert_eq!(first.owners[3].sold, 2);
        assert_eq!(second.owners[2].sold, 2);
        assert_eq!(second.owners[3].sold, 3);
    }

    #[test]
    fn all_one_buy_one_sell_books_conserve_and_respect_limits() {
        let context = context(0x55);
        for buy_limit in 0..TICK_COUNT as u8 {
            for sell_limit in 0..TICK_COUNT as u8 {
                for buy_quantity in 1..=Q_MAX {
                    for sell_quantity in 1..=Q_MAX {
                        let batch = ToyBatch {
                            requested_mode: ExecutionMode::Clear,
                            context,
                            boundary: ToyBoundaryWitness::PASS,
                            slots: [
                                order(
                                    context,
                                    0,
                                    Side::Buy,
                                    buy_limit,
                                    buy_quantity,
                                    buy_quantity * PRICE_TICKS[usize::from(buy_limit)],
                                    501,
                                ),
                                order(
                                    context,
                                    1,
                                    Side::Sell,
                                    sell_limit,
                                    sell_quantity,
                                    sell_quantity,
                                    502,
                                ),
                                Slot::Empty,
                                Slot::Empty,
                            ],
                        };
                        let execution = evaluate(&batch).expect("admissible exhaustive case");
                        if buy_limit < sell_limit {
                            assert_eq!(execution.public.tick, PublicTick::NoTrade);
                            assert_eq!(execution.public.volume, 0);
                        } else {
                            let expected = buy_quantity.min(sell_quantity);
                            assert_eq!(execution.public.tick, PublicTick::Tick(sell_limit));
                            assert_eq!(execution.public.volume, expected);
                            assert_eq!(execution.fills[0], expected);
                            assert_eq!(execution.fills[1], expected);
                            assert!(sell_limit <= buy_limit);
                        }
                        let base_sum: i64 =
                            execution.owners.iter().map(|owner| owner.base_delta).sum();
                        let quote_sum: i64 =
                            execution.owners.iter().map(|owner| owner.quote_delta).sum();
                        assert_eq!(base_sum, 0);
                        assert_eq!(quote_sum, 0);
                    }
                }
            }
        }
    }
}
