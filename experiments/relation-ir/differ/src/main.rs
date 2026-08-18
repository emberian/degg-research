//! Differential harness: the IR Clear lowering against both existing oracles.
//!
//! For every enumerated batch the harness evaluates the frozen IR module's
//! Clear lowering, the reference toy (`dark-fba-toy`), and the independent
//! oracle (`degg-batch-oracle`), and compares complete outputs: accept versus
//! refuse, refusal class (with diagnostics against the toy), clearing tick or
//! no-trade tag, public volume, the per-slot fill vector, and every
//! owner-local output. Every divergence is a finding; the run fails on any.
//!
//! Domains (`all` runs A, B, C, then the fixtures):
//!
//! - A: every book over quantity ceiling 3 (alphabet 25, `25^4` = 390,625),
//!   owner `i` at slot `i`, exact reservations;
//! - B: every book over quantity ceiling 1 (alphabet 9, `9^4` = 6,561) crossed
//!   with all `4^4` = 256 owner assignments at exact reservations, plus the
//!   same books crossed with four reservation-surplus patterns; and
//! - C: six base books crossed with every subset of size at most two drawn
//!   from 82 named perturbation applications (6 batch-level, 19 per-slot at
//!   each of 4 slots).

mod adapter;
mod domains;

use degg_relation_ir::lower::{ClearEvaluator, LoweringTarget, lower};
use degg_relation_ir::module::dark_fba_n4_k4_q15_v0;
use domains::{applications, base_books, decode_book};

/// Aggregated result of one domain run.
#[derive(Default)]
struct Stats {
    cases: u64,
    settled: u64,
    refused: u64,
    divergences: u64,
    examples: Vec<String>,
}

impl Stats {
    fn case(
        &mut self,
        evaluator: &ClearEvaluator,
        batch: &degg_relation_ir::batch::BatchInput,
        label: &str,
    ) {
        self.cases += 1;
        let outcome = evaluator.evaluate(batch);
        match outcome {
            degg_relation_ir::lower::Outcome::Settled(_) => self.settled += 1,
            degg_relation_ir::lower::Outcome::Refused(_) => self.refused += 1,
        }
        if let Some(divergence) = adapter::compare(batch, &outcome) {
            self.divergences += 1;
            if self.examples.len() < 20 {
                self.examples.push(format!("{label}: {divergence}"));
            }
        }
    }

    fn merge(&mut self, other: Stats) {
        self.cases += other.cases;
        self.settled += other.settled;
        self.refused += other.refused;
        self.divergences += other.divergences;
        self.examples.extend(other.examples);
    }

    fn report(&self, name: &str) {
        println!(
            "domain {name}: cases={} settled={} refused={} divergences={}",
            self.cases, self.settled, self.refused, self.divergences
        );
        for example in &self.examples {
            println!("  divergence: {example}");
        }
    }
}

fn evaluator() -> ClearEvaluator {
    lower(&dark_fba_n4_k4_q15_v0(), LoweringTarget::Clear).expect("frozen module lowers")
}

/// Domain A at a given quantity ceiling: exhaustive books, owner `i` at slot
/// `i`, exact reservations.
fn domain_a(evaluator: &ClearEvaluator, ceiling: u64) -> Stats {
    let mut stats = Stats::default();
    let radix = domains::alphabet(ceiling);
    let total = radix.pow(4);
    for code in 0..total {
        let batch = decode_book(code, ceiling, [0, 1, 2, 3], [0; 4]);
        stats.case(evaluator, &batch, &format!("a/ceiling-{ceiling}/{code}"));
    }
    stats
}

/// Domain B: quantity-ceiling-1 books crossed with owner maps and with
/// reservation-surplus patterns.
fn domain_b(evaluator: &ClearEvaluator, owner_maps: &[[u8; 4]]) -> Stats {
    let mut stats = Stats::default();
    let radix = domains::alphabet(1);
    let total = radix.pow(4);
    for code in 0..total {
        for owners in owner_maps {
            let batch = decode_book(code, 1, *owners, [0; 4]);
            stats.case(evaluator, &batch, &format!("b/owners-{owners:?}/{code}"));
        }
        for surplus in [[1, 1, 1, 1], [1, 0, 0, 0], [0, 0, 0, 7], [5, 0, 2, 0]] {
            let batch = decode_book(code, 1, [0, 1, 2, 3], surplus);
            stats.case(evaluator, &batch, &format!("b/surplus-{surplus:?}/{code}"));
        }
    }
    stats
}

fn all_owner_maps() -> Vec<[u8; 4]> {
    (0..256u16)
        .map(|index| {
            [
                (index & 3) as u8,
                ((index >> 2) & 3) as u8,
                ((index >> 4) & 3) as u8,
                ((index >> 6) & 3) as u8,
            ]
        })
        .collect()
}

