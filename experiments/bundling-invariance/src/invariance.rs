//! The invariance test, run exhaustively over the corpus.
//!
//! ## The test
//!
//! A criterion is **bundling-invariant** when, for every payoff object `w` and
//! every decomposition of `w` into parts carrying the same economic facts, if
//! every part receives the same label `L` then `w` receives `L`.
//!
//! That is the strict reading. Each criterion may also declare extra labels it
//! considers acceptable for a whole whose parts are unanimous; the sweep counts
//! violations under both readings and reports both, so the result cannot be
//! dismissed as an uncharitable test, and a criterion that survives only by
//! permitting several answers for the same parts is visibly doing that.
//!
//! ## Why a violation is an arbitrage
//!
//! Both decomposition families are the market operations of [`crate::market`]:
//! a binary split moves part of one position into another, and the elementary
//! unbundling states the position as its individual claims. Neither mints a
//! claim, burns one, or moves collateral; the claims outstanding in every cell
//! are the same before and after. So a criterion that answers differently for
//! the two sides has attached a different label to the same claims held a
//! different way, and the holder chooses which label to hold at no cost.

use crate::corpus::{binary_decompositions, index_of, vector_count, vectors_for};
use crate::criteria::{Category, candidates, fact_profiles};
use crate::payoff::{Facts, MAX_CELLS, MIN_CELLS, PAYOUT_CEILING, Payoff, PayoffObject};

/// Which decomposition produced a witness.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Family {
    /// The whole was split into two parts.
    BinarySplit,
    /// The whole was stated as its individual one-cell claims.
    ElementaryUnbundle,
}

impl Family {
    /// Stable lower-case name used in the corpus file.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::BinarySplit => "binary_split",
            Self::ElementaryUnbundle => "elementary_unbundle",
        }
    }
}

/// Which reading of the criterion's aggregation story a witness violates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Reading {
    /// Unanimous parts must fix the whole's label.
    Strict,
    /// Unanimous parts must fix the whole's label, or one of the extra labels
    /// the criterion itself declared acceptable.
    DeclaredStory,
}

impl Reading {
    /// Stable lower-case name used in the corpus file.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Strict => "strict",
            Self::DeclaredStory => "declared_story",
        }
    }
}

/// One concrete classification arbitrage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Witness {
    /// Which decomposition family produced it.
    pub family: Family,
    /// Cell count of the partition.
    pub cells: usize,
    /// Index of the fact profile in corpus order.
    pub profile: usize,
    /// Economic facts shared by the whole and every part.
    pub facts: Facts,
    /// The bundled object.
    pub whole: Payoff,
    /// The parts, which sum to the whole cell by cell.
    pub parts: Vec<Payoff>,
    /// Label the criterion gives the bundle.
    pub whole_label: Category,
    /// The single label the criterion gives every part.
    pub part_label: Category,
    /// Labels the criterion's story permitted for the bundle.
    pub permitted: Vec<Category>,
    /// Complete sets contained in the bundle, that is, the units of collateral
    /// it can be recombined into before resolution.
    pub complete_sets: u64,
}

impl Witness {
    /// Claims outstanding across the parts, which equal the bundle cell by cell.
    ///
    /// # Panics
    ///
    /// Panics if the parts do not share the bundle's partition, which the sweep
    /// never constructs.
    #[must_use]
    pub fn parts_total(&self) -> Payoff {
        let mut total = Payoff::zero(self.cells);
        for part in &self.parts {
            total = total.add(part).expect("parts share the partition");
        }
        total
    }

    /// Whether the decomposition conserves the claims outstanding in every cell.
    /// True for every witness the sweep emits; checked, not assumed.
    #[must_use]
    pub fn conserves_claims(&self) -> bool {
        self.parts_total() == self.whole
    }

    fn key(&self) -> (usize, u64, Payoff, Family, usize, Vec<Payoff>) {
        (
            self.cells,
            self.whole.total(),
            self.whole.clone(),
            self.family,
            self.profile,
            self.parts.clone(),
        )
    }

