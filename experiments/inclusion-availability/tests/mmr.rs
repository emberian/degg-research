//! The commitment primitive: shape, exhaustive proof soundness at small sizes,
//! and a tamper battery against every field of a proof object.

use degg_inclusion_availability::mmr::{
    ConsistencyProof, Mmr, NodePosition, NodeProof, ProofDefect, leaf_hash, peak_heights,
    root_from_peaks, verify_consistency, verify_node_proof,
};

const DOMAIN: [u8; 32] = [0x11; 32];
const OTHER_DOMAIN: [u8; 32] = [0x12; 32];

fn leaf(index: u64) -> [u8; 32] {
    leaf_hash(&index.to_be_bytes())
}

fn built(domain: [u8; 32], count: u64) -> Mmr {
    let mut mmr = Mmr::new(domain);
    for index in 0..count {
        mmr.append(leaf(index));
    }
    mmr
}

#[test]
fn peak_heights_are_the_set_bits_of_the_leaf_count() {
    for count in 0u64..=256 {
        let heights = peak_heights(count);
        assert_eq!(heights.len(), count.count_ones() as usize, "count {count}");
        assert!(
            heights.windows(2).all(|w| w[0] > w[1]),
            "heights must strictly decrease at {count}"
        );
        let span: u64 = heights.iter().map(|h| 1u64 << h).sum();
        assert_eq!(span, count, "peaks must span exactly {count} leaves");
    }
}

#[test]
fn peaks_match_the_shape_the_leaf_count_implies() {
    let mmr = built(DOMAIN, 173);
    for count in 0u64..=173 {
        let peaks: Vec<u32> = mmr.peaks_at(count).into_iter().map(|p| p.height).collect();
        assert_eq!(peaks, peak_heights(count), "count {count}");
    }
}

#[test]
fn every_append_moves_the_root_and_no_two_sizes_share_one() {
    let mut mmr = Mmr::new(DOMAIN);
    let mut seen = vec![mmr.root()];
    for index in 0..64 {
        mmr.append(leaf(index));
        let root = mmr.root();
        assert!(!seen.contains(&root), "root repeated at size {}", index + 1);
        seen.push(root);
    }
}

#[test]
fn a_root_is_reconstructible_from_the_peaks_it_was_built_from() {
    let mmr = built(DOMAIN, 45);
    for count in 0u64..=45 {
        let hashes: Vec<[u8; 32]> = mmr.peaks_at(count).into_iter().map(|p| p.hash).collect();
        assert_eq!(root_from_peaks(&DOMAIN, count, &hashes), mmr.root_at(count));
    }
}

#[test]
fn the_root_binds_the_leaf_count_and_the_domain() {
    let mmr = built(DOMAIN, 12);
    let hashes: Vec<[u8; 32]> = mmr.peaks().into_iter().map(|p| p.hash).collect();
    assert_ne!(
        root_from_peaks(&DOMAIN, 12, &hashes),
        root_from_peaks(&DOMAIN, 13, &hashes),
        "the same peaks under a different count must not share a root"
    );
    assert_ne!(
        root_from_peaks(&DOMAIN, 12, &hashes),
        root_from_peaks(&OTHER_DOMAIN, 12, &hashes),
        "the same peaks under a different domain must not share a root"
    );
    assert_ne!(mmr.root(), built(OTHER_DOMAIN, 12).root());
}

#[test]
fn appending_never_rewrites_an_existing_node() {
    let mut mmr = Mmr::new(DOMAIN);
    let mut recorded: Vec<(NodePosition, [u8; 32])> = Vec::new();
    for index in 0..64u64 {
        mmr.append(leaf(index));
        for (position, hash) in &recorded {
            assert_eq!(
                mmr.node(*position),
                Some(*hash),
                "node {position:?} changed after appending leaf {index}"
            );
        }
        for height in 0..8u32 {
            let mut node_index = 0u64;
            while let Some(hash) = mmr.node(NodePosition {
                height,
                index: node_index,
            }) {
                let position = NodePosition {
                    height,
                    index: node_index,
                };
                if !recorded.iter().any(|(p, _)| *p == position) {
                    recorded.push((position, hash));
                }
                node_index += 1;
            }
        }
    }
    assert!(recorded.len() > 64);
}

