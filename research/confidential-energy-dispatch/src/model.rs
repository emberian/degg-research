//! Frozen input domains for `confidential-energy-dispatch/p3-t3-b2-q4/v0`.

/// Relation identifier. Any semantic change requires a new identifier.
pub const RELATION_ID: &str = "confidential-energy-dispatch/p3-t3-b2-q4/v0";
/// Exactly three padded provider slots.
pub const PROVIDERS: usize = 3;
/// Exactly three planning periods.
pub const PERIODS: usize = 3;
/// Two buses joined by one lossless constrained line.
pub const BUSES: usize = 2;
/// Two convex marginal-cost segments per occupied provider.
pub const SEGMENTS: usize = 2;
/// Maximum generation in one provider-period, in exact energy atoms.
pub const MAX_OUTPUT: u8 = 4;
/// Maximum allowed marginal cost, in exact quote atoms per energy atom.
pub const MAX_MARGINAL_COST: u32 = 1_000_000;
/// Maximum aggregate energy at this frozen capacity.
pub const MAX_SYSTEM_OUTPUT: u8 = 12;

/// One piece of an occupied provider's exact cost curve.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CostSegment {
    /// Number of output atoms in this segment.
    pub width: u8,
    /// Quote atoms charged for each output atom in this segment.
    pub marginal_cost: u32,
}

/// One private provider slot.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ProviderInput {
    /// Whether the padded slot is occupied.
    pub occupied: bool,
    /// Private binding used to address the provider's authorized local output.
    pub owner_binding: [u8; 32],
    /// Bus 0 or 1.
    pub bus: u8,
    /// Minimum nonzero output. Zero means off, not a violation of this minimum.
    pub min_output: u8,
    /// Maximum output.
    pub max_output: u8,
    /// Maximum increase between consecutive online states.
    pub ramp_up: u8,
    /// Maximum decrease between consecutive online states.
    pub ramp_down: u8,
    /// Output immediately before the planning horizon.
    pub initial_output: u8,
    /// Forced availability. An unavailable period has output zero and the
    /// outage boundary bypasses the ordinary ramp constraint.
    pub available: [bool; PERIODS],
    /// Sequential-fill, nondecreasing marginal-cost segments. Widths sum to
    /// `max_output` exactly.
    pub segments: [CostSegment; SEGMENTS],
}

impl ProviderInput {
    /// Canonical all-zero padding slot.
    #[must_use]
    pub const fn padding() -> Self {
        Self {
            occupied: false,
            owner_binding: [0; 32],
            bus: 0,
            min_output: 0,
            max_output: 0,
            ramp_up: 0,
            ramp_down: 0,
            initial_output: 0,
            available: [false; PERIODS],
            segments: [CostSegment {
                width: 0,
                marginal_cost: 0,
            }; SEGMENTS],
        }
    }
}

/// Public physical policy and replay domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PublicDomain {
    /// Nonzero market or coordination instance binding.
    pub instance: [u8; 32],
    /// Public coarse planning epoch; no submission or internal timing is part
    /// of the relation.
    pub epoch: u64,
    /// Exact energy demand by period and bus.
    pub demand: [[u8; BUSES]; PERIODS],
    /// Required system-wide upward reserve by period.
    pub reserve: [u8; PERIODS],
    /// Absolute capacity of the lossless line joining buses 0 and 1.
    pub line_limit: [u8; PERIODS],
}

/// Public request. The accepted-input commitment is domain-bound and must be
/// finalized before execution begins.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DispatchRequest {
    /// Physical and replay domain.
    pub domain: PublicDomain,
    /// Commitment to all three canonical private provider slots.
    pub accepted_inputs: [u8; 32],
    /// External admission finalized exactly one commitment.
    pub admission_final: bool,
    /// Every accepted fixed-size payload is available to computation.
    pub payloads_available: bool,
}

/// The fixed-size private witness.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Witness {
    /// Exactly three provider or canonical padding slots.
    pub providers: [ProviderInput; PROVIDERS],
}

/// Public-domain validation defect.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DomainDefect {
    /// The replay-domain instance is zero.
    ZeroInstance,
    /// Per-bus or aggregate demand exceeds the frozen bounded domain.
    DemandOutOfDomain { period: u8 },
    /// Reserve exceeds total nameplate output.
    ReserveOutOfDomain { period: u8 },
    /// Line capacity exceeds total nameplate output.
    LineLimitOutOfDomain { period: u8 },
}

