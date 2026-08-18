//! An append-only Merkle mountain range and its standalone proof verifiers.
//!
//! The structure is the commitment primitive for the admission log. It is
//! implemented here rather than imported, so nothing in this experiment depends
//! on an external crate, and so the exact shape of a proof object is visible
//! and testable.
//!
//! Shape. Leaves are appended one at a time. After `n` leaves the structure is
//! a list of perfect binary trees, one per set bit of `n`, with strictly
//! decreasing heights, covering disjoint consecutive leaf ranges. Those roots
//! are the *peaks*. Appending merges equal-height peaks and never rewrites an
//! existing node, which is the mechanical sense in which the log is
//! append-only.
//!
//! Commitment. The root is
//! `tagged(ROOT_TAG, domain || leaf_count || bag(peaks))`. Two things matter:
//! the `leaf_count` is inside the root, so a root commits to the exact number
//! of admitted records rather than only to their hashes; and `domain` is a
//! caller-supplied digest, so a root produced for one relation, batch, market,
//! or cutoff can never be replayed as a root for another.
//!
//! Proofs. [`NodeProof`] proves that a given hash sits at a determined position
//! in a determined tree. Its verifier *derives* the position from the proof
//! rather than trusting a claimed one, so a proof cannot be moved to a
//! different index. A leaf inclusion proof is the height-zero case; a
//! [`ConsistencyProof`] is a list of node proofs, one per peak of the earlier
//! log, and proves that the earlier root is a genuine prefix of the later one.
//!
//! What this is not: no network, no signature, no availability. A root proves
//! that a record was committed; it says nothing about whether the record's
//! payload can be retrieved. That is the separate obligation modelled in
//! [`crate::lifecycle`].

use crate::hash::tagged;

/// Tag for a leaf preimage.
pub const LEAF_TAG: &[u8] = b"degg/inclusion-availability/v0/leaf";
/// Tag for an interior node.
pub const NODE_TAG: &[u8] = b"degg/inclusion-availability/v0/node";
/// Tag for one right-to-left peak-bagging step.
pub const BAG_TAG: &[u8] = b"degg/inclusion-availability/v0/bag";
/// Tag for the root, which binds the domain digest and the exact leaf count.
pub const ROOT_TAG: &[u8] = b"degg/inclusion-availability/v0/root";

/// Highest peak height the model admits. Bounds every shift below.
pub const MAX_HEIGHT: u32 = 48;

/// Hash a leaf preimage into a leaf node.
#[must_use]
pub fn leaf_hash(preimage: &[u8]) -> [u8; 32] {
    tagged(LEAF_TAG, &[preimage])
}

/// Hash two child nodes into their parent.
#[must_use]
pub fn node_hash(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    tagged(NODE_TAG, &[left, right])
}

/// Fold peak hashes right to left into a single bag value.
///
/// An empty peak list bags to the tagged empty value; the root separately binds
/// a zero leaf count, so the empty log has its own distinct root.
#[must_use]
pub fn bag(peaks: &[[u8; 32]]) -> [u8; 32] {
    match peaks.split_last() {
        None => tagged(BAG_TAG, &[]),
        Some((last, rest)) => {
            let mut accumulator = *last;
            for peak in rest.iter().rev() {
                accumulator = tagged(BAG_TAG, &[peak, &accumulator]);
            }
            accumulator
        }
    }
}

/// Compute the root from a domain digest, a leaf count, and a bag value.
#[must_use]
pub fn root_of(domain: &[u8; 32], leaf_count: u64, bagged: &[u8; 32]) -> [u8; 32] {
    tagged(ROOT_TAG, &[domain, &leaf_count.to_be_bytes(), bagged])
}

/// Compute the root directly from a domain digest, a leaf count, and the peaks.
#[must_use]
pub fn root_from_peaks(domain: &[u8; 32], leaf_count: u64, peaks: &[[u8; 32]]) -> [u8; 32] {
    root_of(domain, leaf_count, &bag(peaks))
}

/// The heights of the peaks of an `n`-leaf range, highest first.
///
/// These are exactly the set bits of `n`, so the peak shape is a function of
/// the leaf count alone. That is why binding `leaf_count` in the root is enough
/// to fix the peak structure.
#[must_use]
pub fn peak_heights(leaf_count: u64) -> Vec<u32> {
    let mut out = Vec::new();
    for height in (0..=MAX_HEIGHT).rev() {
        if (leaf_count >> height) & 1 == 1 {
            out.push(height);
        }
    }
    out
}

/// A peak: its height and its hash. The leaf span is `1 << height`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PeakRef {
    /// Height above the leaf level.
    pub height: u32,
    /// Node hash at that height.
    pub hash: [u8; 32],
}

