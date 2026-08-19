//! Byte-stable corpus for the bounded Clear oracle.

use std::fmt::Write;

use crate::canonical::inputs_commitment;
use crate::model::{CostSegment, DispatchRequest, ProviderInput, PublicDomain, Witness};
use crate::oracle::{PrivateDiagnostic, solve_clear};
use crate::sha256::hex;

/// Canonical balanced fixture shared by the vector renderer and tests.
#[must_use]
pub fn balanced_fixture() -> (DispatchRequest, Witness) {
    let witness = Witness {
        providers: [
            ProviderInput {
                occupied: true,
                owner_binding: [1; 32],
                bus: 0,
                min_output: 1,
                max_output: 4,
                ramp_up: 3,
                ramp_down: 3,
                initial_output: 1,
                available: [true, true, true],
                segments: [
                    CostSegment {
                        width: 2,
                        marginal_cost: 3,
                    },
                    CostSegment {
                        width: 2,
                        marginal_cost: 8,
                    },
                ],
            },
            ProviderInput {
                occupied: true,
                owner_binding: [2; 32],
                bus: 1,
                min_output: 1,
                max_output: 4,
                ramp_up: 2,
                ramp_down: 2,
                initial_output: 2,
                available: [true, true, true],
                segments: [
                    CostSegment {
                        width: 3,
                        marginal_cost: 5,
                    },
                    CostSegment {
                        width: 1,
                        marginal_cost: 9,
                    },
                ],
            },
            ProviderInput {
                occupied: true,
                owner_binding: [3; 32],
                bus: 1,
                min_output: 2,
                max_output: 4,
                ramp_up: 4,
                ramp_down: 4,
                initial_output: 0,
                available: [true, false, true],
                segments: [
                    CostSegment {
                        width: 2,
                        marginal_cost: 2,
                    },
                    CostSegment {
                        width: 2,
                        marginal_cost: 12,
                    },
                ],
            },
        ],
    };
    let domain = PublicDomain {
        instance: [0x44; 32],
        epoch: 42,
        demand: [[2, 3], [3, 2], [2, 4]],
        reserve: [2, 1, 2],
        line_limit: [2, 2, 2],
    };
    let request = DispatchRequest {
        domain,
        accepted_inputs: inputs_commitment(&domain, &witness),
        admission_final: true,
        payloads_available: true,
    };
    (request, witness)
}

/// Render the stable representative success and refusal transcript.
///
/// # Panics
///
/// Panics if the checked-in fixture ceases to be feasible or an in-memory
/// string write fails. Either condition is a corpus-construction bug.
#[must_use]
pub fn render() -> String {
    let (request, witness) = balanced_fixture();
    let execution = solve_clear(&request, &witness);
    let candidate = execution.candidate.expect("fixture is feasible");
    let mut out = String::new();
    writeln!(&mut out, "{} corpus", crate::model::RELATION_ID).expect("string write");
    writeln!(&mut out, "status={:?}", candidate.public.status).expect("string write");
    writeln!(&mut out, "domain={}", hex(&candidate.public.domain)).expect("string write");
    writeln!(&mut out, "inputs={}", hex(&candidate.public.inputs)).expect("string write");
    writeln!(&mut out, "plan={}", hex(&candidate.public.plan)).expect("string write");
    writeln!(&mut out, "deliveries={}", hex(&candidate.public.deliveries)).expect("string write");
    writeln!(&mut out, "generation={:?}", candidate.plan.generation).expect("string write");
    writeln!(&mut out, "flow={:?}", candidate.plan.line_flow).expect("string write");
    writeln!(&mut out, "reserve={:?}", candidate.plan.upward_reserve).expect("string write");
    writeln!(&mut out, "credits={:?}", candidate.plan.provider_credit).expect("string write");
    writeln!(&mut out, "load-debit={}", candidate.plan.load_debit).expect("string write");
    writeln!(&mut out, "objective={}", candidate.plan.objective_cost).expect("string write");
    writeln!(
        &mut out,
        "search=visited:{} feasible:{}",
        candidate.certificate.visited_pairs, candidate.certificate.feasible_candidates
    )
    .expect("string write");
    writeln!(
        &mut out,
        "public-frame-bytes={}",
        candidate.public.frame().len()
    )
    .expect("string write");

    let mut unavailable = request;
    unavailable.payloads_available = false;
    let refusal = solve_clear(&unavailable, &witness);
    assert_eq!(
        refusal.private_diagnostic,
        PrivateDiagnostic::PayloadUnavailable
    );
    writeln!(&mut out, "unavailable-status={:?}", refusal.public.status).expect("string write");
    writeln!(
        &mut out,
        "unavailable-public-frame-bytes={}",
        refusal.public.frame().len()
    )
    .expect("string write");
    out
}
