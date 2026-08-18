//! The frozen authorization order: every defect has a witness, the earlier
//! rule wins, and an honest run authorizes exactly its published effects.

mod common;

use common::{balanced_residual, computed, no_trade, ready, ready_tampered, under_reserved};
use degg_inclusion_availability::lifecycle::{AbortClass, Phase};
use degg_relation_ir::batch::Side;
use degg_relation_ir::canon::Canonical;
use degg_relation_ir::lower::{ClearedTick, PublicOutcome, PublicResult};
use degg_relation_ir::receipt::ReceiptStatus;
use degg_settlement_model::authorize::{AuthorizationDefect, authorize};
use degg_settlement_model::custody::{Asset, CustodyLedger};
use degg_settlement_model::harness::{authorize_position, settlement_inputs, spending_asset};
use degg_shielded_baseline::executor::Tamper;
use degg_shielded_baseline::receipt::{
    CutoffBinding, DeliveryCommitment, DeliveryEntry, ShieldedReceipt, SlotOutcome,
};

#[test]
fn every_position_of_the_honest_run_authorizes_its_published_effect() {
    let ready = ready(&balanced_residual());
    let expected = [
        (
            0u32,
            0u8,
            Side::Buy,
            (Asset::Quote, 0u64),
            (Asset::Base, 5u64),
            15u64,
        ),
        (1, 1, Side::Buy, (Asset::Quote, 6), (Asset::Base, 0), 0),
        (2, 2, Side::Sell, (Asset::Base, 1), (Asset::Quote, 9), 3),
        (3, 3, Side::Sell, (Asset::Base, 2), (Asset::Quote, 6), 2),
    ];
    for (seq, owner, side, released, acquired, consumed) in expected {
        let authorization = authorize_position(&ready.session, &ready.run, &ready.custody, seq)
            .expect("an honest position authorizes");
        assert_eq!(authorization.seq, seq);
        assert_eq!(authorization.owner, owner);
        assert_eq!(authorization.side, side);
        assert_eq!(authorization.released, released);
        assert_eq!(authorization.acquired, acquired);
        assert_eq!(authorization.escrow_consumed, consumed);
        assert_eq!(authorization.receipt_binding, ready.run.run.receipt.binding);
    }
}

#[test]
fn settlement_nullifiers_are_pairwise_distinct_and_domain_separated() {
    let ready = ready(&balanced_residual());
    let mut nullifiers = Vec::new();
    for seq in 0..4 {
        let authorization = authorize_position(&ready.session, &ready.run, &ready.custody, seq)
            .expect("authorizes");
        assert_ne!(
            authorization.settlement_nullifier, authorization.admission_nullifier,
            "the settlement nullifier is derived, not reused"
        );
        nullifiers.push(authorization.settlement_nullifier);
    }
    nullifiers.sort_unstable();
    nullifiers.dedup();
    assert_eq!(nullifiers.len(), 4, "one settlement nullifier per position");
}

#[test]
fn a_malformed_receipt_is_refused_first() {
    let ready = ready(&balanced_residual());
    let mut receipt = ready.run.run.receipt;
    receipt.binding[0] ^= 1;
    let mut inputs = settlement_inputs(&ready.session, &ready.run, 0);
    inputs.receipt = &receipt;
    assert!(matches!(
        authorize(&inputs, &ready.custody),
        Err(AuthorizationDefect::Receipt(_))
    ));
}

#[test]
fn a_publicly_refused_batch_authorizes_nothing() {
    let (session, run, custody) = computed(&under_reserved());
    let refused_code = match run.phase {
        Phase::Aborted(AbortClass::RelationRefused { class_code }) => class_code,
        phase => panic!("the batch refuses publicly, got {phase:?}"),
    };
    for seq in 0..4 {
        let inputs = settlement_inputs(&session, &run, seq);
        assert_eq!(
            authorize(&inputs, &custody),
            Err(AuthorizationDefect::BatchRefusedNoAllocation {
                class_code: refused_code
            })
        );
    }
}

#[test]
fn the_refusal_class_precedes_the_phase_check() {
    // A multiply-defective claim: refused batch and a non-settled phase. The
    // frozen order reports the refusal, not the phase.
    let (session, run, custody) = computed(&under_reserved());
    let mut inputs = settlement_inputs(&session, &run, 0);
    inputs.phase = Phase::Open;
    assert!(matches!(
        authorize(&inputs, &custody),
        Err(AuthorizationDefect::BatchRefusedNoAllocation { .. })
    ));
}

