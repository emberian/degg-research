//! The candidate criteria: totality, determinism, and what each one says about
//! objects a reader can check by hand.

use degg_bundling_invariance::corpus::vectors_for;
use degg_bundling_invariance::criteria::{
    BOUNDED_PAYOUT_CEILING, BinaryPayout, BoundedPayout, Category, CompleteSet, ConstantLabel,
    Criterion, DistinctValues, PrefundingExclusion, ReferenceAndSettlement, SupportNonEmpty,
    candidates, fact_profiles,
};
use degg_bundling_invariance::payoff::{
    Facts, MAX_CELLS, MIN_CELLS, Payoff, PayoffObject, ReferenceKind,
};

fn object(amounts: &[u64]) -> PayoffObject {
    PayoffObject::new(
        Payoff::new(amounts.to_vec()),
        Facts::new(ReferenceKind::SecurityPrice, true, true),
    )
}

fn issuer_object(amounts: &[u64]) -> PayoffObject {
    PayoffObject::new(
        Payoff::new(amounts.to_vec()),
        Facts::new(ReferenceKind::IssuerFact, true, true),
    )
}

#[test]
fn criterion_names_are_unique_and_stable() {
    let names: Vec<&str> = candidates().iter().map(|entry| entry.name()).collect();
    let mut sorted = names.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), names.len());
    assert_eq!(
        names,
        vec![
            "binary-payout",
            "bounded-payout",
            "distinct-values",
            "complete-set",
            "support-nonempty",
            "prefunding-exclusion",
            "reference-and-settlement",
            "constant-label",
        ]
    );
}

#[test]
fn every_criterion_is_total_and_deterministic_over_the_corpus() {
    for criterion in candidates() {
        for cells in MIN_CELLS..=MAX_CELLS {
            for payoff in vectors_for(cells) {
                for facts in fact_profiles() {
                    let object = PayoffObject::new(payoff.clone(), facts);
                    let first = criterion.classify(&object);
                    assert_eq!(first, criterion.classify(&object));
                }
            }
        }
    }
}

#[test]
fn the_binary_payout_criterion_reads_cash_or_nothing() {
    let criterion = BinaryPayout;
    assert_eq!(
        criterion.classify(&object(&[1, 0])),
        Category::EventContract
    );
    assert_eq!(
        criterion.classify(&object(&[0, 0, 3, 0])),
        Category::EventContract
    );
    // The complete set pays the same amount in every state, so it is not
    // cash-or-nothing, though each of its claims is.
    assert_eq!(criterion.classify(&object(&[1, 1])), Category::SwapLike);
    assert_eq!(criterion.classify(&object(&[1, 2])), Category::SwapLike);
    assert_eq!(criterion.classify(&object(&[0, 0])), Category::SwapLike);
}

#[test]
fn the_bounded_payout_criterion_reads_the_largest_payout() {
    let criterion = BoundedPayout {
        ceiling: BOUNDED_PAYOUT_CEILING,
    };
    assert_eq!(criterion.parameters(), "ceiling=2");
    assert_eq!(
        criterion.classify(&object(&[2, 0])),
        Category::ExcludedSmallPayout
    );
    assert_eq!(
        criterion.classify(&object(&[1, 2])),
        Category::ExcludedSmallPayout
    );
    assert_eq!(
        criterion.classify(&object(&[3, 0])),
        Category::IncludedLargePayout
    );
}

#[test]
fn the_distinct_values_criterion_counts_distinct_amounts() {
    let criterion = DistinctValues;
    assert_eq!(criterion.classify(&object(&[0, 1])), Category::SimpleClaim);
    assert_eq!(
        criterion.classify(&object(&[2, 2, 2])),
        Category::SimpleClaim
    );
    assert_eq!(
        criterion.classify(&object(&[0, 1, 2])),
        Category::PortfolioInstrument
    );
}

#[test]
fn the_complete_set_criterion_reads_the_outcome_set() {
    let criterion = CompleteSet;
    assert_eq!(
        criterion.classify(&object(&[0, 0])),
        Category::NotAnInstrument
    );
    assert_eq!(
        criterion.classify(&object(&[3, 0, 0])),
        Category::OptionOnSecurityOrIndex
    );
    assert_eq!(
        criterion.classify(&object(&[2, 2, 2])),
        Category::CollateralEquivalent
    );
    assert_eq!(
        criterion.classify(&object(&[1, 2, 0])),
        Category::PortfolioInstrument
    );
}

#[test]
fn the_support_criterion_reads_only_whether_anything_is_paid() {
    let criterion = SupportNonEmpty;
    assert_eq!(
        criterion.classify(&object(&[0, 0, 0])),
        Category::NotAnInstrument
    );
    assert_eq!(criterion.classify(&object(&[0, 0, 1])), Category::Claim);
}

#[test]
fn the_prefunding_criterion_reads_a_fact_and_not_the_payoff() {
    let criterion = PrefundingExclusion;
    let funded = PayoffObject::new(
        Payoff::new(vec![3, 0]),
        Facts::new(ReferenceKind::SecurityPrice, true, false),
    );
    let unfunded = PayoffObject::new(
        Payoff::new(vec![3, 0]),
        Facts::new(ReferenceKind::SecurityPrice, false, false),
    );
    assert_eq!(criterion.classify(&funded), Category::ExcludedPrefunded);
    assert_eq!(
        criterion.classify(&unfunded),
        Category::NotExcludedPrefunded
    );
}

#[test]
fn the_control_reads_the_reference_and_the_settlement_function() {
    let criterion = ReferenceAndSettlement;
    assert_eq!(
        criterion.classify(&object(&[1, 0])),
        Category::OptionOnSecurityOrIndex
    );
    assert_eq!(
        criterion.classify(&object(&[1, 1, 1])),
        Category::OptionOnSecurityOrIndex
    );
    assert_eq!(
        criterion.classify(&issuer_object(&[1, 0])),
        Category::SecurityBasedSwap
    );
    let index = PayoffObject::new(
        Payoff::new(vec![1, 0]),
        Facts::new(ReferenceKind::IndexValue, false, false),
    );
    assert_eq!(
        criterion.classify(&index),
        Category::OptionOnSecurityOrIndex
    );
}

#[test]
fn the_control_gives_a_binary_option_and_a_complete_set_the_same_answer() {
    // Position 2 of the memo: the payoff is not the distinguishing
    // characteristic. Every claim in a complete set on one security's price, and
    // the set itself, receive one label.
    let criterion = ReferenceAndSettlement;
    let claims = [
        object(&[1, 0, 0]),
        object(&[0, 1, 0]),
        object(&[0, 0, 1]),
        object(&[1, 1, 1]),
    ];
    for claim in &claims {
        assert_eq!(
            criterion.classify(claim),
            Category::OptionOnSecurityOrIndex,
            "{claim}"
        );
    }
}

#[test]
fn the_degenerate_control_emits_one_label() {
    let criterion = ConstantLabel;
    assert_eq!(criterion.classify(&object(&[1, 0])), Category::Unclassified);
    assert_eq!(
        criterion.classify(&issuer_object(&[3, 3, 3])),
        Category::Unclassified
    );
}
