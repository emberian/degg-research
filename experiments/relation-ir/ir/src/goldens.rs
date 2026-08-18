//! Golden digest rendering.
//!
//! One line per named IR object: its canonical `degg-cbe/v1` bytes hashed with
//! SHA-256. The checked-in `goldens/v1.txt` is compared byte-for-byte by a
//! unit test; any change to a frozen field, an encoding, or the evaluator's
//! outputs on the fixture cases changes this file visibly.

use crate::canon::Canonical;
use crate::fixtures;
use crate::lower::{ClearEvaluator, LoweringTarget, Outcome, OwnerOutputs, lower};
use crate::module::{RelationModule, dark_fba_n4_k4_q15_v0};
use crate::policy::rejected_alternative_check_order;
use crate::sha256::hex;

fn line(output: &mut String, name: &str, object: &impl Canonical) {
    output.push_str(&format!("object={name}|sha256={}\n", hex(&object.digest())));
}

fn fixture_lines(
    output: &mut String,
    evaluator: &ClearEvaluator,
    name: &str,
    batch: &crate::batch::BatchInput,
) {
    line(output, &format!("batch/{name}"), batch);
    let outcome = evaluator.evaluate(batch);
    line(output, &format!("public-outcome/{name}"), &outcome.public());
    match outcome {
        Outcome::Settled(settled) => {
            line(
                output,
                &format!("owner-outputs/{name}"),
                &OwnerOutputs(settled.owners.clone()),
            );
            line(
                output,
                &format!("receipt/computation/{name}"),
                &settled.receipt,
            );
            line(
                output,
                &format!("receipt/output-delivery/{name}/owner-0"),
                &settled.delivery[0],
            );
        }
        Outcome::Refused(refused) => {
            line(
                output,
                &format!("receipt/computation/{name}"),
                &refused.receipt,
            );
        }
    }
}

/// Render the full golden corpus.
pub fn render_goldens_v1() -> String {
    let module = dark_fba_n4_k4_q15_v0();
    let evaluator = lower(&module, LoweringTarget::Clear).expect("frozen module lowers");
    let alternative = RelationModule {
        admission: rejected_alternative_check_order(),
        ..module.clone()
    };
    let alternative_evaluator =
        lower(&alternative, LoweringTarget::Clear).expect("alternative module lowers");

    let mut output = String::from("relation-ir-goldens-v1\nencoder=degg-cbe/v1\n");
    line(&mut output, "module/dark-fba-n4-k4-q15-v0", &module);
    line(
        &mut output,
        "admission-policy/frozen-v0-check-order",
        &module.admission,
    );
    line(
        &mut output,
        "module/rejected-alternative-check-order",
        &alternative,
    );

    for (name, batch) in [
        ("balanced-residual", fixtures::balanced_residual()),
        ("price-tie-low", fixtures::price_tie_low()),
        ("no-trade", fixtures::no_trade()),
        ("dark-target-request", fixtures::dark_target_request()),
        ("duplicate-nullifier", fixtures::duplicate_nullifier()),
    ] {
        fixture_lines(&mut output, &evaluator, name, &batch);
    }

    // The check-priority freeze, made visible: the same witness bytes refuse
    // with different public classes under the frozen and the rejected order.
    for (name, batch) in [
        (
            "witness-quantity-vs-limit",
            fixtures::witness_quantity_vs_limit(),
        ),
        (
            "witness-nullifier-vs-reservation",
            fixtures::witness_nullifier_vs_reservation(),
        ),
    ] {
        line(&mut output, &format!("batch/{name}"), &batch);
        line(
            &mut output,
            &format!("public-outcome/{name}/frozen-order"),
            &evaluator.evaluate(&batch).public(),
        );
        line(
            &mut output,
            &format!("public-outcome/{name}/rejected-alternative-order"),
            &alternative_evaluator.evaluate(&batch).public(),
        );
    }
    output
}