#[test]
fn a_phase_short_of_settled_refuses() {
    let ready = ready(&balanced_residual());
    let mut inputs = settlement_inputs(&ready.session, &ready.run, 0);
    inputs.phase = Phase::Open;
    assert_eq!(
        authorize(&inputs, &ready.custody),
        Err(AuthorizationDefect::PhaseNotSettled { phase: Phase::Open })
    );
}

#[test]
fn a_settled_phase_naming_another_result_refuses() {
    let ready = ready(&balanced_residual());
    let mut inputs = settlement_inputs(&ready.session, &ready.run, 0);
    inputs.phase = Phase::Settled {
        result_digest: [0u8; 32],
    };
    assert_eq!(
        authorize(&inputs, &ready.custody),
        Err(AuthorizationDefect::PhaseResultMismatch)
    );
}

#[test]
fn a_tampered_inclusion_receipt_refuses() {
    let ready = ready(&balanced_residual());
    let mut inclusion = ready.session.receipts[0].clone();
    inclusion.record.nullifier[0] ^= 1;
    let mut inputs = settlement_inputs(&ready.session, &ready.run, 0);
    inputs.inclusion = &inclusion;
    assert!(matches!(
        authorize(&inputs, &ready.custody),
        Err(AuthorizationDefect::Inclusion(_))
    ));
}

#[test]
fn an_inclusion_receipt_for_another_position_refuses() {
    let ready = ready(&balanced_residual());
    let mut inputs = settlement_inputs(&ready.session, &ready.run, 1);
    inputs.inclusion = &ready.session.receipts[0];
    assert_eq!(
        authorize(&inputs, &ready.custody),
        Err(AuthorizationDefect::PositionMismatch {
            record_seq: 0,
            opening_seq: 1
        })
    );
}

#[test]
fn a_padding_position_settles_nothing() {
    let ready = ready(&no_trade());
    let inputs = settlement_inputs(&ready.session, &ready.run, 2);
    assert_eq!(
        authorize(&inputs, &ready.custody),
        Err(AuthorizationDefect::PaddingPositionSettlesNothing { seq: 2 })
    );
}

#[test]
fn the_padding_rule_precedes_the_opening_rule() {
    // The padding position's opening is also tampered; the earlier rule wins.
    let ready = ready(&no_trade());
    let mut opening = settlement_inputs(&ready.session, &ready.run, 2)
        .opening
        .clone();
    opening.proof.path[0].hash[0] ^= 1;
    let mut inputs = settlement_inputs(&ready.session, &ready.run, 2);
    inputs.opening = &opening;
    assert_eq!(
        authorize(&inputs, &ready.custody),
        Err(AuthorizationDefect::PaddingPositionSettlesNothing { seq: 2 })
    );
}

#[test]
fn a_tampered_delivery_opening_refuses() {
    let ready = ready(&balanced_residual());
    let mut opening = settlement_inputs(&ready.session, &ready.run, 0)
        .opening
        .clone();
    if let DeliveryEntry::Produced(ref mut outcome) = opening.entry {
        outcome.fill += 1;
    }
    let mut inputs = settlement_inputs(&ready.session, &ready.run, 0);
    inputs.opening = &opening;
    assert!(matches!(
        authorize(&inputs, &ready.custody),
        Err(AuthorizationDefect::Opening(_))
    ));
}

#[test]
fn an_omitted_position_has_no_local_output_and_cannot_settle() {
    let ready = ready_tampered(&balanced_residual(), &Tamper::OmitPosition { seq: 2 });
    let inputs = settlement_inputs(&ready.session, &ready.run, 2);
    assert_eq!(
        authorize(&inputs, &ready.custody),
        Err(AuthorizationDefect::NoLocalOutputAtPosition { seq: 2 })
    );
}

fn forged(outcome: SlotOutcome, seq: u32) -> Tamper {
    Tamper::ForgeSlotOutcome { seq, outcome }
}

fn honest_effect(seq: u32) -> SlotOutcome {
    let ready = ready(&balanced_residual());
    match settlement_inputs(&ready.session, &ready.run, seq)
        .opening
        .entry
    {
        DeliveryEntry::Produced(outcome) => outcome,
        DeliveryEntry::NoLocalOutput => panic!("occupied positions produce"),
    }
}

