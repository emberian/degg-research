//! Classification criteria, as decidable functions from a payoff object's
//! economic facts to a category label.
//!
//! A criterion here is exactly what the memo says a classification criterion is:
//! a total, decidable function from what a drafter can read off a contract's own
//! frozen terms to a category. Nothing is hidden from it; nothing else is
//! available to it. Each criterion also states its **aggregation story** — its
//! own account of how the labels of the parts bear on the label of the whole —
//! so that the invariance test never has to invent a story on a criterion's
//! behalf.
//!
//! The family below is drawn from the memo's discussion: payoff-shape tests of
//! the kind Question 8 invites (binary payout, bounded payout, how many distinct
//! amounts appear, whether the outcome set is exhaustive), a prefunding test,
//! and the facts-based control the memo actually proposes.

use crate::payoff::{Facts, PayoffObject, ReferenceKind};

/// A category label a criterion can assign.
///
/// The labels are deliberately the criteria's own vocabulary rather than
/// statutory categories. Nothing here is a legal conclusion; a label is a name
/// for "the box this criterion puts the object in".
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Category {
    /// An all-or-nothing claim on a stated outcome.
    EventContract,
    /// A portfolio of all-or-nothing claims, if a criterion has such a label.
    EventContractPortfolio,
    /// Anything the binary-payout criterion does not call an event contract.
    SwapLike,
    /// Payout small enough for the bounded-payout criterion to exclude it.
    ExcludedSmallPayout,
    /// Payout too large for the bounded-payout criterion to exclude it.
    IncludedLargePayout,
    /// Few enough distinct amounts to look like a single claim.
    SimpleClaim,
    /// Enough distinct amounts to look like a portfolio.
    PortfolioInstrument,
    /// A put, call, straddle, option, or privilege on a security or index.
    OptionOnSecurityOrIndex,
    /// A complete set, economically identical to the collateral it was issued
    /// against.
    CollateralEquivalent,
    /// Pays nothing in any state.
    NotAnInstrument,
    /// Pays something in some state.
    Claim,
    /// Settles on a fact about an issuer rather than on a price or value.
    SecurityBasedSwap,
    /// Excluded because the maximum payout is prefunded.
    ExcludedPrefunded,
    /// Not excluded by the prefunding criterion.
    NotExcludedPrefunded,
    /// The degenerate control's only label.
    Unclassified,
}

impl Category {
    /// Stable lower-case name used in the corpus file.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::EventContract => "event_contract",
            Self::EventContractPortfolio => "event_contract_portfolio",
            Self::SwapLike => "swap_like",
            Self::ExcludedSmallPayout => "excluded_small_payout",
            Self::IncludedLargePayout => "included_large_payout",
            Self::SimpleClaim => "simple_claim",
            Self::PortfolioInstrument => "portfolio_instrument",
            Self::OptionOnSecurityOrIndex => "option_on_security_or_index",
            Self::CollateralEquivalent => "collateral_equivalent",
            Self::NotAnInstrument => "not_an_instrument",
            Self::Claim => "claim",
            Self::SecurityBasedSwap => "security_based_swap",
            Self::ExcludedPrefunded => "excluded_prefunded",
            Self::NotExcludedPrefunded => "not_excluded_prefunded",
            Self::Unclassified => "unclassified",
        }
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

/// A classification criterion.
pub trait Criterion {
    /// Stable name used in the corpus file.
    fn name(&self) -> &'static str;

    /// Frozen parameters, rendered stably. Empty when there are none.
    fn parameters(&self) -> String {
        String::new()
    }

    /// One sentence saying what the criterion reads and what it answers.
    fn statement(&self) -> &'static str;

    /// The criterion's own account of how part labels bear on the whole's label.
    fn story(&self) -> &'static str;

    /// The label this criterion assigns to one payoff object.
    fn classify(&self, object: &PayoffObject) -> Category;

    /// Labels the criterion declares acceptable for a whole whose parts all
    /// carry `unanimous`, beyond `unanimous` itself.
    ///
    /// This is the criterion's escape hatch, and it is reported: a criterion
    /// that survives only by permitting several different answers for the same
    /// parts has declined to classify rather than passed the test.
    fn declared_extras(&self, _unanimous: Category) -> &'static [Category] {
        &[]
    }
}

/// "A binary payout makes it an event contract; anything else is swap-like."
///
/// The payoff test Question 8 invites most directly: a claim paying a fixed
/// amount on a stated outcome and nothing otherwise is cash-or-nothing.
#[derive(Clone, Copy, Debug, Default)]
pub struct BinaryPayout;

