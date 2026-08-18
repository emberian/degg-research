//! Input validity: what an owner holding an inclusion receipt detects, and
//! precisely what it does not.
//!
//! The last test in this file is the honest core of the packet. It exhibits a
//! substitution that changes the public clearing tick and the public volume and
//! that every owner's complete check battery accepts.

mod common;

use common::{NOW, balanced_residual, session};
use degg_relation_ir::batch::Side;
use degg_relation_ir::lower::{ClearedTick, PublicOutcome};
use degg_relation_ir::receipt::ReceiptStatus;
use degg_relation_ir::refusal::RefusalClass;
use degg_shielded_baseline::dispute::{OmissionDefect, OmissionProof, verify_omission};
use degg_shielded_baseline::executor::Tamper;
use degg_shielded_baseline::owner::{Finding, audit, pro_rata_feasible};
use degg_shielded_baseline::receipt::SlotOutcome;
use degg_shielded_baseline::seal::PlainOrder;

fn plain_of(session: &degg_shielded_baseline::scenario::Session, index: usize) -> PlainOrder {
    session.submissions[index].plain
}

#[test]
fn an_omitted_position_is_detected_and_the_proof_is_transferable() {
    let scenario = balanced_residual();
    let mut session = session(&scenario);
    let run = session
        .compute(&Tamper::OmitPosition { seq: 2 }, NOW)
        .expect("computes");

    // Owner 2 holds position 2 and finds it dropped.
    let evidence = session.owner_evidence(&run, 2);
    let report = audit(
        &session.module,
        &evidence,
        &run.run.receipt,
        &run.run.public,
    );
    assert!(
        report
            .findings
            .contains(&Finding::OmittedFromComputation { seq: 2 })
    );

    // The finding reduces to an object anyone can check from public data plus
    // the owner's inclusion receipt.
    let proof = OmissionProof {
        cutoff: session.cutoff,
        receipt: run.run.receipt,
        inclusion: session.receipts[2].clone(),
        opening: run.openings[2].clone(),
    };
    let verdict = verify_omission(&proof).expect("the omission verifies");
    assert_eq!(verdict.seq, 2);
    assert_eq!(verdict.executor, session.executor.id());
    assert_eq!(verdict.cutoff.root, session.cutoff.root);
    // Content addressed: the same omission always names the same verdict.
    assert_eq!(
        verify_omission(&proof).expect("stable").digest,
        verdict.digest
    );

    // No other owner sees anything wrong, which is the point of the next test.
    for owner in [0u8, 1, 3] {
        let evidence = session.owner_evidence(&run, owner);
        let report = audit(
            &session.module,
            &evidence,
            &run.run.receipt,
            &run.run.public,
        );
        assert!(
            report.is_consistent(),
            "owner {owner} saw {:?}",
            report.findings
        );
    }
}

#[test]
fn an_honest_run_yields_no_omission_verdict_at_any_position() {
    let scenario = balanced_residual();
    let mut session = session(&scenario);
    let run = session.compute(&Tamper::None, NOW).expect("computes");
    for seq in 0..u32::try_from(session.cutoff.leaf_count).expect("bounded") {
        let index = usize::try_from(seq).expect("bounded");
        let proof = OmissionProof {
            cutoff: session.cutoff,
            receipt: run.run.receipt,
            inclusion: session.receipts[index].clone(),
            opening: run.openings[index].clone(),
        };
        assert_eq!(
            verify_omission(&proof),
            Err(OmissionDefect::OutputWasCommitted { seq })
        );
    }
}

#[test]
fn a_padding_position_carrying_no_output_is_not_an_omission() {
    let scenario = common::price_tie_low();
    let mut session = session(&scenario);
    let run = session.compute(&Tamper::None, NOW).expect("computes");
    for seq in [2u32, 3] {
        let index = usize::try_from(seq).expect("bounded");
        assert!(session.receipts[index].record.is_padding(&session.domain));
        let proof = OmissionProof {
            cutoff: session.cutoff,
            receipt: run.run.receipt,
            inclusion: session.receipts[index].clone(),
            opening: run.openings[index].clone(),
        };
        assert_eq!(
            verify_omission(&proof),
            Err(OmissionDefect::PositionIsPadding { seq })
        );
    }
}

