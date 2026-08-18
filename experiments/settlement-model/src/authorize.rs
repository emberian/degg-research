//! Settlement authorization: the owner's right to one position's payout,
//! derived from published objects and re-derived arithmetic, never from the
//! executor's word.
//!
//! An authorization is the conjunction the specification names: the published
//! computation receipt (checked, settled, and bound to the observed cutoff
//! root and phase), the position's inclusion receipt under that root, the
//! delivery opening of the same position under the receipt's delivery root,
//! and a custody entry whose owner, asset, and amount agree exactly with the
//! local effect the opening commits. The settlement relation trusts none of
//! the numbers it is handed: every delta is recomputed from the side, the
//! fill, and the public price, and the reservation the effect implies must
//! equal the reservation custody actually holds.
//!
//! What authorization deliberately does **not** check is that the published
//! result is correct. `SHIELDED_BASELINE.md` section 6.2 measures the
//! executor's freedom in the public result, and nothing here shrinks it: a
//! wrong-but-consistent result authorizes wrong-but-conserving settlements.
//! `tests/residual_inheritance.rs` demonstrates that inheritance rather than
//! hiding it.
//!
//! The check order below is frozen, documented on [`authorize`], and pinned
//! by `tests/authorization.rs`, for the same reason the relation froze its
//! admission order in `DARK_FBA_RELATION.md` section 4.1: a typed refusal is
//! an observable, and two conforming implementations must not disagree about
//! which one a multiply-defective claim receives.

use degg_inclusion_availability::hash::tagged;
use degg_inclusion_availability::lifecycle::Phase;
use degg_inclusion_availability::log::{
    CutoffRoot, InclusionReceipt, ReceiptDefect, verify_receipt,
};
use degg_relation_ir::batch::Side;
use degg_relation_ir::canon::Canonical;
use degg_relation_ir::lower::{ClearedTick, PublicOutcome};
use degg_relation_ir::module::RelationModule;
use degg_shielded_baseline::receipt::{
    DeliveryEntry, DeliveryOpening, OpeningDefect, ReceiptRejection, ShieldedReceipt,
    delivery_domain, public_check,
};

use crate::custody::{Asset, CustodyLedger, CustodyState};

/// Tag for the settlement nullifier of one delivery position.
pub const SETTLE_NULLIFIER_TAG: &[u8] = b"degg/settlement-model/v0/settlement-nullifier";
/// Tag for the execution digest of one settlement instruction.
pub const EXECUTION_TAG: &[u8] = b"degg/settlement-model/v0/execution";

/// Everything a settlement claim is checked against. All of it is either
/// published or owner-held; the executor is not consulted.
#[derive(Clone, Copy, Debug)]
pub struct SettlementInputs<'a> {
    /// The published computation receipt.
    pub receipt: &'a ShieldedReceipt,
    /// The observed cutoff root.
    pub cutoff: &'a CutoffRoot,
    /// The published public outcome.
    pub outcome: &'a PublicOutcome,
    /// The lifecycle phase the relying party's own machine reached.
    pub phase: Phase,
    /// The frozen relation module.
    pub module: &'a RelationModule,
    /// The position's inclusion receipt.
    pub inclusion: &'a InclusionReceipt,
    /// The position's delivery opening.
    pub opening: &'a DeliveryOpening,
}