#[test]
fn every_leaf_of_every_small_log_proves_at_its_own_index() {
    for count in 1u64..=33 {
        let mmr = built(DOMAIN, count);
        let root = mmr.root();
        for index in 0..count {
            let proof = mmr.leaf_proof(index).expect("leaf is present");
            let derived = verify_node_proof(&DOMAIN, &root, count, &leaf(index), &proof)
                .unwrap_or_else(|defect| panic!("count {count} index {index}: {defect:?}"));
            assert_eq!(derived, NodePosition { height: 0, index });
        }
    }
}

#[test]
fn a_leaf_proof_never_verifies_at_another_leafs_hash() {
    let count = 17u64;
    let mmr = built(DOMAIN, count);
    let root = mmr.root();
    for index in 0..count {
        let proof = mmr.leaf_proof(index).expect("leaf is present");
        for other in 0..count {
            if other == index {
                continue;
            }
            assert_eq!(
                verify_node_proof(&DOMAIN, &root, count, &leaf(other), &proof),
                Err(ProofDefect::RootMismatch),
                "leaf {other} accepted under leaf {index}'s proof"
            );
        }
    }
}

#[test]
fn every_prefix_of_every_small_log_proves_consistent() {
    for count in 0u64..=17 {
        let mmr = built(DOMAIN, count);
        let root = mmr.root();
        for prefix in 0..=count {
            let proof = mmr
                .consistency_proof(prefix)
                .expect("prefix is within the log");
            let derived = verify_consistency(&DOMAIN, &root, count, &proof)
                .unwrap_or_else(|defect| panic!("count {count} prefix {prefix}: {defect:?}"));
            assert_eq!(
                derived,
                mmr.root_at(prefix),
                "count {count} prefix {prefix} must recover its own root"
            );
        }
    }
}

#[test]
fn a_consistency_proof_from_a_forked_log_recovers_a_different_prefix_root() {
    let honest = built(DOMAIN, 9);
    let mut forked = built(DOMAIN, 4);
    forked.append(leaf(400));
    for index in 5..9u64 {
        forked.append(leaf(index));
    }
    let proof = forked
        .consistency_proof(5)
        .expect("prefix is within the log");
    let derived = verify_consistency(&DOMAIN, &forked.root(), 9, &proof).expect("proof verifies");
    assert_ne!(derived, honest.root_at(5));
    assert_eq!(derived, forked.root_at(5));
}

#[test]
fn a_consistency_proof_longer_than_its_log_is_refused() {
    let mmr = built(DOMAIN, 5);
    assert!(mmr.consistency_proof(6).is_none());
    let proof = ConsistencyProof {
        prefix_leaf_count: 6,
        prefix_peaks: Vec::new(),
    };
    assert_eq!(
        verify_consistency(&DOMAIN, &mmr.root(), 5, &proof),
        Err(ProofDefect::PrefixLongerThanLog { prefix: 6, log: 5 })
    );
}

#[test]
fn a_consistency_proof_with_the_wrong_peak_count_is_refused() {
    let mmr = built(DOMAIN, 11);
    let mut proof = mmr.consistency_proof(7).expect("prefix is within the log");
    proof.prefix_peaks.pop();
    assert_eq!(
        verify_consistency(&DOMAIN, &mmr.root(), 11, &proof),
        Err(ProofDefect::PrefixShapeInvalid)
    );
}

#[test]
fn a_consistency_proof_with_a_reordered_peak_is_refused() {
    let mmr = built(DOMAIN, 11);
    let mut proof = mmr.consistency_proof(7).expect("prefix is within the log");
    proof.prefix_peaks.swap(0, 1);
    assert!(matches!(
        verify_consistency(&DOMAIN, &mmr.root(), 11, &proof),
        Err(ProofDefect::PrefixShapeInvalid | ProofDefect::PositionMismatch { .. })
    ));
}

