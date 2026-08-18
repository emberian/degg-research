//! The invariance test itself: what the sweep found, and that each witness is a
//! real one.

use degg_bundling_invariance::criteria::{
    CONTROL, Category, DEGENERATE_CONTROL, candidates, fact_profiles,
};
use degg_bundling_invariance::invariance::{CriterionReport, Family, Witness};
use degg_bundling_invariance::payoff::{Payoff, PayoffObject};
use degg_bundling_invariance::report;

fn entry(name: &str) -> &'static CriterionReport {
    report()
        .criterion(name)
        .unwrap_or_else(|| panic!("no report for {name}"))
}

#[test]
fn the_corpus_bounds_are_the_ones_the_experiment_claims() {
    let bounds = report().bounds;
    assert_eq!(bounds.min_cells, 2);
    assert_eq!(bounds.max_cells, 5);
    assert_eq!(bounds.ceiling, 3);
    // 4^2 + 4^3 + 4^4 + 4^5 payoff vectors.
    assert_eq!(bounds.vectors, 1_360);
    assert_eq!(bounds.fact_profiles, 12);
    assert_eq!(bounds.objects, 16_320);
    // (10^n + 2^n) / 2 unordered binary splits plus one elementary unbundling
    // per nonzero vector, summed over n in 2..=5.
    assert_eq!(bounds.decompositions, 56_936);
    for criterion in &report().criteria {
        assert_eq!(criterion.objects_classified, bounds.objects);
        assert_eq!(
            criterion.decompositions_checked,
            bounds.decompositions * bounds.fact_profiles
        );
    }
}

#[test]
fn the_control_criterion_is_invariant_and_still_separates_objects() {
    let control = entry(CONTROL);
    assert!(control.invariant_strict(), "the control produced a witness");
    assert!(control.invariant_declared_story());
    assert!(
        control.discriminating(),
        "an invariant criterion that separates nothing proves nothing"
    );
    assert_eq!(control.labels.len(), 2);
    assert!(!control.reads_payoff);
    assert!(control.reads_facts);
    assert_eq!(control.minimal_strict, None);
}

#[test]
fn the_degenerate_control_shows_that_invariance_alone_buys_nothing() {
    let degenerate = entry(DEGENERATE_CONTROL);
    assert!(degenerate.invariant_strict());
    assert!(
        !degenerate.discriminating(),
        "the degenerate control is supposed to separate nothing"
    );
    assert_eq!(degenerate.labels, vec![Category::Unclassified]);
}

#[test]
fn each_shape_reading_candidate_from_the_memo_produces_witnesses() {
    for name in [
        "binary-payout",
        "bounded-payout",
        "distinct-values",
        "complete-set",
    ] {
        let candidate = entry(name);
        assert!(candidate.reads_payoff, "{name} does not read the payoff");
        assert!(
            !candidate.invariant_strict(),
            "{name} produced no witness under the strict reading"
        );
        assert!(
            candidate.minimal_strict.is_some(),
            "{name} has violations but no minimized witness"
        );
    }
}

#[test]
fn the_per_criterion_witness_counts_are_the_ones_recorded_in_the_memo() {
    let expected = [
        ("binary-payout", 49_032, 49_032, 15_084),
        ("bounded-payout", 180_696, 180_696, 12_000),
        ("distinct-values", 159_120, 113_472, 14_616),
        ("complete-set", 27_420, 0, 15_768),
        ("support-nonempty", 0, 0, 0),
        ("prefunding-exclusion", 0, 0, 0),
        ("reference-and-settlement", 0, 0, 0),
        ("constant-label", 0, 0, 0),
    ];
    for (name, strict, story, objects) in expected {
        let candidate = entry(name);
        assert_eq!(candidate.violations_strict, strict, "{name} strict");
        assert_eq!(
            candidate.violations_declared_story, story,
            "{name} declared story"
        );
        assert_eq!(
            candidate.violating_objects_strict, objects,
            "{name} violating objects"
        );
    }
}

