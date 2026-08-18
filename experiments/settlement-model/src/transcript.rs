//! A deterministic transcript of the settlement model, rendered as bytes.
//!
//! The corpus under `vectors/` is this function's output, checked in and
//! compared byte for byte by `tests/vectors.rs`, so a change to a tag, a
//! custody rule, an authorization check, or a surface projection shows up as
//! a diff rather than as a quietly different system.

use std::fmt::Write as _;

use degg_inclusion_availability::hash::hex;
use degg_relation_ir::batch::Side;
use degg_relation_ir::lower::{ClearedTick, PublicOutcome};
use degg_shielded_baseline::executor::Tamper;
use degg_shielded_baseline::scenario::{BookOrder, CUTOFF_EPOCH, Scenario, Session};

use crate::MODEL;
use crate::custody::CustodyLedger;
use crate::harness::{escrow_admitted, occupied_positions, settle_all};
use crate::relation::{SettlementBook, refund};
use crate::surface::{PublicProjection, SettlementSurface, project, reconstruct};

fn balanced_residual() -> Scenario {
    Scenario::new(
        "balanced-residual",
        vec![
            BookOrder::exact(0, Side::Buy, 2, 5),
            BookOrder::exact(1, Side::Buy, 1, 3),
            BookOrder::exact(2, Side::Sell, 0, 4),
            BookOrder::exact(3, Side::Sell, 2, 4),
        ],
    )
}

fn no_trade() -> Scenario {
    Scenario::new(
        "no-trade",
        vec![
            BookOrder::exact(0, Side::Buy, 0, 2),
            BookOrder::exact(1, Side::Sell, 3, 2),
        ],
    )
}

fn under_reserved() -> Scenario {
    let mut orders = vec![
        BookOrder::exact(0, Side::Buy, 2, 4),
        BookOrder::exact(1, Side::Sell, 1, 4),
    ];
    orders[1].reserved = Some(0);
    Scenario::new("under-reserved", orders)
}

fn render_custody(out: &mut String, custody: &CustodyLedger) {
    out.push_str("positions:\n");
    for (nullifier, entry) in custody.entries() {
        let _ = writeln!(
            out,
            "  owner={} asset={} escrow={} state={} nullifier={}",
            entry.owner,
            entry.asset.name(),
            entry.amount,
            entry.state.name(),
            hex(nullifier),
        );
    }
    let pool = custody.pool();
    let _ = writeln!(out, "pool: base={} quote={}", pool.base, pool.quote);
    out.push_str("accounts:\n");
    for owner in 0..4u8 {
        let account = custody.account(owner);
        let _ = writeln!(
            out,
            "  owner={owner} base={} quote={}",
            account.base, account.quote
        );
    }
    let _ = writeln!(out, "conserves: {}", custody.conserves());
}

fn render_surfaces(out: &mut String, custody: &CustodyLedger, book: &SettlementBook, price: u64) {
    let per_claim = project(custody, book, SettlementSurface::PublicPerClaim)
        .expect("the per-claim surface projects");
    if let PublicProjection::PerClaim {
        deposits,
        claims,
        refunds,
    } = &per_claim
    {
        out.push_str("per-claim surface:\n");
        for row in deposits {
            let _ = writeln!(
                out,
                "  deposit account={} asset={} amount={} epoch={}",
                row.account,
                row.asset.name(),
                row.amount,
                row.epoch
            );
        }
        for row in claims {
            let _ = writeln!(
                out,
                "  claim account={} seq={} base={} quote={} epoch={}",
                row.account, row.seq, row.credited_base, row.credited_quote, row.epoch
            );
        }
        for row in refunds {
            let _ = writeln!(
                out,
                "  refund account={} asset={} amount={} epoch={}",
                row.account,
                row.asset.name(),
                row.amount,
                row.epoch
            );
        }
        if !claims.is_empty() {
            let tick_prices = [1u64, 2, 3, 4];
            match reconstruct(deposits, claims, price, &tick_prices) {
                Ok(positions) => {
                    out.push_str("reconstruction from the public surface alone:\n");
                    for position in positions {
                        let side = match position.side {
                            Side::Buy => "buy",
                            Side::Sell => "sell",
                        };
                        let quantity = match position.sell_quantity_bound {
                            Some(bound) => format!(" sell-quantity<={bound}"),
                            None => String::new(),
                        };
                        let candidates = if position.buy_candidates.is_empty() {
                            String::new()
                        } else {
                            format!(" buy-candidates={:?}", position.buy_candidates)
                        };
                        let _ = writeln!(
                            out,
                            "  account={} seq={} side={side} fill={} budget={}{quantity}{candidates}",
                            position.account, position.seq, position.fill, position.budget
                        );
                    }
                }
                Err(defect) => {
                    let _ = writeln!(out, "reconstruction refused: {defect:?}");
                }
            }
        }
    }
    if let Ok(PublicProjection::Netted { flows }) =
        project(custody, book, SettlementSurface::PublicNetted)
    {
        out.push_str("netted surface:\n");
        for flow in flows {
            let _ = writeln!(
                out,
                "  account={} base={:+} quote={:+}",
                flow.account, flow.base, flow.quote
            );
        }
    }
    if let Ok(PublicProjection::Aggregate {
        agent,
        credited,
        refunded,
    }) = project(custody, book, SettlementSurface::ShieldedAgent)
    {
        let _ = writeln!(
            out,
            "shielded-agent surface: agent={agent} credited base={} quote={} refunded base={} quote={}",
            credited.base, credited.quote, refunded.base, refunded.quote
        );
    }
    let dark = project(custody, book, SettlementSurface::DarkTarget);
    let _ = writeln!(out, "dark target: {dark:?}");
}

