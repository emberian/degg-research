//! What an owner can actually detect. This module is the honest core.
//!
//! An owner holds four things: its own plaintext order, the sealed payload it
//! handed over, an [`InclusionReceipt`] against the published cutoff root, and
//! whatever the executor delivered back. From those, and from the public
//! outcome, it runs the battery below. Every check here is **necessary**: it
//! fires only on an outcome that no honest evaluation of any admitted multiset
//! containing this owner's order could have produced. None of them is
//! sufficient, and the gap between the two is the trust the Shielded baseline
//! still asks for.
//!
//! ## The battery
//!
//! 1. **Root binding.** The published receipt must be bound to the same cutoff
//!    root the owner's inclusion receipt verifies against. A run against any
//!    other admitted set is a mismatch, and
//!    [`degg_inclusion_availability::lifecycle::AbortClass::ResultUnbound`]
//!    already types the consequence.
//! 2. **Commitment opening.** The record committed at the owner's position
//!    must be the commitment of the owner's own sealed payload, under the
//!    owner's own derived log nullifier.
//! 3. **Delivery opening.** The entry at the owner's position must open under
//!    the receipt's delivery root, at that position and no other, because the
//!    verifier derives the position from the proof.
//! 4. **Omission.** On a settled run, an entry of
//!    [`crate::receipt::DeliveryEntry::NoLocalOutput`] at a position whose
//!    committed record is not the deterministic padding record is a
//!    contradiction, and a transferable one:
//!    [`crate::dispute::OmissionProof`].
//! 5. **Local arithmetic.** The delivered slot outcome must be exactly
//!    [`crate::receipt::SlotOutcome::derive`] of the owner's own plaintext at
//!    the public price and the committed fill. The executor has no free
//!    parameter in it except the fill.
//! 6. **Fill admissibility.** The fill cannot exceed the order's quantity, or
//!    the public volume; a no-trade result admits no fill; a fill at a tick
//!    the order's limit does not reach is impossible.
//! 7. **Pro-rata feasibility.** [`pro_rata_feasible`] asks whether *any* side
//!    total in the module's bounded range makes the committed fill a
//!    largest-remainder share of the public volume. This is the only check
//!    that constrains the executor's arithmetic from outside the owner's own
//!    row.
//! 8. **Aggregate consistency.** The delivered owner output must equal the sum
//!    of the owner's own committed slot outcomes, and must claim no position
//!    the owner did not submit.
//!
//! ## What the battery cannot do
//!
//! - It cannot see another owner's row. An executor that substitutes or drops
//!   an order belonging to an owner who does not check is invisible to
//!   everyone else, because the public outcome is only a tick and a volume.
//! - It cannot check that the plaintext the executor evaluated is the one the
//!   commitment opens to. Only the owner and the executor can open a payload,
//!   and the owner sees the substitution only through its effect on its own
//!   row. A substitution that changes nothing the owner can measure is not
//!   detected.
//! - It cannot check a refusal. [`attribute_refusal`] lets an owner ask
//!   whether the published class is one *its own* order genuinely violates,
//!   which is a real answer for that owner and no answer at all about the
//!   batch: a fabricated per-slot refusal blaming a position the owner does
//!   not hold is indistinguishable from a genuine one.
//! - It establishes no attribution. Nothing signs, so every finding is "these
//!   published objects contradict each other", never "this party lied".

use degg_inclusion_availability::log::{
    CutoffRoot, InclusionReceipt, ReceiptDefect, verify_receipt,
};
use degg_relation_ir::batch::{
    BatchInput, BoundaryStatements, OrderWitness, RequestedMode, Side, SlotInput,
};
use degg_relation_ir::lower::{
    ClearedTick, LoweringTarget, Outcome, OwnerOutput, PublicOutcome, lower,
};
use degg_relation_ir::module::RelationModule;
use degg_relation_ir::receipt::ReceiptStatus;
use degg_relation_ir::refusal::RefusalClass;