/// Which side of the parent a supplied sibling sits on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    /// The sibling is the left child, so the proved node is the right child.
    Left,
    /// The sibling is the right child, so the proved node is the left child.
    Right,
}

/// One authentication-path step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sibling {
    /// Side the sibling occupies.
    pub side: Side,
    /// The sibling's node hash.
    pub hash: [u8; 32],
}

/// A position in the structure: height above the leaves and index at that height.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NodePosition {
    /// Height above the leaf level; zero for a leaf.
    pub height: u32,
    /// Index among the nodes of that height, counting from the first leaf.
    pub index: u64,
}

/// A proof that one node hash sits at one determined position of one root.
///
/// The verifier derives the position from `path` and `left_peaks`; the position
/// is therefore not a free parameter a prover may choose.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeProof {
    /// Height of the proved node.
    pub height: u32,
    /// Authentication path from the proved node up to its containing peak.
    pub path: Vec<Sibling>,
    /// Peaks strictly to the left of the containing peak, highest first.
    pub left_peaks: Vec<PeakRef>,
    /// Peaks strictly to the right of the containing peak, highest first.
    pub right_peaks: Vec<PeakRef>,
}

/// A proof that an `m`-leaf log is a prefix of an `n`-leaf log, `m <= n`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConsistencyProof {
    /// Leaf count of the earlier log.
    pub prefix_leaf_count: u64,
    /// One entry per peak of the earlier log, highest first.
    pub prefix_peaks: Vec<PrefixPeak>,
}

/// One peak of the earlier log, together with its proof in the later log.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrefixPeak {
    /// The peak's hash in the earlier log.
    pub hash: [u8; 32],
    /// A proof that the same node is present, at the same position, later.
    pub proof: NodeProof,
}

/// Every way a proof can fail to verify.
///
/// The classes are distinct so a test can assert *why* an adversarial proof was
/// rejected, not merely that it was.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProofDefect {
    /// The log is empty, so no node proof can exist.
    EmptyLog,
    /// A peak height exceeds [`MAX_HEIGHT`], or the path is longer than that.
    HeightOutOfDomain,
    /// Peak heights are not strictly decreasing, or straddle the containing peak.
    PeakOrderInvalid,
    /// The peak spans do not add up to the stated leaf count.
    LeafCountMismatch {
        /// Leaf count implied by the proof's peak structure.
        implied: u64,
        /// Leaf count the root was taken at.
        stated: u64,
    },
    /// The recomputed root differs from the root the proof was checked against.
    RootMismatch,
    /// The derived position is not the position the caller required.
    PositionMismatch {
        /// Position derived from the proof.
        derived: NodePosition,
        /// Position the caller required.
        required: NodePosition,
    },
    /// The prefix peak list does not match the peak shape of its leaf count.
    PrefixShapeInvalid,
    /// The prefix leaf count exceeds the leaf count of the later log.
    PrefixLongerThanLog {
        /// Claimed earlier leaf count.
        prefix: u64,
        /// Leaf count of the later log.
        log: u64,
    },
}

/// An append-only Merkle mountain range over one domain digest.
#[derive(Clone, Debug)]
pub struct Mmr {
    domain: [u8; 32],
    levels: Vec<Vec<[u8; 32]>>,
}

impl Mmr {
    /// An empty range bound to `domain`.
    #[must_use]
    pub fn new(domain: [u8; 32]) -> Self {
        Self {
            domain,
            levels: vec![Vec::new()],
        }
    }

    /// The domain digest this range is bound to.
    #[must_use]
    pub fn domain(&self) -> [u8; 32] {
        self.domain
    }

    /// Number of leaves appended so far.
    #[must_use]
    pub fn leaf_count(&self) -> u64 {
        self.levels[0].len() as u64
    }

    /// Append one leaf hash and return its index.
    ///
    /// Existing nodes are never rewritten: the append only pushes new nodes.
    pub fn append(&mut self, leaf: [u8; 32]) -> u64 {
        let index = self.leaf_count();
        assert!(
            index < 1u64 << MAX_HEIGHT,
            "leaf count exceeds the model bound"
        );
        self.levels[0].push(leaf);
        let mut height = 0usize;
        while self.levels[height].len().is_multiple_of(2) {
            let len = self.levels[height].len();
            let parent = node_hash(&self.levels[height][len - 2], &self.levels[height][len - 1]);
            if self.levels.len() == height + 1 {
                self.levels.push(Vec::new());
            }
            self.levels[height + 1].push(parent);
            height += 1;
        }
        index
    }

    /// A node hash by position, if that node exists.
    #[must_use]
    pub fn node(&self, position: NodePosition) -> Option<[u8; 32]> {
        let level = self.levels.get(position.height as usize)?;
        level.get(usize::try_from(position.index).ok()?).copied()
    }

