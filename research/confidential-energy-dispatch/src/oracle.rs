//! Bounded exhaustive Clear oracle and exact settlement derivation.

use crate::canonical::{
    delivery_commitment, domain_commitment, inputs_commitment, plan_commitment, relation_commitment,
};
use crate::model::{
    DispatchRequest, MAX_OUTPUT, PERIODS, PROVIDERS, ProviderInput, PublicDomain, Witness,
};
use crate::surface::{PublicOutcome, PublicStatus};

// `(MAX_OUTPUT + 1)^PERIODS = 5^3`; kept literal so the array length is an
// uncomplicated compile-time constant on every supported target.
const TRAJECTORY_COUNT: usize = 125;

/// Exact plan and settlement. Provider-major generation order also defines the
/// deterministic tie: at equal cost, a larger earliest-slot dispatch wins.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Plan {
    /// Provider-major generation atoms.
    pub generation: [[u8; PERIODS]; PROVIDERS],
    /// Lossless line flow; positive is bus 0 to bus 1.
    pub line_flow: [i16; PERIODS],
    /// Exact upward reserve under the frozen reserve rule.
    pub upward_reserve: [u8; PERIODS],
    /// Pay-as-cost private provider credits.
    pub provider_credit: [u64; PROVIDERS],
    /// Single load-side debit, equal to all provider credits.
    pub load_debit: u64,
    /// Total production cost and optimization objective.
    pub objective_cost: u64,
}

/// One fixed-size provider-local output.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ProviderLocalOutput {
    /// Whether this is an occupied provider rather than padding.
    pub present: bool,
    /// Private delivery-recipient binding.
    pub recipient_binding: [u8; 32],
    /// Authorized three-period dispatch.
    pub dispatch: [u8; PERIODS],
    /// Exact pay-as-cost credit across the horizon.
    pub credit: u64,
}

/// Load-side local output.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LoadLocalOutput {
    /// Exact debit funding all provider credits.
    pub debit: u64,
    /// Total energy served across all periods and buses.
    pub served_energy: u64,
}

/// Fixed-size private output delivery set.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PrivateDelivery {
    /// Three padded provider outputs.
    pub providers: [ProviderLocalOutput; PROVIDERS],
    /// Aggregate load-side output.
    pub load: LoadLocalOutput,
}

/// What the exhaustive verifier binds. `visited_pairs` and
/// `feasible_candidates` are private diagnostics, not proof soundness inputs
/// and never occur in the public frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OptimalityCertificate {
    /// Relation binding.
    pub relation: [u8; 32],
    /// Domain binding.
    pub domain: [u8; 32],
    /// Private input binding.
    pub inputs: [u8; 32],
    /// Claimed canonical plan binding.
    pub plan: [u8; 32],
    /// Claimed exact objective.
    pub objective_cost: u64,
    /// Number of first-two-provider trajectory pairs visited by the oracle.
    pub visited_pairs: u32,
    /// Number of feasible complete schedules found.
    pub feasible_candidates: u32,
}

/// A solver-supplied plan, delivery set, commitments, and certificate. This
/// structure is untrusted until [`crate::verify::verify_candidate`] succeeds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CandidateBundle {
    /// Candidate plan.
    pub plan: Plan,
    /// Candidate local outputs.
    pub delivery: PrivateDelivery,
    /// Candidate public outcome.
    pub public: PublicOutcome,
    /// Candidate optimality claim.
    pub certificate: OptimalityCertificate,
}

/// Detailed private diagnostic. Only [`PublicStatus`] is publicly projected.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrivateDiagnostic {
    /// Public-domain validation failed.
    Domain(crate::model::DomainDefect),
    /// Admission root not final.
    AdmissionNotFinal,
    /// One or more accepted payloads unavailable.
    PayloadUnavailable,
    /// Witness did not open the accepted-input commitment.
    InputCommitmentMismatch,
    /// Private provider semantics invalid.
    Witness(crate::model::WitnessDefect),
    /// No feasible plan exists.
    Infeasible,
    /// Checked arithmetic refused.
    Arithmetic(ArithmeticError),
    /// Canonical optimum settled.
    Settled,
}

