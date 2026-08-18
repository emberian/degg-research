//! Lowering: from a relation module (data) to an executable evaluator.
//!
//! The only lowering that exists is Clear. The evaluator is an interpreter
//! over the module's frozen fields: parameters, admission-check order,
//! clearing rule, and allocation rule are read from the data, so a module
//! carrying a different frozen order is visibly a different relation and
//! evaluates differently.
//!
//! Visibility honesty: the module annotates ports `PrivateToOwner` and
//! `Executor`, and the Clear lowering satisfies none of that. The one process
//! executing [`ClearEvaluator::evaluate`] receives every order and computes
//! every owner-local output. That widening is declared here, loudly, instead
//! of silently: see [`CLEAR_VISIBILITY_DISCLOSURE`]. Requesting a lowering
//! that would have to honor the annotations refuses.

use crate::batch::{BatchInput, OrderWitness, RequestedMode, Side, SlotInput};
use crate::canon::{Canonical, Sink};
use crate::module::{AllocationMethod, PriceObjective, PriceTie, RelationModule, ResidualTie};
use crate::policy::{AdmissionStep, BoundaryCheck, SlotRule};
use crate::receipt::{ComputationReceipt, OutputDeliveryReceipt, ReceiptStatus};
use crate::refusal::{Refusal, RefusalClass};

/// The declared widening of the Clear lowering.
///
/// No privacy property of any kind is provided, approximated, or implied by
/// executing a module under this lowering, whatever the annotations say.
pub const CLEAR_VISIBILITY_DISCLOSURE: &str = "clear lowering: the executing process learns every \
     input and every output, including every port annotated private-to-owner or executor";

/// A backend a module could be lowered to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoweringTarget {
    /// One process that sees everything. The only target that exists.
    Clear,
    /// A named committee/threshold backend honoring `Executor` boundaries.
    /// Does not exist here; requesting it refuses.
    ShieldedCommittee,
    /// A backend honoring the `PrivateToOwner`/`Public` annotations within a
    /// corruption model. Does not exist; requesting it refuses, with the same
    /// discipline as the oracles' `DarkTarget` execution refusal.
    DarkTarget,
}

/// Why a lowering was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoweringRefusal {
    /// No Dark backend exists. An IR annotation is not an implementation.
    DarkBackendAbsent,
    /// No Shielded committee/threshold backend exists. A single-executor
    /// "shielded" run is the Clear evaluator relabeled at run time, exactly as
    /// in the oracles, and is requested per batch, not per lowering.
    ShieldedBackendAbsent,
    /// The module is outside the family this evaluator supports, or its check
    /// order is not well formed. The message names the first violation.
    UnsupportedModule(&'static str),
}

/// The tick outcome of a settled batch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClearedTick {
    /// Maximum volume is zero; a valid outcome, not a refusal.
    NoTrade,
    /// The selected tick index.
    Tick(u8),
}

impl Canonical for ClearedTick {
    fn tag(&self) -> &'static str {
        "ir/cleared-tick"
    }
    fn body(&self, sink: &mut Sink) {
        match self {
            Self::NoTrade => sink.u32(0),
            Self::Tick(tick) => {
                sink.u32(1);
                sink.u8(*tick);
            }
        }
    }
}

/// The frozen public result of a settled batch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicResult {
    /// Relation identifier.
    pub relation: String,
    /// Batch identifier.
    pub batch_id: u64,
    /// Market identifier.
    pub market_id: u64,
    /// Accepted-input root, preserved verbatim.
    pub accepted_input_root: [u8; 32],
    /// Selected tick or the no-trade tag.
    pub tick: ClearedTick,
    /// Aggregate matched volume; zero exactly when no trade.
    pub volume: u64,
}

impl Canonical for PublicResult {
    fn tag(&self) -> &'static str {
        "ir/public-result"
    }
    fn body(&self, sink: &mut Sink) {
        sink.str(&self.relation);
        sink.u64(self.batch_id);
        sink.u64(self.market_id);
        sink.digest(&self.accepted_input_root);
        sink.nested(&self.tick);
        sink.u64(self.volume);
    }
}

