//! The settlement relation: observe one settled receipt, execute authorized
//! instructions exactly once, and route terminal aborts to full refunds.
//!
//! Two disciplines carry the module.
//!
//! **One settlement per delivery position.** The spent set is keyed by the
//! settlement nullifier, which binds the delivery domain, the admission
//! nullifier, and the position. Replaying an instruction is answered with
//! [`Execution::AlreadyExecuted`] and moves nothing, which is what makes a
//! crashed settlement adapter safe to restart: retry until `Executed` or
//! `AlreadyExecuted`, and the funds moved exactly once either way.
//!
//! **A refusal and a settlement are mutually exclusive per reservation.** The
//! custody states enforce it locally, and the phase gates enforce it
//! globally: [`SettlementBook::observe_settled`] requires the settled phase,
//! [`refund`] requires a terminal abort, and one machine cannot be both.

use std::collections::BTreeMap;

use degg_inclusion_availability::lifecycle::Phase;
use degg_relation_ir::receipt::ReceiptStatus;
use degg_shielded_baseline::receipt::ShieldedReceipt;

use crate::authorize::SettlementAuthorization;
use crate::custody::{CustodyError, CustodyLedger};

/// One public claim event, as a transparent settlement surface would show it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClaimRow {
    /// The account credited.
    pub account: u8,
    /// The committed position the claim settles. Public on a transparent
    /// surface, which discloses the admission rank.
    pub seq: u32,
    /// Base atoms credited: the acquired leg for a buy, the released
    /// remainder for a sell.
    pub credited_base: u64,
    /// Quote atoms credited: the released remainder for a buy, the acquired
    /// leg for a sell.
    pub credited_quote: u64,
    /// The epoch of the claim.
    pub epoch: u64,
}

/// One executed settlement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SettlementExecution {
    /// The settlement nullifier spent.
    pub settlement_nullifier: [u8; 32],
    /// The execution digest recorded for it.
    pub execution: [u8; 32],
    /// The account credited.
    pub owner: u8,
    /// The position settled.
    pub seq: u32,
    /// Base atoms credited.
    pub credited_base: u64,
    /// Quote atoms credited.
    pub credited_quote: u64,
}

/// The outcome of submitting one settlement instruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Execution {
    /// The instruction executed and the payout moved.
    Executed(
        /// What was executed.
        SettlementExecution,
    ),
    /// The settlement nullifier was already spent by this same instruction.
    /// Nothing moved. This is the idempotent-retry answer, not an error.
    AlreadyExecuted {
        /// The execution digest recorded when it first ran.
        execution: [u8; 32],
    },
}

/// Why a settlement instruction was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettlementRefusal {
    /// No settled receipt has been observed, or the instruction is bound to a
    /// different receipt than the observed one.
    ReceiptNotObserved,
    /// The settlement nullifier was spent by a *different* instruction.
    ///
    /// Defence in depth: under one observed receipt binding, the execution
    /// digest is a function of that binding, the position, and the committed
    /// effect, and the delivery root inside the binding commits the effect at
    /// each position — so producing two different execution digests for one
    /// settlement nullifier requires a hash collision. The check is kept for
    /// the same reason upstream keeps `ReceiptDefect::SequenceMismatch`: a
    /// future refactor must not turn this from impossible into silent.
    ConflictingExecution {
        /// The digest already recorded.
        spent: [u8; 32],
        /// The digest the offered instruction carries.
        offered: [u8; 32],
    },
    /// Custody refused the transition.
    Custody(
        /// The custody error.
        CustodyError,
    ),
}

/// Why a settled receipt could not be observed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObserveError {
    /// The receipt's binding does not match its own contents.
    ReceiptMalformed,
    /// The receipt reports a public refusal; a refused batch obligates
    /// nothing and refunds instead.
    ReceiptStatusRefused {
        /// The relation's public refusal-class code.
        class_code: u32,
    },
    /// The lifecycle machine did not reach `Settled`.
    PhaseNotSettled {
        /// The phase it did reach.
        phase: Phase,
    },
    /// The settled phase's result digest is not this receipt's outcome.
    PhaseResultMismatch,
    /// A different receipt was already observed for this book.
    ///
    /// Two well-formed settled receipts for one cutoff are an outcome
    /// equivocation; the object that adjudicates it is
    /// [`degg_shielded_baseline::dispute`]'s verdict, not this book.
    ConflictingReceipt {
        /// The binding already observed.
        observed: [u8; 32],
        /// The binding offered.
        offered: [u8; 32],
    },
}

/// Why a refund was refused by the settlement relation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefundRefusal {
    /// The batch is not in a terminal abort.
    PhaseNotRefundable {
        /// The phase it is in.
        phase: Phase,
    },
    /// Custody refused the transition.
    Custody(
        /// The custody error.
        CustodyError,
    ),
}

