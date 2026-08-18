//! Exhaustive differential harness over two independent implementations of
//! `dark-fba/n4-k4-q15/v0`.
//!
//! The two oracles are `degg-batch-oracle` (written from the specification
//! alone, without reading the other source) and `dark-fba-toy` (the existing
//! offline toy). This binary enumerates precisely stated finite domains of
//! batches, runs both oracles on every one, and compares their complete
//! outputs: refusal class, clearing tick, public volume, per-slot allocation,
//! and every owner-local output.
//!
//! A divergence is a finding. Neither implementation is edited to make a
//! divergence go away.

#![deny(missing_docs)]

mod adapter;
mod domains;
mod report;
mod vectors;

use std::thread;

use adapter::Divergence;
use degg_batch_oracle::book::{Batch, Direction, Order, Slot};
use degg_batch_oracle::params::SLOTS;
use domains::{
    BATCH_PERTURBATIONS, FULL_CEILING, Perturbation, SLOT_PERTURBATIONS, alphabet, decode_book,
    empty_batch,
};
use report::{CLASSES, Report, describe, shrink, verdicts};

fn main() {
    let selector = std::env::args().nth(1).unwrap_or_else(|| "all".to_owned());
    let mut findings: Vec<(&'static str, Divergence, Batch, String)> = Vec::new();

    if selector == "all" || selector == "a" {
        let report = domain_a();
        emit("A: complete order-book domain", &report, &mut findings, "A");
    }
    if selector == "all" || selector == "b" {
        let report = domain_b();
        emit(
            "B: owner assignment and reservation surplus",
            &report,
            &mut findings,
            "B",
        );
    }
    if selector == "all" || selector == "c" {
        let report = domain_c();
        emit(
            "C: admission perturbations up to arity three",
            &report,
            &mut findings,
            "C",
        );
    }
    if selector == "all" || selector == "vectors" {
        vectors::report();
    }

    if selector == "all" || selector == "a" || selector == "b" || selector == "c" {
        println!();
        println!("== shrunk divergence witnesses ==");
        if findings.is_empty() {
            println!("none");
        }
        for (domain, class, batch, label) in &findings {
            let signature = label.split(" :: ").next().unwrap_or(label).to_owned();
            let small = shrink(batch, &signature);
            println!(
                "[{domain}] {class:?}{}",
                if label.is_empty() {
                    String::new()
                } else {
                    format!("  {label}")
                }
            );
            println!("    unshrunk: {}", describe(batch));
            println!("    minimal:  {}", describe(&small));
            println!("    verdicts: {}", verdicts(&small));
        }
    }
}

fn emit(
    title: &str,
    report: &Report,
    findings: &mut Vec<(&'static str, Divergence, Batch, String)>,
    tag: &'static str,
) {
    println!("== domain {title} ==");
    println!("  cases enumerated:     {}", report.cases);
    println!("  both settled:         {}", report.settled);
    println!("    of which no-trade:  {}", report.no_trade);
    println!("  both refused:         {}", report.refused);
    println!("  divergences:          {}", report.divergences());
    for (index, class) in CLASSES.iter().enumerate() {
        if report.by_class[index] > 0 {
            println!("    {class:?}: {}", report.by_class[index]);
        }
    }
    if !report.pairs.is_empty() {
        println!("  refusal-class pairs (independent -> existing), all priority unless noted:");
        for ((ours, theirs), count) in &report.pairs {
            println!("    {ours} vs {theirs}: {count}");
        }
        println!(
            "  refusal-class divergences naming a rule the witness does not violate: {}",
            report.unjustified
        );
    }
    for (class, batch, label) in &report.examples {
        findings.push((tag, *class, *batch, label.clone()));
    }
    println!();
}

/// Domain A: every book over the relation's complete frozen order domain.
///
/// Each of the four padded slots independently takes one of `1 + 2 * 4 * 15 =
/// 121` letters: vacancy, or a (side, tick, quantity) triple with quantity in
/// `1..=15`. That is `121^4 = 214_358_881` books. Admission fields are
/// canonical and admissible throughout, so this domain isolates the clearing
/// and allocation semantics: aggregate curves, maximum-volume/ties-low tick
/// selection, largest-remainder apportionment, residual ranks, and owner
/// deltas.
fn domain_a() -> Report {
    let radix = u64::from(alphabet(FULL_CEILING));
    let total = radix.pow(SLOTS as u32);
    let threads = thread::available_parallelism()
        .map(std::num::NonZero::get)
        .unwrap_or(1)
        .min(radix as usize);
    let mut report = Report::default();
    let stride = total / threads as u64;
    thread::scope(|scope| {
        let mut handles = Vec::new();
        for index in 0..threads as u64 {
            let start = index * stride;
            let end = if index + 1 == threads as u64 {
                total
            } else {
                start + stride
            };
            handles.push(scope.spawn(move || {
                let mut local = Report::default();
                for code in start..end {
                    let batch = decode_book(code, FULL_CEILING, [0, 1, 2, 3], [0; SLOTS]);
                    local.observe(&batch);
                }
                local
            }));
        }
        for handle in handles {
            report.merge(&handle.join().expect("worker panicked"));
        }
    });
    report
}

/// Domain B: owner aggregation and reservation release.
///
/// Domain A fixes owner `i` to slot `i` and reserves exactly the required
/// amount, so it never exercises several slots owned by one participant or a
/// surplus reservation. Domain B enumerates every book over the reduced
/// alphabet with quantity in `1..=2` (`17^4 = 83_521` books), crossed with all
/// `4^4 = 256` owner assignments and four reservation-surplus patterns:
/// `83_521 * 256 * 4 = 85_525_504` batches.
fn domain_b() -> Report {
    const SURPLUS: [[u64; SLOTS]; 4] = [[0; SLOTS], [1, 2, 3, 4], [7; SLOTS], [0, 3, 0, 3]];
    const CEILING: u32 = 2;
    let radix = u64::from(alphabet(CEILING));
    let books = radix.pow(SLOTS as u32);
    let threads = thread::available_parallelism()
        .map(std::num::NonZero::get)
        .unwrap_or(1);
    let mut report = Report::default();
    let stride = books / threads as u64;
    thread::scope(|scope| {
        let mut handles = Vec::new();
        for index in 0..threads as u64 {
            let start = index * stride;
            let end = if index + 1 == threads as u64 {
                books
            } else {
                start + stride
            };
            handles.push(scope.spawn(move || {
                let mut local = Report::default();
                for code in start..end {
                    for assignment in 0..256u32 {
                        let owners = [
                            (assignment & 3) as u8,
                            ((assignment >> 2) & 3) as u8,
                            ((assignment >> 4) & 3) as u8,
                            ((assignment >> 6) & 3) as u8,
                        ];
                        for surplus in SURPLUS {
                            let batch = decode_book(code, CEILING, owners, surplus);
                            local.observe(&batch);
                        }
                    }
                }
                local
            }));
        }
        for handle in handles {
            report.merge(&handle.join().expect("worker panicked"));
        }
    });
    report
}

fn base_books() -> Vec<Batch> {
    let make = |specs: &[(u8, Direction, u8, u32)]| -> Batch {
        let mut batch = empty_batch();
        for (index, (owner, direction, limit_index, quantity)) in specs.iter().enumerate() {
            batch.slots[index] = Slot::Taken(Order {
                batch: domains::BATCH_ID,
                market: domains::MARKET_ID,
                owner: *owner,
                direction: *direction,
                limit_index: *limit_index,
                quantity: *quantity,
                reserved: degg_batch_oracle::admit::required_reservation(
                    *direction,
                    *limit_index,
                    *quantity,
                ),
                nullifier: index as u64 + 1,
                arrival: domains::CUTOFF,
                authorized: true,
                eligible: true,
                included: true,
            });
        }
        batch
    };
    use Direction::{Buy, Sell};
    vec![
        make(&[
            (0, Buy, 2, 5),
            (1, Buy, 1, 3),
            (2, Sell, 0, 4),
            (3, Sell, 2, 4),
        ]),
        make(&[(0, Buy, 3, 4), (1, Sell, 0, 4)]),
        make(&[(0, Buy, 0, 2), (1, Sell, 3, 2)]),
        make(&[
            (0, Buy, 0, 1),
            (1, Buy, 1, 2),
            (2, Buy, 2, 3),
            (3, Buy, 3, 15),
        ]),
        make(&[(0, Buy, 3, 1), (1, Buy, 3, 1), (2, Sell, 0, 1)]),
        make(&[]),
    ]
}

/// Domain C: admission perturbations.
///
/// The action catalogue is every batch-level perturbation (six: four boundary
/// statements plus the two non-default modes) together with every per-slot
/// perturbation (eighteen) at each of the four slots: `6 + 18 * 4 = 78`
/// actions. For each of six base books the harness applies every subset of size
/// zero, one, two, and three in ascending action order:
/// `1 + 78 + 3003 + 76_076 = 79_158` batches per base book, `474_948` in total.
///
/// Multi-action subsets are the point: they are what expose a disagreement in
/// which rule wins when a witness violates several at once.
fn domain_c() -> Report {
    let actions: Vec<(Perturbation, usize)> = BATCH_PERTURBATIONS
        .iter()
        .map(|perturbation| (*perturbation, 0usize))
        .chain(
            SLOT_PERTURBATIONS
                .iter()
                .flat_map(|perturbation| (0..SLOTS).map(move |slot| (*perturbation, slot))),
        )
        .collect();
    let count = actions.len();
    let mut report = Report::default();
    let name = |index: usize| -> String {
        let (perturbation, slot) = actions[index];
        if perturbation.label.starts_with("boundary.") || perturbation.label.starts_with("mode.") {
            perturbation.label.to_owned()
        } else {
            format!("s{slot}.{}", perturbation.label)
        }
    };
    for (book, base) in base_books().into_iter().enumerate() {
        report.observe_labeled(&base, &format!("book{book}"));
        for first in 0..count {
            let mut batch = base;
            (actions[first].0.apply)(&mut batch, actions[first].1);
            report.observe_labeled(&batch, &format!("book{book}+{}", name(first)));
            for second in (first + 1)..count {
                let mut batch = batch;
                (actions[second].0.apply)(&mut batch, actions[second].1);
                report.observe_labeled(
                    &batch,
                    &format!("book{book}+{}+{}", name(first), name(second)),
                );
                for (third, (action, slot)) in actions.iter().enumerate().skip(second + 1) {
                    let mut batch = batch;
                    (action.apply)(&mut batch, *slot);
                    report.observe_labeled(
                        &batch,
                        &format!(
                            "book{book}+{}+{}+{}",
                            name(first),
                            name(second),
                            name(third)
                        ),
                    );
                }
            }
        }
    }
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use adapter::Divergence;
    use degg_batch_oracle::book::{Boundary, Order};
    use report::signature;

    const PUBLISHED: &str = include_str!("../../../dark-fba/vectors/v1.txt");

    /// Every number in the published corpus is reproduced by the independent
    /// oracle. Only the refusal spelling comes from the declared vocabulary map.
    #[test]
    fn published_vectors_reproduce_byte_for_byte_under_the_vocabulary_map() {
        assert_eq!(vectors::reproduce(true), PUBLISHED);
    }

    /// The two taxonomies really are different: without the map, exactly the
    /// five refusal lines differ and every settled line still matches.
    #[test]
    fn own_vocabulary_differs_only_in_refusal_spelling() {
        let native = vectors::reproduce(false);
        let differing: Vec<usize> = native
            .lines()
            .zip(PUBLISHED.lines())
            .enumerate()
            .filter(|(_, (ours, theirs))| ours != theirs)
            .map(|(index, _)| index + 1)
            .collect();
        assert_eq!(differing, vec![5, 6, 7, 8, 9]);
        for line in native.lines().filter(|line| line.contains("status=ok")) {
            assert!(PUBLISHED.contains(line), "settled line drifted: {line}");
        }
    }

    /// A bounded slice of domain A, small enough for the default test run.
    #[test]
    fn bounded_differential_domain_agrees() {
        const CEILING: u32 = 3;
        let radix = u64::from(alphabet(CEILING));
        let mut report = Report::default();
        for code in 0..radix.pow(SLOTS as u32) {
            report.observe(&decode_book(code, CEILING, [0, 1, 2, 3], [0; SLOTS]));
        }
        assert_eq!(report.cases, 390_625);
        assert_eq!(report.settled, report.cases);
        assert_eq!(report.divergences(), 0, "{:?}", report.examples);
    }

    fn slot(
        owner: u8,
        direction: Direction,
        limit_index: u8,
        quantity: u32,
        reserved: u64,
        nullifier: u64,
    ) -> Slot {
        Slot::Taken(Order {
            batch: domains::BATCH_ID,
            market: domains::MARKET_ID,
            owner,
            direction,
            limit_index,
            quantity,
            reserved,
            nullifier,
            arrival: domains::CUTOFF,
            authorized: true,
            eligible: true,
            included: true,
        })
    }

    /// Finding 1: when a witness is out of domain in both quantity and limit,
    /// the two oracles name different rules. This test asserts the divergence
    /// still exists; it is recorded, not reconciled.
    #[test]
    fn finding_quantity_and_limit_priority_disagree() {
        let mut batch = empty_batch();
        batch.slots[0] = slot(0, Direction::Buy, 4, 16, 64, 1);
        assert_eq!(
            signature(&batch).as_deref(),
            Some("limit-out-of-domain vs quantity-out-of-domain")
        );
        assert_eq!(adapter::violations(&batch).len(), 2);
    }

    /// Finding 2: when one slot both repeats a nullifier and under-reserves,
    /// the two oracles name different rules.
    #[test]
    fn finding_nullifier_and_per_slot_priority_disagree() {
        let mut batch = empty_batch();
        batch.slots[0] = slot(0, Direction::Buy, 0, 1, 1, 1);
        batch.slots[1] = slot(0, Direction::Buy, 0, 1, 0, 1);
        assert_eq!(
            signature(&batch).as_deref(),
            Some("reservation-insufficient vs nullifier-repeated")
        );
    }

    /// Both oracles refuse the same set of batches across the whole
    /// perturbation catalogue: divergence never crosses accept and refuse.
    #[test]
    fn refusal_disagreements_are_never_about_whether_to_refuse() {
        let report = domain_c();
        assert_eq!(report.cases, 474_948);
        for (index, class) in CLASSES.iter().enumerate() {
            if *class == Divergence::RefusalClass {
                continue;
            }
            assert_eq!(report.by_class[index], 0, "{class:?} appeared");
        }
        assert_eq!(report.unjustified, 0);
        assert!(report.by_class[CLASSES.len() - 1] > 0);
    }

    /// The boundary statements and the mode gate behave identically.
    #[test]
    fn boundary_and_mode_gates_agree() {
        let mut base = empty_batch();
        base.slots[0] = slot(0, Direction::Buy, 3, 5, 20, 1);
        base.slots[1] = slot(1, Direction::Sell, 0, 5, 5, 2);
        assert_eq!(signature(&base), None);
        for edit in [
            |b: &mut Batch| b.boundary.log_final = false,
            |b: &mut Batch| b.boundary.root_binds_slots = false,
            |b: &mut Batch| b.boundary.no_conflicting_root = false,
            |b: &mut Batch| b.boundary.payloads_available = false,
        ] {
            let mut batch = base;
            edit(&mut batch);
            assert_ne!(batch.boundary, Boundary::SATISFIED);
            assert_eq!(signature(&batch), None);
        }
        for mode in [
            degg_batch_oracle::book::Mode::Clear,
            degg_batch_oracle::book::Mode::ShieldedSingleExecutor,
            degg_batch_oracle::book::Mode::DarkTarget,
        ] {
            let mut batch = base;
            batch.mode = mode;
            assert_eq!(signature(&batch), None, "{mode:?}");
        }
    }
}
