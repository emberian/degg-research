//! Divergence accounting, shrinking, and rendering.

use std::collections::BTreeMap;

use crate::adapter::{Divergence, compare, to_toy, toy_class, toy_slot, violations};
use dark_fba_toy as toy;
use degg_batch_oracle as mine;
use degg_batch_oracle::book::{Batch, Direction, Mode, Slot};
use degg_batch_oracle::params::SLOTS;

/// Every divergence class, in report order.
pub const CLASSES: [Divergence; 8] = [
    Divergence::AcceptanceDisagreement,
    Divergence::ClearingTick,
    Divergence::Volume,
    Divergence::Allocation,
    Divergence::OwnerOutput,
    Divergence::ExecutedMode,
    Divergence::PublicBinding,
    Divergence::RefusalClass,
];

fn class_index(divergence: Divergence) -> usize {
    CLASSES
        .iter()
        .position(|class| *class == divergence)
        .expect("class table is total")
}

/// The slot one of my refusal classes points at, if any.
fn our_slot(refusal: mine::admit::Refusal) -> Option<u8> {
    use mine::admit::Refusal as Mine;
    match refusal {
        Mine::DarkTargetUnavailable
        | Mine::AdmissionLogNotFinal
        | Mine::RootBindingAbsent
        | Mine::RootEquivocation
        | Mine::PayloadUnavailable
        | Mine::AccumulatorOverflow => None,
        Mine::BatchBindingMismatch { slot }
        | Mine::MarketBindingMismatch { slot }
        | Mine::OwnerOutOfDomain { slot }
        | Mine::LimitOutOfDomain { slot }
        | Mine::QuantityOutOfDomain { slot }
        | Mine::LateArrival { slot }
        | Mine::Unauthorized { slot }
        | Mine::Ineligible { slot }
        | Mine::InclusionAbsent { slot }
        | Mine::CustodyBindingAbsent { slot }
        | Mine::NullifierZero { slot }
        | Mine::ReservationInsufficient { slot }
        | Mine::NullifierRepeated { slot, .. } => Some(slot),
    }
}

/// The full divergence signature of a batch: the class, refined by the pair of
/// refusal classes where the class is a refusal-class disagreement.
///
/// Shrinking preserves the signature rather than only the class, so a minimal
/// witness still exhibits the exact disagreement it was recorded for.
pub fn signature(batch: &Batch) -> Option<String> {
    let ours = mine::evaluate(batch);
    let theirs = toy::evaluate(&to_toy(batch));
    let divergence = compare(batch, &ours, &theirs)?;
    if divergence == Divergence::RefusalClass
        && let (mine::Outcome::Refused(our_class), Err(their_class)) = (&ours, &theirs)
    {
        return Some(format!(
            "{} vs {}",
            our_class.class(),
            toy_class(their_class)
        ));
    }
    Some(format!("{divergence:?}"))
}

/// Running totals for one domain.
#[derive(Clone, Default)]
pub struct Report {
    /// Batches enumerated.
    pub cases: u64,
    /// Batches both oracles settled.
    pub settled: u64,
    /// Settled batches that cleared no trade.
    pub no_trade: u64,
    /// Batches both oracles refused.
    pub refused: u64,
    /// Divergences by class.
    pub by_class: [u64; CLASSES.len()],
    /// One witness per distinct divergence signature, for shrinking. The
    /// signature is the class, refined by the refusal-class pair where there is
    /// one, so a report keeps a minimal example of every distinct disagreement
    /// rather than several copies of the first one found.
    pub examples: Vec<(Divergence, Batch, String)>,
    /// For refusal-class divergences, how often each (independent, existing)
    /// class pair occurred.
    pub pairs: BTreeMap<(String, String), u64>,
    /// Refusal-class divergences where one oracle named a rule the witness does
    /// not actually violate. These would be genuine defects, not priority.
    pub unjustified: u64,
}

const EXAMPLES_PER_SIGNATURE: usize = 1;

impl Report {
    /// Record one enumerated case with no distinguishing label.
    pub fn observe(&mut self, batch: &Batch) {
        self.observe_labeled(batch, "");
    }