/// One owner's local output.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnerOutput {
    /// Owner index.
    pub owner: u8,
    /// Base atoms received.
    pub bought: u64,
    /// Base atoms delivered.
    pub sold: u64,
    /// Signed base delta.
    pub base_delta: i64,
    /// Signed quote delta at the selected public price.
    pub quote_delta: i64,
    /// Base reservation returned.
    pub released_base_reservation: u64,
    /// Quote reservation returned.
    pub released_quote_reservation: u64,
    /// Per-slot fill for each slot this owner occupies; `None` elsewhere.
    pub owned_slot_fills: Vec<Option<u64>>,
}

impl OwnerOutput {
    fn empty(owner: u8, slots: usize) -> Self {
        Self {
            owner,
            bought: 0,
            sold: 0,
            base_delta: 0,
            quote_delta: 0,
            released_base_reservation: 0,
            released_quote_reservation: 0,
            owned_slot_fills: vec![None; slots],
        }
    }
}

impl Canonical for OwnerOutput {
    fn tag(&self) -> &'static str {
        "ir/owner-output"
    }
    fn body(&self, sink: &mut Sink) {
        sink.u8(self.owner);
        sink.u64(self.bought);
        sink.u64(self.sold);
        sink.i64(self.base_delta);
        sink.i64(self.quote_delta);
        sink.u64(self.released_base_reservation);
        sink.u64(self.released_quote_reservation);
        sink.count(self.owned_slot_fills.len());
        for fill in &self.owned_slot_fills {
            sink.option(fill.as_ref(), |sink, fill| sink.u64(*fill));
        }
    }
}

/// The complete owner-output vector, as one canonical object.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnerOutputs(pub Vec<OwnerOutput>);

impl Canonical for OwnerOutputs {
    fn tag(&self) -> &'static str {
        "ir/owner-outputs"
    }
    fn body(&self, sink: &mut Sink) {
        sink.count(self.0.len());
        for owner in &self.0 {
            sink.nested(owner);
        }
    }
}

/// The public projection of a refusal: relation, batch, market, and root as
/// specification section 3 requires, plus the class tag and nothing else.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicRefusal {
    /// Relation identifier.
    pub relation: String,
    /// Batch identifier.
    pub batch_id: u64,
    /// Market identifier.
    pub market_id: u64,
    /// Accepted-input root, preserved verbatim.
    pub accepted_input_root: [u8; 32],
    /// The public refusal class, with every diagnostic dropped.
    pub class: RefusalClass,
}

impl Canonical for PublicRefusal {
    fn tag(&self) -> &'static str {
        "ir/public-refusal"
    }
    fn body(&self, sink: &mut Sink) {
        sink.str(&self.relation);
        sink.u64(self.batch_id);
        sink.u64(self.market_id);
        sink.digest(&self.accepted_input_root);
        sink.nested(&self.class);
    }
}

/// What crosses the public boundary for one evaluation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PublicOutcome {
    /// The settled public result.
    Settled(PublicResult),
    /// The public refusal.
    Refused(PublicRefusal),
}

impl Canonical for PublicOutcome {
    fn tag(&self) -> &'static str {
        "ir/public-outcome"
    }
    fn body(&self, sink: &mut Sink) {
        match self {
            Self::Settled(result) => {
                sink.u32(0);
                sink.nested(result);
            }
            Self::Refused(refusal) => {
                sink.u32(1);
                sink.nested(refusal);
            }
        }
    }
}

/// A settled evaluation: everything the one executing process holds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settled {
    /// `"clear"` or `"shielded-single-executor"`; a label about who may
    /// inspect the process, not a mechanism.
    pub executed_label: &'static str,
    /// The frozen public result.
    pub public: PublicResult,
    /// Per-slot fills; executor-visible diagnostic, not a public output.
    pub fills: Vec<u64>,
    /// Owner-local outputs, indexed by owner.
    pub owners: Vec<OwnerOutput>,
    /// The computation receipt.
    pub receipt: ComputationReceipt,
    /// One delivery receipt per owner. Held by the executor; nothing is
    /// actually delivered anywhere.
    pub delivery: Vec<OutputDeliveryReceipt>,
}