impl Criterion for BinaryPayout {
    fn name(&self) -> &'static str {
        "binary-payout"
    }

    fn statement(&self) -> &'static str {
        "a payoff taking exactly two values, one of them zero, is an event contract; anything else is swap-like"
    }

    fn story(&self) -> &'static str {
        "parts that are all event contracts make an event contract, or a portfolio of them if the vocabulary had such a label; parts that are all swap-like make a swap-like object"
    }

    fn classify(&self, object: &PayoffObject) -> Category {
        let values = object.payoff().distinct_values();
        if values.len() == 2 && values[0] == 0 {
            Category::EventContract
        } else {
            Category::SwapLike
        }
    }

    fn declared_extras(&self, unanimous: Category) -> &'static [Category] {
        match unanimous {
            Category::EventContract => &[Category::EventContractPortfolio],
            _ => &[],
        }
    }
}

/// "A payout bounded by the ceiling is excluded."
///
/// The form the memo's Position 6 objects to, stated as a criterion so that the
/// objection can be measured rather than asserted.
#[derive(Clone, Copy, Debug)]
pub struct BoundedPayout {
    /// Largest payout the criterion treats as small.
    pub ceiling: u64,
}

impl Criterion for BoundedPayout {
    fn name(&self) -> &'static str {
        "bounded-payout"
    }

    fn parameters(&self) -> String {
        format!("ceiling={}", self.ceiling)
    }

    fn statement(&self) -> &'static str {
        "a payoff whose largest payout is at most the ceiling is excluded; anything larger is included"
    }

    fn story(&self) -> &'static str {
        "parts that are all excluded make an excluded object; parts that are all included make an included object"
    }

    fn classify(&self, object: &PayoffObject) -> Category {
        if object.payoff().max_payout() <= self.ceiling {
            Category::ExcludedSmallPayout
        } else {
            Category::IncludedLargePayout
        }
    }
}

/// "More than two distinct amounts makes it a portfolio instrument."
#[derive(Clone, Copy, Debug, Default)]
pub struct DistinctValues;

impl Criterion for DistinctValues {
    fn name(&self) -> &'static str {
        "distinct-values"
    }

    fn statement(&self) -> &'static str {
        "a payoff vector with more than two distinct amounts is a portfolio instrument; anything else is a simple claim"
    }

    fn story(&self) -> &'static str {
        "parts that are all simple claims make a simple claim or a portfolio instrument, the criterion's own name for a bundle of them; parts that are all portfolio instruments make a portfolio instrument"
    }

    fn classify(&self, object: &PayoffObject) -> Category {
        if object.payoff().distinct_values().len() > 2 {
            Category::PortfolioInstrument
        } else {
            Category::SimpleClaim
        }
    }

    fn declared_extras(&self, unanimous: Category) -> &'static [Category] {
        match unanimous {
            Category::SimpleClaim => &[Category::PortfolioInstrument],
            _ => &[],
        }
    }
}

/// "A single outcome makes it an option; an exhaustive set makes it collateral."
///
/// The memo's fact (iii) promoted from a fact to a criterion: it reads whether
/// the outcome set the object pays across is exhaustive, non-overlapping, or a
/// single cell.
#[derive(Clone, Copy, Debug, Default)]
pub struct CompleteSet;

impl Criterion for CompleteSet {
    fn name(&self) -> &'static str {
        "complete-set"
    }

    fn statement(&self) -> &'static str {
        "a payoff on one outcome is an option; a level payoff across every outcome is collateral-equivalent; a payoff on some but not all outcomes is a portfolio; a payoff on none is not an instrument"
    }

    fn story(&self) -> &'static str {
        "parts that are all options make an option, a portfolio, or collateral; parts that are all portfolios make a portfolio or collateral; parts that are all collateral make collateral"
    }

    fn classify(&self, object: &PayoffObject) -> Category {
        let payoff = object.payoff();
        if payoff.is_zero() {
            Category::NotAnInstrument
        } else if payoff.is_constant() {
            Category::CollateralEquivalent
        } else if payoff.support().len() == 1 {
            Category::OptionOnSecurityOrIndex
        } else {
            Category::PortfolioInstrument
        }
    }

    fn declared_extras(&self, unanimous: Category) -> &'static [Category] {
        match unanimous {
            Category::OptionOnSecurityOrIndex => &[
                Category::PortfolioInstrument,
                Category::CollateralEquivalent,
            ],
            Category::PortfolioInstrument => &[Category::CollateralEquivalent],
            _ => &[],
        }
    }
}