/// Everything below tampers with one field of one otherwise-valid proof.
mod tamper {
    use super::{DOMAIN, OTHER_DOMAIN, built, leaf};
    use degg_inclusion_availability::mmr::{
        NodeProof, PeakRef, ProofDefect, Side, verify_node_proof,
    };

    const COUNT: u64 = 11;
    const INDEX: u64 = 9;

    fn fixture() -> ([u8; 32], NodeProof) {
        let mmr = built(DOMAIN, COUNT);
        let proof = mmr.leaf_proof(INDEX).expect("leaf is present");
        (mmr.root(), proof)
    }

    fn reject(proof: &NodeProof) -> ProofDefect {
        let (root, _) = fixture();
        verify_node_proof(&DOMAIN, &root, COUNT, &leaf(INDEX), proof)
            .expect_err("tampered proof must be rejected")
    }

    #[test]
    fn the_untampered_fixture_verifies() {
        let (root, proof) = fixture();
        assert!(verify_node_proof(&DOMAIN, &root, COUNT, &leaf(INDEX), &proof).is_ok());
    }

    #[test]
    fn a_wrong_root_is_rejected() {
        let (_, proof) = fixture();
        let wrong = built(DOMAIN, COUNT + 1).root();
        assert_eq!(
            verify_node_proof(&DOMAIN, &wrong, COUNT, &leaf(INDEX), &proof),
            Err(ProofDefect::RootMismatch)
        );
    }

    #[test]
    fn a_wrong_domain_is_rejected() {
        let (root, proof) = fixture();
        assert_eq!(
            verify_node_proof(&OTHER_DOMAIN, &root, COUNT, &leaf(INDEX), &proof),
            Err(ProofDefect::RootMismatch)
        );
    }

    #[test]
    fn a_wrong_leaf_count_is_rejected() {
        let (root, proof) = fixture();
        assert_eq!(
            verify_node_proof(&DOMAIN, &root, COUNT + 1, &leaf(INDEX), &proof),
            Err(ProofDefect::LeafCountMismatch {
                implied: COUNT,
                stated: COUNT + 1
            })
        );
    }

    #[test]
    fn every_sibling_byte_flip_is_rejected() {
        let (_, proof) = fixture();
        for step in 0..proof.path.len() {
            for byte in [0usize, 7, 31] {
                let mut tampered = proof.clone();
                tampered.path[step].hash[byte] ^= 0x01;
                assert_eq!(reject(&tampered), ProofDefect::RootMismatch);
            }
        }
    }

    #[test]
    fn every_side_flip_is_rejected_or_moves_the_index() {
        let (root, proof) = fixture();
        for step in 0..proof.path.len() {
            let mut tampered = proof.clone();
            tampered.path[step].side = match tampered.path[step].side {
                Side::Left => Side::Right,
                Side::Right => Side::Left,
            };
            let outcome = verify_node_proof(&DOMAIN, &root, COUNT, &leaf(INDEX), &tampered);
            assert_eq!(
                outcome,
                Err(ProofDefect::RootMismatch),
                "side flip at step {step} must not verify"
            );
        }
    }

    #[test]
    fn a_lengthened_path_is_rejected() {
        let (_, proof) = fixture();
        let mut tampered = proof.clone();
        tampered.path.push(tampered.path[0]);
        assert!(matches!(
            reject(&tampered),
            ProofDefect::LeafCountMismatch { .. } | ProofDefect::PeakOrderInvalid
        ));
    }

    #[test]
    fn a_shortened_path_is_rejected() {
        let (_, proof) = fixture();
        let mut tampered = proof.clone();
        tampered.path.pop();
        assert!(matches!(
            reject(&tampered),
            ProofDefect::LeafCountMismatch { .. } | ProofDefect::PeakOrderInvalid
        ));
    }