/// Why a settlement claim was refused authorization.
///
/// The order of these variants is the frozen check order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthorizationDefect {
    /// The published receipt fails the public check against the cutoff root,
    /// module, and outcome.
    Receipt(
        /// The rejection.
        ReceiptRejection,
    ),
    /// The relation publicly refused the batch. A refusal is a complete
    /// public answer with no allocation, so nothing settles; the funds path
    /// is the refund lane.
    BatchRefusedNoAllocation {
        /// The relation's own public refusal-class code.
        class_code: u32,
    },
    /// The lifecycle machine did not reach `Settled`.
    PhaseNotSettled {
        /// The phase it did reach.
        phase: Phase,
    },
    /// The settled phase's result digest is not this receipt's outcome.
    PhaseResultMismatch,
    /// The inclusion receipt does not verify against the cutoff root.
    Inclusion(
        /// The defect.
        ReceiptDefect,
    ),
    /// The inclusion receipt and the delivery opening name different
    /// positions.
    PositionMismatch {
        /// Position the admitted record holds.
        record_seq: u32,
        /// Position the opening claims.
        opening_seq: u32,
    },
    /// The position is a padding record. Padding escrows nothing and settles
    /// nothing.
    PaddingPositionSettlesNothing {
        /// The position.
        seq: u32,
    },
    /// The delivery opening does not verify against the receipt.
    Opening(
        /// The defect.
        OpeningDefect,
    ),
    /// A settled batch committed no local output at an occupied position.
    ///
    /// This is an omission: the owner's recourse is the transferable
    /// [`degg_shielded_baseline::dispute::OmissionProof`], not a settlement.
    /// This relation has no adjudication rule that consumes the verdict; the
    /// reservation stays obligated, which the specification records as an
    /// open item.
    NoLocalOutputAtPosition {
        /// The position.
        seq: u32,
    },
    /// The committed local effect names a different position than the opening.
    OutcomeSequenceMismatch {
        /// Position inside the committed effect.
        entry_seq: u32,
        /// Position the opening proves.
        opening_seq: u32,
    },
    /// The committed local effect names an owner outside the relation.
    OwnerOutOfDomain {
        /// The owner claimed.
        owner: u8,
    },
    /// The published result selects a tick outside the frozen grid.
    TickOutOfRange {
        /// The tick claimed.
        tick: u8,
    },
    /// A positive fill was committed under a no-trade result.
    FillPositiveAtNoTrade {
        /// The fill.
        fill: u64,
    },
    /// The committed fill exceeds the published aggregate volume.
    FillExceedsVolume {
        /// The fill.
        fill: u64,
        /// The published volume.
        volume: u64,
    },
    /// Exact re-derivation overflowed. Nothing about the claim is trusted
    /// past this point.
    ArithmeticOverflow,
    /// The committed deltas or released amounts are not what the side, fill,
    /// and public price imply.
    EffectInconsistent {
        /// The position.
        seq: u32,
    },
    /// Nothing is escrowed under the record's nullifier.
    NotEscrowed,
    /// The escrow belongs to a different owner than the committed effect.
    EscrowOwnerMismatch {
        /// Owner custody holds the escrow for.
        escrowed: u8,
        /// Owner the effect names.
        claimed: u8,
    },
    /// The escrow is in the wrong asset for the committed side.
    EscrowAssetMismatch,
    /// The escrow amount is not the reservation the committed effect implies.
    EscrowAmountMismatch {
        /// Amount custody holds.
        escrowed: u64,
        /// Amount the effect implies.
        implied: u64,
    },
    /// The position was already refunded; a refunded reservation cannot
    /// settle. Unreachable when the phase gates are honored, and typed
    /// anyway.
    RefundedPositionCannotSettle,
}

/// One authorized settlement instruction, fully determined by published
/// objects.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SettlementAuthorization {
    /// The committed position.
    pub seq: u32,
    /// The owner account paid.
    pub owner: u8,
    /// The side of the settled order.
    pub side: Side,
    /// The admission nullifier the escrow is keyed by.
    pub admission_nullifier: [u8; 32],
    /// The settlement nullifier: one settlement per delivery position.
    pub settlement_nullifier: [u8; 32],
    /// Binding of the computation receipt this settlement executes.
    pub receipt_binding: [u8; 32],
    /// Content-addressed identity of this exact instruction.
    pub execution: [u8; 32],
    /// The part of the reservation the trade consumed.
    pub escrow_consumed: u64,
    /// The unspent remainder of the reservation, returned to the owner.
    pub released: (Asset, u64),
    /// The acquired leg of the trade, paid from the pool.
    pub acquired: (Asset, u64),
}

fn signed(value: u64) -> Result<i64, AuthorizationDefect> {
    i64::try_from(value).map_err(|_| AuthorizationDefect::ArithmeticOverflow)
}

