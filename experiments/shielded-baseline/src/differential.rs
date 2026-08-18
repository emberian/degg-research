//! The Shielded run against the Clear lowering, over enumerated book domains.
//!
//! What is compared, and what is not. A Shielded run *is* the module's Clear
//! evaluator executed by one named process — that is exactly what
//! [`degg_relation_ir::lower::LoweringRefusal::ShieldedBackendAbsent`] says a
//! Shielded run is, and this packet adds no lowering target. Both sides of
//! this differential therefore share the evaluator by construction, and
//! comparing them says nothing about the relation's clearing or allocation
//! semantics. Those were compared elsewhere: 2,116,916 cases by
//! `experiments/relation-ir` against two oracles, and 300,436,169 by the
//! two-oracle run of `DARK_FBA_RELATION.md` section 13.6. Saying so plainly is
//! the point: a differential whose two sides share their answer would
//! otherwise be a tautology dressed as evidence.
//!
//! What this differential does test is the composed **assembly path**: sealing,
//! admission, padded cutoff sealing, position assignment, payload opening,
//! commitment matching, padding recognition, and the derived boundary and
//! per-slot statements. The left-hand side is built from what the *submitters*
//! hold, in admission order, with every statement satisfied; the right-hand
//! side is what the executor reconstructs from the committed log alone. If any
//! step of the composition moved a slot, dropped a padding position, opened
//! the wrong payload, or derived a statement wrongly, the two sides diverge.
//!
//! The claim it supports is exactly: **the executor adds trust, and no
//! semantics.**

use degg_relation_ir::batch::Side;
use degg_relation_ir::lower::{ClearEvaluator, Outcome};

use crate::executor::Tamper;
use crate::scenario::{BookOrder, CUTOFF_EPOCH, Scenario, Session};

/// Every slot value of an enumerated domain: vacant, or a (side, tick,
/// quantity) triple over the stated quantity bound.
#[must_use]
pub fn slot_alphabet(quantity_ceiling: u64) -> Vec<Option<(Side, u8, u64)>> {
    let mut alphabet = vec![None];
    for side in [Side::Buy, Side::Sell] {
        for tick in 0..4u8 {
            for quantity in 1..=quantity_ceiling {
                alphabet.push(Some((side, tick, quantity)));
            }
        }
    }
    alphabet
}

/// Compare one book. Returns a description of the first divergence.
pub fn compare(scenario: &Scenario, evaluator: &ClearEvaluator) -> Result<(), String> {
    let mut session =
        Session::open(scenario, CUTOFF_EPOCH).map_err(|error| format!("open: {error:?}"))?;
    let reference = session.clear_reference_batch();
    let run = session
        .compute(&Tamper::None, CUTOFF_EPOCH)
        .map_err(|error| format!("compute: {error:?}"))?;

    if run.assembly.batch.slots != reference.slots {
        return Err(format!(
            "assembled slots differ: {:?} vs {:?}",
            run.assembly.batch.slots, reference.slots
        ));
    }
    let clear = evaluator.evaluate(&reference);
    if clear.public() != run.run.public {
        return Err(format!(
            "public outcome differs: {:?} vs {:?}",
            clear.public(),
            run.run.public
        ));
    }
    match (&clear, &run.run.outcome) {
        (Outcome::Settled(left), Outcome::Settled(right)) => {
            if left.fills != right.fills {
                return Err(format!(
                    "fills differ: {:?} vs {:?}",
                    left.fills, right.fills
                ));
            }
            if left.owners != right.owners {
                return Err("owner outputs differ".to_owned());
            }
        }
        (Outcome::Refused(left), Outcome::Refused(right)) => {
            if left.refusal != right.refusal {
                return Err(format!(
                    "refusal differs: {:?} vs {:?}",
                    left.refusal, right.refusal
                ));
            }
        }
        _ => return Err("one side settled and the other refused".to_owned()),
    }
    Ok(())
}

/// Enumerate every book over the alphabet and compare all of them.
///
/// Owner `i` holds slot `i` and every order reserves exactly its worst-case
/// obligation, so the enumerated dimensions are occupancy, side, tick, and
/// quantity. The number of occupied positions ranges over `0..=4`, which
/// exercises padded sealing at every occupancy.
pub fn run_domain(quantity_ceiling: u64) -> Result<usize, String> {
    let module = degg_relation_ir::module::dark_fba_n4_k4_q15_v0();
    let evaluator = Session::clear_evaluator(&module);
    let alphabet = slot_alphabet(quantity_ceiling);
    let mut cases = 0usize;
    for a in &alphabet {
        for b in &alphabet {
            for c in &alphabet {
                for d in &alphabet {
                    let mut orders = Vec::new();
                    for (slot, entry) in [a, b, c, d].into_iter().enumerate() {
                        if let Some((side, tick, quantity)) = entry {
                            let owner = u8::try_from(slot).expect("four slots");
                            orders.push(BookOrder::exact(owner, *side, *tick, *quantity));
                        }
                    }
                    let scenario = Scenario::new("differential", orders);
                    compare(&scenario, &evaluator)
                        .map_err(|reason| format!("divergence at case {cases}: {reason}"))?;
                    cases += 1;
                }
            }
        }
    }
    Ok(cases)
}