    /// Record one enumerated case.
    pub fn observe_labeled(&mut self, batch: &Batch, label: &str) {
        self.cases += 1;
        let ours = mine::evaluate(batch);
        let theirs = toy::evaluate(&to_toy(batch));
        match (&ours, &theirs) {
            (mine::Outcome::Settled(settlement), Ok(_)) => {
                self.settled += 1;
                if settlement.clearing == mine::curve::Clearing::NoTrade {
                    self.no_trade += 1;
                }
            }
            (mine::Outcome::Refused(_), Err(_)) => self.refused += 1,
            _ => {}
        }
        if let Some(divergence) = compare(batch, &ours, &theirs) {
            self.by_class[class_index(divergence)] += 1;
            let mut key = format!("{divergence:?}");
            if divergence == Divergence::RefusalClass
                && let (mine::Outcome::Refused(our_class), Err(their_class)) = (&ours, &theirs)
            {
                let our_tag = our_class.class().to_owned();
                let their_tag = toy_class(their_class).to_owned();
                key = format!("{our_tag} vs {their_tag}");
                *self
                    .pairs
                    .entry((our_tag.clone(), their_tag.clone()))
                    .or_default() += 1;
                let actual = violations(batch);
                let justified = |tag: &str, slot: Option<u8>| {
                    actual
                        .iter()
                        .any(|(found, at)| found == tag && (*at == slot || slot.is_none()))
                };
                if !justified(&our_tag, our_slot(*our_class))
                    || !justified(&their_tag, toy_slot(their_class))
                {
                    self.unjustified += 1;
                }
            }
            let held = self
                .examples
                .iter()
                .filter(|(_, _, held)| held.starts_with(&key))
                .count();
            if held < EXAMPLES_PER_SIGNATURE {
                self.examples
                    .push((divergence, *batch, format!("{key} :: {label}")));
            }
        }
    }

    /// Fold another report into this one.
    pub fn merge(&mut self, other: &Report) {
        self.cases += other.cases;
        self.settled += other.settled;
        self.no_trade += other.no_trade;
        self.refused += other.refused;
        for index in 0..CLASSES.len() {
            self.by_class[index] += other.by_class[index];
        }
        self.unjustified += other.unjustified;
        for (key, count) in &other.pairs {
            *self.pairs.entry(key.clone()).or_default() += count;
        }
        for (class, batch, label) in &other.examples {
            let signature = label.split(" :: ").next().unwrap_or(label);
            let held = self
                .examples
                .iter()
                .filter(|(_, _, held)| held.starts_with(signature))
                .count();
            if held < EXAMPLES_PER_SIGNATURE {
                self.examples.push((*class, *batch, label.clone()));
            }
        }
    }

    /// Total divergences across all classes.
    pub fn divergences(&self) -> u64 {
        self.by_class.iter().sum()
    }
}

/// A well-founded size measure on batches.
///
/// Shrinking only ever accepts a strictly smaller candidate, so the search
/// terminates whatever the candidate generator proposes.
fn measure(batch: &Batch) -> u128 {
    let mut size = 0u128;
    if batch.mode != Mode::Clear {
        size += 1;
    }
    let boundary = batch.boundary;
    for present in [
        boundary.log_final,
        boundary.root_binds_slots,
        boundary.no_conflicting_root,
        boundary.payloads_available,
    ] {
        if !present {
            size += 1;
        }
    }
    for (index, slot) in batch.slots.iter().enumerate() {
        let Slot::Taken(order) = slot else { continue };
        size += 1;
        size += u128::from(order.quantity);
        size += u128::from(order.limit_index);
        size += u128::from(order.reserved);
        size += u128::from(order.owner);
        size += u128::from(order.nullifier.abs_diff(index as u64 + 1));
        size += u128::from(order.arrival.abs_diff(batch.cutoff));
        size += u128::from(order.batch.abs_diff(batch.batch));
        size += u128::from(order.market.abs_diff(batch.market));
        for present in [
            order.authorized,
            order.eligible,
            order.included,
            order.custody_bound,
        ] {
            if !present {
                size += 1;
            }
        }
    }
    size
}

fn candidates(batch: &Batch) -> Vec<Batch> {
    let mut out = Vec::new();
    let mut push = |candidate: Batch| out.push(candidate);
    for slot in 0..SLOTS {
        let mut vacated = *batch;
        vacated.slots[slot] = Slot::Vacant;
        push(vacated);
        let Slot::Taken(order) = batch.slots[slot] else {
            continue;
        };
        // Quantity: the in-domain floor, the smallest out-of-domain value, and
        // one step down. The first two collapse huge witnesses immediately.
        for quantity in [1u32, 16, order.quantity.saturating_sub(1)] {
            let mut candidate = *batch;
            if let Slot::Taken(ref mut edit) = candidate.slots[slot] {
                edit.quantity = quantity;
            }
            push(candidate);
        }
        for limit in [0u8, 4, order.limit_index.saturating_sub(1)] {
            let mut candidate = *batch;
            if let Slot::Taken(ref mut edit) = candidate.slots[slot] {
                edit.limit_index = limit;
            }
            push(candidate);
        }
        let required = if usize::from(order.limit_index) < mine::params::TICKS {
            mine::admit::required_reservation(order.direction, order.limit_index, order.quantity)
        } else {
            0
        };
        for reserved in [0u64, required.saturating_sub(1), required] {
            let mut candidate = *batch;
            if let Slot::Taken(ref mut edit) = candidate.slots[slot] {
                edit.reserved = reserved;
            }
            push(candidate);
        }
        for owner in [0u8, 4] {
            let mut candidate = *batch;
            if let Slot::Taken(ref mut edit) = candidate.slots[slot] {
                edit.owner = owner;
            }
            push(candidate);
        }
        for nullifier in [slot as u64 + 1, 1, 0] {
            let mut candidate = *batch;
            if let Slot::Taken(ref mut edit) = candidate.slots[slot] {
                edit.nullifier = nullifier;
            }
            push(candidate);
        }
        for arrival in [batch.cutoff, batch.cutoff + 1] {
            let mut candidate = *batch;
            if let Slot::Taken(ref mut edit) = candidate.slots[slot] {
                edit.arrival = arrival;
            }
            push(candidate);
        }
        for (order_batch, order_market) in [
            (batch.batch, batch.market),
            (batch.batch + 1, batch.market),
            (batch.batch, batch.market + 1),
        ] {
            let mut candidate = *batch;
            if let Slot::Taken(ref mut edit) = candidate.slots[slot] {
                edit.batch = order_batch;
                edit.market = order_market;
            }
            push(candidate);
        }
    }
    let mut clear = *batch;
    clear.mode = Mode::Clear;
    push(clear);
    let mut satisfied = *batch;
    satisfied.boundary = degg_batch_oracle::book::Boundary::SATISFIED;
    push(satisfied);
    out
}

