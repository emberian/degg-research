//! A deterministic transcript of the whole model, rendered as bytes.
//!
//! The corpus under `vectors/` is this function's output. It is checked into
//! the repository and compared byte for byte by `tests/vectors.rs`, so a
//! change to a tag, a commitment rule, a derived statement, a receipt binding,
//! a delivery entry, or an abort consequence shows up as a diff rather than as
//! a quietly different system.

use std::fmt::Write as _;

use degg_inclusion_availability::hash::hex;
use degg_inclusion_availability::lifecycle::{
    AbortClass, BatchMachine, Entitlement, Phase, RefundError,
};
use degg_relation_ir::batch::Side;
use degg_relation_ir::canon::Canonical;
use degg_relation_ir::lower::{ClearedTick, Outcome, PublicOutcome};
use degg_relation_ir::receipt::ReceiptStatus;

use crate::MODEL;
use crate::dispute::{
    OmissionProof, OutcomeEquivocationProof, verify_omission, verify_outcome_equivocation,
};
use crate::executor::Tamper;
use crate::owner::audit;
use crate::receipt::DeliveryEntry;
use crate::scenario::{BookOrder, CUTOFF_EPOCH, Scenario, Session};

fn scenarios() -> Vec<Scenario> {
    vec![
        Scenario::new(
            "balanced-residual",
            vec![
                BookOrder::exact(0, Side::Buy, 2, 5),
                BookOrder::exact(1, Side::Buy, 1, 3),
                BookOrder::exact(2, Side::Sell, 0, 4),
                BookOrder::exact(3, Side::Sell, 2, 4),
            ],
        ),
        Scenario::new(
            "price-tie-low",
            vec![
                BookOrder::exact(0, Side::Buy, 2, 4),
                BookOrder::exact(1, Side::Sell, 1, 4),
            ],
        ),
        Scenario::new(
            "no-trade",
            vec![
                BookOrder::exact(0, Side::Buy, 0, 2),
                BookOrder::exact(1, Side::Sell, 3, 2),
            ],
        ),
        {
            let mut orders = vec![
                BookOrder::exact(0, Side::Buy, 2, 4),
                BookOrder::exact(1, Side::Sell, 1, 4),
            ];
            orders[1].reserved = Some(0);
            Scenario::new("under-reserved", orders)
        },
    ]
}

fn tick(tick: ClearedTick) -> String {
    match tick {
        ClearedTick::NoTrade => "no-trade".to_owned(),
        ClearedTick::Tick(index) => format!("tick-{index}"),
    }
}

fn status(status: ReceiptStatus) -> String {
    match status {
        ReceiptStatus::Settled => "settled".to_owned(),
        ReceiptStatus::Refused(class) => format!("refused:{}", class.tag()),
    }
}

fn render_scenario(out: &mut String, scenario: &Scenario) {
    let mut session = Session::open(scenario, CUTOFF_EPOCH).expect("scenario opens");
    let run = session
        .compute(&Tamper::None, CUTOFF_EPOCH)
        .expect("computes");
    let receipt = run.run.receipt;

    let _ = writeln!(out, "\n[scenario {}]", scenario.name);
    let _ = writeln!(
        out,
        "log submitted={} admitted={} refused={} leaves={}",
        session.submissions.len(),
        session.admitted.len(),
        session.refused.len(),
        session.cutoff.leaf_count
    );
    let _ = writeln!(out, "cutoff_root {}", hex(&session.cutoff.root));
    for (index, fact) in run.assembly.boundary.facts.iter().enumerate() {
        let _ = writeln!(
            out,
            "boundary {index} port={} holds={} object={}",
            fact.port, fact.holds, fact.object
        );
    }
    match &run.run.public {
        PublicOutcome::Settled(result) => {
            let _ = writeln!(
                out,
                "public settled tick={} volume={}",
                tick(result.tick),
                result.volume
            );
        }
        PublicOutcome::Refused(refusal) => {
            let _ = writeln!(out, "public refused class={}", refusal.class.tag());
        }
    }
    if let Outcome::Settled(settled) = &run.run.outcome {
        let _ = writeln!(out, "label {}", settled.executed_label);
    }
    let _ = writeln!(out, "receipt status={}", status(receipt.status));
    let _ = writeln!(out, "receipt module_digest {}", hex(&receipt.module_digest));
    let _ = writeln!(out, "receipt input_digest {}", hex(&receipt.input_digest));
    let _ = writeln!(
        out,
        "receipt outcome_digest {}",
        hex(&receipt.outcome_digest)
    );
    let _ = writeln!(out, "receipt delivery_root {}", hex(&receipt.delivery_root));
    let _ = writeln!(out, "receipt binding {}", hex(&receipt.binding));
    for opening in &run.openings {
        let kind = match opening.entry {
            DeliveryEntry::NoLocalOutput => "none".to_owned(),
            DeliveryEntry::Produced(outcome) => format!("fill={}", outcome.fill),
        };
        let padding = session.receipts[usize::try_from(opening.seq).expect("bounded")]
            .record
            .is_padding(&session.domain);
        let _ = writeln!(
            out,
            "delivery seq={} padding={} {} entry={} path={}",
            opening.seq,
            padding,
            kind,
            hex(&opening.entry.preimage(opening.seq)),
            opening.proof.path.len()
        );
    }
    for owner in 0..session.module.params.owners {
        let evidence = session.owner_evidence(&run, owner);
        let report = audit(&session.module, &evidence, &receipt, &run.run.public);
        let delivered = run.delivered[usize::from(owner)]
            .as_ref()
            .map_or_else(|| "none".to_owned(), |output| hex(&output.digest()));
        let _ = writeln!(
            out,
            "owner {owner} positions={} findings={} refusal={:?} delivered={delivered}",
            evidence.positions.len(),
            report.findings.len(),
            report.refusal
        );
    }
}

