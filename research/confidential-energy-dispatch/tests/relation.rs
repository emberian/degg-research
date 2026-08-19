use degg_confidential_energy_dispatch::canonical::{
    PRIVATE_WITNESS_LEN, inputs_commitment, witness_bytes,
};
use degg_confidential_energy_dispatch::model::{
    CostSegment, MAX_MARGINAL_COST, ProviderInput, PublicDomain, Witness, WitnessDefect,
};
use degg_confidential_energy_dispatch::oracle::{
    PlanDefect, PrivateDiagnostic, evaluate_plan, seal_untrusted_plan, solve_clear,
};
use degg_confidential_energy_dispatch::surface::{PUBLIC_FRAME_LEN, PublicStatus};
use degg_confidential_energy_dispatch::transcript::balanced_fixture;
use degg_confidential_energy_dispatch::verify::{VerificationError, verify_candidate};
use degg_confidential_energy_dispatch::{DispatchRequest, PERIODS};

fn request_for(domain: PublicDomain, witness: &Witness) -> DispatchRequest {
    DispatchRequest {
        domain,
        accepted_inputs: inputs_commitment(&domain, witness),
        admission_final: true,
        payloads_available: true,
    }
}

fn cheap_provider(owner: u8, bus: u8, marginal_cost: u32) -> ProviderInput {
    ProviderInput {
        occupied: true,
        owner_binding: [owner; 32],
        bus,
        min_output: 0,
        max_output: 4,
        ramp_up: 4,
        ramp_down: 4,
        initial_output: 0,
        available: [true; PERIODS],
        segments: [
            CostSegment {
                width: 4,
                marginal_cost,
            },
            CostSegment {
                width: 0,
                marginal_cost,
            },
        ],
    }
}

fn tie_fixture() -> (DispatchRequest, Witness) {
    let witness = Witness {
        providers: [
            cheap_provider(1, 0, 7),
            cheap_provider(2, 0, 7),
            ProviderInput::padding(),
        ],
    };
    let domain = PublicDomain {
        instance: [9; 32],
        epoch: 7,
        demand: [[1, 0]; PERIODS],
        reserve: [0; PERIODS],
        line_limit: [0; PERIODS],
    };
    (request_for(domain, &witness), witness)
}

#[test]
fn balanced_clear_oracle_settles_and_direct_verifier_agrees() {
    let (request, witness) = balanced_fixture();
    let execution = solve_clear(&request, &witness);
    assert_eq!(execution.public.status, PublicStatus::Settled);
    let candidate = execution.candidate.expect("settled candidate");
    assert_eq!(candidate.plan.generation, [[2, 2, 2], [1, 3, 2], [2, 0, 2]]);
    assert_eq!(candidate.plan.line_flow, [0, -1, 0]);
    assert_eq!(candidate.plan.upward_reserve, [6, 3, 6]);
    assert_eq!(candidate.plan.provider_credit, [18, 30, 8]);
    assert_eq!(candidate.plan.load_debit, 56);
    assert_eq!(candidate.plan.objective_cost, 56);
    assert_eq!(candidate.certificate.visited_pairs, 8_025);
    assert_eq!(candidate.certificate.feasible_candidates, 468);
    assert_eq!(verify_candidate(&request, &witness, &candidate), Ok(()));
}

#[test]
fn omitted_or_substituted_provider_does_not_open_the_accepted_input_set() {
    let (request, mut witness) = balanced_fixture();
    witness.providers[2] = ProviderInput::padding();
    let execution = solve_clear(&request, &witness);
    assert_eq!(execution.public.status, PublicStatus::WitnessRefused);
    assert_eq!(
        execution.private_diagnostic,
        PrivateDiagnostic::InputCommitmentMismatch
    );
}

#[test]
fn infeasible_demand_is_not_reinterpreted_as_a_partial_plan() {
    let (_, witness) = balanced_fixture();
    let domain = PublicDomain {
        instance: [0x51; 32],
        epoch: 99,
        demand: [[2, 3], [6, 6], [2, 4]],
        reserve: [0; PERIODS],
        line_limit: [12; PERIODS],
    };
    let request = request_for(domain, &witness);
    let execution = solve_clear(&request, &witness);
    assert_eq!(execution.public.status, PublicStatus::Infeasible);
    assert_eq!(execution.candidate, None);
    assert_eq!(execution.private_diagnostic, PrivateDiagnostic::Infeasible);
}