    /// The peaks of the current range, highest first.
    #[must_use]
    pub fn peaks(&self) -> Vec<PeakRef> {
        self.peaks_at(self.leaf_count())
    }

    /// The peaks of the `leaf_count`-leaf prefix, highest first.
    ///
    /// Every such node is present in the current range, because appending never
    /// rewrites a node. Returns an empty list for a zero-leaf prefix.
    #[must_use]
    pub fn peaks_at(&self, leaf_count: u64) -> Vec<PeakRef> {
        self.peak_positions(leaf_count)
            .into_iter()
            .map(|(position, _start)| PeakRef {
                height: position.height,
                hash: self.node(position).expect("peak node is present"),
            })
            .collect()
    }

    /// Peak positions together with the first leaf each peak covers.
    fn peak_positions(&self, leaf_count: u64) -> Vec<(NodePosition, u64)> {
        let mut out = Vec::new();
        let mut start = 0u64;
        for height in peak_heights(leaf_count) {
            out.push((
                NodePosition {
                    height,
                    index: start >> height,
                },
                start,
            ));
            start += 1u64 << height;
        }
        out
    }

    /// The current root.
    #[must_use]
    pub fn root(&self) -> [u8; 32] {
        self.root_at(self.leaf_count())
    }

    /// The root the range had, or will have been shown to have had, at `leaf_count` leaves.
    #[must_use]
    pub fn root_at(&self, leaf_count: u64) -> [u8; 32] {
        let hashes: Vec<[u8; 32]> = self
            .peaks_at(leaf_count)
            .into_iter()
            .map(|p| p.hash)
            .collect();
        root_from_peaks(&self.domain, leaf_count, &hashes)
    }

    /// A proof for the node at `position` against the current root.
    #[must_use]
    pub fn node_proof(&self, position: NodePosition) -> Option<NodeProof> {
        self.node_proof_at(position, self.leaf_count())
    }

    /// A proof for the node at `position` against the root at `leaf_count` leaves.
    #[must_use]
    pub fn node_proof_at(&self, position: NodePosition, leaf_count: u64) -> Option<NodeProof> {
        if position.height > MAX_HEIGHT || self.node(position).is_none() {
            return None;
        }
        let first_leaf = position.index.checked_shl(position.height)?;
        let peaks = self.peak_positions(leaf_count);
        let containing = peaks.iter().position(|(peak, start)| {
            peak.height >= position.height
                && *start <= first_leaf
                && first_leaf < start + (1u64 << peak.height)
        })?;
        let (peak, _) = peaks[containing];

        let mut path = Vec::new();
        let mut index = position.index;
        for height in position.height..peak.height {
            let sibling = index ^ 1;
            let hash = self.node(NodePosition {
                height,
                index: sibling,
            })?;
            let side = if index.is_multiple_of(2) {
                Side::Right
            } else {
                Side::Left
            };
            path.push(Sibling { side, hash });
            index >>= 1;
        }

        let peak_ref = |(at, _): &(NodePosition, u64)| PeakRef {
            height: at.height,
            hash: self.node(*at).expect("peak node is present"),
        };
        Some(NodeProof {
            height: position.height,
            path,
            left_peaks: peaks[..containing].iter().map(peak_ref).collect(),
            right_peaks: peaks[containing + 1..].iter().map(peak_ref).collect(),
        })
    }

    /// A leaf inclusion proof against the current root.
    #[must_use]
    pub fn leaf_proof(&self, index: u64) -> Option<NodeProof> {
        self.node_proof(NodePosition { height: 0, index })
    }

    /// A leaf inclusion proof against the root at `leaf_count` leaves.
    #[must_use]
    pub fn leaf_proof_at(&self, index: u64, leaf_count: u64) -> Option<NodeProof> {
        if index >= leaf_count {
            return None;
        }
        self.node_proof_at(NodePosition { height: 0, index }, leaf_count)
    }

    /// A proof that the `prefix_leaf_count`-leaf log is a prefix of this one.
    #[must_use]
    pub fn consistency_proof(&self, prefix_leaf_count: u64) -> Option<ConsistencyProof> {
        if prefix_leaf_count > self.leaf_count() {
            return None;
        }
        let mut prefix_peaks = Vec::new();
        for (position, _start) in self.peak_positions(prefix_leaf_count) {
            prefix_peaks.push(PrefixPeak {
                hash: self.node(position)?,
                proof: self.node_proof(position)?,
            });
        }
        Some(ConsistencyProof {
            prefix_leaf_count,
            prefix_peaks,
        })
    }
}