use crate::receipt::{
    DeliveryEntry, DeliveryOpening, OpeningDefect, ReceiptRejection, ShieldedReceipt, SlotOutcome,
    verify_opening,
};
use crate::seal::PlainOrder;
use crate::submit::Submission;

/// One position the owner holds, with everything the owner knows about it.
#[derive(Clone, Debug)]
pub struct OwnedPosition<'a> {
    /// The owner's own submission.
    pub submission: &'a Submission,
    /// Its inclusion receipt against the published cutoff root.
    pub inclusion: &'a InclusionReceipt,
    /// The delivery opening the executor supplied for this position.
    pub opening: Option<&'a DeliveryOpening>,
}

impl OwnedPosition<'_> {
    /// The committed position.
    #[must_use]
    pub fn seq(&self) -> u32 {
        self.inclusion.record.seq
    }
}

/// Everything one owner brings to an audit.
#[derive(Clone, Debug)]
pub struct OwnerEvidence<'a> {
    /// The owner index.
    pub owner: u8,
    /// The owner's committed positions.
    pub positions: Vec<OwnedPosition<'a>>,
    /// The published cutoff root the owner observed.
    pub cutoff: &'a CutoffRoot,
    /// The owner's delivered local output, once opened; `None` if nothing was
    /// delivered or the delivery key did not open it.
    pub delivered: Option<&'a OwnerOutput>,
}

/// One thing an owner found wrong.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Finding {
    /// The published receipt failed the public check.
    PublicCheck(
        /// The rejection class.
        ReceiptRejection,
    ),
    /// The owner's inclusion receipt does not verify against the cutoff root.
    InclusionReceiptInvalid {
        /// The position claimed.
        seq: u32,
        /// Why it failed.
        defect: ReceiptDefect,
    },
    /// The committed record does not commit the owner's own sealed payload.
    PayloadCommitmentMismatch {
        /// The position.
        seq: u32,
    },
    /// The committed record does not carry the owner's own log nullifier.
    NullifierMismatch {
        /// The position.
        seq: u32,
    },
    /// No delivery opening was supplied for a committed position.
    DeliveryOpeningMissing {
        /// The position.
        seq: u32,
    },
    /// The delivery opening does not verify under the receipt's delivery root.
    DeliveryOpeningInvalid {
        /// The position.
        seq: u32,
        /// Why it failed.
        defect: OpeningDefect,
    },
    /// A settled run committed no local output at a non-padding position the
    /// owner holds. Transferable: see [`crate::dispute::OmissionProof`].
    OmittedFromComputation {
        /// The position.
        seq: u32,
    },
    /// The committed fill exceeds the order's quantity.
    FillExceedsQuantity {
        /// The position.
        seq: u32,
        /// Fill committed.
        fill: u64,
        /// Quantity submitted.
        quantity: u64,
    },
    /// The committed fill exceeds the public volume.
    FillExceedsVolume {
        /// The position.
        seq: u32,
        /// Fill committed.
        fill: u64,
        /// Public volume.
        volume: u64,
    },
    /// A no-trade result committed a nonzero fill.
    FilledUnderNoTrade {
        /// The position.
        seq: u32,
        /// Fill committed.
        fill: u64,
    },
    /// The order's limit does not reach the cleared tick, yet it was filled.
    FilledWhileIneligible {
        /// The position.
        seq: u32,
        /// The cleared tick.
        tick: u8,
    },
    /// No admissible side total makes this fill a largest-remainder share of
    /// the public volume.
    ProRataInfeasible {
        /// The position.
        seq: u32,
        /// Fill committed.
        fill: u64,
    },
    /// The committed slot outcome is not the one this owner's plaintext, the
    /// public price, and the committed fill determine.
    SlotOutcomeMismatch {
        /// The position.
        seq: u32,
    },
    /// Nothing was delivered for a settled run.
    LocalOutputMissing,
    /// The delivered local output belongs to another owner.
    LocalOutputWrongOwner {
        /// Owner the delivery claims.
        claimed: u8,
    },
    /// The delivered local output claims a position the owner did not submit.
    UnexpectedOwnedPosition {
        /// The position claimed.
        seq: u32,
    },
    /// The delivered local output omits a position the owner did submit.
    MissingOwnedPosition {
        /// The position.
        seq: u32,
    },
    /// The delivered local output disagrees with the committed slot outcomes.
    AggregateMismatch,
}