#[cfg(test)]
fn smoke_owner_maps() -> Vec<[u8; 4]> {
    let mut maps = Vec::new();
    for k in 0..4u8 {
        for j in 0..4u8 {
            maps.push([j, (k + j) % 4, (2 * k + j) % 4, (3 * k + j) % 4]);
        }
    }
    maps
}

/// Domain C: the refusal surface. Six base books crossed with every subset of
/// size at most two of the 82 perturbation applications.
fn domain_c(evaluator: &ClearEvaluator) -> Stats {
    let mut stats = Stats::default();
    let applications = applications();
    for (base_name, base) in base_books() {
        stats.case(evaluator, &base, &format!("c/{base_name}/unperturbed"));
        for (index, (first_label, first_apply, first_slot)) in applications.iter().enumerate() {
            let mut batch = base.clone();
            first_apply(&mut batch, *first_slot);
            stats.case(
                evaluator,
                &batch,
                &format!("c/{base_name}/{first_label}@{first_slot}"),
            );
            for (second_label, second_apply, second_slot) in &applications[index + 1..] {
                let mut batch = base.clone();
                first_apply(&mut batch, *first_slot);
                second_apply(&mut batch, *second_slot);
                stats.case(
                    evaluator,
                    &batch,
                    &format!(
                        "c/{base_name}/{first_label}@{first_slot}+{second_label}@{second_slot}"
                    ),
                );
            }
        }
    }
    stats
}

/// The named fixtures, including the two minimal check-priority witnesses.
fn fixtures_domain(evaluator: &ClearEvaluator) -> Stats {
    use degg_relation_ir::fixtures;
    let mut stats = Stats::default();
    for (name, batch) in [
        ("balanced-residual", fixtures::balanced_residual()),
        ("price-tie-low", fixtures::price_tie_low()),
        ("no-trade", fixtures::no_trade()),
        ("dark-target-request", fixtures::dark_target_request()),
        ("duplicate-nullifier", fixtures::duplicate_nullifier()),
        (
            "witness-quantity-vs-limit",
            fixtures::witness_quantity_vs_limit(),
        ),
        (
            "witness-nullifier-vs-reservation",
            fixtures::witness_nullifier_vs_reservation(),
        ),
    ] {
        stats.case(evaluator, &batch, &format!("fixtures/{name}"));
    }
    stats
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "all".to_owned());
    let evaluator = evaluator();
    let mut total = Stats::default();
    let mut run = |name: &str, stats: Stats| {
        stats.report(name);
        total.merge(stats);
    };
    match mode.as_str() {
        "a" => run("a", domain_a(&evaluator, 3)),
        "b" => run("b", domain_b(&evaluator, &all_owner_maps())),
        "c" => run("c", domain_c(&evaluator)),
        "fixtures" => run("fixtures", fixtures_domain(&evaluator)),
        "all" => {
            run("a", domain_a(&evaluator, 3));
            run("b", domain_b(&evaluator, &all_owner_maps()));
            run("c", domain_c(&evaluator));
            run("fixtures", fixtures_domain(&evaluator));
        }
        other => {
            eprintln!("unknown mode {other:?}; use a, b, c, fixtures, or all");
            std::process::exit(2);
        }
    }
    println!(
        "total: cases={} settled={} refused={} divergences={}",
        total.cases, total.settled, total.refused, total.divergences
    );
    if total.divergences > 0 {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixtures_agree_with_both_oracles() {
        let stats = fixtures_domain(&evaluator());
        assert_eq!(stats.cases, 7);
        assert_eq!(stats.settled, 3);
        assert_eq!(stats.refused, 4);
        assert_eq!(stats.divergences, 0, "{:?}", stats.examples);
    }

    #[test]
    fn domain_a_smoke_at_ceiling_two_agrees_everywhere() {
        let stats = domain_a(&evaluator(), 2);
        assert_eq!(stats.cases, 17u64.pow(4));
        assert_eq!(stats.divergences, 0, "{:?}", stats.examples);
    }

    #[test]
    fn domain_b_smoke_agrees_everywhere() {
        let stats = domain_b(&evaluator(), &smoke_owner_maps());
        assert_eq!(stats.cases, 9u64.pow(4) * (16 + 4));
        assert_eq!(stats.divergences, 0, "{:?}", stats.examples);
    }

    #[test]
    fn domain_c_agrees_everywhere() {
        let applications = applications().len() as u64;
        let stats = domain_c(&evaluator());
        assert_eq!(applications, 82);
        assert_eq!(
            stats.cases,
            6 * (1 + applications + applications * (applications - 1) / 2)
        );
        assert_eq!(stats.divergences, 0, "{:?}", stats.examples);
    }
}
