//! The transfer-surface measurements: what each settlement surface
//! mechanically publishes, measured against ground truth the surfaces were
//! never shown.

mod common;

use common::{balanced_residual, computed, ready, under_reserved};
use degg_relation_ir::batch::Side;
use degg_settlement_model::custody::{Asset, Balances};
use degg_settlement_model::harness::settle_all;
use degg_settlement_model::relation::{SettlementBook, refund};
use degg_settlement_model::surface::{
    PublicProjection, SettlementSurface, SurfaceRefusal, project, reconstruct,
};

fn settled_balanced() -> common::Ready {
    let mut ready = ready(&balanced_residual());
    settle_all(
        &ready.session,
        &ready.run,
        &mut ready.custody,
        &mut ready.book,
        common::NOW,
    );
    ready
}

#[test]
fn the_per_claim_surface_reconstructs_the_book_the_computation_hid() {
    // The reconstruction consumes only what the public holds on a
    // transparent settlement surface: deposit rows, claim rows, the public
    // price, and the frozen grid. It recovers every settled position's
    // owner, side, and exact fill; every sell's exact quantity; and for
    // every buy a candidate set containing the true limit and quantity.
    // The computation's leakage table hid all of it.
    let ready = settled_balanced();
    let projection = project(
        &ready.custody,
        &ready.book,
        SettlementSurface::PublicPerClaim,
    )
    .expect("projects");
    let PublicProjection::PerClaim {
        deposits, claims, ..
    } = &projection
    else {
        panic!("the per-claim surface projects rows");
    };
    let positions = reconstruct(deposits, claims, 3, &[1, 2, 3, 4]).expect("reconstructs");
    assert_eq!(positions.len(), 4);

    // Ground truth, from the submitters' own plaintexts.
    let truth = [
        (0u8, 0u32, Side::Buy, 5u64, 15u64),
        (1, 1, Side::Buy, 0, 6),
        (2, 2, Side::Sell, 3, 4),
        (3, 3, Side::Sell, 2, 4),
    ];
    for ((account, seq, side, fill, budget), inferred) in truth.into_iter().zip(&positions) {
        assert_eq!(inferred.account, account);
        assert_eq!(inferred.seq, seq, "the claim discloses the admission rank");
        assert_eq!(inferred.side, side, "the deposit asset discloses the side");
        assert_eq!(
            inferred.fill, fill,
            "the credited amounts disclose the fill"
        );
        assert_eq!(inferred.budget, budget);
    }

    // Sells: the reservation equals the quantity when reserved at the exact
    // worst case, as these are.
    assert_eq!(positions[2].sell_quantity_bound, Some(4));
    assert_eq!(positions[3].sell_quantity_bound, Some(4));

    // Buys: the true (limit, quantity) pair is inside the candidate set.
    assert_eq!(positions[0].buy_candidates, vec![(0, 15), (2, 5)]);
    assert!(positions[0].buy_candidates.contains(&(2, 5)));
    assert_eq!(positions[1].buy_candidates, vec![(0, 6), (1, 3), (2, 2)]);
    assert!(positions[1].buy_candidates.contains(&(1, 3)));
}

#[test]
fn the_netted_surface_publishes_exactly_the_owner_deltas() {
    // Netting hides per-position rows and reveals precisely the signed
    // per-owner deltas — the very numbers the computation's leakage table
    // marks owner-local — plus exact participation, including the zero-flow
    // owner.
    let ready = settled_balanced();
    let projection =
        project(&ready.custody, &ready.book, SettlementSurface::PublicNetted).expect("projects");
    let PublicProjection::Netted { flows } = &projection else {
        panic!("the netted surface projects flows");
    };
    assert_eq!(flows.len(), 4, "participation is exact");
    for flow in flows {
        let delivered = ready.run.delivered[usize::from(flow.account)]
            .as_ref()
            .expect("every owner received a local output");
        assert_eq!(flow.base, delivered.base_delta, "owner {}", flow.account);
        assert_eq!(flow.quote, delivered.quote_delta, "owner {}", flow.account);
    }
    // The zero-fill owner appears with zero flow: netting still names it.
    assert_eq!(flows[1].base, 0);
    assert_eq!(flows[1].quote, 0);
}

#[test]
fn the_shielded_agent_surface_publishes_totals_and_names_who_sees_the_rest() {
    let ready = settled_balanced();
    let projection = project(
        &ready.custody,
        &ready.book,
        SettlementSurface::ShieldedAgent,
    )
    .expect("projects");
    let PublicProjection::Aggregate {
        agent,
        credited,
        refunded,
    } = projection
    else {
        panic!("the agent surface projects totals");
    };
    assert_eq!(agent, "degg-named-settlement-agent/v0");
    assert_eq!(credited, Balances { base: 8, quote: 21 });
    assert_eq!(refunded, Balances::zero());
    // The projection type itself carries no account, position, or amount
    // rows; the named agent holds every row, which is what Shielded means.
}

#[test]
fn the_dark_settlement_target_refuses() {
    let ready = settled_balanced();
    assert_eq!(
        project(&ready.custody, &ready.book, SettlementSurface::DarkTarget),
        Err(SurfaceRefusal::DarkSettlementAbsent)
    );
}

#[test]
fn public_refunds_disclose_budgets_and_sides_without_any_trade() {
    // An aborted batch never traded, and a transparent refund lane still
    // publishes each admitted order's spending asset — the side — and its
    // full reservation.
    let (session, run, mut custody) = computed(&under_reserved());
    for index in &session.admitted {
        let submission = &session.submissions[*index];
        refund(
            &mut custody,
            run.phase,
            &submission.request.nullifier,
            common::NOW,
        )
        .expect("refunds");
    }
    let book = SettlementBook::new();
    let projection = project(&custody, &book, SettlementSurface::PublicPerClaim).expect("projects");
    let PublicProjection::PerClaim { refunds, .. } = &projection else {
        panic!("rows");
    };
    assert_eq!(refunds.len(), 2);
    assert_eq!(refunds[0].account, 0);
    assert_eq!(refunds[0].asset, Asset::Quote);
    assert_eq!(refunds[0].amount, 12);
    assert_eq!(refunds[1].account, 1);
    assert_eq!(refunds[1].asset, Asset::Base);
    assert_eq!(refunds[1].amount, 0);
}

#[test]
fn deposits_alone_disclose_sides_before_the_batch_even_runs() {
    // The escrow deposits precede computation, and on a two-asset
    // transparent custody they already reveal each participant's side. The
    // computation's "side: hidden" row is falsified at the settlement layer
    // before any clearing happens.
    let ready = ready(&balanced_residual());
    let projection = project(
        &ready.custody,
        &ready.book,
        SettlementSurface::PublicPerClaim,
    )
    .expect("projects");
    let PublicProjection::PerClaim { deposits, .. } = &projection else {
        panic!("rows");
    };
    let sides: Vec<(u8, Asset)> = deposits
        .iter()
        .map(|deposit| (deposit.account, deposit.asset))
        .collect();
    assert_eq!(
        sides,
        vec![
            (0, Asset::Quote),
            (1, Asset::Quote),
            (2, Asset::Base),
            (3, Asset::Base),
        ],
        "quote deposit = buy, base deposit = sell"
    );
}