/// The settlement relation's book for one batch: the observed receipt binding,
/// the spent set, and the public claim rows.
#[derive(Clone, Debug, Default)]
pub struct SettlementBook {
    receipt_binding: Option<[u8; 32]>,
    spent: BTreeMap<[u8; 32], [u8; 32]>,
    claims: Vec<ClaimRow>,
}

impl SettlementBook {
    /// An empty book.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The observed receipt binding, if any.
    #[must_use]
    pub fn receipt_binding(&self) -> Option<[u8; 32]> {
        self.receipt_binding
    }

    /// The public claim rows, in execution order.
    #[must_use]
    pub fn claims(&self) -> &[ClaimRow] {
        &self.claims
    }

    /// Observe one settled computation receipt and obligate every reserved
    /// custody entry to it.
    ///
    /// Returns how many reservations newly became obligated. Observing the
    /// same receipt again is harmless and obligates nothing further; a
    /// different receipt is refused.
    pub fn observe_settled(
        &mut self,
        custody: &mut CustodyLedger,
        receipt: &ShieldedReceipt,
        phase: Phase,
    ) -> Result<u32, ObserveError> {
        if !receipt.is_well_formed() {
            return Err(ObserveError::ReceiptMalformed);
        }
        if let ReceiptStatus::Refused(class) = receipt.status {
            return Err(ObserveError::ReceiptStatusRefused {
                class_code: class.code(),
            });
        }
        let result_digest = match phase {
            Phase::Settled { result_digest } => result_digest,
            phase => return Err(ObserveError::PhaseNotSettled { phase }),
        };
        if result_digest != receipt.outcome_digest {
            return Err(ObserveError::PhaseResultMismatch);
        }
        if let Some(observed) = self.receipt_binding
            && observed != receipt.binding
        {
            return Err(ObserveError::ConflictingReceipt {
                observed,
                offered: receipt.binding,
            });
        }
        self.receipt_binding = Some(receipt.binding);
        Ok(custody.obligate_all(receipt.binding))
    }

    /// Execute one authorized settlement instruction, idempotently by its
    /// settlement nullifier.
    pub fn execute(
        &mut self,
        custody: &mut CustodyLedger,
        authorization: &SettlementAuthorization,
        epoch: u64,
    ) -> Result<Execution, SettlementRefusal> {
        if self.receipt_binding != Some(authorization.receipt_binding) {
            return Err(SettlementRefusal::ReceiptNotObserved);
        }
        if let Some(spent) = self.spent.get(&authorization.settlement_nullifier) {
            if *spent == authorization.execution {
                return Ok(Execution::AlreadyExecuted { execution: *spent });
            }
            return Err(SettlementRefusal::ConflictingExecution {
                spent: *spent,
                offered: authorization.execution,
            });
        }
        custody
            .settle(
                &authorization.admission_nullifier,
                &authorization.receipt_binding,
                authorization.execution,
                authorization.released,
                authorization.acquired,
            )
            .map_err(SettlementRefusal::Custody)?;
        let mut credited_base = 0u64;
        let mut credited_quote = 0u64;
        for (asset, amount) in [authorization.released, authorization.acquired] {
            match asset {
                crate::custody::Asset::Base => credited_base += amount,
                crate::custody::Asset::Quote => credited_quote += amount,
            }
        }
        let executed = SettlementExecution {
            settlement_nullifier: authorization.settlement_nullifier,
            execution: authorization.execution,
            owner: authorization.owner,
            seq: authorization.seq,
            credited_base,
            credited_quote,
        };
        self.spent
            .insert(authorization.settlement_nullifier, authorization.execution);
        self.claims.push(ClaimRow {
            account: authorization.owner,
            seq: authorization.seq,
            credited_base,
            credited_quote,
            epoch,
        });
        Ok(Execution::Executed(executed))
    }
}

/// Refund one reservation under a terminal abort.
///
/// The phase gate is this function's whole contribution: custody state alone
/// cannot know that the batch aborted. Entitlement verification — that the
/// claimant's record is genuinely admitted under the repudiated or published
/// root — is the upstream lane's
/// [`degg_inclusion_availability::lifecycle::BatchMachine::claim_refund`],
/// which the composed tests drive alongside this ledger.
pub fn refund(
    custody: &mut CustodyLedger,
    phase: Phase,
    nullifier: &[u8; 32],
    epoch: u64,
) -> Result<u64, RefundRefusal> {
    match phase {
        Phase::Aborted(class) if class.is_terminal() => {}
        phase => return Err(RefundRefusal::PhaseNotRefundable { phase }),
    }
    custody
        .refund(nullifier, epoch)
        .map_err(RefundRefusal::Custody)
}