/// A refused evaluation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RefusedBatch {
    /// The refusal, with executor-visible diagnostics.
    pub refusal: Refusal,
    /// The public projection of the refusal.
    pub public: PublicRefusal,
    /// The computation receipt for the typed abort.
    pub receipt: ComputationReceipt,
}

/// One complete evaluation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The batch settled; a valid no-trade settles.
    Settled(Settled),
    /// The batch was refused with one typed class.
    Refused(RefusedBatch),
}

impl Outcome {
    /// The public projection of this outcome.
    pub fn public(&self) -> PublicOutcome {
        match self {
            Self::Settled(settled) => PublicOutcome::Settled(settled.public.clone()),
            Self::Refused(refused) => PublicOutcome::Refused(refused.public.clone()),
        }
    }
}

/// The worst-case obligation a slot must have reserved, or `None` if the
/// limit tick is out of the module's domain or the product overflows.
pub fn required_reservation(
    tick_prices: &[u64],
    side: Side,
    limit_tick: u8,
    quantity: u64,
) -> Option<u64> {
    match side {
        Side::Buy => quantity.checked_mul(*tick_prices.get(usize::from(limit_tick))?),
        Side::Sell => Some(quantity),
    }
}

/// Lower a module to a target backend.
pub fn lower(
    module: &RelationModule,
    target: LoweringTarget,
) -> Result<ClearEvaluator, LoweringRefusal> {
    match target {
        LoweringTarget::Clear => ClearEvaluator::compile(module),
        LoweringTarget::ShieldedCommittee => Err(LoweringRefusal::ShieldedBackendAbsent),
        LoweringTarget::DarkTarget => Err(LoweringRefusal::DarkBackendAbsent),
    }
}

fn validate_policy(module: &RelationModule) -> Result<(), LoweringRefusal> {
    let unsupported = LoweringRefusal::UnsupportedModule;
    let steps = &module.admission.steps;
    if steps.first() != Some(&AdmissionStep::RequestedModeExecutable) {
        return Err(unsupported("first admission step must be the mode check"));
    }
    let count = |step: &AdmissionStep| steps.iter().filter(|other| *other == step).count();
    if count(&AdmissionStep::RequestedModeExecutable) != 1 {
        return Err(unsupported("mode check must appear exactly once"));
    }
    for check in [
        BoundaryCheck::LogFinal,
        BoundaryCheck::RootBindsSlots,
        BoundaryCheck::NoConflictingRoot,
        BoundaryCheck::PayloadsAvailable,
    ] {
        if count(&AdmissionStep::Boundary(check)) != 1 {
            return Err(unsupported("each boundary check must appear exactly once"));
        }
    }
    let per_slot: Vec<&Vec<SlotRule>> = steps
        .iter()
        .filter_map(|step| match step {
            AdmissionStep::PerSlot(rules) => Some(rules),
            _ => None,
        })
        .collect();
    let [rules] = per_slot.as_slice() else {
        return Err(unsupported("exactly one per-slot step is required"));
    };
    let position = |rule: SlotRule| rules.iter().position(|other| *other == rule);
    for rule in [
        SlotRule::BatchBinding,
        SlotRule::MarketBinding,
        SlotRule::OwnerInDomain,
        SlotRule::QuantityInDomain,
        SlotRule::LimitInDomain,
        SlotRule::NullifierNonzero,
        SlotRule::ArrivedByCutoff,
        SlotRule::Authorized,
        SlotRule::Eligible,
        SlotRule::IncludedUnderRoot,
        SlotRule::CustodyBound,
        SlotRule::ReservationCovers,
    ] {
        if rules.iter().filter(|other| **other == rule).count() != 1 {
            return Err(unsupported("every per-slot rule must appear exactly once"));
        }
    }
    let inline_uniqueness =
        position(SlotRule::NullifierDistinctFromEarlierSlots).is_some() as usize;
    let sweep = count(&AdmissionStep::NullifierSweep);
    if inline_uniqueness + sweep != 1 {
        return Err(unsupported(
            "exactly one nullifier-uniqueness mechanism is required",
        ));
    }
    let reservation = position(SlotRule::ReservationCovers).expect("checked present");
    if position(SlotRule::QuantityInDomain).expect("checked present") > reservation
        || position(SlotRule::LimitInDomain).expect("checked present") > reservation
    {
        return Err(unsupported(
            "reservation check requires prior quantity and limit domain checks",
        ));
    }
    Ok(())
}