#[test]
fn an_inflated_reservation_is_refused_by_the_derived_custody_binding() {
    // The reserve ledger escrows exactly what the submitter reserved, keyed by
    // the record's nullifier. `custody-bound` is derived from that ledger, so
    // an executor that evaluates a *larger* reservation than the one escrowed
    // produces a public typed refusal rather than a silently better fill. The
    // binding is the composition's, not the relation's: the relation carries
    // rule 17 as a boolean and the toy always supplies it as true.
    let scenario = balanced_residual();
    let mut session = session(&scenario);
    let mut substituted = plain_of(&session, 0);
    substituted.reserved = 99;
    let run = session
        .compute(
            &Tamper::SubstitutePlaintext {
                seq: 0,
                plain: substituted,
            },
            NOW,
        )
        .expect("computes");
    assert_eq!(
        run.run.receipt.status,
        ReceiptStatus::Refused(RefusalClass::CustodyBindingAbsent)
    );
}

#[test]
fn a_deflated_reservation_is_refused_by_the_relations_own_rule_18() {
    let scenario = balanced_residual();
    let mut session = session(&scenario);
    let mut substituted = plain_of(&session, 0);
    substituted.reserved = 14;
    let run = session
        .compute(
            &Tamper::SubstitutePlaintext {
                seq: 0,
                plain: substituted,
            },
            NOW,
        )
        .expect("computes");
    assert_eq!(
        run.run.receipt.status,
        ReceiptStatus::Refused(RefusalClass::ReservationInsufficient)
    );
}

#[test]
fn re_attributing_an_order_to_another_owner_is_refused_by_the_credential_registry() {
    // The log commits the submitter's admission-credential commitment. The
    // executor derives `authorized` by looking that commitment up in the
    // enrolment table and comparing it with the owner the payload claims, so a
    // re-attributed order refuses publicly instead of paying the wrong party.
    let scenario = balanced_residual();
    let mut session = session(&scenario);
    let mut substituted = plain_of(&session, 0);
    substituted.owner = 1;
    let run = session
        .compute(
            &Tamper::SubstitutePlaintext {
                seq: 0,
                plain: substituted,
            },
            NOW,
        )
        .expect("computes");
    assert_eq!(
        run.run.receipt.status,
        ReceiptStatus::Refused(RefusalClass::Unauthorized)
    );
}

#[test]
fn a_substituted_side_is_detected_by_the_owner_that_holds_the_position() {
    // Side is not bound by the ledger or the registry, and it settles. What
    // catches it is the slot outcome: a buy releases quote and a sell releases
    // base, so the committed entry cannot match what the owner recomputes.
    let scenario = balanced_residual();
    let mut session = session(&scenario);
    let mut substituted = plain_of(&session, 0);
    substituted.side = Side::Sell;
    let run = session
        .compute(
            &Tamper::SubstitutePlaintext {
                seq: 0,
                plain: substituted,
            },
            NOW,
        )
        .expect("computes");
    assert_eq!(run.run.receipt.status, ReceiptStatus::Settled);
    let evidence = session.owner_evidence(&run, 0);
    let report = audit(
        &session.module,
        &evidence,
        &run.run.receipt,
        &run.run.public,
    );
    assert!(
        report
            .findings
            .contains(&Finding::SlotOutcomeMismatch { seq: 0 })
    );
}

#[test]
fn a_forged_fill_above_the_owners_quantity_is_detected() {
    let scenario = balanced_residual();
    let mut session = session(&scenario);
    let plain = plain_of(&session, 1);
    let forged = SlotOutcome {
        seq: 1,
        owner: plain.owner,
        side: plain.side,
        fill: 5,
        base_delta: 5,
        quote_delta: -15,
        released_base: 0,
        released_quote: 0,
    };
    let run = session
        .compute(
            &Tamper::ForgeSlotOutcome {
                seq: 1,
                outcome: forged,
            },
            NOW,
        )
        .expect("computes");
    let evidence = session.owner_evidence(&run, 1);
    let report = audit(
        &session.module,
        &evidence,
        &run.run.receipt,
        &run.run.public,
    );
    assert!(report.findings.contains(&Finding::FillExceedsQuantity {
        seq: 1,
        fill: 5,
        quantity: 3,
    }));
}

