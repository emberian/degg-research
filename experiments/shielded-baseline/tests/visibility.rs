//! The role boundary: what the public verifier can hold, and what it cannot
//! read.
//!
//! The compile-time half of this property lives in `compile_fail` doctests on
//! [`degg_shielded_baseline::seal::SealedPayload`] and
//! [`degg_shielded_baseline::seal::SealedLocalOutput`]: a public-role function
//! that reaches for an order field or an owner output does not compile, and
//! `cargo test --doc` is what runs those. This suite covers the run-time half.
//!
//! Neither half is confidentiality. The named executor holds the secret both
//! sealed forms are derived from and reads every one of them.

mod common;

use common::{NOW, balanced_residual, session};
use degg_relation_ir::ty::{Ty, Visibility};
use degg_shielded_baseline::SHIELDED_VISIBILITY_DISCLOSURE;
use degg_shielded_baseline::executor::Tamper;
use degg_shielded_baseline::roles::ExecutorKey;
use degg_shielded_baseline::seal::{SealDefect, WIRE_LEN};

fn contains_window(haystack: &[u8], needle: &[u8]) -> bool {
    needle.len() <= haystack.len() && haystack.windows(needle.len()).any(|w| w == needle)
}

#[test]
fn the_disclosure_says_the_executor_learns_every_private_field() {
    assert!(SHIELDED_VISIBILITY_DISCLOSURE.contains("learns every private order field"));
    assert!(SHIELDED_VISIBILITY_DISCLOSURE.contains("never proved"));
}

#[test]
fn the_public_wire_form_carries_no_order_field() {
    let scenario = balanced_residual();
    let session = session(&scenario);
    for submission in &session.submissions {
        let plain = submission.plain;
        let (_, ciphertext) = submission.sealed.wire();
        assert_eq!(ciphertext.len(), WIRE_LEN);
        assert_ne!(ciphertext, plain.encode());
        for field in [
            plain.quantity.to_be_bytes(),
            plain.reserved.to_be_bytes(),
            plain.nullifier.to_be_bytes(),
            plain.arrived_at.to_be_bytes(),
        ] {
            assert!(
                !contains_window(&ciphertext, &field),
                "a plaintext field appears in the wire form"
            );
        }
    }
}

#[test]
fn the_published_commitment_material_carries_no_order_field() {
    // The scan covers the receipt's commitment material and the cutoff root.
    // It deliberately does *not* cover the public outcome: a cleared tick and
    // an aggregate volume are declared public ports of the relation, and a
    // volume that happens to equal some order's quantity is a coincidence a
    // byte scan cannot tell from a disclosure. The declared port list is the
    // contract there, and the test below checks it directly.
    let scenario = balanced_residual();
    let mut session = session(&scenario);
    let run = session.compute(&Tamper::None, NOW).expect("computes");
    let mut published = Vec::new();
    published.extend_from_slice(&run.run.receipt.binding);
    published.extend_from_slice(&run.run.receipt.outcome_digest);
    published.extend_from_slice(&run.run.receipt.delivery_root);
    published.extend_from_slice(&run.run.receipt.input_digest);
    published.extend_from_slice(&session.cutoff.root);
    for opening in &run.openings {
        published.extend_from_slice(&opening.entry.preimage(opening.seq));
    }
    for submission in &session.submissions {
        let plain = submission.plain;
        for field in [
            plain.quantity.to_be_bytes(),
            plain.reserved.to_be_bytes(),
            plain.nullifier.to_be_bytes(),
        ] {
            assert!(!contains_window(&published, &field));
        }
    }
}

#[test]
fn the_public_result_carries_exactly_the_relations_declared_public_ports() {
    // The composition adds no public field to the relation. Whatever the
    // module declares `Public` is what crosses, and the Shielded receipt adds
    // only digests, a delivery root, and a typed status on top.
    let module = degg_relation_ir::module::dark_fba_n4_k4_q15_v0();
    let public_result = module
        .outputs
        .iter()
        .find(|port| port.name == "public-result")
        .expect("the module declares a public result");
    let Ty::Record { ref fields, .. } = public_result.ty else {
        panic!("the public result is a record");
    };
    let names: Vec<&str> = fields.iter().map(|field| field.name.as_str()).collect();
    assert_eq!(
        names,
        vec![
            "batch-id",
            "market-id",
            "accepted-input-root",
            "tick",
            "volume"
        ]
    );
    for field in fields {
        assert_eq!(field.visibility, Visibility::Public);
    }
    // Owner outputs stay private to their owner in the declaration, and the
    // composition delivers them sealed rather than published.
    let owner_outputs = module
        .outputs
        .iter()
        .find(|port| port.name == "owner-outputs")
        .expect("the module declares owner outputs");
    assert_eq!(owner_outputs.visibility, Visibility::PrivateToOwner);
}

#[test]
fn another_executors_key_does_not_open_a_payload() {
    let scenario = balanced_residual();
    let session = session(&scenario);
    let (_, impostor) = ExecutorKey::commission("impostor", &[0xff; 32]);
    for submission in &session.submissions {
        assert_ne!(
            submission.sealed.open(&impostor),
            Ok(submission.plain),
            "an unrelated key recovered the plaintext"
        );
    }
}

#[test]
fn only_the_owners_own_delivery_key_opens_its_local_output() {
    let scenario = balanced_residual();
    let mut session = session(&scenario);
    let run = session.compute(&Tamper::None, NOW).expect("computes");
    for sealed in &run.run.deliveries {
        let owner = sealed.owner();
        sealed
            .open(&session.executor.delivery_key(owner))
            .expect("the owner opens its own output");
        for other in 0..session.module.params.owners {
            if other == owner {
                continue;
            }
            assert_eq!(
                sealed.open(&session.executor.delivery_key(other)),
                Err(SealDefect::NotAuthorized)
            );
        }
        let (_, impostor) = ExecutorKey::commission("impostor", &[0xff; 32]);
        assert_eq!(
            sealed.open(&impostor.grant_delivery(owner)),
            Err(SealDefect::NotAuthorized)
        );
    }
}

#[test]
fn the_plaintext_encoding_is_injective_and_canonical() {
    let scenario = balanced_residual();
    let session = session(&scenario);
    for submission in &session.submissions {
        let bytes = submission.plain.encode();
        assert_eq!(
            degg_shielded_baseline::seal::PlainOrder::decode(&bytes),
            Ok(submission.plain)
        );
        let mut padded = bytes;
        padded[WIRE_LEN - 1] = 1;
        assert_eq!(
            degg_shielded_baseline::seal::PlainOrder::decode(&padded),
            Err(SealDefect::MalformedPlaintext)
        );
        let mut side = bytes;
        side[17] = 2;
        assert_eq!(
            degg_shielded_baseline::seal::PlainOrder::decode(&side),
            Err(SealDefect::MalformedPlaintext)
        );
    }
}