fn render_settled(out: &mut String, scenario: &Scenario) {
    let _ = writeln!(out, "== scenario {}", scenario.name);
    let mut session = Session::open(scenario, CUTOFF_EPOCH).expect("scenario opens");
    let run = session
        .compute(&Tamper::None, CUTOFF_EPOCH)
        .expect("scenario computes");
    let price = match &run.run.public {
        PublicOutcome::Settled(result) => {
            let (tick, price) = match result.tick {
                ClearedTick::NoTrade => ("no-trade".to_string(), 0),
                ClearedTick::Tick(tick) => (
                    format!("tick={tick}"),
                    session.module.params.tick_prices[usize::from(tick)],
                ),
            };
            let _ = writeln!(
                out,
                "public outcome: settled {tick} volume={}",
                result.volume
            );
            price
        }
        PublicOutcome::Refused(refusal) => {
            let _ = writeln!(
                out,
                "public outcome: refused class={}",
                refusal.class.code()
            );
            0
        }
    };
    let _ = writeln!(out, "receipt binding: {}", hex(&run.run.receipt.binding));
    let _ = writeln!(
        out,
        "delivery root: {}",
        hex(&run.run.receipt.delivery_root)
    );
    let mut custody = escrow_admitted(&session);
    let mut book = SettlementBook::new();
    let obligated = book
        .observe_settled(&mut custody, &run.run.receipt, run.phase)
        .expect("a settled run obligates");
    let _ = writeln!(out, "obligated: {obligated}");
    let executions = settle_all(&session, &run, &mut custody, &mut book, CUTOFF_EPOCH);
    out.push_str("executions:\n");
    for execution in &executions {
        let _ = writeln!(
            out,
            "  seq={} owner={} base={} quote={} settlement-nullifier={}",
            execution.seq,
            execution.owner,
            execution.credited_base,
            execution.credited_quote,
            hex(&execution.settlement_nullifier),
        );
    }
    render_custody(out, &custody);
    render_surfaces(out, &custody, &book, price);
}

fn render_refused(out: &mut String, scenario: &Scenario) {
    let _ = writeln!(out, "== scenario {}", scenario.name);
    let mut session = Session::open(scenario, CUTOFF_EPOCH).expect("scenario opens");
    let run = session
        .compute(&Tamper::None, CUTOFF_EPOCH)
        .expect("scenario computes");
    if let PublicOutcome::Refused(refusal) = &run.run.public {
        let _ = writeln!(
            out,
            "public outcome: refused class={}",
            refusal.class.code()
        );
    }
    let _ = writeln!(out, "abort: {}", run.phase.name());
    let mut custody = escrow_admitted(&session);
    let book = SettlementBook::new();
    let mut refunded = 0u64;
    for index in &session.admitted {
        let submission = &session.submissions[*index];
        refunded += refund(
            &mut custody,
            run.phase,
            &submission.request.nullifier,
            CUTOFF_EPOCH,
        )
        .expect("a terminal abort refunds");
    }
    let _ = writeln!(out, "refunded: {refunded}");
    render_custody(out, &custody);
    render_surfaces(out, &custody, &book, 0);
}

fn render_exhausted(out: &mut String) {
    let scenario = balanced_residual();
    let _ = writeln!(out, "== scenario compute-exhausted");
    let mut session = Session::open(&scenario, CUTOFF_EPOCH).expect("scenario opens");
    session
        .machine
        .begin_compute(CUTOFF_EPOCH)
        .expect("availability was reported");
    session.machine.tick(15);
    session.machine.resume(15).expect("one retry is permitted");
    let phase = session.machine.tick(19);
    let _ = writeln!(out, "abort: {}", phase.name());
    let mut custody = escrow_admitted(&session);
    let book = SettlementBook::new();
    let mut refunded = 0u64;
    for index in &session.admitted {
        let submission = &session.submissions[*index];
        refunded += refund(&mut custody, phase, &submission.request.nullifier, 19)
            .expect("a terminal abort refunds");
    }
    let _ = writeln!(out, "refunded: {refunded}");
    render_custody(out, &custody);
    render_surfaces(out, &custody, &book, 0);
    let positions = occupied_positions(&session);
    let _ = writeln!(out, "occupied positions: {positions:?}");
}

/// Render the byte-stable transcript.
#[must_use]
pub fn render() -> String {
    let mut out = String::new();
    let _ = writeln!(out, "{MODEL} transcript");
    render_settled(&mut out, &balanced_residual());
    render_settled(&mut out, &no_trade());
    render_refused(&mut out, &under_reserved());
    render_exhausted(&mut out);
    out
}