#[test]
fn public_frame_shape_is_fixed_and_contains_no_search_or_exact_timing_field() {
    let (settled_request, witness) = balanced_fixture();
    let settled = solve_clear(&settled_request, &witness);

    let mut unavailable_request = settled_request;
    unavailable_request.payloads_available = false;
    let unavailable = solve_clear(&unavailable_request, &witness);

    let mut infeasible_domain = settled_request.domain;
    infeasible_domain.demand[1] = [6, 6];
    infeasible_domain.line_limit = [12; PERIODS];
    let infeasible_request = request_for(infeasible_domain, &witness);
    let infeasible = solve_clear(&infeasible_request, &witness);

    for frame in [
        settled.public.frame(),
        unavailable.public.frame(),
        infeasible.public.frame(),
    ] {
        assert_eq!(frame.len(), PUBLIC_FRAME_LEN);
        assert_eq!(&frame[..8], b"DEGGEDV0");
        assert_eq!(&frame[13..16], &[0; 3]);
    }
    assert_eq!(unavailable.public.status, PublicStatus::InputUnavailable);
    assert_eq!(infeasible.public.status, PublicStatus::Infeasible);
    // Iteration counters exist only in the private candidate certificate. The
    // model makes no constant-time claim about this Clear implementation.
    assert!(settled.candidate.is_some());
    assert!(unavailable.candidate.is_none());
    assert!(infeasible.candidate.is_none());
}

#[test]
fn occupied_and_padded_plaintext_witnesses_have_one_canonical_shape() {
    let (_, balanced) = balanced_fixture();
    let (_, partly_padded) = tie_fixture();
    let all_padding = Witness {
        providers: [ProviderInput::padding(); 3],
    };
    for witness in [balanced, partly_padded, all_padding] {
        assert_eq!(witness_bytes(&witness).len(), PRIVATE_WITNESS_LEN);
    }
}

#[test]
fn feasible_but_more_expensive_plan_is_rejected_as_not_optimal() {
    let (request, witness) = balanced_fixture();
    let canonical = solve_clear(&request, &witness)
        .candidate
        .expect("canonical optimum");
    let more_expensive =
        evaluate_plan(&request.domain, &witness, [[1, 2, 1], [2, 3, 3], [2, 0, 2]])
            .expect("physically feasible alternative");
    assert!(more_expensive.objective_cost > canonical.plan.objective_cost);
    let forged = seal_untrusted_plan(
        &request,
        &witness,
        more_expensive,
        canonical.certificate.visited_pairs,
        canonical.certificate.feasible_candidates,
    )
    .expect("coherent untrusted bundle");
    assert_eq!(
        verify_candidate(&request, &witness, &forged),
        Err(VerificationError::NotCanonicalOptimum)
    );
}

#[test]
fn settlement_nonconservation_is_rejected_even_when_recommitted() {
    let (request, witness) = balanced_fixture();
    let canonical = solve_clear(&request, &witness)
        .candidate
        .expect("canonical optimum");
    let mut plan = canonical.plan;
    plan.load_debit += 1;
    let forged = seal_untrusted_plan(
        &request,
        &witness,
        plan,
        canonical.certificate.visited_pairs,
        canonical.certificate.feasible_candidates,
    )
    .expect("untrusted assembler permits the coherent forgery");
    assert_eq!(
        verify_candidate(&request, &witness, &forged),
        Err(VerificationError::Plan(PlanDefect::SettlementNotConserving))
    );
}

#[test]
fn overflow_shaped_cost_is_refused_before_any_cost_multiplication() {
    let (_, mut witness) = balanced_fixture();
    witness.providers[0].segments[1].marginal_cost = MAX_MARGINAL_COST + 1;
    let domain = PublicDomain {
        instance: [0x62; 32],
        epoch: 1,
        demand: [[0, 0]; PERIODS],
        reserve: [0; PERIODS],
        line_limit: [0; PERIODS],
    };
    let request = request_for(domain, &witness);
    let execution = solve_clear(&request, &witness);
    assert_eq!(execution.public.status, PublicStatus::WitnessRefused);
    assert_eq!(
        execution.private_diagnostic,
        PrivateDiagnostic::Witness(WitnessDefect::MarginalCostOutOfDomain { slot: 0 })
    );
}

#[test]
fn untrusted_max_integer_settlement_refuses_without_wrapping_or_panicking() {
    let (request, witness) = balanced_fixture();
    let canonical = solve_clear(&request, &witness)
        .candidate
        .expect("candidate");
    let mut plan = canonical.plan;
    plan.provider_credit = [u64::MAX, u64::MAX, u64::MAX];
    plan.load_debit = u64::MAX;
    plan.objective_cost = u64::MAX;
    let forged = seal_untrusted_plan(
        &request,
        &witness,
        plan,
        canonical.certificate.visited_pairs,
        canonical.certificate.feasible_candidates,
    )
    .expect("encoding untrusted integers does not add them");
    assert_eq!(
        verify_candidate(&request, &witness, &forged),
        Err(VerificationError::Plan(PlanDefect::CreditMismatch {
            provider: 0
        }))
    );
}

