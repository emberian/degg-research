//! A deterministic transcript of the whole experiment, rendered as bytes.
//!
//! The corpus file under `vectors/` is this function's output. It is checked
//! into the repository and compared byte for byte by `tests/vectors.rs`, so any
//! change to a bound, a criterion, an aggregation story, a witness order, or a
//! count shows up as a diff rather than as a quietly different result.

use crate::census::CensusRow;
use crate::criteria::candidates;
use crate::invariance::{CriterionReport, Reading, Witness};
use crate::market::{Market, Op};
use crate::payoff::{Facts, Payoff, PayoffObject, ReferenceKind, unbundle_elementary};

/// Render the full transcript.
///
/// # Panics
///
/// Panics if the scripted market demonstration is refused, which would be a bug
/// in this crate.
#[must_use]
pub fn render() -> String {
    let report = crate::report();
    let bounds = report.bounds;
    let mut out = String::new();

    out.push_str(&format!("model {}\n", crate::MODEL));
    out.push_str(&format!("memo {}\n", crate::MEMO));
    out.push_str(&format!(
        "corpus cells={}..{} ceiling={} vectors={} fact_profiles={} objects={} decompositions_per_criterion={}\n",
        bounds.min_cells,
        bounds.max_cells,
        bounds.ceiling,
        bounds.vectors,
        bounds.fact_profiles,
        bounds.objects,
        bounds.decompositions,
    ));
    out.push('\n');

    operations_section(&mut out);
    paired_payoffs_section(&mut out);
    for criterion in &report.criteria {
        criterion_section(&mut out, criterion);
    }
    census_section(&mut out, crate::census_rows());
    summary_section(&mut out, &report.criteria);
    out
}

fn operations_section(out: &mut String) {
    out.push_str("[operations]\n");
    out.push_str(
        "claims are minted only as complete sets against collateral and burned only as complete sets\n",
    );
    let script = [
        Op::Deposit {
            holder: 0,
            units: 2,
        },
        Op::Split {
            holder: 0,
            take: Payoff::new(vec![1, 0, 0]),
        },
        Op::Transfer {
            from: 0,
            to: 1,
            cell: 1,
            units: 1,
        },
        Op::Bundle { to: 1, from: 2 },
        Op::Recombine {
            holder: 0,
            units: 1,
        },
    ];
    let mut market = Market::open(3, 2);
    out.push_str(&format!(
        "open cells=3 holders=2 outstanding={} locked={} conserved={}\n",
        market.outstanding(),
        market.collateral_locked(),
        market.conserved(),
    ));
    for op in &script {
        market
            .apply(op)
            .expect("the scripted market is well formed");
        let positions: Vec<String> = market
            .positions()
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        out.push_str(&format!(
            "{op} -> positions {} outstanding={} locked={} moves_collateral={} conserved={}\n",
            positions.join(" "),
            market.outstanding(),
            market.collateral_locked(),
            Market::moves_collateral(op),
            market.conserved(),
        ));
    }
    out.push('\n');
}

/// The four payoff shapes the memo names, plus the complete set, each expressed
/// twice: once as a bespoke instrument with its own settlement function, and
/// once as a portfolio over the exhaustive basis of one-cell claims.
///
/// The five cells are read as bands of one reference variable, low to high.
const PAIRED_PAYOFFS: [(&str, [u64; 5]); 5] = [
    ("digital", [0, 1, 0, 0, 0]),
    ("range", [0, 1, 1, 0, 0]),
    ("capped_directional", [0, 1, 2, 3, 3]),
    ("tail", [0, 0, 0, 0, 3]),
    ("complete_set", [1, 1, 1, 1, 1]),
];

fn paired_payoffs_section(out: &mut String) {
    out.push_str("[paired_payoffs]\n");
    out.push_str(
        "each shape expressed twice: as one bespoke instrument, and as a portfolio over the basis of one-cell claims\n",
    );
    let criteria = candidates();
    let facts = Facts::new(ReferenceKind::SecurityPrice, true, true);
    for (name, amounts) in PAIRED_PAYOFFS {
        let whole = Payoff::new(amounts.to_vec());
        let object = PayoffObject::new(whole.clone(), facts);
        let parts = unbundle_elementary(&object);
        let mut rebuilt = Payoff::zero(whole.cells());
        for part in &parts {
            rebuilt = rebuilt
                .add(part.payoff())
                .expect("the parts share the partition");
        }
        let portfolio: Vec<String> = (0..whole.cells())
            .filter(|cell| whole.get(*cell) > 0)
            .map(|cell| format!("{}x{}", whole.get(cell), Payoff::unit(whole.cells(), cell)))
            .collect();
        out.push_str(&format!(
            "shape {name} bespoke={whole} portfolio={} cashflows={rebuilt} equal_in_every_state={}\n",
            portfolio.join("+"),
            rebuilt == whole,
        ));
        for criterion in &criteria {
            let whole_label = criterion.classify(&object);
            let first = criterion.classify(&parts[0]);
            let unanimous = parts
                .iter()
                .all(|part| criterion.classify(part) == first)
                .then_some(first);
            let parts_label =
                unanimous.map_or_else(|| "mixed".to_owned(), |l| l.label().to_owned());
            let consistent = unanimous.is_none_or(|l| l == whole_label);
            out.push_str(&format!(
                "  {} bespoke={whole_label} parts={parts_label} same_answer={consistent}\n",
                criterion.name(),
            ));
        }
    }
    out.push('\n');
}