    #[test]
    fn a_tampered_peak_hash_is_rejected() {
        let (_, proof) = fixture();
        for side in 0..2 {
            let mut tampered = proof.clone();
            let peaks = if side == 0 {
                &mut tampered.left_peaks
            } else {
                &mut tampered.right_peaks
            };
            if peaks.is_empty() {
                continue;
            }
            peaks[0].hash[0] ^= 0x01;
            assert_eq!(reject(&tampered), ProofDefect::RootMismatch);
        }
    }

    #[test]
    fn a_dropped_peak_is_rejected() {
        let (_, proof) = fixture();
        let mut tampered = proof.clone();
        assert!(!tampered.left_peaks.is_empty());
        tampered.left_peaks.remove(0);
        assert!(matches!(
            reject(&tampered),
            ProofDefect::LeafCountMismatch { .. }
        ));
    }

    #[test]
    fn an_invented_peak_is_rejected() {
        let (_, proof) = fixture();
        let mut tampered = proof.clone();
        tampered.right_peaks.push(PeakRef {
            height: 0,
            hash: [0x77; 32],
        });
        assert!(matches!(
            reject(&tampered),
            ProofDefect::LeafCountMismatch { .. } | ProofDefect::PeakOrderInvalid
        ));
    }

    #[test]
    fn out_of_order_peaks_are_rejected() {
        let (_, proof) = fixture();
        let mut tampered = proof.clone();
        assert!(!tampered.left_peaks.is_empty());
        tampered.left_peaks.push(PeakRef {
            height: 40,
            hash: [0x88; 32],
        });
        assert_eq!(reject(&tampered), ProofDefect::PeakOrderInvalid);
    }

    #[test]
    fn a_claimed_height_that_contradicts_the_path_is_rejected() {
        let (_, proof) = fixture();
        let mut tampered = proof.clone();
        tampered.height = 1;
        assert!(matches!(
            reject(&tampered),
            ProofDefect::LeafCountMismatch { .. }
                | ProofDefect::PeakOrderInvalid
                | ProofDefect::RootMismatch
        ));
    }
}

#[test]
fn an_empty_log_admits_no_node_proof() {
    let mmr = Mmr::new(DOMAIN);
    assert!(mmr.leaf_proof(0).is_none());
    let proof = NodeProof {
        height: 0,
        path: Vec::new(),
        left_peaks: Vec::new(),
        right_peaks: Vec::new(),
    };
    assert_eq!(
        verify_node_proof(&DOMAIN, &mmr.root(), 0, &leaf(0), &proof),
        Err(ProofDefect::EmptyLog)
    );
}

#[test]
fn a_single_leaf_log_has_a_pathless_proof() {
    let mmr = built(DOMAIN, 1);
    let proof = mmr.leaf_proof(0).expect("leaf is present");
    assert!(proof.path.is_empty());
    assert!(proof.left_peaks.is_empty());
    assert!(proof.right_peaks.is_empty());
    assert_eq!(
        verify_node_proof(&DOMAIN, &mmr.root(), 1, &leaf(0), &proof),
        Ok(NodePosition {
            height: 0,
            index: 0
        })
    );
}

#[test]
fn a_proof_taller_than_the_model_bound_is_rejected() {
    let mmr = built(DOMAIN, 4);
    let honest = mmr.leaf_proof(0).expect("leaf is present");
    let mut absurd = honest.clone();
    while absurd.path.len() <= 48 {
        absurd.path.push(honest.path[0]);
    }
    assert_eq!(
        verify_node_proof(&DOMAIN, &mmr.root(), 4, &leaf(0), &absurd),
        Err(ProofDefect::HeightOutOfDomain)
    );

    let mut tall = honest;
    tall.height = 49;
    assert_eq!(
        verify_node_proof(&DOMAIN, &mmr.root(), 4, &leaf(0), &tall),
        Err(ProofDefect::HeightOutOfDomain)
    );
}