/// Whether an owner can attribute a published refusal class.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefusalAttribution {
    /// One of the owner's own positions genuinely violates the published class.
    OwnPositionViolates {
        /// The position.
        seq: u32,
    },
    /// The class is a batch-level class. The owner has no witness either way.
    BatchLevelUncheckable,
    /// The class is a per-slot class no position of the owner violates. It may
    /// be another owner's genuine fault or a fabrication; the owner cannot
    /// distinguish the two, and neither can the public.
    NotAttributable,
}

/// One owner's audit of one published run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuditReport {
    /// Everything the battery found wrong, in check order.
    pub findings: Vec<Finding>,
    /// On a refused run, how far the owner can attribute the class.
    pub refusal: Option<RefusalAttribution>,
}

impl AuditReport {
    /// Whether the battery found nothing.
    #[must_use]
    pub fn is_consistent(&self) -> bool {
        self.findings.is_empty()
    }
}

/// Whether any admissible side total makes `fill` a largest-remainder share.
///
/// Section 7 of `DARK_FBA_RELATION.md` fixes `base_i = floor(q_i * V / T)` with
/// at most one residual atom, where `T` is the total eligible quantity on the
/// order's side. An owner does not know `T`, but it knows `T >= q_i` (its own
/// order is in the total), `T >= V` (the volume is a minimum over the two
/// sides), and `T <= max_side_total` (the module's slot capacity times its
/// quantity ceiling). Quantifying over that bounded range is exact and cheap:
/// the range has at most `max_side_total` values.
#[must_use]
pub fn pro_rata_feasible(quantity: u64, volume: u64, fill: u64, max_side_total: u64) -> bool {
    if volume == 0 {
        return fill == 0;
    }
    if fill > quantity || fill > volume {
        return false;
    }
    let lower = quantity.max(volume);
    if lower > max_side_total {
        return false;
    }
    (lower..=max_side_total).any(|total| {
        let Some(product) = quantity.checked_mul(volume) else {
            return false;
        };
        let base = product / total;
        fill == base || (fill == base + 1 && base < quantity)
    })
}

fn eligible_at(side: Side, limit_tick: u8, tick: u8) -> bool {
    match side {
        Side::Buy => limit_tick >= tick,
        Side::Sell => limit_tick <= tick,
    }
}