    fn beats(&self, other: &Self) -> bool {
        self.key() < other.key()
    }
}

/// What was measured about one criterion.
#[derive(Clone, Debug)]
pub struct CriterionReport {
    /// Stable criterion name.
    pub name: &'static str,
    /// Frozen parameters, or the empty string.
    pub parameters: String,
    /// What the criterion reads and answers.
    pub statement: &'static str,
    /// The criterion's own aggregation story.
    pub story: &'static str,
    /// Distinct labels the criterion emitted over the corpus, ascending.
    pub labels: Vec<Category>,
    /// Whether the criterion's answer ever changes with the payoff vector.
    pub reads_payoff: bool,
    /// Whether the criterion's answer ever changes with the economic facts.
    pub reads_facts: bool,
    /// Whether the criterion's story ever permits more than one label for the
    /// same unanimous parts.
    pub story_permits_alternatives: bool,
    /// Payoff objects classified.
    pub objects_classified: u64,
    /// Decompositions checked, over both families.
    pub decompositions_checked: u64,
    /// Decompositions whose parts all carried the same label.
    pub unanimous_decompositions: u64,
    /// Violations under the strict reading.
    pub violations_strict: u64,
    /// Violations under the criterion's declared story.
    pub violations_declared_story: u64,
    /// Distinct bundled objects with at least one strict violation.
    pub violating_objects_strict: u64,
    /// Smallest strict witness, by cell count, then bundle size, then vector
    /// order, then family, then facts, then parts.
    pub minimal_strict: Option<Witness>,
    /// Smallest witness under the declared story, ordered the same way.
    pub minimal_declared_story: Option<Witness>,
}

impl CriterionReport {
    /// Whether the criterion passed the strict test on this corpus.
    #[must_use]
    pub const fn invariant_strict(&self) -> bool {
        self.violations_strict == 0
    }

    /// Whether the criterion passed under its own declared story.
    #[must_use]
    pub const fn invariant_declared_story(&self) -> bool {
        self.violations_declared_story == 0
    }

    /// Whether the criterion separates anything at all.
    #[must_use]
    pub fn discriminating(&self) -> bool {
        self.labels.len() > 1
    }
}

/// The corpus bounds, restated in the output so the reader never has to guess.
#[derive(Clone, Copy, Debug)]
pub struct Bounds {
    /// Smallest cell count.
    pub min_cells: usize,
    /// Largest cell count.
    pub max_cells: usize,
    /// Largest per-cell payout.
    pub ceiling: u64,
    /// Payoff vectors enumerated.
    pub vectors: u64,
    /// Fact profiles enumerated.
    pub fact_profiles: u64,
    /// Payoff objects enumerated, that is, vectors times fact profiles.
    pub objects: u64,
    /// Distinct decompositions checked per criterion.
    pub decompositions: u64,
}

/// Everything the sweep measured.
#[derive(Clone, Debug)]
pub struct Report {
    /// The corpus bounds.
    pub bounds: Bounds,
    /// One entry per criterion, in corpus order.
    pub criteria: Vec<CriterionReport>,
}

impl Report {
    /// The report for one criterion by name.
    #[must_use]
    pub fn criterion(&self, name: &str) -> Option<&CriterionReport> {
        self.criteria.iter().find(|entry| entry.name == name)
    }
}

struct Acc {
    labels: Vec<Category>,
    reads_payoff: bool,
    reads_facts: bool,
    objects_classified: u64,
    decompositions_checked: u64,
    unanimous_decompositions: u64,
    violations_strict: u64,
    violations_declared_story: u64,
    violating_objects_strict: u64,
    minimal_strict: Option<Witness>,
    minimal_declared_story: Option<Witness>,
}

impl Acc {
    fn new() -> Self {
        Self {
            labels: Vec::new(),
            reads_payoff: false,
            reads_facts: false,
            objects_classified: 0,
            decompositions_checked: 0,
            unanimous_decompositions: 0,
            violations_strict: 0,
            violations_declared_story: 0,
            violating_objects_strict: 0,
            minimal_strict: None,
            minimal_declared_story: None,
        }
    }