/// Verify a node proof and return the position it determines.
///
/// The caller supplies the node hash; the proof supplies the shape. Nothing
/// about the position is taken on the prover's word.
pub fn verify_node_proof(
    domain: &[u8; 32],
    root: &[u8; 32],
    leaf_count: u64,
    node: &[u8; 32],
    proof: &NodeProof,
) -> Result<NodePosition, ProofDefect> {
    if leaf_count == 0 {
        return Err(ProofDefect::EmptyLog);
    }
    let path_len = u32::try_from(proof.path.len()).map_err(|_| ProofDefect::HeightOutOfDomain)?;
    let peak_height = proof
        .height
        .checked_add(path_len)
        .ok_or(ProofDefect::HeightOutOfDomain)?;
    if peak_height > MAX_HEIGHT {
        return Err(ProofDefect::HeightOutOfDomain);
    }

    let mut previous = None;
    for peak in &proof.left_peaks {
        if peak.height > MAX_HEIGHT || peak.height <= peak_height {
            return Err(ProofDefect::PeakOrderInvalid);
        }
        if previous.is_some_and(|p: u32| p <= peak.height) {
            return Err(ProofDefect::PeakOrderInvalid);
        }
        previous = Some(peak.height);
    }
    let mut previous = Some(peak_height);
    for peak in &proof.right_peaks {
        if peak.height >= peak_height {
            return Err(ProofDefect::PeakOrderInvalid);
        }
        if previous.is_some_and(|p: u32| p <= peak.height) {
            return Err(ProofDefect::PeakOrderInvalid);
        }
        previous = Some(peak.height);
    }

    let start: u64 = proof.left_peaks.iter().map(|p| 1u64 << p.height).sum();
    let tail: u64 = proof.right_peaks.iter().map(|p| 1u64 << p.height).sum();
    let implied = start + (1u64 << peak_height) + tail;
    if implied != leaf_count {
        return Err(ProofDefect::LeafCountMismatch {
            implied,
            stated: leaf_count,
        });
    }

    let mut current = *node;
    let mut local = 0u64;
    for (step, sibling) in proof.path.iter().enumerate() {
        current = match sibling.side {
            Side::Right => node_hash(&current, &sibling.hash),
            Side::Left => {
                local |= 1u64 << step;
                node_hash(&sibling.hash, &current)
            }
        };
    }

    let mut hashes: Vec<[u8; 32]> = proof.left_peaks.iter().map(|p| p.hash).collect();
    hashes.push(current);
    hashes.extend(proof.right_peaks.iter().map(|p| p.hash));
    if root_from_peaks(domain, leaf_count, &hashes) != *root {
        return Err(ProofDefect::RootMismatch);
    }

    Ok(NodePosition {
        height: proof.height,
        index: (start >> proof.height) + local,
    })
}

/// Verify a node proof and additionally require an exact position.
pub fn verify_node_proof_at(
    domain: &[u8; 32],
    root: &[u8; 32],
    leaf_count: u64,
    node: &[u8; 32],
    proof: &NodeProof,
    required: NodePosition,
) -> Result<(), ProofDefect> {
    let derived = verify_node_proof(domain, root, leaf_count, node, proof)?;
    if derived == required {
        Ok(())
    } else {
        Err(ProofDefect::PositionMismatch { derived, required })
    }
}

/// Verify a consistency proof and return the earlier log's root.
///
/// A caller compares the returned root against whatever earlier root it was
/// given. Equality means the later log genuinely extends the earlier one;
/// inequality is the append-only violation modelled in
/// [`crate::equivocation`].
pub fn verify_consistency(
    domain: &[u8; 32],
    root: &[u8; 32],
    leaf_count: u64,
    proof: &ConsistencyProof,
) -> Result<[u8; 32], ProofDefect> {
    if proof.prefix_leaf_count > leaf_count {
        return Err(ProofDefect::PrefixLongerThanLog {
            prefix: proof.prefix_leaf_count,
            log: leaf_count,
        });
    }
    let heights = peak_heights(proof.prefix_leaf_count);
    if heights.len() != proof.prefix_peaks.len() {
        return Err(ProofDefect::PrefixShapeInvalid);
    }

    let mut start = 0u64;
    let mut hashes = Vec::with_capacity(heights.len());
    for (height, peak) in heights.iter().zip(proof.prefix_peaks.iter()) {
        if peak.proof.height != *height {
            return Err(ProofDefect::PrefixShapeInvalid);
        }
        verify_node_proof_at(
            domain,
            root,
            leaf_count,
            &peak.hash,
            &peak.proof,
            NodePosition {
                height: *height,
                index: start >> height,
            },
        )?;
        hashes.push(peak.hash);
        start += 1u64 << height;
    }

    Ok(root_from_peaks(domain, proof.prefix_leaf_count, &hashes))
}