fn validate_params(module: &RelationModule) -> Result<(), LoweringRefusal> {
    let unsupported = LoweringRefusal::UnsupportedModule;
    let params = &module.params;
    if params.owners == 0 || params.slots == 0 {
        return Err(unsupported("owner and slot capacities must be nonzero"));
    }
    if params.tick_prices.is_empty() || params.tick_prices.contains(&0) {
        return Err(unsupported(
            "tick grid must be nonempty with nonzero prices",
        ));
    }
    if params.quantity_floor == 0 || params.quantity_ceiling < params.quantity_floor {
        return Err(unsupported(
            "quantity domain must be a nonempty range from at least one",
        ));
    }
    let max_price = *params.tick_prices.iter().max().expect("nonempty");
    let volume_ceiling = u64::from(params.slots).checked_mul(params.quantity_ceiling);
    // Static width proof: every curve sum, allocation product, and quote figure
    // must fit u64 exactly, or the module is refused at lowering.
    let quote_ceiling = volume_ceiling.and_then(|volume| volume.checked_mul(max_price));
    let allocation_ceiling =
        quote_ceiling.and_then(|quote| quote.checked_mul(params.quantity_ceiling));
    if allocation_ceiling.is_none()
        || quote_ceiling
            .map(i64::try_from)
            .and_then(Result::ok)
            .is_none()
    {
        return Err(unsupported("static bounds do not prove nonoverflow"));
    }
    Ok(())
}

/// The executable Clear evaluator produced by [`lower`].
///
/// It interprets the module it was compiled from; see
/// [`CLEAR_VISIBILITY_DISCLOSURE`] for what the executing process learns.
#[derive(Clone, Debug)]
pub struct ClearEvaluator {
    module: RelationModule,
    module_digest: [u8; 32],
}

impl ClearEvaluator {
    fn compile(module: &RelationModule) -> Result<Self, LoweringRefusal> {
        validate_params(module)?;
        validate_policy(module)?;
        Ok(Self {
            module: module.clone(),
            module_digest: module.digest(),
        })
    }

    /// The module this evaluator interprets.
    pub fn module(&self) -> &RelationModule {
        &self.module
    }

    /// The module's canonical digest: the relation identity this evaluator
    /// realizes.
    pub fn module_digest(&self) -> [u8; 32] {
        self.module_digest
    }

    /// Evaluate one batch.
    pub fn evaluate(&self, batch: &BatchInput) -> Outcome {
        let input_digest = batch.digest();
        match self.run(batch, input_digest) {
            Ok(settled) => settled,
            Err(refusal) => {
                let public = PublicRefusal {
                    relation: self.module.identity.relation.clone(),
                    batch_id: batch.batch_id,
                    market_id: batch.market_id,
                    accepted_input_root: batch.accepted_input_root,
                    class: refusal.class,
                };
                let outcome = PublicOutcome::Refused(public.clone());
                Outcome::Refused(RefusedBatch {
                    refusal,
                    public,
                    receipt: ComputationReceipt {
                        module_digest: self.module_digest,
                        input_digest,
                        outcome_digest: outcome.digest(),
                        status: ReceiptStatus::Refused(refusal.class),
                    },
                })
            }
        }
    }