fn render_disputes(out: &mut String) {
    let scenario = &scenarios()[0];
    let mut honest = Session::open(scenario, CUTOFF_EPOCH).expect("opens");
    let honest_run = honest
        .compute(&Tamper::None, CUTOFF_EPOCH)
        .expect("computes");

    let mut dropped = Session::open(scenario, CUTOFF_EPOCH).expect("opens");
    let dropped_run = dropped
        .compute(&Tamper::OmitPosition { seq: 2 }, CUTOFF_EPOCH)
        .expect("computes");

    let mut substituted_plain = honest.submissions[0].plain;
    substituted_plain.limit_tick = 0;
    let mut substituted = Session::open(scenario, CUTOFF_EPOCH).expect("opens");
    let substituted_run = substituted
        .compute(
            &Tamper::SubstitutePlaintext {
                seq: 0,
                plain: substituted_plain,
            },
            CUTOFF_EPOCH,
        )
        .expect("computes");

    let _ = writeln!(out, "\n[disputes]");
    let omission = OmissionProof {
        cutoff: dropped.cutoff,
        receipt: dropped_run.run.receipt,
        inclusion: dropped.receipts[2].clone(),
        opening: dropped_run.openings[2].clone(),
    };
    match verify_omission(&omission) {
        Ok(verdict) => {
            let _ = writeln!(
                out,
                "omission seq={} verdict {}",
                verdict.seq,
                hex(&verdict.digest)
            );
        }
        Err(defect) => {
            let _ = writeln!(out, "omission rejected {defect:?}");
        }
    }
    let proof = OutcomeEquivocationProof {
        left: honest_run.run.receipt,
        right: substituted_run.run.receipt,
    };
    match verify_outcome_equivocation(&proof) {
        Ok(verdict) => {
            let _ = writeln!(
                out,
                "outcome_equivocation class={} verdict {}",
                verdict.class,
                hex(&verdict.digest)
            );
        }
        Err(defect) => {
            let _ = writeln!(out, "outcome_equivocation rejected {defect:?}");
        }
    }
    let _ = writeln!(
        out,
        "substituted_public_result_evades_every_owner_check {}",
        (0..substituted.module.params.owners).all(|owner| {
            let evidence = substituted.owner_evidence(&substituted_run, owner);
            audit(
                &substituted.module,
                &evidence,
                &substituted_run.run.receipt,
                &substituted_run.run.public,
            )
            .is_consistent()
        })
    );
}