#[test]
fn equal_cost_tie_dispatches_earliest_provider_first() {
    let (request, witness) = tie_fixture();
    let candidate = solve_clear(&request, &witness)
        .candidate
        .expect("feasible tie");
    assert_eq!(candidate.plan.generation[0], [1, 1, 1]);
    assert_eq!(candidate.plan.generation[1], [0, 0, 0]);
    assert_eq!(candidate.plan.generation[2], [0, 0, 0]);
    assert_eq!(verify_candidate(&request, &witness, &candidate), Ok(()));
}

#[test]
fn replay_under_a_changed_epoch_and_fresh_input_root_still_fails_domain_binding() {
    let (request, witness) = balanced_fixture();
    let candidate = solve_clear(&request, &witness)
        .candidate
        .expect("candidate");
    let mut changed = request;
    changed.domain.epoch += 1;
    changed.accepted_inputs = inputs_commitment(&changed.domain, &witness);
    assert_eq!(
        verify_candidate(&changed, &witness, &candidate),
        Err(VerificationError::DomainBindingMismatch)
    );
}

#[test]
fn certificate_objective_and_search_transcript_are_checked_not_trusted() {
    let (request, witness) = balanced_fixture();
    let mut objective_forgery = solve_clear(&request, &witness)
        .candidate
        .expect("candidate");
    objective_forgery.certificate.objective_cost += 1;
    assert_eq!(
        verify_candidate(&request, &witness, &objective_forgery),
        Err(VerificationError::CertificateObjectiveMismatch)
    );

    let mut counter_forgery = solve_clear(&request, &witness)
        .candidate
        .expect("candidate");
    counter_forgery.certificate.feasible_candidates += 1;
    assert_eq!(
        verify_candidate(&request, &witness, &counter_forgery),
        Err(VerificationError::SearchTranscriptMismatch)
    );
}

#[test]
fn minimum_output_defeats_naive_cheapest_atom_dispatch() {
    let mut cheap = cheap_provider(1, 0, 1);
    cheap.min_output = 2;
    let witness = Witness {
        providers: [cheap, cheap_provider(2, 0, 9), ProviderInput::padding()],
    };
    let domain = PublicDomain {
        instance: [0x69; 32],
        epoch: 2,
        demand: [[1, 0]; PERIODS],
        reserve: [0; PERIODS],
        line_limit: [0; PERIODS],
    };
    let request = request_for(domain, &witness);
    let plan = solve_clear(&request, &witness)
        .candidate
        .expect("expensive provider makes the plan feasible")
        .plan;
    assert_eq!(plan.generation[0], [0; PERIODS]);
    assert_eq!(plan.generation[1], [1; PERIODS]);
    assert_eq!(plan.objective_cost, 27);
}

#[test]
fn interperiod_ramp_coupling_defeats_independent_period_merit_order() {
    let mut cheap = cheap_provider(1, 0, 1);
    cheap.initial_output = 1;
    cheap.ramp_up = 1;
    cheap.ramp_down = 1;
    let witness = Witness {
        providers: [cheap, cheap_provider(2, 0, 10), ProviderInput::padding()],
    };
    let domain = PublicDomain {
        instance: [0x6a; 32],
        epoch: 2,
        demand: [[1, 0], [3, 0], [1, 0]],
        reserve: [0; PERIODS],
        line_limit: [0; PERIODS],
    };
    let request = request_for(domain, &witness);
    let plan = solve_clear(&request, &witness)
        .candidate
        .expect("coupled plan exists")
        .plan;
    assert_eq!(plan.generation[0], [1, 2, 1]);
    assert_eq!(plan.generation[1], [0, 1, 0]);
    assert_eq!(plan.objective_cost, 14);
    assert_eq!(
        evaluate_plan(&domain, &witness, [[1, 3, 1], [0; 3], [0; 3]]),
        Err(PlanDefect::ProviderTrajectory { provider: 0 })
    );
}

#[test]
fn forced_outage_boundaries_bypass_ramp_but_elective_transition_does_not() {
    let mut provider = cheap_provider(1, 0, 1);
    provider.min_output = 2;
    provider.initial_output = 2;
    provider.ramp_up = 0;
    provider.ramp_down = 0;
    provider.available = [true, false, true];
    let outage_witness = Witness {
        providers: [provider, ProviderInput::padding(), ProviderInput::padding()],
    };
    let domain = PublicDomain {
        instance: [0x71; 32],
        epoch: 5,
        demand: [[2, 0], [0, 0], [2, 0]],
        reserve: [0; PERIODS],
        line_limit: [0; PERIODS],
    };
    assert!(evaluate_plan(&domain, &outage_witness, [[2, 0, 2], [0; 3], [0; 3]]).is_ok());

    let mut elective_witness = outage_witness;
    elective_witness.providers[0].available = [true; PERIODS];
    assert_eq!(
        evaluate_plan(&domain, &elective_witness, [[2, 0, 2], [0; 3], [0; 3]]),
        Err(PlanDefect::ProviderTrajectory { provider: 0 })
    );
}