    fn run(&self, batch: &BatchInput, input_digest: [u8; 32]) -> Result<Outcome, Refusal> {
        let params = &self.module.params;
        let slots = usize::from(params.slots);
        if batch.slots.len() != slots {
            return Err(Refusal::batch(RefusalClass::MalformedEncoding));
        }

        let mut executed_label = "clear";
        for step in &self.module.admission.steps {
            match step {
                AdmissionStep::RequestedModeExecutable => {
                    executed_label = match batch.requested_mode {
                        RequestedMode::Clear => "clear",
                        RequestedMode::ShieldedSingleExecutor => "shielded-single-executor",
                        RequestedMode::DarkTarget => {
                            return Err(Refusal::batch(RefusalClass::DarkTargetUnavailable));
                        }
                    };
                }
                AdmissionStep::Boundary(check) => self.check_boundary(*check, batch)?,
                AdmissionStep::PerSlot(rules) => {
                    for (index, slot) in batch.slots.iter().enumerate() {
                        let SlotInput::Occupied(order) = slot else {
                            continue;
                        };
                        for rule in rules {
                            self.check_slot_rule(*rule, index as u8, order, batch)?;
                        }
                    }
                }
                AdmissionStep::NullifierSweep => {
                    for (index, slot) in batch.slots.iter().enumerate() {
                        let SlotInput::Occupied(order) = slot else {
                            continue;
                        };
                        self.check_nullifier_against_earlier(index as u8, order, batch)?;
                    }
                }
            }
        }

        let (demand, supply) = self.aggregate_curves(batch)?;
        let (tick, volume) = self.select_tick(&demand, &supply);
        let fills = self.allocate(batch, tick, volume)?;
        let owners = self.owner_outputs(batch, tick, &fills)?;
        self.audit(batch, tick, volume, &fills, &owners)?;

        let public = PublicResult {
            relation: self.module.identity.relation.clone(),
            batch_id: batch.batch_id,
            market_id: batch.market_id,
            accepted_input_root: batch.accepted_input_root,
            tick,
            volume,
        };
        let outcome_digest = PublicOutcome::Settled(public.clone()).digest();
        let delivery = owners
            .iter()
            .map(|owner| OutputDeliveryReceipt {
                owner: owner.owner,
                module_digest: self.module_digest,
                input_digest,
                output_digest: owner.digest(),
            })
            .collect();
        Ok(Outcome::Settled(Settled {
            executed_label,
            public,
            fills,
            owners,
            receipt: ComputationReceipt {
                module_digest: self.module_digest,
                input_digest,
                outcome_digest,
                status: ReceiptStatus::Settled,
            },
            delivery,
        }))
    }

    fn check_boundary(&self, check: BoundaryCheck, batch: &BatchInput) -> Result<(), Refusal> {
        let (holds, class) = match check {
            BoundaryCheck::LogFinal => {
                (batch.boundary.log_final, RefusalClass::AdmissionLogNotFinal)
            }
            BoundaryCheck::RootBindsSlots => (
                batch.boundary.root_binds_slots,
                RefusalClass::RootBindingAbsent,
            ),
            BoundaryCheck::NoConflictingRoot => (
                batch.boundary.no_conflicting_root,
                RefusalClass::RootEquivocation,
            ),
            BoundaryCheck::PayloadsAvailable => (
                batch.boundary.payloads_available,
                RefusalClass::PayloadUnavailable,
            ),
        };
        if holds {
            Ok(())
        } else {
            Err(Refusal::batch(class))
        }
    }

