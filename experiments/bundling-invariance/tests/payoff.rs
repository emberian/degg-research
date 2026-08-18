//! The payoff object and the two costless operations.

use degg_bundling_invariance::corpus::{binary_decompositions, index_of, vectors_for};
use degg_bundling_invariance::criteria::fact_profiles;
use degg_bundling_invariance::payoff::{
    Facts, MAX_CELLS, MIN_CELLS, OpError, Payoff, PayoffObject, ReferenceKind, bundle, split,
    unbundle_elementary,
};

fn facts() -> Facts {
    Facts::new(ReferenceKind::SecurityPrice, true, true)
}

#[test]
fn a_split_and_a_bundle_are_inverse_on_the_whole_corpus() {
    for cells in MIN_CELLS..=MAX_CELLS {
        for whole in vectors_for(cells) {
            let object = PayoffObject::new(whole.clone(), facts());
            for (first, second) in binary_decompositions(&whole) {
                let (left, right) = split(&object, &first).expect("the part is dominated");
                assert_eq!(left.payoff(), &first);
                assert_eq!(right.payoff(), &second);
                let rebundled = bundle(&[left, right]).expect("the parts share the partition");
                assert_eq!(rebundled, object);
            }
        }
    }
}

#[test]
fn elementary_unbundling_conserves_the_payoff_cell_by_cell() {
    for cells in MIN_CELLS..=MAX_CELLS {
        for whole in vectors_for(cells) {
            if whole.is_zero() {
                continue;
            }
            let object = PayoffObject::new(whole.clone(), facts());
            let parts = unbundle_elementary(&object);
            assert_eq!(parts.len() as u64, whole.total());
            let rebundled = bundle(&parts).expect("the parts share the partition");
            assert_eq!(rebundled.payoff(), &whole);
        }
    }
}

#[test]
fn the_index_encoding_round_trips_over_the_whole_corpus() {
    for cells in MIN_CELLS..=MAX_CELLS {
        let vectors = vectors_for(cells);
        for (index, payoff) in vectors.iter().enumerate() {
            assert_eq!(index_of(payoff), index);
        }
    }
}

#[test]
fn binary_decompositions_are_exhaustive_and_unordered() {
    // Every ordered pair summing to the whole appears exactly once, up to order.
    for cells in MIN_CELLS..=MAX_CELLS {
        for whole in vectors_for(cells) {
            let unordered = binary_decompositions(&whole);
            let mut ordered = 0_usize;
            for first in vectors_for(cells) {
                if let Ok(second) = whole.sub(&first) {
                    assert_eq!(first.add(&second).expect("same partition"), whole);
                    ordered += 1;
                }
            }
            let self_paired = usize::from(whole.amounts().iter().all(|amount| amount % 2 == 0));
            assert_eq!(unordered.len(), (ordered + self_paired) / 2);
            for (first, second) in &unordered {
                assert!(first <= second);
                assert_eq!(first.add(second).expect("same partition"), whole);
            }
        }
    }
}

#[test]
fn bundling_refuses_parts_that_do_not_share_the_economic_facts() {
    let left = PayoffObject::new(Payoff::new(vec![1, 0]), facts());
    let right = PayoffObject::new(
        Payoff::new(vec![0, 1]),
        Facts::new(ReferenceKind::IssuerFact, true, true),
    );
    assert_eq!(bundle(&[left, right]), Err(OpError::FactsMismatch));
    assert_eq!(bundle(&[]), Err(OpError::EmptyBundle));
}

#[test]
fn splitting_refuses_a_part_the_position_does_not_hold() {
    let object = PayoffObject::new(Payoff::new(vec![1, 0]), facts());
    assert_eq!(
        split(&object, &Payoff::new(vec![2, 0])),
        Err(OpError::PartExceedsWhole)
    );
    assert_eq!(
        split(&object, &Payoff::new(vec![0, 0, 0])),
        Err(OpError::CellMismatch { left: 2, right: 3 })
    );
}

#[test]
fn the_corpus_carries_every_fact_profile_exactly_once() {
    let profiles = fact_profiles();
    assert_eq!(profiles.len(), 12);
    let mut sorted = profiles.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), profiles.len());
}