#[test]
fn malformed_padding_and_duplicate_delivery_bindings_refuse() {
    let (request, mut witness) = tie_fixture();
    witness.providers[2].bus = 1;
    let request = request_for(request.domain, &witness);
    assert_eq!(
        solve_clear(&request, &witness).private_diagnostic,
        PrivateDiagnostic::Witness(WitnessDefect::NonCanonicalPadding { slot: 2 })
    );

    let (request, mut witness) = tie_fixture();
    witness.providers[1].owner_binding = witness.providers[0].owner_binding;
    let request = request_for(request.domain, &witness);
    assert_eq!(
        solve_clear(&request, &witness).private_diagnostic,
        PrivateDiagnostic::Witness(WitnessDefect::DuplicateOwnerBinding {
            first: 0,
            second: 1
        })
    );
}

#[test]
fn prehorizon_output_must_respect_the_same_minimum_domain() {
    let (request, mut witness) = tie_fixture();
    witness.providers[0].min_output = 2;
    witness.providers[0].initial_output = 1;
    let request = request_for(request.domain, &witness);
    assert_eq!(
        solve_clear(&request, &witness).private_diagnostic,
        PrivateDiagnostic::Witness(WitnessDefect::InitialOutputOutOfDomain { slot: 0 })
    );
}

#[test]
fn refusal_priority_checks_availability_before_private_opening() {
    let (mut request, mut witness) = balanced_fixture();
    request.payloads_available = false;
    witness.providers[0].owner_binding[0] ^= 1;
    let execution = solve_clear(&request, &witness);
    assert_eq!(execution.public.status, PublicStatus::InputUnavailable);
    assert_eq!(
        execution.private_diagnostic,
        PrivateDiagnostic::PayloadUnavailable
    );
}

#[test]
fn line_and_reserve_constraints_are_not_advisory() {
    let (request, witness) = balanced_fixture();
    let candidate = solve_clear(&request, &witness)
        .candidate
        .expect("candidate");
    let mut flow_forgery = candidate.plan;
    flow_forgery.line_flow[0] += 1;
    let forged = seal_untrusted_plan(
        &request,
        &witness,
        flow_forgery,
        candidate.certificate.visited_pairs,
        candidate.certificate.feasible_candidates,
    )
    .expect("forge");
    assert_eq!(
        verify_candidate(&request, &witness, &forged),
        Err(VerificationError::Plan(PlanDefect::FlowMismatch {
            period: 0
        }))
    );

    let mut reserve_forgery = candidate.plan;
    reserve_forgery.upward_reserve[0] += 1;
    let forged = seal_untrusted_plan(
        &request,
        &witness,
        reserve_forgery,
        candidate.certificate.visited_pairs,
        candidate.certificate.feasible_candidates,
    )
    .expect("forge");
    assert_eq!(
        verify_candidate(&request, &witness, &forged),
        Err(VerificationError::Plan(PlanDefect::ReserveMismatch {
            period: 0
        }))
    );
}

#[test]
fn repeated_execution_is_byte_deterministic() {
    let (request, witness) = balanced_fixture();
    let first = solve_clear(&request, &witness);
    for _ in 0..16 {
        assert_eq!(solve_clear(&request, &witness), first);
    }
}

#[test]
fn independent_small_cartesian_search_matches_the_derived_third_oracle() {
    let (request, witness) = tie_fixture();
    let oracle = solve_clear(&request, &witness)
        .candidate
        .expect("oracle plan")
        .plan;
    let mut reference = None;
    // Independent search shape for the two occupied providers: decode every
    // 5^6 output matrix directly, rather than deriving provider 3 from demand.
    for ordinal in 0u32..5u32.pow(6) {
        let mut digits = ordinal;
        let mut values = [0u8; 6];
        for value in values.iter_mut().rev() {
            *value = u8::try_from(digits % 5).expect("base-5 digit");
            digits /= 5;
        }
        let generation = [
            [values[0], values[1], values[2]],
            [values[3], values[4], values[5]],
            [0; PERIODS],
        ];
        let Ok(candidate) = evaluate_plan(&request.domain, &witness, generation) else {
            continue;
        };
        if reference.is_none_or(|incumbent: degg_confidential_energy_dispatch::Plan| {
            candidate.objective_cost < incumbent.objective_cost
                || (candidate.objective_cost == incumbent.objective_cost
                    && candidate.generation > incumbent.generation)
        }) {
            reference = Some(candidate);
        }
    }
    assert_eq!(reference, Some(oracle));
}