impl PublicDomain {
    /// Validate public dimensions and arithmetic bounds.
    ///
    /// # Errors
    ///
    /// Returns the first public field outside the frozen bounded domain.
    ///
    /// # Panics
    ///
    /// Panics only if the compile-time `PERIODS` constant exceeds `u8`, which
    /// would already violate the frozen relation identifier.
    pub fn validate(&self) -> Result<(), DomainDefect> {
        if self.instance == [0; 32] {
            return Err(DomainDefect::ZeroInstance);
        }
        for period in 0..PERIODS {
            let total = self.demand[period][0]
                .checked_add(self.demand[period][1])
                .ok_or(DomainDefect::DemandOutOfDomain {
                    period: u8::try_from(period).expect("PERIODS fits u8"),
                })?;
            if self.demand[period]
                .iter()
                .any(|value| *value > MAX_SYSTEM_OUTPUT)
                || total > MAX_SYSTEM_OUTPUT
            {
                return Err(DomainDefect::DemandOutOfDomain {
                    period: u8::try_from(period).expect("PERIODS fits u8"),
                });
            }
            if self.reserve[period] > MAX_SYSTEM_OUTPUT {
                return Err(DomainDefect::ReserveOutOfDomain {
                    period: u8::try_from(period).expect("PERIODS fits u8"),
                });
            }
            if self.line_limit[period] > MAX_SYSTEM_OUTPUT {
                return Err(DomainDefect::LineLimitOutOfDomain {
                    period: u8::try_from(period).expect("PERIODS fits u8"),
                });
            }
        }
        Ok(())
    }
}

/// Private-witness validation defect. This detailed class is never part of the
/// proposed public Dark frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WitnessDefect {
    /// An empty slot carried latent data.
    NonCanonicalPadding { slot: u8 },
    /// An occupied slot omitted its output-recipient binding.
    ZeroOwnerBinding { slot: u8 },
    /// Bus is not 0 or 1.
    BusOutOfDomain { slot: u8 },
    /// Capacity or minimum is inconsistent with the frozen output domain.
    CapacityOutOfDomain { slot: u8 },
    /// Initial output exceeds maximum output.
    InitialOutputOutOfDomain { slot: u8 },
    /// A ramp bound exceeds the frozen output domain.
    RampOutOfDomain { slot: u8 },
    /// Segment widths do not cover exactly `max_output`.
    SegmentWidthMismatch { slot: u8 },
    /// Marginal costs are not nondecreasing.
    NonConvexSegments { slot: u8 },
    /// A cost exceeds the frozen bound; rejection precedes multiplication.
    MarginalCostOutOfDomain { slot: u8 },
    /// Two occupied slots name the same local-output recipient.
    DuplicateOwnerBinding { first: u8, second: u8 },
}

impl Witness {
    /// Validate all provider slots in a fixed order.
    ///
    /// # Errors
    ///
    /// Returns the first provider defect in slot order, then checks duplicate
    /// output-recipient bindings in pair order.
    ///
    /// # Panics
    ///
    /// Panics only if the compile-time `PROVIDERS` constant exceeds `u8`, which
    /// would already violate the frozen relation identifier.
    pub fn validate(&self) -> Result<(), WitnessDefect> {
        for (slot, provider) in self.providers.iter().enumerate() {
            let slot = u8::try_from(slot).expect("PROVIDERS fits u8");
            if !provider.occupied {
                if *provider != ProviderInput::padding() {
                    return Err(WitnessDefect::NonCanonicalPadding { slot });
                }
                continue;
            }
            if provider.owner_binding == [0; 32] {
                return Err(WitnessDefect::ZeroOwnerBinding { slot });
            }
            if usize::from(provider.bus) >= BUSES {
                return Err(WitnessDefect::BusOutOfDomain { slot });
            }
            if provider.max_output == 0
                || provider.max_output > MAX_OUTPUT
                || provider.min_output > provider.max_output
            {
                return Err(WitnessDefect::CapacityOutOfDomain { slot });
            }
            if provider.initial_output > provider.max_output
                || (provider.initial_output != 0 && provider.initial_output < provider.min_output)
            {
                return Err(WitnessDefect::InitialOutputOutOfDomain { slot });
            }
            if provider.ramp_up > MAX_OUTPUT || provider.ramp_down > MAX_OUTPUT {
                return Err(WitnessDefect::RampOutOfDomain { slot });
            }
            let widths = provider.segments[0]
                .width
                .checked_add(provider.segments[1].width)
                .ok_or(WitnessDefect::SegmentWidthMismatch { slot })?;
            if widths != provider.max_output {
                return Err(WitnessDefect::SegmentWidthMismatch { slot });
            }
            if provider.segments[0].marginal_cost > provider.segments[1].marginal_cost {
                return Err(WitnessDefect::NonConvexSegments { slot });
            }
            if provider
                .segments
                .iter()
                .any(|segment| segment.marginal_cost > MAX_MARGINAL_COST)
            {
                return Err(WitnessDefect::MarginalCostOutOfDomain { slot });
            }
        }
        for second in 0..PROVIDERS {
            if !self.providers[second].occupied {
                continue;
            }
            for first in 0..second {
                if self.providers[first].occupied
                    && self.providers[first].owner_binding == self.providers[second].owner_binding
                {
                    return Err(WitnessDefect::DuplicateOwnerBinding {
                        first: u8::try_from(first).expect("PROVIDERS fits u8"),
                        second: u8::try_from(second).expect("PROVIDERS fits u8"),
                    });
                }
            }
        }
        Ok(())
    }
}