fn criterion_section(out: &mut String, criterion: &CriterionReport) {
    out.push_str(&format!("[criterion {}]\n", criterion.name));
    let parameters = if criterion.parameters.is_empty() {
        "none".to_owned()
    } else {
        criterion.parameters.clone()
    };
    out.push_str(&format!("parameters {parameters}\n"));
    out.push_str(&format!("statement {}\n", criterion.statement));
    out.push_str(&format!("story {}\n", criterion.story));
    let labels: Vec<&str> = criterion.labels.iter().map(|label| label.label()).collect();
    out.push_str(&format!("labels {}\n", labels.join(" ")));
    out.push_str(&format!(
        "reads payoff={} facts={}\n",
        criterion.reads_payoff, criterion.reads_facts
    ));
    out.push_str(&format!(
        "story_permits_alternatives {}\n",
        criterion.story_permits_alternatives
    ));
    out.push_str(&format!("classified {}\n", criterion.objects_classified));
    out.push_str(&format!(
        "decompositions checked={} unanimous={}\n",
        criterion.decompositions_checked, criterion.unanimous_decompositions
    ));
    out.push_str(&format!(
        "violations strict={} declared_story={}\n",
        criterion.violations_strict, criterion.violations_declared_story
    ));
    out.push_str(&format!(
        "violating_objects strict={}\n",
        criterion.violating_objects_strict
    ));
    out.push_str(&format!(
        "verdict strict={} declared_story={}\n",
        verdict(criterion.invariant_strict()),
        verdict(criterion.invariant_declared_story()),
    ));
    witness_block(out, Reading::Strict, criterion.minimal_strict.as_ref());
    witness_block(
        out,
        Reading::DeclaredStory,
        criterion.minimal_declared_story.as_ref(),
    );
    out.push('\n');
}

const fn verdict(invariant: bool) -> &'static str {
    if invariant { "invariant" } else { "arbitrage" }
}

fn witness_block(out: &mut String, reading: Reading, witness: Option<&Witness>) {
    let Some(witness) = witness else {
        out.push_str(&format!("minimal_witness {} none\n", reading.label()));
        return;
    };
    out.push_str(&format!(
        "minimal_witness {} family={} cells={} {}\n",
        reading.label(),
        witness.family.label(),
        witness.cells,
        witness.facts.label(),
    ));
    out.push_str(&format!(
        "  whole {} label={}\n",
        witness.whole, witness.whole_label
    ));
    for part in &witness.parts {
        out.push_str(&format!("  part {} label={}\n", part, witness.part_label));
    }
    let permitted: Vec<&str> = witness
        .permitted
        .iter()
        .map(|label| label.label())
        .collect();
    out.push_str(&format!("  permitted {}\n", permitted.join(" ")));
    out.push_str(&format!(
        "  conserved claims_before={} claims_after={} equal={} collateral_delta=0 complete_sets={}\n",
        witness.whole,
        witness.parts_total(),
        witness.conserves_claims(),
        witness.complete_sets,
    ));
    out.push_str(&format!("  path {}\n", path_of(witness)));
}

fn path_of(witness: &Witness) -> String {
    let base = match witness.family {
        crate::invariance::Family::BinarySplit => {
            "split the position in two, then bundle it back; no claim is minted or burned and no collateral moves"
        }
        crate::invariance::Family::ElementaryUnbundle => {
            "state the position as its individual one-cell claims, then hold them together again; no claim is minted or burned and no collateral moves"
        }
    };
    match witness.complete_sets {
        0 => base.to_owned(),
        1 => format!(
            "{base}; the bundle also contains 1 complete set, recombinable into its collateral before resolution"
        ),
        sets => format!(
            "{base}; the bundle also contains {sets} complete sets, recombinable into their collateral before resolution"
        ),
    }
}

fn census_section(out: &mut String, rows: &[CensusRow]) {
    out.push_str("[census]\n");
    out.push_str("every two-label criterion that reads only which outcomes an object pays in\n");
    for row in rows {
        out.push_str(&format!(
            "cells={} criteria={} invariant={} invariant_discriminating={}\n",
            row.cells, row.total, row.invariant, row.invariant_discriminating
        ));
    }
    out.push('\n');
}

fn summary_section(out: &mut String, criteria: &[CriterionReport]) {
    out.push_str("[summary]\n");
    for criterion in criteria {
        out.push_str(&format!(
            "{} reads_payoff={} reads_facts={} labels={} strict={} declared_story={} verdict={}\n",
            criterion.name,
            criterion.reads_payoff,
            criterion.reads_facts,
            criterion.labels.len(),
            criterion.violations_strict,
            criterion.violations_declared_story,
            verdict(criterion.invariant_strict()),
        ));
    }
}
