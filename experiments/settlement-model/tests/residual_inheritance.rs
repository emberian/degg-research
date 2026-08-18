//! What settlement inherits from `SHIELDED_BASELINE.md` section 6.2, measured
//! rather than hidden: the settlement relation checks binding, arithmetic
//! consistency, and custody — never correctness. A wrong-but-consistent
//! result settles and conserves; an imbalanced forgery is caught only as
//! terminal insolvency, and lands on whoever claims last; an omitted position
//! is stranded with a verdict but no funds path.

mod common;

use common::{balanced_residual, ready, ready_tampered};
use degg_relation_ir::batch::Side;
use degg_relation_ir::lower::{ClearedTick, PublicOutcome};
use degg_settlement_model::authorize::AuthorizationDefect;
use degg_settlement_model::custody::{Asset, Balances, CustodyError, CustodyState};
use degg_settlement_model::harness::{authorize_position, settle_all};
use degg_settlement_model::relation::{Execution, RefundRefusal, SettlementRefusal, refund};
use degg_shielded_baseline::dispute::{OmissionProof, verify_omission};
use degg_shielded_baseline::executor::Tamper;
use degg_shielded_baseline::receipt::{DeliveryEntry, SlotOutcome};
use degg_shielded_baseline::scenario::{BATCH_ID, CUTOFF_EPOCH, MARKET_ID};
use degg_shielded_baseline::seal::PlainOrder;

fn tick_and_volume(outcome: &PublicOutcome) -> (ClearedTick, u64) {
    match outcome {
        PublicOutcome::Settled(result) => (result.tick, result.volume),
        PublicOutcome::Refused(_) => panic!("these runs settle"),
    }
}

#[test]
fn settlement_conserves_value_around_a_result_it_cannot_know_is_wrong() {
    // The executor substitutes owner 0's limit inside the same escrowed
    // budget. Every published object is internally consistent, so every
    // check in this crate passes, every position settles, and the pool
    // drains exactly — around a result the honest book never produced.
    // Only a verifiable statement about the evaluation closes this, and no
    // such object exists in this repository.
    let honest = ready(&balanced_residual());
    let (honest_tick, honest_volume) = tick_and_volume(&honest.run.run.public);
    assert_eq!((honest_tick, honest_volume), (ClearedTick::Tick(2), 5));

    let substituted = PlainOrder {
        batch_id: BATCH_ID,
        market_id: MARKET_ID,
        owner: 0,
        side: Side::Buy,
        limit_tick: 0,
        quantity: 5,
        reserved: 15,
        nullifier: 101,
        arrived_at: CUTOFF_EPOCH,
    };
    let mut wrong = ready_tampered(
        &balanced_residual(),
        &Tamper::SubstitutePlaintext {
            seq: 0,
            plain: substituted,
        },
    );
    let (wrong_tick, wrong_volume) = tick_and_volume(&wrong.run.run.public);
    assert_eq!(
        (wrong_tick, wrong_volume),
        (ClearedTick::Tick(0), 4),
        "every participant now trades at a different price"
    );

    let executions = settle_all(
        &wrong.session,
        &wrong.run,
        &mut wrong.custody,
        &mut wrong.book,
        common::NOW,
    );
    assert_eq!(executions.len(), 4, "all four positions settle");
    assert_eq!(wrong.custody.pool(), Balances::zero());
    assert!(wrong.custody.conserves());
}

#[test]
fn an_imbalanced_forgery_settles_per_position_and_strands_an_honest_claimant() {
    // The executor forges one delivery entry: owner 3's fill becomes 3
    // instead of 2, internally consistent with its own escrow. Per-position
    // checking cannot see the batch total, so the forged claim authorizes
    // and executes. The batch as a whole now owes more quote than the pool
    // holds, and the shortfall surfaces as `PoolInsolvent` for whoever
    // claims last — here an honest owner, not the forger. Conservation
    // still holds: nothing was created, the loss is a stranded obligation.
    let forged_effect = SlotOutcome {
        seq: 3,
        owner: 3,
        side: Side::Sell,
        fill: 3,
        base_delta: -3,
        quote_delta: 9,
        released_base: 1,
        released_quote: 0,
    };
    let mut ready = ready_tampered(
        &balanced_residual(),
        &Tamper::ForgeSlotOutcome {
            seq: 3,
            outcome: forged_effect,
        },
    );

    // The forger claims first, then two honest owners; the last honest owner
    // is refused.
    for seq in [3u32, 0, 1] {
        let authorization = authorize_position(&ready.session, &ready.run, &ready.custody, seq)
            .expect("per-position checks cannot see the batch total");
        let executed = ready
            .book
            .execute(&mut ready.custody, &authorization, common::NOW)
            .expect("executes");
        assert!(matches!(executed, Execution::Executed(_)));
        assert!(ready.custody.conserves());
    }
    let last = authorize_position(&ready.session, &ready.run, &ready.custody, 2)
        .expect("the claim itself is well formed");
    assert_eq!(
        ready.book.execute(&mut ready.custody, &last, common::NOW),
        Err(SettlementRefusal::Custody(CustodyError::PoolInsolvent {
            asset: Asset::Quote,
            needed: 9,
            available: 6,
        }))
    );
    assert!(ready.custody.conserves());
    let stranded = ready
        .custody
        .entry(&last.admission_nullifier)
        .expect("entry");
    assert!(matches!(stranded.state, CustodyState::Obligated { .. }));
}

#[test]
fn an_omitted_position_is_stranded_obligated_with_a_verdict_but_no_funds_path() {
    // The executor omits owner 2's committed position. The other three
    // settle against the reduced result; the omitted owner holds a
    // transferable omission verdict, but this relation has no adjudication
    // rule that turns a verdict into funds: the reservation stays obligated,
    // exactly the pool's residue.
    let mut ready = ready_tampered(&balanced_residual(), &Tamper::OmitPosition { seq: 2 });
    for seq in [0u32, 1, 3] {
        let authorization = authorize_position(&ready.session, &ready.run, &ready.custody, seq)
            .expect("the untouched positions authorize");
        ready
            .book
            .execute(&mut ready.custody, &authorization, common::NOW)
            .expect("executes");
    }

    let inputs = degg_settlement_model::harness::settlement_inputs(&ready.session, &ready.run, 2);
    assert_eq!(
        degg_settlement_model::authorize::authorize(&inputs, &ready.custody),
        Err(AuthorizationDefect::NoLocalOutputAtPosition { seq: 2 })
    );
    let nullifier = ready.session.submissions[2].request.nullifier;
    assert_eq!(
        refund(&mut ready.custody, ready.run.phase, &nullifier, common::NOW),
        Err(RefundRefusal::PhaseNotRefundable {
            phase: ready.run.phase
        })
    );

    // The residue is exactly the stranded reservation.
    assert_eq!(ready.custody.pool(), Balances { base: 4, quote: 0 });
    assert!(ready.custody.conserves());
    let entry = ready.custody.entry(&nullifier).expect("entry");
    assert!(matches!(entry.state, CustodyState::Obligated { .. }));

    // The omission verdict the owner does hold, checkable by any third party.
    let opening = ready
        .run
        .openings
        .iter()
        .find(|opening| opening.seq == 2)
        .expect("opening")
        .clone();
    assert!(matches!(opening.entry, DeliveryEntry::NoLocalOutput));
    let proof = OmissionProof {
        cutoff: ready.session.cutoff,
        receipt: ready.run.run.receipt,
        inclusion: ready.session.receipts[2].clone(),
        opening,
    };
    verify_omission(&proof).expect("the omission verdict verifies");
}