    fn check_slot_rule(
        &self,
        rule: SlotRule,
        slot: u8,
        order: &OrderWitness,
        batch: &BatchInput,
    ) -> Result<(), Refusal> {
        let params = &self.module.params;
        let fail = |class| Err(Refusal::at_slot(class, slot));
        match rule {
            SlotRule::BatchBinding => {
                if order.batch_id != batch.batch_id {
                    return fail(RefusalClass::BatchBindingMismatch);
                }
            }
            SlotRule::MarketBinding => {
                if order.market_id != batch.market_id {
                    return fail(RefusalClass::MarketBindingMismatch);
                }
            }
            SlotRule::OwnerInDomain => {
                if order.owner >= params.owners {
                    return fail(RefusalClass::OwnerOutOfDomain);
                }
            }
            SlotRule::QuantityInDomain => {
                if order.quantity < params.quantity_floor
                    || order.quantity > params.quantity_ceiling
                {
                    return fail(RefusalClass::QuantityOutOfDomain);
                }
            }
            SlotRule::LimitInDomain => {
                if usize::from(order.limit_tick) >= params.ticks() {
                    return fail(RefusalClass::LimitOutOfDomain);
                }
            }
            SlotRule::NullifierNonzero => {
                if order.nullifier == 0 {
                    return fail(RefusalClass::NullifierZero);
                }
            }
            SlotRule::NullifierDistinctFromEarlierSlots => {
                self.check_nullifier_against_earlier(slot, order, batch)?;
            }
            SlotRule::ArrivedByCutoff => {
                if order.arrived_at > batch.cutoff {
                    return fail(RefusalClass::LateArrival);
                }
            }
            SlotRule::Authorized => {
                if !order.authorized {
                    return fail(RefusalClass::Unauthorized);
                }
            }
            SlotRule::Eligible => {
                if !order.eligible {
                    return fail(RefusalClass::Ineligible);
                }
            }
            SlotRule::IncludedUnderRoot => {
                if !order.included_under_root {
                    return fail(RefusalClass::InclusionAbsent);
                }
            }
            SlotRule::CustodyBound => {
                if !order.custody_bound {
                    return fail(RefusalClass::CustodyBindingAbsent);
                }
            }
            SlotRule::ReservationCovers => {
                let required = required_reservation(
                    &params.tick_prices,
                    order.side,
                    order.limit_tick,
                    order.quantity,
                )
                .ok_or(Refusal::at_slot(RefusalClass::AccumulatorOverflow, slot))?;
                if order.reserved < required {
                    return fail(RefusalClass::ReservationInsufficient);
                }
            }
        }
        Ok(())
    }

    fn check_nullifier_against_earlier(
        &self,
        slot: u8,
        order: &OrderWitness,
        batch: &BatchInput,
    ) -> Result<(), Refusal> {
        for (earlier, other) in batch.slots.iter().enumerate().take(usize::from(slot)) {
            let SlotInput::Occupied(other) = other else {
                continue;
            };
            if other.nullifier == order.nullifier {
                return Err(Refusal {
                    class: RefusalClass::NullifierRepeated,
                    slot: Some(slot),
                    first_slot: Some(earlier as u8),
                });
            }
        }
        Ok(())
    }

    fn aggregate_curves(&self, batch: &BatchInput) -> Result<(Vec<u64>, Vec<u64>), Refusal> {
        let ticks = self.module.params.ticks();
        let mut demand = vec![0u64; ticks];
        let mut supply = vec![0u64; ticks];
        let overflow = Refusal::batch(RefusalClass::AccumulatorOverflow);
        for slot in &batch.slots {
            let SlotInput::Occupied(order) = slot else {
                continue;
            };
            for tick in 0..ticks {
                let contributes = match order.side {
                    Side::Buy => usize::from(order.limit_tick) >= tick,
                    Side::Sell => usize::from(order.limit_tick) <= tick,
                };
                if contributes {
                    let target = match order.side {
                        Side::Buy => &mut demand[tick],
                        Side::Sell => &mut supply[tick],
                    };
                    *target = target.checked_add(order.quantity).ok_or(overflow)?;
                }
            }
        }
        Ok((demand, supply))
    }