/// "If it pays in some state it is a claim; if it never pays it is not."
///
/// A payoff-shape criterion included because it reads the payoff vector and
/// still has a chance of surviving. Whether it does is a measurement, not a
/// design choice.
#[derive(Clone, Copy, Debug, Default)]
pub struct SupportNonEmpty;

impl Criterion for SupportNonEmpty {
    fn name(&self) -> &'static str {
        "support-nonempty"
    }

    fn statement(&self) -> &'static str {
        "a payoff that pays something in some state is a claim; a payoff that never pays is not an instrument"
    }

    fn story(&self) -> &'static str {
        "parts that are all claims make a claim; parts that all never pay make an object that never pays"
    }

    fn classify(&self, object: &PayoffObject) -> Category {
        if object.payoff().is_zero() {
            Category::NotAnInstrument
        } else {
            Category::Claim
        }
    }
}

/// "A fully prefunded claim is excluded."
///
/// Reads a fact, not the payoff vector. Included to separate two different
/// objections: a criterion can be bundling-invariant and still be one this
/// program argues against, and the corpus should be able to show that.
#[derive(Clone, Copy, Debug, Default)]
pub struct PrefundingExclusion;

impl Criterion for PrefundingExclusion {
    fn name(&self) -> &'static str {
        "prefunding-exclusion"
    }

    fn statement(&self) -> &'static str {
        "a claim whose maximum payout is locked against collateral before it exists is excluded; anything else is not"
    }

    fn story(&self) -> &'static str {
        "parts that are all excluded make an excluded object; parts that are all not excluded make an object that is not excluded"
    }

    fn classify(&self, object: &PayoffObject) -> Category {
        if object.facts().funded {
            Category::ExcludedPrefunded
        } else {
            Category::NotExcludedPrefunded
        }
    }
}

/// The control: the memo's own test, reading the reference variable and the
/// settlement function and nothing else.
#[derive(Clone, Copy, Debug, Default)]
pub struct ReferenceAndSettlement;

impl Criterion for ReferenceAndSettlement {
    fn name(&self) -> &'static str {
        "reference-and-settlement"
    }

    fn statement(&self) -> &'static str {
        "an object whose settlement amount is a function of the price or value of a security or index is an option on that reference; an object whose settlement amount is a function of an issuer fact is a security-based swap"
    }

    fn story(&self) -> &'static str {
        "parts of a bundle share a reference variable and a settlement function, so the whole carries the label the parts carry"
    }

    fn classify(&self, object: &PayoffObject) -> Category {
        match object.facts().reference {
            ReferenceKind::SecurityPrice | ReferenceKind::IndexValue => {
                Category::OptionOnSecurityOrIndex
            }
            ReferenceKind::IssuerFact => Category::SecurityBasedSwap,
        }
    }
}

/// The degenerate control: one label for everything.
///
/// Present so that the corpus can say out loud what invariance does and does not
/// buy. This criterion is invariant and useless, which is the point.
#[derive(Clone, Copy, Debug, Default)]
pub struct ConstantLabel;

impl Criterion for ConstantLabel {
    fn name(&self) -> &'static str {
        "constant-label"
    }

    fn statement(&self) -> &'static str {
        "everything receives the same label"
    }

    fn story(&self) -> &'static str {
        "the whole carries the label the parts carry, there being only one"
    }

    fn classify(&self, _object: &PayoffObject) -> Category {
        Category::Unclassified
    }
}

/// Ceiling frozen into the bounded-payout candidate.
pub const BOUNDED_PAYOUT_CEILING: u64 = 2;

/// The candidate criteria, in corpus order.
#[must_use]
pub fn candidates() -> Vec<Box<dyn Criterion>> {
    vec![
        Box::new(BinaryPayout),
        Box::new(BoundedPayout {
            ceiling: BOUNDED_PAYOUT_CEILING,
        }),
        Box::new(DistinctValues),
        Box::new(CompleteSet),
        Box::new(SupportNonEmpty),
        Box::new(PrefundingExclusion),
        Box::new(ReferenceAndSettlement),
        Box::new(ConstantLabel),
    ]
}

/// Name of the criterion the memo proposes, which the tests hold to invariance.
pub const CONTROL: &str = "reference-and-settlement";

/// Name of the degenerate control, which the tests hold to invariance and to
/// emitting exactly one label.
pub const DEGENERATE_CONTROL: &str = "constant-label";

/// Every fact profile in the corpus, in corpus order.
#[must_use]
pub fn fact_profiles() -> Vec<Facts> {
    let mut profiles = Vec::new();
    for reference in ReferenceKind::ALL {
        for funded in [true, false] {
            for transferable in [true, false] {
                profiles.push(Facts::new(reference, funded, transferable));
            }
        }
    }
    profiles
}