/// Ask whether one of the owner's own orders genuinely violates `class`.
///
/// The owner runs the *public* relation module through the same Clear lowering
/// on a probe batch holding only its own order, with every boundary statement
/// and every executor-supplied statement satisfied. The module is public data,
/// so this reuses the exact frozen check order of section 4.1 rather than
/// restating it.
#[must_use]
pub fn attribute_refusal(
    module: &RelationModule,
    cutoff: &CutoffRoot,
    positions: &[OwnedPosition<'_>],
    class: RefusalClass,
) -> RefusalAttribution {
    let batch_level = matches!(
        class,
        RefusalClass::DarkTargetUnavailable
            | RefusalClass::AdmissionLogNotFinal
            | RefusalClass::RootBindingAbsent
            | RefusalClass::RootEquivocation
            | RefusalClass::PayloadUnavailable
            | RefusalClass::AccumulatorOverflow
            | RefusalClass::InternalInvariant
            | RefusalClass::MalformedEncoding
    );
    if batch_level {
        return RefusalAttribution::BatchLevelUncheckable;
    }
    let Ok(evaluator) = lower(module, LoweringTarget::Clear) else {
        return RefusalAttribution::NotAttributable;
    };
    for position in positions {
        let plain = position.submission.plain;
        let mut slots = vec![SlotInput::Empty; usize::from(module.params.slots)];
        slots[0] = SlotInput::Occupied(OrderWitness {
            batch_id: plain.batch_id,
            market_id: plain.market_id,
            owner: plain.owner,
            side: plain.side,
            limit_tick: plain.limit_tick,
            quantity: plain.quantity,
            reserved: plain.reserved,
            nullifier: plain.nullifier,
            arrived_at: plain.arrived_at,
            authorized: true,
            eligible: true,
            included_under_root: true,
            custody_bound: true,
        });
        let probe = BatchInput {
            requested_mode: RequestedMode::ShieldedSingleExecutor,
            batch_id: cutoff.domain.batch,
            market_id: cutoff.domain.market,
            cutoff: cutoff.domain.cutoff_epoch,
            accepted_input_root: cutoff.root,
            boundary: BoundaryStatements::SATISFIED,
            slots,
        };
        if let Outcome::Refused(refused) = evaluator.evaluate(&probe)
            && refused.refusal.class == class
        {
            return RefusalAttribution::OwnPositionViolates {
                seq: position.seq(),
            };
        }
    }
    RefusalAttribution::NotAttributable
}

/// Run one owner's check battery over one published run.
pub fn audit(
    module: &RelationModule,
    evidence: &OwnerEvidence<'_>,
    receipt: &ShieldedReceipt,
    public: &PublicOutcome,
) -> AuditReport {
    let mut findings = Vec::new();
    let domain_digest = evidence.cutoff.domain.digest();
    let max_side_total = u64::from(module.params.slots) * module.params.quantity_ceiling;

    if let Err(rejection) = crate::receipt::public_check(
        receipt,
        evidence.cutoff,
        &degg_relation_ir::canon::Canonical::digest(module),
        public,
    ) {
        findings.push(Finding::PublicCheck(rejection));
    }

    let settled = match public {
        PublicOutcome::Settled(result) => Some(result),
        PublicOutcome::Refused(_) => None,
    };
    let price = settled.and_then(|result| match result.tick {
        ClearedTick::NoTrade => Some(0),
        ClearedTick::Tick(tick) => module.params.tick_prices.get(usize::from(tick)).copied(),
    });

    let mut own_outcomes: Vec<SlotOutcome> = Vec::new();
    for position in &evidence.positions {
        let seq = position.seq();
        let plain = position.submission.plain;
        if let Err(defect) = verify_receipt(evidence.cutoff, position.inclusion) {
            findings.push(Finding::InclusionReceiptInvalid { seq, defect });
        }
        if position.inclusion.record.payload_commitment != position.submission.sealed.commitment() {
            findings.push(Finding::PayloadCommitmentMismatch { seq });
        }
        if position.inclusion.record.nullifier != plain.log_nullifier(&domain_digest) {
            findings.push(Finding::NullifierMismatch { seq });
        }
        let Some(opening) = position.opening else {
            findings.push(Finding::DeliveryOpeningMissing { seq });
            continue;
        };
        if let Err(defect) = verify_opening(receipt, opening) {
            findings.push(Finding::DeliveryOpeningInvalid { seq, defect });
            continue;
        }
        if opening.seq != seq {
            findings.push(Finding::DeliveryOpeningInvalid {
                seq,
                defect: OpeningDefect::PositionMismatch {
                    claimed: opening.seq,
                    derived: u64::from(seq),
                },
            });
            continue;
        }
        let Some(result) = settled else {
            continue;
        };
        let DeliveryEntry::Produced(outcome) = opening.entry else {
            findings.push(Finding::OmittedFromComputation { seq });
            continue;
        };
        own_outcomes.push(outcome);
        if outcome.fill > plain.quantity {
            findings.push(Finding::FillExceedsQuantity {
                seq,
                fill: outcome.fill,
                quantity: plain.quantity,
            });
        }
        if outcome.fill > result.volume {
            findings.push(Finding::FillExceedsVolume {
                seq,
                fill: outcome.fill,
                volume: result.volume,
            });
        }
        match result.tick {
            ClearedTick::NoTrade => {
                if outcome.fill != 0 {
                    findings.push(Finding::FilledUnderNoTrade {
                        seq,
                        fill: outcome.fill,
                    });
                }
            }
            ClearedTick::Tick(tick) => {
                if outcome.fill > 0 && !eligible_at(plain.side, plain.limit_tick, tick) {
                    findings.push(Finding::FilledWhileIneligible { seq, tick });
                } else if eligible_at(plain.side, plain.limit_tick, tick)
                    && !pro_rata_feasible(
                        plain.quantity,
                        result.volume,
                        outcome.fill,
                        max_side_total,
                    )
                {
                    findings.push(Finding::ProRataInfeasible {
                        seq,
                        fill: outcome.fill,
                    });
                }
            }
        }
        let expected =
            price.and_then(|price| SlotOutcome::derive(seq, &plain, price, outcome.fill));
        if expected != Some(outcome) {
            findings.push(Finding::SlotOutcomeMismatch { seq });
        }
    }

    if settled.is_some() {
        findings.extend(check_delivered(evidence, &own_outcomes));
    }

    let refusal = match receipt.status {
        ReceiptStatus::Settled => None,
        ReceiptStatus::Refused(class) => Some(attribute_refusal(
            module,
            evidence.cutoff,
            &evidence.positions,
            class,
        )),
    };
    AuditReport { findings, refusal }
}

fn check_delivered(evidence: &OwnerEvidence<'_>, own: &[SlotOutcome]) -> Vec<Finding> {
    let mut findings = Vec::new();
    let Some(delivered) = evidence.delivered else {
        findings.push(Finding::LocalOutputMissing);
        return findings;
    };
    if delivered.owner != evidence.owner {
        findings.push(Finding::LocalOutputWrongOwner {
            claimed: delivered.owner,
        });
    }
    let own_seqs: Vec<u32> = evidence.positions.iter().map(OwnedPosition::seq).collect();
    for (index, fill) in delivered.owned_slot_fills.iter().enumerate() {
        let seq = u32::try_from(index).unwrap_or(u32::MAX);
        match (fill, own_seqs.contains(&seq)) {
            (Some(_), false) => findings.push(Finding::UnexpectedOwnedPosition { seq }),
            (None, true) => findings.push(Finding::MissingOwnedPosition { seq }),
            _ => {}
        }
    }
    let mut expected = OwnerOutput {
        owner: evidence.owner,
        bought: 0,
        sold: 0,
        base_delta: 0,
        quote_delta: 0,
        released_base_reservation: 0,
        released_quote_reservation: 0,
        owned_slot_fills: vec![None; delivered.owned_slot_fills.len()],
    };
    for outcome in own {
        let index = usize::try_from(outcome.seq).unwrap_or(usize::MAX);
        if index >= expected.owned_slot_fills.len() {
            continue;
        }
        expected.owned_slot_fills[index] = Some(outcome.fill);
        match outcome.side {
            Side::Buy => expected.bought += outcome.fill,
            Side::Sell => expected.sold += outcome.fill,
        }
        expected.base_delta += outcome.base_delta;
        expected.quote_delta += outcome.quote_delta;
        expected.released_base_reservation += outcome.released_base;
        expected.released_quote_reservation += outcome.released_quote;
    }
    if expected != *delivered {
        findings.push(Finding::AggregateMismatch);
    }
    findings
}

/// The plaintext an owner audits with, for callers that hold it loose.
#[must_use]
pub fn own_plaintext(position: &OwnedPosition<'_>) -> PlainOrder {
    position.submission.plain
}