#[test]
fn a_forged_fill_under_a_no_trade_result_is_detected() {
    let scenario = common::no_trade();
    let mut session = session(&scenario);
    let plain = plain_of(&session, 0);
    let forged = SlotOutcome {
        seq: 0,
        owner: plain.owner,
        side: plain.side,
        fill: 1,
        base_delta: 1,
        quote_delta: 0,
        released_base: 0,
        released_quote: 2,
    };
    let run = session
        .compute(
            &Tamper::ForgeSlotOutcome {
                seq: 0,
                outcome: forged,
            },
            NOW,
        )
        .expect("computes");
    let evidence = session.owner_evidence(&run, 0);
    let report = audit(
        &session.module,
        &evidence,
        &run.run.receipt,
        &run.run.public,
    );
    assert!(
        report
            .findings
            .contains(&Finding::FilledUnderNoTrade { seq: 0, fill: 1 })
    );
    assert!(report.findings.contains(&Finding::FillExceedsVolume {
        seq: 0,
        fill: 1,
        volume: 0
    }));
}

#[test]
fn a_forged_fill_outside_the_pro_rata_interval_is_detected() {
    // The only owner check that constrains the executor's arithmetic from
    // outside the owner's own row. On the deep book the feasible fills for a
    // quantity-15 order at volume 30 are exactly 7 through 15, so a committed
    // 6 is impossible for every admissible side total.
    let scenario = common::deep_book();
    let mut session = session(&scenario);
    let plain = plain_of(&session, 0);
    let honest_price = 1;
    let forged = SlotOutcome::derive(0, &plain, honest_price, 6).expect("derives");
    let run = session
        .compute(
            &Tamper::ForgeSlotOutcome {
                seq: 0,
                outcome: forged,
            },
            NOW,
        )
        .expect("computes");
    let PublicOutcome::Settled(result) = &run.run.public else {
        panic!("settles");
    };
    assert_eq!(result.tick, ClearedTick::Tick(0));
    assert_eq!(result.volume, 30);
    let evidence = session.owner_evidence(&run, 0);
    let report = audit(
        &session.module,
        &evidence,
        &run.run.receipt,
        &run.run.public,
    );
    assert!(
        report
            .findings
            .contains(&Finding::ProRataInfeasible { seq: 0, fill: 6 })
    );
    // The forged entry is internally consistent with the owner's own plaintext,
    // so nothing else in the battery fires about the slot outcome itself.
    assert!(
        !report
            .findings
            .contains(&Finding::SlotOutcomeMismatch { seq: 0 })
    );
}

#[test]
fn the_pro_rata_interval_never_rejects_an_honest_fill() {
    // Soundness of the sharpest check, over the module's whole bounded domain:
    // for every quantity, every volume, and every admissible side total, the
    // largest-remainder share and the share plus its residual atom are both
    // accepted. A check that fires on an honest run would be worse than no
    // check at all.
    let module = degg_relation_ir::module::dark_fba_n4_k4_q15_v0();
    let max_total = u64::from(module.params.slots) * module.params.quantity_ceiling;
    for quantity in 1..=module.params.quantity_ceiling {
        for volume in 0..=max_total {
            for total in quantity.max(volume)..=max_total {
                if volume == 0 {
                    assert!(pro_rata_feasible(quantity, 0, 0, max_total));
                    continue;
                }
                let base = quantity * volume / total;
                assert!(
                    pro_rata_feasible(quantity, volume, base, max_total),
                    "q={quantity} v={volume} t={total} base={base}"
                );
                if base < quantity && base < volume {
                    assert!(
                        pro_rata_feasible(quantity, volume, base + 1, max_total),
                        "q={quantity} v={volume} t={total} base+1"
                    );
                }
            }
        }
    }
}

