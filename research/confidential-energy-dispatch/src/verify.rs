//! Direct-recomputation verification of an untrusted solver candidate.
//!
//! This verifier is intentionally not succinct: after cheap binding and
//! feasibility checks, it runs the same bounded exhaustive oracle as the Clear
//! solver. That distinction is the experiment's central result. A future
//! proof-carrying or vFHE backend must refine this verdict byte-for-byte without
//! treating a signature or an asserted objective as a proof of computation.

use crate::canonical::{
    delivery_commitment, domain_commitment, inputs_commitment, plan_commitment, relation_commitment,
};
use crate::model::{DispatchRequest, Witness};
use crate::oracle::{
    ArithmeticError, CandidateBundle, PlanDefect, derive_deliveries, evaluate_plan,
    recompute_optimum,
};
use crate::surface::PublicStatus;

/// Why an untrusted candidate failed verification. This is verifier-local; a
/// deployment must separately freeze which failures, if any, become public.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerificationError {
    /// Public domain invalid.
    Domain(crate::model::DomainDefect),
    /// Admission root is not final.
    AdmissionNotFinal,
    /// Accepted payloads are unavailable.
    PayloadUnavailable,
    /// Witness does not open the accepted input commitment.
    InputCommitmentMismatch,
    /// Private witness invalid.
    Witness(crate::model::WitnessDefect),
    /// Candidate names another relation.
    RelationBindingMismatch,
    /// Candidate names another public domain.
    DomainBindingMismatch,
    /// Candidate names another accepted input set.
    InputBindingMismatch,
    /// Candidate is not labeled settled.
    StatusNotSettled,
    /// A settled candidate omitted one of the four invariant flags.
    PublicInvariantFlagFalse,
    /// Plan commitment does not open to the candidate plan.
    PlanCommitmentMismatch,
    /// Local-output commitment does not open to candidate delivery.
    DeliveryCommitmentMismatch,
    /// Certificate binding differs from the public candidate.
    CertificateBindingMismatch,
    /// Certificate objective differs from its plan.
    CertificateObjectiveMismatch,
    /// Explicit candidate fields fail feasibility or exact derivation.
    Plan(PlanDefect),
    /// Derived authorized local outputs differ.
    DeliveryMismatch,
    /// Checked arithmetic refused.
    Arithmetic(ArithmeticError),
    /// Valid inputs are infeasible, so no settled candidate can verify.
    NoFeasiblePlan,
    /// Candidate is feasible but is not the canonical global optimum.
    NotCanonicalOptimum,
    /// Exhaustive-search counters do not match. Counters are reproducibility
    /// evidence, not a shortcut for the actual optimality comparison.
    SearchTranscriptMismatch,
}

/// Verify all bindings, exact physical and settlement semantics, then global
/// optimality by direct bounded recomputation.
///
/// # Errors
///
/// Returns the first defect in the frozen verifier order. Success means this
/// bounded Clear verifier recomputed the same canonical optimum; it is not a
/// succinct or zero-knowledge proof.
pub fn verify_candidate(
    request: &DispatchRequest,
    witness: &Witness,
    candidate: &CandidateBundle,
) -> Result<(), VerificationError> {
    request
        .domain
        .validate()
        .map_err(VerificationError::Domain)?;
    if !request.admission_final {
        return Err(VerificationError::AdmissionNotFinal);
    }
    if !request.payloads_available {
        return Err(VerificationError::PayloadUnavailable);
    }
    if inputs_commitment(&request.domain, witness) != request.accepted_inputs {
        return Err(VerificationError::InputCommitmentMismatch);
    }
    witness.validate().map_err(VerificationError::Witness)?;

    if candidate.public.relation != relation_commitment() {
        return Err(VerificationError::RelationBindingMismatch);
    }
    if candidate.public.domain != domain_commitment(&request.domain) {
        return Err(VerificationError::DomainBindingMismatch);
    }
    if candidate.public.inputs != request.accepted_inputs {
        return Err(VerificationError::InputBindingMismatch);
    }
    if candidate.public.status != PublicStatus::Settled {
        return Err(VerificationError::StatusNotSettled);
    }
    if !candidate.public.demand_satisfied
        || !candidate.public.line_satisfied
        || !candidate.public.reserve_satisfied
        || !candidate.public.settlement_conserves
    {
        return Err(VerificationError::PublicInvariantFlagFalse);
    }

    let plan_hash = plan_commitment(request, &candidate.plan);
    if candidate.public.plan != plan_hash {
        return Err(VerificationError::PlanCommitmentMismatch);
    }
    let delivery_hash = delivery_commitment(request, &plan_hash, &candidate.delivery);
    if candidate.public.deliveries != delivery_hash {
        return Err(VerificationError::DeliveryCommitmentMismatch);
    }
    let certificate = candidate.certificate;
    if certificate.relation != candidate.public.relation
        || certificate.domain != candidate.public.domain
        || certificate.inputs != candidate.public.inputs
        || certificate.plan != candidate.public.plan
    {
        return Err(VerificationError::CertificateBindingMismatch);
    }
    if certificate.objective_cost != candidate.plan.objective_cost {
        return Err(VerificationError::CertificateObjectiveMismatch);
    }

    let derived = evaluate_plan(&request.domain, witness, candidate.plan.generation)
        .map_err(VerificationError::Plan)?;
    if derived != candidate.plan {
        return Err(VerificationError::Plan(settlement_or_derived_defect(
            &derived,
            &candidate.plan,
        )));
    }
    let delivery = derive_deliveries(witness, &derived).map_err(VerificationError::Arithmetic)?;
    if delivery != candidate.delivery {
        return Err(VerificationError::DeliveryMismatch);
    }

    let (optimum, visited_pairs, feasible_candidates) =
        recompute_optimum(&request.domain, witness).map_err(VerificationError::Arithmetic)?;
    let optimum = optimum.ok_or(VerificationError::NoFeasiblePlan)?;
    if optimum != candidate.plan {
        return Err(VerificationError::NotCanonicalOptimum);
    }
    if certificate.visited_pairs != visited_pairs
        || certificate.feasible_candidates != feasible_candidates
    {
        return Err(VerificationError::SearchTranscriptMismatch);
    }
    Ok(())
}

fn settlement_or_derived_defect(
    derived: &crate::oracle::Plan,
    claimed: &crate::oracle::Plan,
) -> PlanDefect {
    for provider in 0..crate::model::PROVIDERS {
        if derived.provider_credit[provider] != claimed.provider_credit[provider] {
            return PlanDefect::CreditMismatch {
                provider: u8::try_from(provider).expect("PROVIDERS fits u8"),
            };
        }
    }
    let claimed_credit_sum = claimed
        .provider_credit
        .iter()
        .try_fold(0u64, |total, credit| total.checked_add(*credit));
    if derived.load_debit != claimed.load_debit
        || derived.objective_cost != claimed.objective_cost
        || claimed_credit_sum != Some(claimed.load_debit)
    {
        return PlanDefect::SettlementNotConserving;
    }
    for period in 0..crate::model::PERIODS {
        if derived.line_flow[period] != claimed.line_flow[period] {
            return PlanDefect::FlowMismatch {
                period: u8::try_from(period).expect("PERIODS fits u8"),
            };
        }
        if derived.upward_reserve[period] != claimed.upward_reserve[period] {
            return PlanDefect::ReserveMismatch {
                period: u8::try_from(period).expect("PERIODS fits u8"),
            };
        }
    }
    PlanDefect::SettlementNotConserving
}