    fn select_tick(&self, demand: &[u64], supply: &[u64]) -> (ClearedTick, u64) {
        let PriceObjective::MaximizeVolume = self.module.clearing.objective;
        let PriceTie::LowestTick = self.module.clearing.tie;
        let mut best_tick = 0usize;
        let mut best_volume = 0u64;
        for tick in 0..demand.len() {
            let volume = demand[tick].min(supply[tick]);
            // Strict improvement plus ascending scan realizes ties-low.
            if volume > best_volume {
                best_tick = tick;
                best_volume = volume;
            }
        }
        if best_volume == 0 {
            (ClearedTick::NoTrade, 0)
        } else {
            (ClearedTick::Tick(best_tick as u8), best_volume)
        }
    }

    fn eligible(order: &OrderWitness, side: Side, tick: u8) -> bool {
        order.side == side
            && match side {
                Side::Buy => order.limit_tick >= tick,
                Side::Sell => order.limit_tick <= tick,
            }
    }

    fn allocate(
        &self,
        batch: &BatchInput,
        tick: ClearedTick,
        target: u64,
    ) -> Result<Vec<u64>, Refusal> {
        let slots = batch.slots.len();
        let mut fills = vec![0u64; slots];
        let ClearedTick::Tick(tick) = tick else {
            return Ok(fills);
        };
        for side in [Side::Buy, Side::Sell] {
            let awards = self.ration_side(batch, side, tick, target)?;
            for (fill, award) in fills.iter_mut().zip(awards) {
                *fill = fill
                    .checked_add(award)
                    .ok_or(Refusal::batch(RefusalClass::AccumulatorOverflow))?;
            }
        }
        Ok(fills)
    }

    fn ration_side(
        &self,
        batch: &BatchInput,
        side: Side,
        tick: u8,
        target: u64,
    ) -> Result<Vec<u64>, Refusal> {
        let AllocationMethod::LargestRemainder = self.module.allocation.method;
        let ResidualTie::EarliestCanonicalSlot = self.module.allocation.residual_tie;
        let overflow = Refusal::batch(RefusalClass::AccumulatorOverflow);
        let invariant = Refusal::batch(RefusalClass::InternalInvariant);

        let slots = batch.slots.len();
        let eligible: Vec<Option<u64>> = batch
            .slots
            .iter()
            .map(|slot| match slot {
                SlotInput::Occupied(order) if Self::eligible(order, side, tick) => {
                    Some(order.quantity)
                }
                _ => None,
            })
            .collect();
        let mut total = 0u64;
        for quantity in eligible.iter().flatten() {
            total = total.checked_add(*quantity).ok_or(overflow)?;
        }
        if target > total || (target > 0 && total == 0) {
            return Err(invariant);
        }

        let mut fills = vec![0u64; slots];
        if target == 0 {
            return Ok(fills);
        }

        let mut remainders = vec![0u64; slots];
        let mut base_sum = 0u64;
        for (index, quantity) in eligible.iter().enumerate() {
            let Some(quantity) = quantity else { continue };
            let product = quantity.checked_mul(target).ok_or(overflow)?;
            fills[index] = product / total;
            remainders[index] = product % total;
            base_sum = base_sum.checked_add(fills[index]).ok_or(overflow)?;
        }

        let residual = target.checked_sub(base_sum).ok_or(invariant)?;
        let mut received_bonus = vec![false; slots];
        for _ in 0..residual {
            let mut best: Option<usize> = None;
            for (index, quantity) in eligible.iter().enumerate() {
                if quantity.is_none() || received_bonus[index] {
                    continue;
                }
                // Strict comparison keeps the earliest slot among equal
                // remainders: the frozen residual tie.
                if best.is_none_or(|current| remainders[index] > remainders[current]) {
                    best = Some(index);
                }
            }
            let best = best.ok_or(invariant)?;
            fills[best] = fills[best].checked_add(1).ok_or(overflow)?;
            received_bonus[best] = true;
        }
        Ok(fills)
    }