#[test]
fn a_substitution_that_changes_the_public_result_can_evade_every_owner_check() {
    // THE HONEST CORE. The executor evaluates owner 0's buy at limit tick 0
    // instead of tick 2, keeping its side, quantity, and reservation. The
    // public clearing tick moves from 2 to 0 and the public volume from 5 to
    // 4, so every participant is paid at a different price -- and the complete
    // owner check battery of `owner::audit` accepts the run for all four
    // owners, because nothing any owner can see is violated.
    //
    // A slot outcome is a function of the owner, the side, the fill, the
    // public price, and the reservation. A substituted *limit tick* touches
    // none of them directly; it moves the clearing tick, which is public and
    // unproven, and the owner's own eligibility check passes because its real
    // limit reaches the lower tick too.
    let scenario = balanced_residual();

    let mut honest = session(&scenario);
    let honest_run = honest.compute(&Tamper::None, NOW).expect("computes");
    let PublicOutcome::Settled(honest_result) = &honest_run.run.public else {
        panic!("settles");
    };
    assert_eq!(honest_result.tick, ClearedTick::Tick(2));
    assert_eq!(honest_result.volume, 5);

    let mut session = session(&scenario);
    let mut substituted = plain_of(&session, 0);
    assert_eq!(substituted.side, Side::Buy);
    assert_eq!(substituted.limit_tick, 2);
    substituted.limit_tick = 0;
    let run = session
        .compute(
            &Tamper::SubstitutePlaintext {
                seq: 0,
                plain: substituted,
            },
            NOW,
        )
        .expect("computes");

    let PublicOutcome::Settled(result) = &run.run.public else {
        panic!("settles");
    };
    assert_eq!(result.tick, ClearedTick::Tick(0));
    assert_eq!(result.volume, 4);
    assert_ne!(result.tick, honest_result.tick);
    assert_ne!(result.volume, honest_result.volume);

    for owner in 0..session.module.params.owners {
        let evidence = session.owner_evidence(&run, owner);
        let report = audit(
            &session.module,
            &evidence,
            &run.run.receipt,
            &run.run.public,
        );
        assert!(
            report.is_consistent(),
            "owner {owner} detected the substitution: {:?}",
            report.findings
        );
    }

    // And the public role is content too: the receipt is well formed, bound to
    // the observed cutoff root, and matches the outcome it was published with.
    degg_shielded_baseline::receipt::public_check(
        &run.run.receipt,
        &session.cutoff,
        &session.executor.module_digest(),
        &run.run.public,
    )
    .expect("the public check passes on a substituted run");
}

#[test]
fn a_quantity_inflated_within_the_escrowed_budget_evades_the_battery_too() {
    // A refinement of the same core, and a correction to a tempting claim. The
    // reserve ledger bounds the reservation and rule 18 bounds
    // `quantity * price[limit]` by it, so the executor cannot inflate a
    // position's *worst-case obligation* past what was escrowed. It can still
    // trade quantity against limit tick inside that budget: owner 0's buy of 5
    // at tick 2 costs 15, and so does a buy of 15 at tick 0. The substituted
    // order carries three times the quantity, moves the public result, and
    // every owner's battery accepts it -- because owner 0's own fill stays
    // inside its own quantity, its own eligibility, and its own pro-rata
    // interval.
    let scenario = balanced_residual();
    let mut session = session(&scenario);
    let mut substituted = plain_of(&session, 0);
    assert_eq!(substituted.quantity, 5);
    assert_eq!(substituted.reserved, 15);
    substituted.quantity = 15;
    substituted.limit_tick = 0;
    let run = session
        .compute(
            &Tamper::SubstitutePlaintext {
                seq: 0,
                plain: substituted,
            },
            NOW,
        )
        .expect("computes");
    let PublicOutcome::Settled(result) = &run.run.public else {
        panic!("settles");
    };
    assert_eq!(result.tick, ClearedTick::Tick(0));
    assert_eq!(result.volume, 4);
    for owner in 0..session.module.params.owners {
        let evidence = session.owner_evidence(&run, owner);
        let report = audit(
            &session.module,
            &evidence,
            &run.run.receipt,
            &run.run.public,
        );
        assert!(
            report.is_consistent(),
            "owner {owner} detected it: {:?}",
            report.findings
        );
    }

    // What is genuinely out of reach is a larger obligation. Inflating the
    // quantity without lowering the limit exceeds the escrowed reservation and
    // is publicly refused.
    let mut over_budget = plain_of(&session, 0);
    over_budget.quantity = 15;
    let mut second = common::session(&scenario);
    let refused = second
        .compute(
            &Tamper::SubstitutePlaintext {
                seq: 0,
                plain: over_budget,
            },
            NOW,
        )
        .expect("computes");
    assert_eq!(
        refused.run.receipt.status,
        ReceiptStatus::Refused(RefusalClass::ReservationInsufficient)
    );
}