/// Whole Clear execution result. Private fields are not a public transcript.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Execution {
    /// Fixed public projection.
    pub public: PublicOutcome,
    /// Candidate and local outputs only on success.
    pub candidate: Option<CandidateBundle>,
    /// Clear-executor-only diagnostic.
    pub private_diagnostic: PrivateDiagnostic,
}

/// Checked arithmetic or impossible internal state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArithmeticError {
    /// Exact production cost overflowed.
    CostOverflow,
    /// Aggregate objective overflowed.
    ObjectiveOverflow,
    /// Reserve aggregation overflowed.
    ReserveOverflow,
    /// Candidate counter overflowed.
    CounterOverflow,
    /// Settlement totals did not conserve.
    SettlementInvariant,
}

/// Feasibility defect for an explicit candidate plan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanDefect {
    /// Output violates availability, capacity, minimum, or ramp semantics.
    ProviderTrajectory { provider: u8 },
    /// Nodal energy balance fails.
    DemandBalance { period: u8 },
    /// Candidate flow is not the uniquely derived lossless flow.
    FlowMismatch { period: u8 },
    /// Line capacity exceeded.
    LineLimit { period: u8 },
    /// Claimed reserve differs from the relation-derived reserve.
    ReserveMismatch { period: u8 },
    /// Required reserve not met.
    ReserveShortfall { period: u8 },
    /// Provider credit does not equal exact production cost.
    CreditMismatch { provider: u8 },
    /// Load debit, credit sum, and objective do not agree exactly.
    SettlementNotConserving,
    /// Checked arithmetic refused.
    Arithmetic(ArithmeticError),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Trajectory {
    output: [u8; PERIODS],
    cost: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct SearchStats {
    visited_pairs: u32,
    feasible_candidates: u32,
}

fn output_allowed(provider: &ProviderInput, period: usize, output: u8) -> bool {
    if !provider.occupied {
        return output == 0;
    }
    if !provider.available[period] {
        return output == 0;
    }
    output == 0 || (output >= provider.min_output && output <= provider.max_output)
}

fn ramp_allowed(provider: &ProviderInput, output: [u8; PERIODS]) -> bool {
    if !provider.occupied {
        return output == [0; PERIODS];
    }
    if provider.available[0] {
        if output[0] >= provider.initial_output {
            if output[0] - provider.initial_output > provider.ramp_up {
                return false;
            }
        } else if provider.initial_output - output[0] > provider.ramp_down {
            return false;
        }
    }
    for period in 1..PERIODS {
        // A forced outage boundary bypasses ordinary ramping. This is not an
        // assertion that the plant can ramp physically; it distinguishes a
        // forced-unavailable state from an elective transition.
        if !provider.available[period - 1] || !provider.available[period] {
            continue;
        }
        if output[period] >= output[period - 1] {
            if output[period] - output[period - 1] > provider.ramp_up {
                return false;
            }
        } else if output[period - 1] - output[period] > provider.ramp_down {
            return false;
        }
    }
    true
}

fn production_cost(provider: &ProviderInput, output: u8) -> Result<u64, ArithmeticError> {
    let mut remaining = output;
    let mut cost = 0u64;
    for segment in provider.segments {
        let used = remaining.min(segment.width);
        cost = cost
            .checked_add(u64::from(used) * u64::from(segment.marginal_cost))
            .ok_or(ArithmeticError::CostOverflow)?;
        remaining -= used;
    }
    if remaining != 0 {
        return Err(ArithmeticError::CostOverflow);
    }
    Ok(cost)
}

fn trajectory_cost(
    provider: &ProviderInput,
    output: [u8; PERIODS],
) -> Result<u64, ArithmeticError> {
    let mut cost = 0u64;
    for amount in output {
        cost = cost
            .checked_add(production_cost(provider, amount)?)
            .ok_or(ArithmeticError::CostOverflow)?;
    }
    Ok(cost)
}

fn trajectory_index(output: [u8; PERIODS]) -> usize {
    let radix = usize::from(MAX_OUTPUT) + 1;
    output
        .into_iter()
        .fold(0usize, |index, value| index * radix + usize::from(value))
}

fn trajectories(
    provider: &ProviderInput,
) -> Result<[Option<Trajectory>; TRAJECTORY_COUNT], ArithmeticError> {
    let mut table = [None; TRAJECTORY_COUNT];
    for a in 0..=MAX_OUTPUT {
        for b in 0..=MAX_OUTPUT {
            for c in 0..=MAX_OUTPUT {
                let output = [a, b, c];
                if output
                    .iter()
                    .enumerate()
                    .all(|(period, value)| output_allowed(provider, period, *value))
                    && ramp_allowed(provider, output)
                {
                    let trajectory = Trajectory {
                        output,
                        cost: trajectory_cost(provider, output)?,
                    };
                    table[trajectory_index(output)] = Some(trajectory);
                }
            }
        }
    }
    Ok(table)
}

fn reserve_capability(provider: &ProviderInput, period: usize, output: u8) -> u8 {
    if !provider.occupied || !provider.available[period] {
        return 0;
    }
    let headroom = provider.max_output - output;
    if output == 0 && provider.min_output > provider.ramp_up {
        0
    } else {
        headroom.min(provider.ramp_up)
    }
}

fn derive_physical(
    domain: &PublicDomain,
    witness: &Witness,
    generation: &[[u8; PERIODS]; PROVIDERS],
) -> Result<([i16; PERIODS], [u8; PERIODS]), PlanDefect> {
    for (index, provider) in witness.providers.iter().enumerate() {
        let output = generation[index];
        if !output
            .iter()
            .enumerate()
            .all(|(period, value)| output_allowed(provider, period, *value))
            || !ramp_allowed(provider, output)
        {
            return Err(PlanDefect::ProviderTrajectory {
                provider: u8::try_from(index).expect("PROVIDERS fits u8"),
            });
        }
    }

    let mut flow = [0i16; PERIODS];
    let mut reserve = [0u8; PERIODS];
    for period in 0..PERIODS {
        let total_generation: u16 = generation.iter().map(|row| u16::from(row[period])).sum();
        let total_demand =
            u16::from(domain.demand[period][0]) + u16::from(domain.demand[period][1]);
        if total_generation != total_demand {
            return Err(PlanDefect::DemandBalance {
                period: u8::try_from(period).expect("PERIODS fits u8"),
            });
        }
        let bus_zero: i16 = witness
            .providers
            .iter()
            .zip(generation)
            .filter(|(provider, _)| provider.occupied && provider.bus == 0)
            .map(|(_, row)| i16::from(row[period]))
            .sum();
        flow[period] = bus_zero - i16::from(domain.demand[period][0]);
        if flow[period].unsigned_abs() > u16::from(domain.line_limit[period]) {
            return Err(PlanDefect::LineLimit {
                period: u8::try_from(period).expect("PERIODS fits u8"),
            });
        }
        for (provider, row) in witness.providers.iter().zip(generation) {
            reserve[period] = reserve[period]
                .checked_add(reserve_capability(provider, period, row[period]))
                .ok_or(PlanDefect::Arithmetic(ArithmeticError::ReserveOverflow))?;
        }
        if reserve[period] < domain.reserve[period] {
            return Err(PlanDefect::ReserveShortfall {
                period: u8::try_from(period).expect("PERIODS fits u8"),
            });
        }
    }
    Ok((flow, reserve))
}

/// Evaluate one explicit generation matrix. This checks feasibility and derives
/// every cost, flow, reserve, and settlement field; it does not prove optimality.
///
/// # Errors
///
/// Returns a typed physical, reserve, settlement, or checked-arithmetic defect.
pub fn evaluate_plan(
    domain: &PublicDomain,
    witness: &Witness,
    generation: [[u8; PERIODS]; PROVIDERS],
) -> Result<Plan, PlanDefect> {
    let (line_flow, upward_reserve) = derive_physical(domain, witness, &generation)?;
    let mut provider_credit = [0u64; PROVIDERS];
    let mut objective_cost = 0u64;
    for provider in 0..PROVIDERS {
        provider_credit[provider] =
            trajectory_cost(&witness.providers[provider], generation[provider])
                .map_err(PlanDefect::Arithmetic)?;
        objective_cost = objective_cost
            .checked_add(provider_credit[provider])
            .ok_or(PlanDefect::Arithmetic(ArithmeticError::ObjectiveOverflow))?;
    }
    Ok(Plan {
        generation,
        line_flow,
        upward_reserve,
        provider_credit,
        load_debit: objective_cost,
        objective_cost,
    })
}

fn tie_precedes(candidate: &Plan, incumbent: &Plan) -> bool {
    candidate
        .generation
        .iter()
        .flatten()
        .cmp(incumbent.generation.iter().flatten())
        .is_gt()
}

fn optimum(
    domain: &PublicDomain,
    witness: &Witness,
) -> Result<(Option<Plan>, SearchStats), ArithmeticError> {
    let tables = [
        trajectories(&witness.providers[0])?,
        trajectories(&witness.providers[1])?,
        trajectories(&witness.providers[2])?,
    ];
    let first: Vec<Trajectory> = tables[0].iter().flatten().copied().collect();
    let second: Vec<Trajectory> = tables[1].iter().flatten().copied().collect();
    let mut best: Option<Plan> = None;
    let mut stats = SearchStats::default();

    for left in &first {
        for middle in &second {
            stats.visited_pairs = stats
                .visited_pairs
                .checked_add(1)
                .ok_or(ArithmeticError::CounterOverflow)?;
            let mut right = [0u8; PERIODS];
            let mut in_domain = true;
            for (period, right_output) in right.iter_mut().enumerate() {
                let demand =
                    u16::from(domain.demand[period][0]) + u16::from(domain.demand[period][1]);
                let used = u16::from(left.output[period]) + u16::from(middle.output[period]);
                let Some(remainder) = demand.checked_sub(used) else {
                    in_domain = false;
                    break;
                };
                let Ok(remainder) = u8::try_from(remainder) else {
                    in_domain = false;
                    break;
                };
                if remainder > MAX_OUTPUT {
                    in_domain = false;
                    break;
                }
                *right_output = remainder;
            }
            if !in_domain {
                continue;
            }
            let Some(right) = tables[2][trajectory_index(right)] else {
                continue;
            };
            let generation = [left.output, middle.output, right.output];
            let Ok(plan) = evaluate_plan(domain, witness, generation) else {
                continue;
            };
            stats.feasible_candidates = stats
                .feasible_candidates
                .checked_add(1)
                .ok_or(ArithmeticError::CounterOverflow)?;
            if best.is_none_or(|incumbent| {
                plan.objective_cost < incumbent.objective_cost
                    || (plan.objective_cost == incumbent.objective_cost
                        && tie_precedes(&plan, &incumbent))
            }) {
                best = Some(plan);
            }
        }
    }
    Ok((best, stats))
}

fn deliveries(witness: &Witness, plan: &Plan) -> Result<PrivateDelivery, ArithmeticError> {
    let delivery = untrusted_deliveries(witness, plan)?;
    let credits = plan
        .provider_credit
        .iter()
        .try_fold(0u64, |total, value| total.checked_add(*value))
        .ok_or(ArithmeticError::ObjectiveOverflow)?;
    if credits != plan.load_debit || credits != plan.objective_cost {
        return Err(ArithmeticError::SettlementInvariant);
    }
    Ok(delivery)
}

fn untrusted_deliveries(
    witness: &Witness,
    plan: &Plan,
) -> Result<PrivateDelivery, ArithmeticError> {
    let mut providers = [ProviderLocalOutput::default(); PROVIDERS];
    for (index, local) in providers.iter_mut().enumerate() {
        let provider = witness.providers[index];
        *local = ProviderLocalOutput {
            present: provider.occupied,
            recipient_binding: provider.owner_binding,
            dispatch: plan.generation[index],
            credit: plan.provider_credit[index],
        };
        if !provider.occupied && *local != ProviderLocalOutput::default() {
            return Err(ArithmeticError::SettlementInvariant);
        }
    }
    let served_energy = plan
        .generation
        .iter()
        .flatten()
        .try_fold(0u64, |total, value| total.checked_add(u64::from(*value)))
        .ok_or(ArithmeticError::ObjectiveOverflow)?;
    Ok(PrivateDelivery {
        providers,
        load: LoadLocalOutput {
            debit: plan.load_debit,
            served_energy,
        },
    })
}

/// Seal an arbitrary untrusted plan into internally binding commitments. This
/// helper intentionally does **not** make the plan valid or optimal; it exists
/// so adversarial tests can present coherent forgeries to the verifier.
///
/// # Errors
///
/// Returns only if construction of the fixed-size untrusted delivery set
/// exceeds the model's checked integer bounds.
pub fn seal_untrusted_plan(
    request: &DispatchRequest,
    witness: &Witness,
    plan: Plan,
    visited_pairs: u32,
    feasible_candidates: u32,
) -> Result<CandidateBundle, ArithmeticError> {
    let delivery = untrusted_deliveries(witness, &plan)?;
    let plan_hash = plan_commitment(request, &plan);
    let delivery_hash = delivery_commitment(request, &plan_hash, &delivery);
    let public = PublicOutcome {
        relation: relation_commitment(),
        domain: domain_commitment(&request.domain),
        inputs: request.accepted_inputs,
        plan: plan_hash,
        deliveries: delivery_hash,
        status: PublicStatus::Settled,
        demand_satisfied: true,
        line_satisfied: true,
        reserve_satisfied: true,
        settlement_conserves: true,
    };
    Ok(CandidateBundle {
        plan,
        delivery,
        public,
        certificate: OptimalityCertificate {
            relation: public.relation,
            domain: public.domain,
            inputs: public.inputs,
            plan: public.plan,
            objective_cost: plan.objective_cost,
            visited_pairs,
            feasible_candidates,
        },
    })
}

fn arithmetic_refusal(request: &DispatchRequest, error: ArithmeticError) -> Execution {
    Execution {
        public: PublicOutcome::refused(request, PublicStatus::ArithmeticRefused),
        candidate: None,
        private_diagnostic: PrivateDiagnostic::Arithmetic(error),
    }
}

/// Execute the exact Clear oracle. Every failure returns one fixed-shape public
/// outcome and a separate Clear-executor-only diagnostic.
#[must_use]
pub fn solve_clear(request: &DispatchRequest, witness: &Witness) -> Execution {
    if let Err(defect) = request.domain.validate() {
        return Execution {
            public: PublicOutcome::refused(request, PublicStatus::MalformedPublic),
            candidate: None,
            private_diagnostic: PrivateDiagnostic::Domain(defect),
        };
    }
    if !request.admission_final {
        return Execution {
            public: PublicOutcome::refused(request, PublicStatus::InputUnavailable),
            candidate: None,
            private_diagnostic: PrivateDiagnostic::AdmissionNotFinal,
        };
    }
    if !request.payloads_available {
        return Execution {
            public: PublicOutcome::refused(request, PublicStatus::InputUnavailable),
            candidate: None,
            private_diagnostic: PrivateDiagnostic::PayloadUnavailable,
        };
    }
    if inputs_commitment(&request.domain, witness) != request.accepted_inputs {
        return Execution {
            public: PublicOutcome::refused(request, PublicStatus::WitnessRefused),
            candidate: None,
            private_diagnostic: PrivateDiagnostic::InputCommitmentMismatch,
        };
    }
    if let Err(defect) = witness.validate() {
        return Execution {
            public: PublicOutcome::refused(request, PublicStatus::WitnessRefused),
            candidate: None,
            private_diagnostic: PrivateDiagnostic::Witness(defect),
        };
    }
    let (plan, stats) = match optimum(&request.domain, witness) {
        Ok(result) => result,
        Err(error) => return arithmetic_refusal(request, error),
    };
    let Some(plan) = plan else {
        return Execution {
            public: PublicOutcome::refused(request, PublicStatus::Infeasible),
            candidate: None,
            private_diagnostic: PrivateDiagnostic::Infeasible,
        };
    };
    let candidate = match seal_untrusted_plan(
        request,
        witness,
        plan,
        stats.visited_pairs,
        stats.feasible_candidates,
    ) {
        Ok(candidate) => candidate,
        Err(error) => return arithmetic_refusal(request, error),
    };
    Execution {
        public: candidate.public,
        candidate: Some(candidate),
        private_diagnostic: PrivateDiagnostic::Settled,
    }
}

pub(crate) fn recompute_optimum(
    domain: &PublicDomain,
    witness: &Witness,
) -> Result<(Option<Plan>, u32, u32), ArithmeticError> {
    let (plan, stats) = optimum(domain, witness)?;
    Ok((plan, stats.visited_pairs, stats.feasible_candidates))
}

pub(crate) fn derive_deliveries(
    witness: &Witness,
    plan: &Plan,
) -> Result<PrivateDelivery, ArithmeticError> {
    deliveries(witness, plan)
}