    fn owner_outputs(
        &self,
        batch: &BatchInput,
        tick: ClearedTick,
        fills: &[u64],
    ) -> Result<Vec<OwnerOutput>, Refusal> {
        let params = &self.module.params;
        let overflow = Refusal::batch(RefusalClass::AccumulatorOverflow);
        let invariant = Refusal::batch(RefusalClass::InternalInvariant);
        let price = match tick {
            ClearedTick::NoTrade => 0,
            ClearedTick::Tick(tick) => params.tick_prices[usize::from(tick)],
        };
        let mut owners: Vec<OwnerOutput> = (0..params.owners)
            .map(|owner| OwnerOutput::empty(owner, batch.slots.len()))
            .collect();
        for (slot_index, slot) in batch.slots.iter().enumerate() {
            let SlotInput::Occupied(order) = slot else {
                continue;
            };
            let fill = fills[slot_index];
            let owner = &mut owners[usize::from(order.owner)];
            owner.owned_slot_fills[slot_index] = Some(fill);
            let fill_signed = i64::try_from(fill).map_err(|_| overflow)?;
            let quote = fill.checked_mul(price).ok_or(overflow)?;
            let quote_signed = i64::try_from(quote).map_err(|_| overflow)?;
            match order.side {
                Side::Buy => {
                    owner.bought = owner.bought.checked_add(fill).ok_or(overflow)?;
                    owner.base_delta = owner.base_delta.checked_add(fill_signed).ok_or(overflow)?;
                    owner.quote_delta = owner
                        .quote_delta
                        .checked_sub(quote_signed)
                        .ok_or(overflow)?;
                    let released = order.reserved.checked_sub(quote).ok_or(invariant)?;
                    owner.released_quote_reservation = owner
                        .released_quote_reservation
                        .checked_add(released)
                        .ok_or(overflow)?;
                }
                Side::Sell => {
                    owner.sold = owner.sold.checked_add(fill).ok_or(overflow)?;
                    owner.base_delta = owner.base_delta.checked_sub(fill_signed).ok_or(overflow)?;
                    owner.quote_delta = owner
                        .quote_delta
                        .checked_add(quote_signed)
                        .ok_or(overflow)?;
                    let released = order.reserved.checked_sub(fill).ok_or(invariant)?;
                    owner.released_base_reservation = owner
                        .released_base_reservation
                        .checked_add(released)
                        .ok_or(overflow)?;
                }
            }
        }
        Ok(owners)
    }

    fn audit(
        &self,
        batch: &BatchInput,
        tick: ClearedTick,
        volume: u64,
        fills: &[u64],
        owners: &[OwnerOutput],
    ) -> Result<(), Refusal> {
        let overflow = Refusal::batch(RefusalClass::AccumulatorOverflow);
        let invariant = Refusal::batch(RefusalClass::InternalInvariant);
        let mut bought = 0u64;
        let mut sold = 0u64;
        for (index, slot) in batch.slots.iter().enumerate() {
            let fill = fills[index];
            let SlotInput::Occupied(order) = slot else {
                if fill != 0 {
                    return Err(invariant);
                }
                continue;
            };
            if fill > order.quantity {
                return Err(invariant);
            }
            let active = match tick {
                ClearedTick::NoTrade => false,
                ClearedTick::Tick(tick) => Self::eligible(order, order.side, tick),
            };
            if !active && fill != 0 {
                return Err(invariant);
            }
            match order.side {
                Side::Buy => bought = bought.checked_add(fill).ok_or(overflow)?,
                Side::Sell => sold = sold.checked_add(fill).ok_or(overflow)?,
            }
        }
        if bought != volume || sold != volume {
            return Err(invariant);
        }
        let mut base_sum = 0i64;
        let mut quote_sum = 0i64;
        for owner in owners {
            base_sum = base_sum.checked_add(owner.base_delta).ok_or(overflow)?;
            quote_sum = quote_sum.checked_add(owner.quote_delta).ok_or(overflow)?;
        }
        if base_sum != 0 || quote_sum != 0 {
            return Err(invariant);
        }
        Ok(())
    }
}