    fn note_label(&mut self, label: Category) {
        if let Err(position) = self.labels.binary_search(&label) {
            self.labels.insert(position, label);
        }
    }

    fn offer(&mut self, reading: Reading, witness: &Witness) {
        let slot = match reading {
            Reading::Strict => &mut self.minimal_strict,
            Reading::DeclaredStory => &mut self.minimal_declared_story,
        };
        match slot {
            Some(current) if !witness.beats(current) => {}
            _ => *slot = Some(witness.clone()),
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn build_witness(
    family: Family,
    cells: usize,
    profile: usize,
    facts: Facts,
    whole: &Payoff,
    parts: Vec<Payoff>,
    whole_label: Category,
    part_label: Category,
    extras: &[Category],
) -> Witness {
    let mut permitted = vec![part_label];
    permitted.extend_from_slice(extras);
    Witness {
        family,
        cells,
        profile,
        facts,
        whole: whole.clone(),
        parts,
        whole_label,
        part_label,
        permitted,
        complete_sets: whole.min_payout(),
    }
}

/// Run the exhaustive sweep.
///
/// Deterministic, offline, and allocation-bounded: nothing here reads a clock, a
/// file, or an environment variable.
///
/// # Panics
///
/// Panics if the corpus enumeration and the payoff arithmetic disagree, which
/// would be a bug in this crate rather than a finding.
#[must_use]
pub fn run() -> Report {
    let criteria = candidates();
    let profiles = fact_profiles();
    let mut accs: Vec<Acc> = criteria.iter().map(|_| Acc::new()).collect();

    for cells in MIN_CELLS..=MAX_CELLS {
        let vectors = vectors_for(cells);
        let tables: Vec<Vec<Vec<Category>>> = criteria
            .iter()
            .map(|criterion| {
                profiles
                    .iter()
                    .map(|facts| {
                        vectors
                            .iter()
                            .map(|payoff| {
                                criterion.classify(&PayoffObject::new(payoff.clone(), *facts))
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();

        for (index, acc) in accs.iter_mut().enumerate() {
            acc.objects_classified += (vectors.len() * profiles.len()) as u64;
            for labels in &tables[index] {
                for label in labels {
                    acc.note_label(*label);
                }
                if labels.windows(2).any(|pair| pair[0] != pair[1]) {
                    acc.reads_payoff = true;
                }
            }
            for vector_index in 0..vectors.len() {
                let first = tables[index][0][vector_index];
                if tables[index]
                    .iter()
                    .any(|labels| labels[vector_index] != first)
                {
                    acc.reads_facts = true;
                }
            }
        }

        let mut seen: Vec<Vec<bool>> = accs
            .iter()
            .map(|_| vec![false; vectors.len() * profiles.len()])
            .collect();

        for (whole_index, whole) in vectors.iter().enumerate() {
            let splits = binary_decompositions(whole);
            let indexed: Vec<(usize, usize)> = splits
                .iter()
                .map(|(first, second)| (index_of(first), index_of(second)))
                .collect();
            for (criterion_index, criterion) in criteria.iter().enumerate() {
                let acc = &mut accs[criterion_index];
                for (profile_index, facts) in profiles.iter().enumerate() {
                    let labels = &tables[criterion_index][profile_index];
                    let whole_label = labels[whole_index];
                    for (split, (first_index, second_index)) in splits.iter().zip(indexed.iter()) {
                        acc.decompositions_checked += 1;
                        let first_label = labels[*first_index];
                        if first_label != labels[*second_index] {
                            continue;
                        }
                        acc.unanimous_decompositions += 1;
                        if whole_label == first_label {
                            continue;
                        }
                        let extras = criterion.declared_extras(first_label);
                        let witness = build_witness(
                            Family::BinarySplit,
                            cells,
                            profile_index,
                            *facts,
                            whole,
                            vec![split.0.clone(), split.1.clone()],
                            whole_label,
                            first_label,
                            extras,
                        );
                        acc.violations_strict += 1;
                        let seen_slot =
                            &mut seen[criterion_index][profile_index * vectors.len() + whole_index];
                        if !*seen_slot {
                            *seen_slot = true;
                            acc.violating_objects_strict += 1;
                        }
                        acc.offer(Reading::Strict, &witness);
                        if !extras.contains(&whole_label) {
                            acc.violations_declared_story += 1;
                            acc.offer(Reading::DeclaredStory, &witness);
                        }
                    }
                }
            }
        }

        for (whole_index, whole) in vectors.iter().enumerate() {
            let support = whole.support();
            if support.is_empty() {
                continue;
            }
            let mut parts = Vec::new();
            let mut part_indices = Vec::new();
            for cell in &support {
                let unit = Payoff::unit(cells, *cell);
                let unit_index = index_of(&unit);
                for _ in 0..whole.get(*cell) {
                    parts.push(unit.clone());
                    part_indices.push(unit_index);
                }
            }
            for (criterion_index, criterion) in criteria.iter().enumerate() {
                let acc = &mut accs[criterion_index];
                for (profile_index, facts) in profiles.iter().enumerate() {
                    let labels = &tables[criterion_index][profile_index];
                    let whole_label = labels[whole_index];
                    acc.decompositions_checked += 1;
                    let first_label = labels[part_indices[0]];
                    if part_indices
                        .iter()
                        .any(|index| labels[*index] != first_label)
                    {
                        continue;
                    }
                    acc.unanimous_decompositions += 1;
                    if whole_label == first_label {
                        continue;
                    }
                    let extras = criterion.declared_extras(first_label);
                    let witness = build_witness(
                        Family::ElementaryUnbundle,
                        cells,
                        profile_index,
                        *facts,
                        whole,
                        parts.clone(),
                        whole_label,
                        first_label,
                        extras,
                    );
                    acc.violations_strict += 1;
                    let seen_slot =
                        &mut seen[criterion_index][profile_index * vectors.len() + whole_index];
                    if !*seen_slot {
                        *seen_slot = true;
                        acc.violating_objects_strict += 1;
                    }
                    acc.offer(Reading::Strict, &witness);
                    if !extras.contains(&whole_label) {
                        acc.violations_declared_story += 1;
                        acc.offer(Reading::DeclaredStory, &witness);
                    }
                }
            }
        }
    }

    let decompositions_per_criterion = accs
        .first()
        .map_or(0, |acc| acc.decompositions_checked / profiles.len() as u64);
    let criteria_reports = criteria
        .iter()
        .zip(accs)
        .map(|(criterion, acc)| {
            let story_permits_alternatives = acc
                .labels
                .iter()
                .any(|label| !criterion.declared_extras(*label).is_empty());
            CriterionReport {
                name: criterion.name(),
                parameters: criterion.parameters(),
                statement: criterion.statement(),
                story: criterion.story(),
                labels: acc.labels,
                reads_payoff: acc.reads_payoff,
                reads_facts: acc.reads_facts,
                story_permits_alternatives,
                objects_classified: acc.objects_classified,
                decompositions_checked: acc.decompositions_checked,
                unanimous_decompositions: acc.unanimous_decompositions,
                violations_strict: acc.violations_strict,
                violations_declared_story: acc.violations_declared_story,
                violating_objects_strict: acc.violating_objects_strict,
                minimal_strict: acc.minimal_strict,
                minimal_declared_story: acc.minimal_declared_story,
            }
        })
        .collect();

    Report {
        bounds: Bounds {
            min_cells: MIN_CELLS,
            max_cells: MAX_CELLS,
            ceiling: PAYOUT_CEILING,
            vectors: vector_count() as u64,
            fact_profiles: profiles.len() as u64,
            objects: (vector_count() * profiles.len()) as u64,
            decompositions: decompositions_per_criterion,
        },
        criteria: criteria_reports,
    }
}