/// Authorize one position's settlement, in the frozen check order.
///
/// 1. the receipt passes the public check against root, module, and outcome;
/// 2. the outcome is settled, not a public refusal;
/// 3. the phase is `Settled` and names this receipt's outcome digest;
/// 4. the inclusion receipt verifies, 5. names the opening's position, and
///    6. is not padding;
/// 7. the delivery opening verifies, 8. commits a produced effect, and
///    9. that effect names its own position and 10. an in-domain owner;
/// 11. the published tick is on the grid; 12. the fill respects the published
///     volume and the no-trade rule;
/// 13. the deltas and released amounts are exactly re-derived from side,
///     fill, and public price, and the implied reservation is computed;
/// 14. custody holds that exact reservation, in the side's asset, for the
///     effect's owner, under the record's nullifier, and it is not refunded.
///
/// Nothing here checks that the published result is *correct*; see the module
/// documentation.
pub fn authorize(
    inputs: &SettlementInputs<'_>,
    custody: &CustodyLedger,
) -> Result<SettlementAuthorization, AuthorizationDefect> {
    let module_digest = inputs.module.digest();
    public_check(
        inputs.receipt,
        inputs.cutoff,
        &module_digest,
        inputs.outcome,
    )
    .map_err(AuthorizationDefect::Receipt)?;

    let result = match inputs.outcome {
        PublicOutcome::Settled(result) => result,
        PublicOutcome::Refused(refusal) => {
            return Err(AuthorizationDefect::BatchRefusedNoAllocation {
                class_code: refusal.class.code(),
            });
        }
    };

    let result_digest = match inputs.phase {
        Phase::Settled { result_digest } => result_digest,
        phase => return Err(AuthorizationDefect::PhaseNotSettled { phase }),
    };
    if result_digest != inputs.receipt.outcome_digest {
        return Err(AuthorizationDefect::PhaseResultMismatch);
    }

    verify_receipt(inputs.cutoff, inputs.inclusion).map_err(AuthorizationDefect::Inclusion)?;
    let record = &inputs.inclusion.record;
    if record.seq != inputs.opening.seq {
        return Err(AuthorizationDefect::PositionMismatch {
            record_seq: record.seq,
            opening_seq: inputs.opening.seq,
        });
    }
    if record.is_padding(&inputs.cutoff.domain) {
        return Err(AuthorizationDefect::PaddingPositionSettlesNothing { seq: record.seq });
    }

    degg_shielded_baseline::receipt::verify_opening(inputs.receipt, inputs.opening)
        .map_err(AuthorizationDefect::Opening)?;
    let effect = match inputs.opening.entry {
        DeliveryEntry::NoLocalOutput => {
            return Err(AuthorizationDefect::NoLocalOutputAtPosition {
                seq: inputs.opening.seq,
            });
        }
        DeliveryEntry::Produced(effect) => effect,
    };
    if effect.seq != inputs.opening.seq {
        return Err(AuthorizationDefect::OutcomeSequenceMismatch {
            entry_seq: effect.seq,
            opening_seq: inputs.opening.seq,
        });
    }
    if effect.owner >= inputs.module.params.owners {
        return Err(AuthorizationDefect::OwnerOutOfDomain {
            owner: effect.owner,
        });
    }

    let price = match result.tick {
        ClearedTick::NoTrade => 0,
        ClearedTick::Tick(tick) => *inputs
            .module
            .params
            .tick_prices
            .get(usize::from(tick))
            .ok_or(AuthorizationDefect::TickOutOfRange { tick })?,
    };
    match result.tick {
        ClearedTick::NoTrade => {
            if effect.fill != 0 {
                return Err(AuthorizationDefect::FillPositiveAtNoTrade { fill: effect.fill });
            }
        }
        ClearedTick::Tick(_) => {
            if effect.fill > result.volume {
                return Err(AuthorizationDefect::FillExceedsVolume {
                    fill: effect.fill,
                    volume: result.volume,
                });
            }
        }
    }

    let spend = effect
        .fill
        .checked_mul(price)
        .ok_or(AuthorizationDefect::ArithmeticOverflow)?;
    let fill_signed = signed(effect.fill)?;
    let spend_signed = signed(spend)?;
    let (expected_base_delta, expected_quote_delta, implied_reserved, spending_asset) =
        match effect.side {
            Side::Buy => {
                if effect.released_base != 0 {
                    return Err(AuthorizationDefect::EffectInconsistent { seq: effect.seq });
                }
                let implied = effect
                    .released_quote
                    .checked_add(spend)
                    .ok_or(AuthorizationDefect::ArithmeticOverflow)?;
                (fill_signed, -spend_signed, implied, Asset::Quote)
            }
            Side::Sell => {
                if effect.released_quote != 0 {
                    return Err(AuthorizationDefect::EffectInconsistent { seq: effect.seq });
                }
                let implied = effect
                    .released_base
                    .checked_add(effect.fill)
                    .ok_or(AuthorizationDefect::ArithmeticOverflow)?;
                (-fill_signed, spend_signed, implied, Asset::Base)
            }
        };
    if effect.base_delta != expected_base_delta || effect.quote_delta != expected_quote_delta {
        return Err(AuthorizationDefect::EffectInconsistent { seq: effect.seq });
    }

    let entry = custody
        .entry(&record.nullifier)
        .ok_or(AuthorizationDefect::NotEscrowed)?;
    if entry.owner != effect.owner {
        return Err(AuthorizationDefect::EscrowOwnerMismatch {
            escrowed: entry.owner,
            claimed: effect.owner,
        });
    }
    if entry.asset != spending_asset {
        return Err(AuthorizationDefect::EscrowAssetMismatch);
    }
    if entry.amount != implied_reserved {
        return Err(AuthorizationDefect::EscrowAmountMismatch {
            escrowed: entry.amount,
            implied: implied_reserved,
        });
    }
    if entry.state == CustodyState::Refunded {
        return Err(AuthorizationDefect::RefundedPositionCannotSettle);
    }

    let (released, acquired, escrow_consumed) = match effect.side {
        Side::Buy => (
            (Asset::Quote, effect.released_quote),
            (Asset::Base, effect.fill),
            spend,
        ),
        Side::Sell => (
            (Asset::Base, effect.released_base),
            (Asset::Quote, spend),
            effect.fill,
        ),
    };

    let domain = delivery_domain(&inputs.receipt.cutoff, &inputs.receipt.module_digest);
    let settlement_nullifier = tagged(
        SETTLE_NULLIFIER_TAG,
        &[&domain, &record.nullifier, &record.seq.to_be_bytes()],
    );
    let execution = tagged(
        EXECUTION_TAG,
        &[
            &inputs.receipt.binding,
            &record.seq.to_be_bytes(),
            &effect.digest(),
        ],
    );

    Ok(SettlementAuthorization {
        seq: record.seq,
        owner: effect.owner,
        side: effect.side,
        admission_nullifier: record.nullifier,
        settlement_nullifier,
        receipt_binding: inputs.receipt.binding,
        execution,
        escrow_consumed,
        released,
        acquired,
    })
}