#[test]
fn the_smallest_witness_is_the_complete_set_at_two_cells() {
    // The memo's own example, found rather than chosen: two cash-or-nothing
    // claims covering both outcomes, and the set they make.
    let witness = entry("binary-payout")
        .minimal_strict
        .clone()
        .expect("binary-payout has witnesses");
    assert_eq!(witness.family, Family::BinarySplit);
    assert_eq!(witness.cells, 2);
    assert_eq!(witness.whole, Payoff::new(vec![1, 1]));
    assert_eq!(
        witness.parts,
        vec![Payoff::new(vec![0, 1]), Payoff::new(vec![1, 0])]
    );
    assert_eq!(witness.part_label, Category::EventContract);
    assert_eq!(witness.whole_label, Category::SwapLike);
    assert_eq!(witness.complete_sets, 1);
    assert!(witness.conserves_claims());
}

#[test]
fn the_smallest_witnesses_of_the_other_candidates_are_the_recorded_ones() {
    let bounded = entry("bounded-payout")
        .minimal_strict
        .clone()
        .expect("bounded-payout has witnesses");
    assert_eq!(bounded.whole, Payoff::new(vec![0, 3]));
    assert_eq!(
        bounded.parts,
        vec![Payoff::new(vec![0, 1]), Payoff::new(vec![0, 2])]
    );

    let distinct_strict = entry("distinct-values")
        .minimal_strict
        .clone()
        .expect("distinct-values has strict witnesses");
    assert_eq!(distinct_strict.whole, Payoff::new(vec![0, 1, 2]));
    assert_eq!(distinct_strict.part_label, Category::SimpleClaim);
    assert_eq!(distinct_strict.whole_label, Category::PortfolioInstrument);

    // Under its own story a bundle of simple claims may be a portfolio, so what
    // survives is the collapse: two portfolios bundling into a simple claim.
    let distinct_story = entry("distinct-values")
        .minimal_declared_story
        .clone()
        .expect("distinct-values has story witnesses");
    assert_eq!(distinct_story.whole, Payoff::new(vec![0, 3, 3]));
    assert_eq!(distinct_story.part_label, Category::PortfolioInstrument);
    assert_eq!(distinct_story.whole_label, Category::SimpleClaim);

    let complete = entry("complete-set")
        .minimal_strict
        .clone()
        .expect("complete-set has strict witnesses");
    assert_eq!(complete.whole, Payoff::new(vec![1, 1]));
    assert_eq!(complete.part_label, Category::OptionOnSecurityOrIndex);
    assert_eq!(complete.whole_label, Category::CollateralEquivalent);
}

#[test]
fn support_nonempty_is_invariant_and_that_is_a_finding_not_a_target() {
    // FINDING, reported rather than tuned away: a payoff-shape criterion can be
    // bundling-invariant. This one reads only whether the object pays in some
    // state, and the outcomes a bundle pays in are exactly the union of the
    // outcomes its parts pay in, so a criterion whose label classes are closed
    // under that union survives. The census measures how few criteria that is.
    let candidate = entry("support-nonempty");
    assert!(candidate.reads_payoff);
    assert!(candidate.invariant_strict());
    assert!(candidate.discriminating());
    assert_eq!(candidate.violations_strict, 0);
}

#[test]
fn prefunding_exclusion_is_invariant_and_that_is_also_a_finding() {
    // FINDING: bundling invariance is not a test of whether a criterion is a
    // good one. This criterion excludes every prefunded claim, which the memo's
    // Position 6 argues against on entirely different grounds, and it is
    // invariant, because prefunding is a fact the parts and the bundle share.
    let candidate = entry("prefunding-exclusion");
    assert!(!candidate.reads_payoff);
    assert!(candidate.reads_facts);
    assert!(candidate.invariant_strict());
    assert!(candidate.discriminating());
}