#[test]
fn a_committed_effect_naming_another_position_refuses() {
    let mut effect = honest_effect(3);
    effect.seq = 0;
    let ready = ready_tampered(&balanced_residual(), &forged(effect, 3));
    let inputs = settlement_inputs(&ready.session, &ready.run, 3);
    assert_eq!(
        authorize(&inputs, &ready.custody),
        Err(AuthorizationDefect::OutcomeSequenceMismatch {
            entry_seq: 0,
            opening_seq: 3
        })
    );
}

#[test]
fn an_out_of_domain_owner_refuses() {
    let mut effect = honest_effect(3);
    effect.owner = 7;
    let ready = ready_tampered(&balanced_residual(), &forged(effect, 3));
    let inputs = settlement_inputs(&ready.session, &ready.run, 3);
    assert_eq!(
        authorize(&inputs, &ready.custody),
        Err(AuthorizationDefect::OwnerOutOfDomain { owner: 7 })
    );
}

#[test]
fn a_fill_above_the_published_volume_refuses() {
    let mut effect = honest_effect(0);
    effect.fill = 6;
    effect.base_delta = 6;
    effect.quote_delta = -18;
    effect.released_quote = 0;
    let ready = ready_tampered(&balanced_residual(), &forged(effect, 0));
    let inputs = settlement_inputs(&ready.session, &ready.run, 0);
    assert_eq!(
        authorize(&inputs, &ready.custody),
        Err(AuthorizationDefect::FillExceedsVolume { fill: 6, volume: 5 })
    );
}

#[test]
fn a_positive_fill_under_no_trade_refuses() {
    let effect = SlotOutcome {
        seq: 0,
        owner: 0,
        side: Side::Buy,
        fill: 1,
        base_delta: 1,
        quote_delta: 0,
        released_base: 0,
        released_quote: 2,
    };
    let ready = ready_tampered(&no_trade(), &forged(effect, 0));
    let inputs = settlement_inputs(&ready.session, &ready.run, 0);
    assert_eq!(
        authorize(&inputs, &ready.custody),
        Err(AuthorizationDefect::FillPositiveAtNoTrade { fill: 1 })
    );
}

#[test]
fn an_inconsistent_delta_refuses() {
    let mut effect = honest_effect(3);
    effect.base_delta = -1;
    let ready = ready_tampered(&balanced_residual(), &forged(effect, 3));
    let inputs = settlement_inputs(&ready.session, &ready.run, 3);
    assert_eq!(
        authorize(&inputs, &ready.custody),
        Err(AuthorizationDefect::EffectInconsistent { seq: 3 })
    );
}

#[test]
fn custody_mismatches_are_each_typed() {
    let ready = ready(&balanced_residual());
    let inputs = settlement_inputs(&ready.session, &ready.run, 0);
    let submission = &ready.session.submissions[0];
    let nullifier = submission.request.nullifier;

    let empty = CustodyLedger::new();
    assert_eq!(
        authorize(&inputs, &empty),
        Err(AuthorizationDefect::NotEscrowed)
    );

    let mut wrong_owner = CustodyLedger::new();
    wrong_owner
        .escrow(nullifier, 1, Asset::Quote, 15, 10)
        .expect("escrows");
    assert_eq!(
        authorize(&inputs, &wrong_owner),
        Err(AuthorizationDefect::EscrowOwnerMismatch {
            escrowed: 1,
            claimed: 0
        })
    );

    let mut wrong_asset = CustodyLedger::new();
    wrong_asset
        .escrow(nullifier, 0, Asset::Base, 15, 10)
        .expect("escrows");
    assert_eq!(
        authorize(&inputs, &wrong_asset),
        Err(AuthorizationDefect::EscrowAssetMismatch)
    );

    let mut wrong_amount = CustodyLedger::new();
    wrong_amount
        .escrow(nullifier, 0, Asset::Quote, 14, 10)
        .expect("escrows");
    assert_eq!(
        authorize(&inputs, &wrong_amount),
        Err(AuthorizationDefect::EscrowAmountMismatch {
            escrowed: 14,
            implied: 15
        })
    );
    assert_eq!(spending_asset(Side::Buy), Asset::Quote);
}