/// Greedily shrink a divergent batch while the same signature persists.
pub fn shrink(batch: &Batch, target: &str) -> Batch {
    let mut best = *batch;
    let mut size = measure(&best);
    loop {
        let mut improved = false;
        for candidate in candidates(&best) {
            let candidate_size = measure(&candidate);
            if candidate_size < size && signature(&candidate).as_deref() == Some(target) {
                best = candidate;
                size = candidate_size;
                improved = true;
                break;
            }
        }
        if !improved {
            return best;
        }
    }
}

/// One-line rendering of a batch, sufficient to rebuild it by hand.
pub fn describe(batch: &Batch) -> String {
    let mut parts = Vec::new();
    parts.push(format!("mode={:?}", batch.mode));
    if batch.boundary != degg_batch_oracle::book::Boundary::SATISFIED {
        parts.push(format!("boundary={:?}", batch.boundary));
    }
    for (index, slot) in batch.slots.iter().enumerate() {
        match slot {
            Slot::Vacant => parts.push(format!("s{index}=empty")),
            Slot::Taken(order) => {
                let side = match order.direction {
                    Direction::Buy => "buy",
                    Direction::Sell => "sell",
                };
                let mut field = format!(
                    "s{index}={side}(owner={},limit={},qty={},reserved={},nullifier={}",
                    order.owner, order.limit_index, order.quantity, order.reserved, order.nullifier
                );
                if order.arrival != batch.cutoff {
                    field.push_str(&format!(",arrival={}", order.arrival));
                }
                if !order.authorized {
                    field.push_str(",unauthorized");
                }
                if !order.eligible {
                    field.push_str(",ineligible");
                }
                if !order.included {
                    field.push_str(",not-included");
                }
                if !order.custody_bound {
                    field.push_str(",custody-unbound");
                }
                if order.batch != batch.batch {
                    field.push_str(",wrong-batch");
                }
                if order.market != batch.market {
                    field.push_str(",wrong-market");
                }
                field.push(')');
                parts.push(field);
            }
        }
    }
    parts.join(" ")
}

/// Both oracles' verdicts on one batch, rendered for a finding.
pub fn verdicts(batch: &Batch) -> String {
    let ours = match mine::evaluate(batch) {
        mine::Outcome::Refused(refusal) => format!("refused:{}", refusal.class()),
        mine::Outcome::Settled(settlement) => match settlement.clearing {
            mine::curve::Clearing::NoTrade => format!("no-trade fills={:?}", settlement.fills),
            mine::curve::Clearing::Trade { tick, volume, .. } => {
                format!("tick={tick} volume={volume} fills={:?}", settlement.fills)
            }
        },
    };
    let theirs = match toy::evaluate(&to_toy(batch)) {
        Err(refusal) => format!("refused:{refusal}"),
        Ok(execution) => {
            let public = execution.public_result();
            let mut fills = [0u64; SLOTS];
            for owner in 0..4u8 {
                if let Some(output) = execution.owner_output(owner) {
                    for fill in output.order_fills.iter().flatten() {
                        fills[usize::from(fill.slot)] = fill.quantity;
                    }
                }
            }
            match public.tick {
                toy::PublicTick::NoTrade => format!("no-trade fills={fills:?}"),
                toy::PublicTick::Tick(tick) => {
                    format!("tick={tick} volume={} fills={fills:?}", public.volume)
                }
            }
        }
    };
    format!("independent[{ours}] existing[{theirs}]")
}