#[test]
fn the_complete_set_criterion_survives_only_by_declining_to_answer() {
    // FINDING: the outcome-set criterion has 27,420 strict witnesses and none
    // under its own declared story, but the story it needs permits three
    // different labels for the same unanimous parts. A rule that answers "an
    // option, or a portfolio, or collateral" has not classified anything.
    let candidate = entry("complete-set");
    assert!(!candidate.invariant_strict());
    assert!(candidate.invariant_declared_story());
    assert!(candidate.story_permits_alternatives);
    let witness = candidate
        .minimal_strict
        .clone()
        .expect("complete-set has strict witnesses");
    assert_eq!(witness.permitted.len(), 3);
}

#[test]
fn every_recorded_witness_is_a_real_decomposition_at_zero_cost() {
    let criteria = candidates();
    let profiles = fact_profiles();
    assert_eq!(criteria.len(), report().criteria.len());
    for (criterion, entry) in criteria.iter().zip(report().criteria.iter()) {
        assert_eq!(criterion.name(), entry.name);
        for witness in [&entry.minimal_strict, &entry.minimal_declared_story]
            .into_iter()
            .flatten()
        {
            check_witness(witness, criterion.as_ref(), &profiles);
        }
    }
}

fn check_witness(
    witness: &Witness,
    criterion: &dyn degg_bundling_invariance::criteria::Criterion,
    profiles: &[degg_bundling_invariance::payoff::Facts],
) {
    assert_eq!(profiles[witness.profile], witness.facts);
    assert_eq!(witness.whole.cells(), witness.cells);
    assert!(!witness.parts.is_empty());
    // The parts are the whole, cell by cell: nothing was minted or burned.
    assert!(witness.conserves_claims());
    // The labels are the criterion's own, recomputed here.
    let whole = PayoffObject::new(witness.whole.clone(), witness.facts);
    assert_eq!(criterion.classify(&whole), witness.whole_label);
    for part in &witness.parts {
        let part_object = PayoffObject::new(part.clone(), witness.facts);
        assert_eq!(criterion.classify(&part_object), witness.part_label);
        assert!(part.dominated_by(&witness.whole));
    }
    // The two labels differ, and the whole's label is not one the criterion
    // permitted for these parts under the reading that produced the witness.
    assert_ne!(witness.whole_label, witness.part_label);
    assert!(witness.permitted.contains(&witness.part_label));
    // The number of complete sets in the bundle is the collateral it could be
    // recombined into, and the decomposition changes it not at all.
    assert_eq!(witness.complete_sets, witness.whole.min_payout());
    assert_eq!(witness.parts_total().min_payout(), witness.complete_sets);
}

#[test]
fn the_memo_four_shapes_have_identical_cashflows_and_still_split_the_labels() {
    // The table the memo asked for: a digital, a range, a capped directional
    // position, and a tail position, each expressed as one bespoke instrument
    // and as a portfolio over the exhaustive basis, with the exact integer
    // cashflow in every state printed on both sides.
    let facts = fact_profiles()[0];
    let shapes: [(&str, [u64; 5]); 5] = [
        ("digital", [0, 1, 0, 0, 0]),
        ("range", [0, 1, 1, 0, 0]),
        ("capped_directional", [0, 1, 2, 3, 3]),
        ("tail", [0, 0, 0, 0, 3]),
        ("complete_set", [1, 1, 1, 1, 1]),
    ];
    let criteria = candidates();
    let mut split_labels = 0;
    for (name, amounts) in shapes {
        let whole = Payoff::new(amounts.to_vec());
        let object = PayoffObject::new(whole.clone(), facts);
        let parts = degg_bundling_invariance::payoff::unbundle_elementary(&object);
        let rebuilt = degg_bundling_invariance::payoff::bundle(&parts).expect("same partition");
        assert_eq!(
            rebuilt.payoff(),
            &whole,
            "{name} does not pay the same amounts in every state"
        );
        for criterion in &criteria {
            let whole_label = criterion.classify(&object);
            let first = criterion.classify(&parts[0]);
            if parts.iter().all(|part| criterion.classify(part) == first) && first != whole_label {
                split_labels += 1;
            }
        }
    }
    // The capped directional alone splits four of the eight criteria, and the
    // complete set splits two more.
    assert_eq!(split_labels, 8);
}