#[test]
fn a_refunded_position_cannot_settle() {
    let ready = ready(&balanced_residual());
    // Reach around the phase gate deliberately: custody is mechanism, and the
    // defect is typed even though the honest gates make it unreachable.
    let nullifier = ready.session.submissions[0].request.nullifier;
    let mut custody = degg_settlement_model::harness::escrow_admitted(&ready.session);
    custody
        .refund(&nullifier, 10)
        .expect("mechanism-level refund");
    let inputs = settlement_inputs(&ready.session, &ready.run, 0);
    assert_eq!(
        authorize(&inputs, &custody),
        Err(AuthorizationDefect::RefundedPositionCannotSettle)
    );
}

/// A synthetic published run: the executor invents a result and a delivery
/// commitment from whole cloth. The public check accepts any well-shaped
/// result — that is `SHIELDED_BASELINE.md` section 6.2 — but the settlement
/// verifier still refuses claims whose arithmetic cannot be re-derived.
fn synthetic_run(
    session: &degg_shielded_baseline::scenario::Session,
    honest: &ShieldedReceipt,
    tick: ClearedTick,
    volume: u64,
    effect: SlotOutcome,
) -> (ShieldedReceipt, PublicOutcome, DeliveryCommitment, Phase) {
    let result = PublicResult {
        relation: "dark-fba/n4-k4-q15/v0".to_string(),
        batch_id: 7,
        market_id: 9,
        accepted_input_root: session.cutoff.root,
        tick,
        volume,
    };
    let outcome = PublicOutcome::Settled(result);
    let cutoff = CutoffBinding::of(&session.cutoff);
    let module_digest = session.module.digest();
    let entries = vec![
        DeliveryEntry::Produced(effect),
        DeliveryEntry::NoLocalOutput,
        DeliveryEntry::NoLocalOutput,
        DeliveryEntry::NoLocalOutput,
    ];
    let delivery = DeliveryCommitment::build(&cutoff, &module_digest, entries);
    let receipt = ShieldedReceipt::new(
        honest.executor,
        module_digest,
        cutoff,
        [0u8; 32],
        outcome.digest(),
        delivery.root(),
        ReceiptStatus::Settled,
    );
    let phase = Phase::Settled {
        result_digest: outcome.digest(),
    };
    (receipt, outcome, delivery, phase)
}

#[test]
fn a_tick_off_the_grid_refuses_even_though_the_public_check_accepts_it() {
    let ready = ready(&balanced_residual());
    let effect = honest_effect(0);
    let (receipt, outcome, delivery, phase) = synthetic_run(
        &ready.session,
        &ready.run.run.receipt,
        ClearedTick::Tick(9),
        1,
        effect,
    );
    let opening = delivery.open(0).expect("opens");
    let inputs = degg_settlement_model::authorize::SettlementInputs {
        receipt: &receipt,
        cutoff: &ready.session.cutoff,
        outcome: &outcome,
        phase,
        module: &ready.session.module,
        inclusion: &ready.session.receipts[0],
        opening: &opening,
    };
    assert_eq!(
        authorize(&inputs, &ready.custody),
        Err(AuthorizationDefect::TickOutOfRange { tick: 9 })
    );
}

#[test]
fn an_overflowing_claim_refuses_instead_of_wrapping() {
    let ready = ready(&balanced_residual());
    let effect = SlotOutcome {
        seq: 0,
        owner: 0,
        side: Side::Buy,
        fill: u64::MAX,
        base_delta: 0,
        quote_delta: 0,
        released_base: 0,
        released_quote: 0,
    };
    let (receipt, outcome, delivery, phase) = synthetic_run(
        &ready.session,
        &ready.run.run.receipt,
        ClearedTick::Tick(3),
        u64::MAX,
        effect,
    );
    let opening = delivery.open(0).expect("opens");
    let inputs = degg_settlement_model::authorize::SettlementInputs {
        receipt: &receipt,
        cutoff: &ready.session.cutoff,
        outcome: &outcome,
        phase,
        module: &ready.session.module,
        inclusion: &ready.session.receipts[0],
        opening: &opening,
    };
    assert_eq!(
        authorize(&inputs, &ready.custody),
        Err(AuthorizationDefect::ArithmeticOverflow)
    );
}
