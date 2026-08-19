//! Canonical fixed-width encodings and domain-separated commitments.
//!
//! Commitments are ordinary SHA-256 commitments to public canonical bytes. No
//! hiding property is claimed: the provider domain is tiny, so a commitment is
//! not a substitute for encryption or a high-entropy blinding construction.

use crate::model::{
    BUSES, DispatchRequest, PERIODS, PROVIDERS, ProviderInput, PublicDomain, RELATION_ID, Witness,
};
use crate::oracle::{Plan, PrivateDelivery};
use crate::sha256::tagged;

const DOMAIN_TAG: &[u8] = b"degg/confidential-energy-dispatch/v0/public-domain";
const INPUT_TAG: &[u8] = b"degg/confidential-energy-dispatch/v0/private-inputs";
const PLAN_TAG: &[u8] = b"degg/confidential-energy-dispatch/v0/plan";
const DELIVERY_TAG: &[u8] = b"degg/confidential-energy-dispatch/v0/delivery";
const RELATION_TAG: &[u8] = b"degg/confidential-energy-dispatch/v0/relation";

/// Exact canonical plaintext witness length. This fixes only the semantic
/// payload shape; a future ciphertext and proof wire format must set its own
/// independently tested constant length.
pub const PRIVATE_WITNESS_LEN: usize = 156;

fn put_bool(out: &mut Vec<u8>, value: bool) {
    out.push(u8::from(value));
}

fn put_i16(out: &mut Vec<u8>, value: i16) {
    out.extend_from_slice(&value.to_be_bytes());
}

fn put_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_be_bytes());
}

fn put_u64(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_be_bytes());
}

/// Hash identity of the frozen relation identifier.
#[must_use]
pub fn relation_commitment() -> [u8; 32] {
    tagged(RELATION_TAG, &[RELATION_ID.as_bytes()])
}

/// Canonical bytes for the public domain.
#[must_use]
pub fn domain_bytes(domain: &PublicDomain) -> Vec<u8> {
    let mut out = Vec::with_capacity(64);
    out.extend_from_slice(RELATION_ID.as_bytes());
    out.push(0);
    out.extend_from_slice(&domain.instance);
    put_u64(&mut out, domain.epoch);
    for period in 0..PERIODS {
        for bus in 0..BUSES {
            out.push(domain.demand[period][bus]);
        }
        out.push(domain.reserve[period]);
        out.push(domain.line_limit[period]);
    }
    out
}

/// Domain commitment.
#[must_use]
pub fn domain_commitment(domain: &PublicDomain) -> [u8; 32] {
    tagged(DOMAIN_TAG, &[&domain_bytes(domain)])
}

fn provider_bytes(provider: &ProviderInput, out: &mut Vec<u8>) {
    put_bool(out, provider.occupied);
    out.extend_from_slice(&provider.owner_binding);
    out.push(provider.bus);
    out.push(provider.min_output);
    out.push(provider.max_output);
    out.push(provider.ramp_up);
    out.push(provider.ramp_down);
    out.push(provider.initial_output);
    for available in provider.available {
        put_bool(out, available);
    }
    for segment in provider.segments {
        out.push(segment.width);
        put_u32(out, segment.marginal_cost);
    }
}

/// Canonical provider-witness bytes, including all padding.
#[must_use]
pub fn witness_bytes(witness: &Witness) -> Vec<u8> {
    let mut out = Vec::with_capacity(PRIVATE_WITNESS_LEN);
    for provider in &witness.providers {
        provider_bytes(provider, &mut out);
    }
    debug_assert_eq!(out.len(), PRIVATE_WITNESS_LEN);
    out
}

/// Domain-bound commitment to every canonical provider slot.
#[must_use]
pub fn inputs_commitment(domain: &PublicDomain, witness: &Witness) -> [u8; 32] {
    let domain = domain_commitment(domain);
    let witness = witness_bytes(witness);
    tagged(INPUT_TAG, &[&domain, &witness])
}

/// Canonical plan bytes, including objective and settlement totals.
#[must_use]
pub fn plan_bytes(plan: &Plan) -> Vec<u8> {
    let mut out = Vec::with_capacity(96);
    for provider in 0..PROVIDERS {
        out.extend_from_slice(&plan.generation[provider]);
    }
    for flow in plan.line_flow {
        put_i16(&mut out, flow);
    }
    for reserve in plan.upward_reserve {
        out.push(reserve);
    }
    for credit in plan.provider_credit {
        put_u64(&mut out, credit);
    }
    put_u64(&mut out, plan.load_debit);
    put_u64(&mut out, plan.objective_cost);
    out
}

/// Bind a plan to its public domain and exact accepted inputs.
#[must_use]
pub fn plan_commitment(request: &DispatchRequest, plan: &Plan) -> [u8; 32] {
    let domain = domain_commitment(&request.domain);
    let plan = plan_bytes(plan);
    tagged(PLAN_TAG, &[&domain, &request.accepted_inputs, &plan])
}

/// Canonical authorized local-output bytes.
#[must_use]
pub fn delivery_bytes(delivery: &PrivateDelivery) -> Vec<u8> {
    let mut out = Vec::with_capacity(192);
    for local in delivery.providers {
        put_bool(&mut out, local.present);
        out.extend_from_slice(&local.recipient_binding);
        out.extend_from_slice(&local.dispatch);
        put_u64(&mut out, local.credit);
    }
    put_u64(&mut out, delivery.load.debit);
    put_u64(&mut out, delivery.load.served_energy);
    out
}

/// Bind all fixed-size local outputs to the committed plan.
#[must_use]
pub fn delivery_commitment(
    request: &DispatchRequest,
    plan_commitment: &[u8; 32],
    delivery: &PrivateDelivery,
) -> [u8; 32] {
    let domain = domain_commitment(&request.domain);
    let delivery = delivery_bytes(delivery);
    tagged(
        DELIVERY_TAG,
        &[
            &domain,
            &request.accepted_inputs,
            plan_commitment,
            &delivery,
        ],
    )
}