fn render_aborts(out: &mut String) {
    let scenario = &scenarios()[0];
    let mut session = Session::open(scenario, CUTOFF_EPOCH).expect("opens");
    let escrowed = session.ledger.total_escrowed();
    let _ = writeln!(out, "\n[abort]");
    let _ = writeln!(out, "escrowed {escrowed}");

    let first = CUTOFF_EPOCH + session.timeouts.compute + 1;
    let phase = session.machine.tick(first);
    let _ = writeln!(out, "crash phase={} epoch={first}", phase.name());
    session.machine.resume(first).expect("one retry");
    let second = CUTOFF_EPOCH + session.timeouts.compute * 2 + 1;
    let phase = session.machine.tick(second);
    let _ = writeln!(out, "crash phase={} epoch={second}", phase.name());
    if let Phase::Aborted(class) = phase {
        let _ = writeln!(out, "consequence {:?}", class.consequence());
    }
    for seq in 0..u32::try_from(session.cutoff.leaf_count).expect("bounded") {
        let receipt = &session.receipts[usize::try_from(seq).expect("bounded")];
        let claim = session
            .machine
            .claim_refund(&mut session.ledger, &Entitlement::Included(receipt));
        let rendered = match claim {
            Ok(amount) => format!("refund={amount}"),
            Err(RefundError::NotEscrowed) => "not-escrowed".to_owned(),
            Err(error) => format!("{error:?}"),
        };
        let _ = writeln!(out, "claim seq={seq} {rendered}");
    }
    let _ = writeln!(
        out,
        "ledger refunded={} settled={} outstanding={} conserves={}",
        session.ledger.total_refunded(),
        session.ledger.total_settled(),
        session.ledger.total_outstanding(),
        session.ledger.conserves()
    );

    // Composition gap C-1, closed: a typed relation refusal is delivered as a
    // refusal, reaches the `relation-refused` abort, and refunds every
    // admitted record.
    let mut refused = Session::open(&scenarios()[3], CUTOFF_EPOCH).expect("opens");
    let run = refused
        .compute(&Tamper::None, CUTOFF_EPOCH)
        .expect("computes");
    let _ = writeln!(
        out,
        "c1 relation={} lifecycle_phase={} terminal={}",
        status(run.run.receipt.status),
        run.phase.name(),
        run.phase.is_terminal()
    );
    if let Phase::Aborted(class) = run.phase {
        let _ = writeln!(out, "c1 consequence {:?}", class.consequence());
    }
    for seq in 0..u32::try_from(refused.cutoff.leaf_count).expect("bounded") {
        let receipt = &refused.receipts[usize::try_from(seq).expect("bounded")];
        let claim = refused
            .machine
            .claim_refund(&mut refused.ledger, &Entitlement::Included(receipt));
        let rendered = match claim {
            Ok(amount) => format!("refund={amount}"),
            Err(RefundError::NotEscrowed) => "not-escrowed".to_owned(),
            Err(RefundError::PhaseNotRefundable { phase }) => {
                format!("phase-not-refundable:{}", phase.name())
            }
            Err(error) => format!("{error:?}"),
        };
        let _ = writeln!(out, "c1 claim seq={seq} {rendered}");
    }
    let _ = writeln!(
        out,
        "c1 ledger refunded={} settled={} outstanding={} conserves={}",
        refused.ledger.total_refunded(),
        refused.ledger.total_settled(),
        refused.ledger.total_outstanding(),
        refused.ledger.conserves()
    );
    let nullifier = refused.submissions[0].request.nullifier;
    let release = refused
        .machine
        .release_to_settlement(&mut refused.ledger, nullifier);
    let _ = writeln!(
        out,
        "c1 release_to_settlement {}",
        match release {
            Ok(amount) => format!("released={amount}"),
            Err(RefundError::PhaseNotSettled { phase }) => {
                format!("phase-not-settled:{}", phase.name())
            }
            Err(error) => format!("{error:?}"),
        }
    );
    let _ = writeln!(
        out,
        "c1 cutoff_withheld_class={}",
        AbortClass::CutoffRootWithheld.class()
    );
    let watcher = BatchMachine::new(refused.domain, refused.timeouts);
    let _ = writeln!(out, "c1 fresh_watcher_phase={}", watcher.phase().name());
}

/// Render the full corpus.
#[must_use]
pub fn render() -> String {
    let module = degg_relation_ir::module::dark_fba_n4_k4_q15_v0();
    let mut out = String::new();
    let _ = writeln!(out, "model {MODEL}");
    let _ = writeln!(out, "relation {}", module.identity.relation);
    let _ = writeln!(out, "module_digest {}", hex(&module.digest()));
    let _ = writeln!(
        out,
        "inclusion_model {}",
        degg_inclusion_availability::MODEL
    );
    let _ = writeln!(
        out,
        "mode shielded-single-executor executor_learns_every_private_field=true"
    );
    for scenario in scenarios() {
        render_scenario(&mut out, &scenario);
    }
    render_disputes(&mut out);
    render_aborts(&mut out);
    out
}
