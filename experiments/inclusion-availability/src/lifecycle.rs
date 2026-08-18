//! What happens to an admitted order that is never processed.
//!
//! Inclusion is only half of the obligation. A holder can commit a record, hand
//! out a perfectly valid receipt, and then withhold the payload, stall the
//! computation, or publish a result bound to some other root. The submitter's
//! funds are already reserved. This module is the state machine that turns each
//! of those failures into a *typed* terminal or retryable abort with an explicit
//! consequence, and that guarantees the reserved amounts come back exactly once.
//!
//! Three rules carry most of the weight.
//!
//! First, an unavailable payload is never silently dropped.
//! `DARK_RELATION_THREAT_MODEL.md` lists "an unavailable payload is converted to
//! an empty slot rather than a typed abort" as a falsifier of the surface
//! contract, and `DARK_FBA_RELATION.md` §5 says the clearing relation never
//! selects a convenient subset. [`BatchMachine::compute_with_subset`] therefore
//! exists only to refuse: there is no transition anywhere in this machine from
//! "a record's payload is missing" to "the batch computes without it".
//!
//! Second, refunds are conserved. Escrow is keyed by nullifier and recorded at
//! submission time, before and independently of the log, so an equivocating
//! holder cannot inflate or deflate what it owes by choosing which log to show.
//! Every nullifier resolves exactly once, to a refund or to settlement, and the
//! ledger's totals are checked against that invariant after every operation the
//! tests perform.
//!
//! Third, a public refusal of the relation is not a settlement.
//! [`BatchMachine::deliver_refusal`] and [`AbortClass::RelationRefused`] type
//! the case where the relation answered completely and correctly by refusing
//! the admitted batch: there is no allocation, so nothing may be released to
//! settlement and every admitted record is refundable. Before that class
//! existed a publicly refused batch reached [`Phase::Settled`] through
//! [`BatchMachine::deliver_result`] and its reserved funds had no path back at
//! all, which `SHIELDED_BASELINE.md` §7.1 recorded as composition gap C-1.

use std::collections::{BTreeMap, BTreeSet};

use crate::equivocation::{
    EquivocationDefect, EquivocationProof, RootStatement, verify_equivocation,
};
use crate::log::{CutoffRoot, InclusionReceipt, LogDomain, ReceiptDefect, verify_receipt};

/// Deadlines, all measured in epochs after the cutoff.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Timeouts {
    /// By when the cutoff root must be published.
    pub seal: u64,
    /// By when every admitted payload must be recoverable.
    pub availability: u64,
    /// Length of one compute attempt.
    pub compute: u64,
    /// How many compute attempts the relation permits after the first.
    pub compute_retries: u32,
}

impl Timeouts {
    /// The deadlines the model uses for the Dark FBA batch relation.
    #[must_use]
    pub fn dark_fba_v0() -> Self {
        Self {
            seal: 1,
            availability: 2,
            compute: 4,
            compute_retries: 1,
        }
    }
}

/// A typed abort, with the consequence it carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AbortClass {
    /// No cutoff root was published by `cutoff + seal`.
    ///
    /// There is no committed admitted set at all, so inclusion cannot be proved
    /// and escrow alone entitles a refund.
    CutoffRootWithheld,
    /// An admitted payload was not recoverable by `cutoff + availability`.
    InputWithheld {
        /// Lowest canonical position whose payload is short of the threshold.
        seq: u32,
    },
    /// A compute attempt ran out of time and another attempt is permitted.
    ComputeTimeout {
        /// Attempts already made.
        attempts: u32,
    },
    /// Every permitted compute attempt has been spent.
    ComputeExhausted {
        /// Attempts made.
        attempts: u32,
    },
    /// A verified equivocation exists for this cutoff.
    Equivocation {
        /// Canonical identifier of the verdict.
        verdict_digest: [u8; 32],
    },
    /// A result was delivered that is not bound to the published cutoff root.
    ResultUnbound,
    /// A result bound to the published cutoff root reports that the relation
    /// publicly refused the admitted batch.
    ///
    /// A refusal is a complete, correct, public answer of the relation, and it
    /// is *not* a settlement: no allocation exists, so nothing may be released
    /// to the settlement relation and every admitted record is refundable.
    /// Without this class a publicly refused batch reached [`Phase::Settled`]
    /// and its reserved funds had no path back at all.
    ///
    /// The machine does not interpret `class_code` and cannot recompute it:
    /// this model never evaluates the relation. The code is the relation's own
    /// public refusal-class code, carried verbatim, in the relation named by
    /// the log domain. A deliverer that publishes a refusal and reports it
    /// through [`BatchMachine::deliver_result`] instead is therefore not
    /// detected here; what closes that is a machine that checks the relation,
    /// which is a different model.
    RelationRefused {
        /// The relation's own public refusal-class code.
        class_code: u32,
    },
}

/// What an abort or a settlement means for reserved funds and for retry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Consequence {
    /// The batch may be recomputed against the same cutoff root.
    RetryAgainstSameCutoffRoot,
    /// Every escrowed submission for this cutoff is refundable; no root exists
    /// to prove inclusion against.
    RefundEveryEscrowedSubmission,
    /// Every record admitted under the cutoff root is refundable.
    RefundEveryAdmittedRecord,
    /// Every record admitted under either repudiated root is refundable, once
    /// per nullifier.
    RefundUnderEitherRepudiatedRoot,
}

impl AbortClass {
    /// Stable class name for transcripts.
    #[must_use]
    pub fn class(&self) -> &'static str {
        match self {
            AbortClass::CutoffRootWithheld => "cutoff-root-withheld",
            AbortClass::InputWithheld { .. } => "input-withheld",
            AbortClass::ComputeTimeout { .. } => "compute-timeout",
            AbortClass::ComputeExhausted { .. } => "compute-exhausted",
            AbortClass::Equivocation { .. } => "equivocation",
            AbortClass::ResultUnbound => "result-unbound",
            AbortClass::RelationRefused { .. } => "relation-refused",
        }
    }

    /// Whether a further compute attempt is permitted from this abort.
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        matches!(self, AbortClass::ComputeTimeout { .. })
    }

    /// Whether this abort ends the batch.
    #[must_use]
    pub fn is_terminal(&self) -> bool {
        !self.is_retryable()
    }

    /// The consequence this abort carries for reserved funds.
    #[must_use]
    pub fn consequence(&self) -> Consequence {
        match self {
            AbortClass::ComputeTimeout { .. } => Consequence::RetryAgainstSameCutoffRoot,
            AbortClass::CutoffRootWithheld => Consequence::RefundEveryEscrowedSubmission,
            AbortClass::InputWithheld { .. }
            | AbortClass::ComputeExhausted { .. }
            | AbortClass::ResultUnbound
            | AbortClass::RelationRefused { .. } => Consequence::RefundEveryAdmittedRecord,
            AbortClass::Equivocation { .. } => Consequence::RefundUnderEitherRepudiatedRoot,
        }
    }
}

/// Where a batch stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// Before the cutoff root is observed.
    Open,
    /// A cutoff root is published; availability is being established.
    Sealed,
    /// A compute attempt is running.
    Computing,
    /// A result bound to the cutoff root was delivered.
    Settled {
        /// Digest of the delivered result.
        result_digest: [u8; 32],
    },
    /// The batch aborted.
    Aborted(AbortClass),
}

impl Phase {
    /// Stable phase name for transcripts.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Phase::Open => "open",
            Phase::Sealed => "sealed",
            Phase::Computing => "computing",
            Phase::Settled { .. } => "settled",
            Phase::Aborted(class) => class.class(),
        }
    }

    /// Whether no further transition is possible.
    #[must_use]
    pub fn is_terminal(&self) -> bool {
        match self {
            Phase::Settled { .. } => true,
            Phase::Aborted(class) => class.is_terminal(),
            Phase::Open | Phase::Sealed | Phase::Computing => false,
        }
    }
}

/// A rejected lifecycle transition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LifecycleError {
    /// The batch is already finished.
    Terminal {
        /// The phase that ended it.
        phase: Phase,
    },
    /// The event is not admissible in the current phase.
    PhaseForbids {
        /// Current phase.
        phase: Phase,
    },
    /// The observed cutoff root belongs to another log.
    CutoffDomainMismatch,
    /// The observed cutoff root admits more records than its own capacity.
    CutoffRootMalformed,
    /// A payload report names a position outside the admitted set.
    SequenceOutOfLog {
        /// Position reported.
        seq: u32,
        /// Number of admitted records.
        leaf_count: u64,
    },
    /// A payload report declares more recoverable shares than were dispersed.
    SharesOutOfRange {
        /// Shares reported.
        reported: u8,
        /// Shares the domain declares.
        dispersed: u8,
    },
    /// Computation cannot begin: a payload is short of the threshold.
    AvailabilityBelowThreshold {
        /// Lowest short position.
        seq: u32,
        /// Shares recoverable there.
        recoverable: u8,
        /// Shares required.
        threshold: u8,
    },
    /// Resumption was attempted from a phase that is not a retryable abort.
    NotResumable {
        /// Current phase.
        phase: Phase,
    },
    /// Selecting a subset of the admitted set is not a transition of this machine.
    ///
    /// Refusing here is the point: a batch that cannot see all of its admitted
    /// inputs aborts, and never quietly computes over the inputs it happens to
    /// hold.
    SubsetSelectionForbidden,
    /// The presented equivocation proof does not verify.
    EquivocationInvalid(EquivocationDefect),
    /// The presented equivocation proof is about another log.
    EquivocationDomainMismatch,
}

/// Why a refund or settlement release was rejected.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefundError {
    /// The batch has not reached a refundable state.
    PhaseNotRefundable {
        /// Current phase.
        phase: Phase,
    },
    /// The batch did not settle, so nothing may be released to settlement.
    PhaseNotSettled {
        /// Current phase.
        phase: Phase,
    },
    /// The entitlement offered does not match the abort's consequence.
    WrongEntitlement {
        /// The consequence in force.
        consequence: Consequence,
    },
    /// The receipt does not verify against the root it was offered under.
    Receipt(ReceiptDefect),
    /// The statement's root is not one this holder was found to have published.
    RootNotRepudiated,
    /// A statement offered with a refund claim is malformed.
    MalformedStatement,
    /// Nothing was escrowed under this nullifier.
    NotEscrowed,
    /// This nullifier was already refunded.
    AlreadyRefunded,
    /// This nullifier was already released to settlement.
    AlreadySettled,
}

/// How a claimant proves entitlement to a refund.
#[derive(Clone, Debug)]
pub enum Entitlement<'a> {
    /// An inclusion receipt against the published cutoff root.
    Included(&'a InclusionReceipt),
    /// An inclusion receipt against one of an equivocating holder's roots.
    IncludedUnderRepudiatedRoot {
        /// The statement naming the root the receipt is taken against.
        statement: RootStatement,
        /// The receipt.
        receipt: &'a InclusionReceipt,
    },
    /// No cutoff root exists; escrow alone entitles the claim.
    Escrowed {
        /// The escrowed nullifier.
        nullifier: [u8; 32],
    },
}

/// What became of one escrowed reservation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disposition {
    /// Still held.
    Outstanding,
    /// Returned to the submitter.
    Refunded,
    /// Consumed by the settlement relation.
    Settled,
}

/// The reserved amounts, keyed by nullifier.
///
/// Escrow is recorded at submission, independently of any log, so the amount
/// owed to a submitter is not a function of which history the holder chooses to
/// publish.
#[derive(Clone, Debug, Default)]
pub struct ReserveLedger {
    escrowed: BTreeMap<[u8; 32], u64>,
    refunded: BTreeSet<[u8; 32]>,
    settled: BTreeSet<[u8; 32]>,
}

impl ReserveLedger {
    /// An empty ledger.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a reservation. A repeated nullifier keeps the first amount.
    pub fn escrow(&mut self, nullifier: [u8; 32], amount: u64) -> bool {
        if self.escrowed.contains_key(&nullifier) {
            return false;
        }
        self.escrowed.insert(nullifier, amount);
        true
    }

    /// The amount escrowed under one nullifier.
    #[must_use]
    pub fn amount(&self, nullifier: &[u8; 32]) -> Option<u64> {
        self.escrowed.get(nullifier).copied()
    }

    /// What became of one nullifier's reservation.
    #[must_use]
    pub fn disposition(&self, nullifier: &[u8; 32]) -> Option<Disposition> {
        if !self.escrowed.contains_key(nullifier) {
            return None;
        }
        if self.refunded.contains(nullifier) {
            Some(Disposition::Refunded)
        } else if self.settled.contains(nullifier) {
            Some(Disposition::Settled)
        } else {
            Some(Disposition::Outstanding)
        }
    }

    /// Total escrowed.
    #[must_use]
    pub fn total_escrowed(&self) -> u64 {
        self.escrowed.values().sum()
    }

    /// Total refunded.
    #[must_use]
    pub fn total_refunded(&self) -> u64 {
        self.sum_over(&self.refunded)
    }

    /// Total released to settlement.
    #[must_use]
    pub fn total_settled(&self) -> u64 {
        self.sum_over(&self.settled)
    }

    /// Total still held.
    #[must_use]
    pub fn total_outstanding(&self) -> u64 {
        self.total_escrowed() - self.total_refunded() - self.total_settled()
    }

    /// The conservation invariant: nothing is created, destroyed, or paid twice.
    #[must_use]
    pub fn conserves(&self) -> bool {
        self.refunded.is_disjoint(&self.settled)
            && self.refunded.iter().all(|n| self.escrowed.contains_key(n))
            && self.settled.iter().all(|n| self.escrowed.contains_key(n))
            && self.total_refunded() + self.total_settled() + self.total_outstanding()
                == self.total_escrowed()
    }

    fn sum_over(&self, keys: &BTreeSet<[u8; 32]>) -> u64 {
        keys.iter()
            .map(|n| self.escrowed.get(n).copied().unwrap_or(0))
            .sum()
    }

    fn resolve(&mut self, nullifier: [u8; 32], to: Disposition) -> Result<u64, RefundError> {
        let amount = self
            .escrowed
            .get(&nullifier)
            .copied()
            .ok_or(RefundError::NotEscrowed)?;
        match self.disposition(&nullifier) {
            Some(Disposition::Refunded) => return Err(RefundError::AlreadyRefunded),
            Some(Disposition::Settled) => return Err(RefundError::AlreadySettled),
            _ => {}
        }
        match to {
            Disposition::Refunded => {
                self.refunded.insert(nullifier);
            }
            Disposition::Settled => {
                self.settled.insert(nullifier);
            }
            Disposition::Outstanding => {}
        }
        Ok(amount)
    }
}

/// The post-cutoff lifecycle of one batch, as seen by a verifier.
///
/// The machine never holds the log. It observes a published cutoff root,
/// availability reports, results, and proofs, which is exactly what a relying
/// party can see.
#[derive(Clone, Debug)]
pub struct BatchMachine {
    domain: LogDomain,
    timeouts: Timeouts,
    phase: Phase,
    cutoff: Option<CutoffRoot>,
    recoverable: BTreeMap<u32, u8>,
    attempts: u32,
    repudiated: Vec<CutoffRoot>,
}

impl BatchMachine {
    /// A machine watching `domain` under `timeouts`, before any root is seen.
    #[must_use]
    pub fn new(domain: LogDomain, timeouts: Timeouts) -> Self {
        Self {
            domain,
            timeouts,
            phase: Phase::Open,
            cutoff: None,
            recoverable: BTreeMap::new(),
            attempts: 0,
            repudiated: Vec::new(),
        }
    }

    /// Current phase.
    #[must_use]
    pub fn phase(&self) -> Phase {
        self.phase
    }

    /// The observed cutoff root, if any.
    #[must_use]
    pub fn cutoff(&self) -> Option<CutoffRoot> {
        self.cutoff
    }

    /// Compute attempts made so far.
    #[must_use]
    pub fn attempts(&self) -> u32 {
        self.attempts
    }

    /// Observe the published cutoff root.
    pub fn observe_cutoff(
        &mut self,
        cutoff: CutoffRoot,
        now_epoch: u64,
    ) -> Result<(), LifecycleError> {
        self.guard_live()?;
        if self.phase != Phase::Open {
            return Err(LifecycleError::PhaseForbids { phase: self.phase });
        }
        if cutoff.domain != self.domain {
            return Err(LifecycleError::CutoffDomainMismatch);
        }
        if cutoff.leaf_count > u64::from(self.domain.capacity) {
            return Err(LifecycleError::CutoffRootMalformed);
        }
        if now_epoch > self.deadline(self.timeouts.seal) {
            self.phase = Phase::Aborted(AbortClass::CutoffRootWithheld);
            return Err(LifecycleError::PhaseForbids { phase: self.phase });
        }
        self.cutoff = Some(cutoff);
        self.phase = Phase::Sealed;
        Ok(())
    }

    /// Report how many shares of one admitted payload are recoverable.
    pub fn report_availability(&mut self, seq: u32, recoverable: u8) -> Result<(), LifecycleError> {
        self.guard_live()?;
        let cutoff = match self.phase {
            Phase::Sealed => self
                .cutoff
                .ok_or(LifecycleError::PhaseForbids { phase: self.phase })?,
            _ => return Err(LifecycleError::PhaseForbids { phase: self.phase }),
        };
        if u64::from(seq) >= cutoff.leaf_count {
            return Err(LifecycleError::SequenceOutOfLog {
                seq,
                leaf_count: cutoff.leaf_count,
            });
        }
        if recoverable > self.domain.availability_shares {
            return Err(LifecycleError::SharesOutOfRange {
                reported: recoverable,
                dispersed: self.domain.availability_shares,
            });
        }
        self.recoverable.insert(seq, recoverable);
        Ok(())
    }

    /// The lowest admitted position whose payload is short of the threshold.
    ///
    /// A position with no report counts as zero recoverable shares: silence is
    /// unavailability, not availability.
    #[must_use]
    pub fn first_unavailable(&self) -> Option<(u32, u8)> {
        let cutoff = self.cutoff?;
        (0..u32::try_from(cutoff.leaf_count).unwrap_or(u32::MAX))
            .map(|seq| (seq, self.recoverable.get(&seq).copied().unwrap_or(0)))
            .find(|(_, shares)| *shares < self.domain.availability_threshold)
    }

    /// Begin a compute attempt.
    pub fn begin_compute(&mut self, now_epoch: u64) -> Result<(), LifecycleError> {
        self.guard_live()?;
        if self.phase != Phase::Sealed {
            return Err(LifecycleError::PhaseForbids { phase: self.phase });
        }
        if let Some((seq, recoverable)) = self.first_unavailable() {
            return Err(LifecycleError::AvailabilityBelowThreshold {
                seq,
                recoverable,
                threshold: self.domain.availability_threshold,
            });
        }
        let _ = now_epoch;
        self.phase = Phase::Computing;
        Ok(())
    }

    /// Refuse, always, to compute over a chosen subset of the admitted set.
    ///
    /// The signature takes the subset the caller wanted so that the refusal is
    /// unambiguous at the call site. There is no argument value that makes this
    /// succeed.
    pub fn compute_with_subset(&mut self, _subset: &[u32]) -> Result<(), LifecycleError> {
        Err(LifecycleError::SubsetSelectionForbidden)
    }

    /// Deliver a result claiming to be bound to `bound_root`.
    pub fn deliver_result(
        &mut self,
        bound_root: [u8; 32],
        result_digest: [u8; 32],
        now_epoch: u64,
    ) -> Result<Phase, LifecycleError> {
        self.guard_live()?;
        if self.phase != Phase::Computing {
            return Err(LifecycleError::PhaseForbids { phase: self.phase });
        }
        let cutoff = self
            .cutoff
            .ok_or(LifecycleError::PhaseForbids { phase: self.phase })?;
        if now_epoch > self.attempt_deadline() {
            self.tick(now_epoch);
            return Ok(self.phase);
        }
        self.phase = if bound_root == cutoff.root {
            Phase::Settled { result_digest }
        } else {
            Phase::Aborted(AbortClass::ResultUnbound)
        };
        Ok(self.phase)
    }

    /// Deliver a *public refusal* of the relation, claiming to be bound to
    /// `bound_root`.
    ///
    /// Every guard [`BatchMachine::deliver_result`] carries is carried here
    /// too, and for the same reasons: a refusal offered outside `Computing` is
    /// inadmissible, a refusal offered after the attempt deadline is a
    /// timeout, and a refusal bound to some other root is `result-unbound`
    /// rather than a refusal, because a statement about another admitted set
    /// says nothing about this one.
    ///
    /// A bound refusal is terminal and refunds every admitted record. It is
    /// deliberately not a settlement: the relation produced no allocation, so
    /// there is nothing for the settlement relation to consume.
    pub fn deliver_refusal(
        &mut self,
        bound_root: [u8; 32],
        class_code: u32,
        now_epoch: u64,
    ) -> Result<Phase, LifecycleError> {
        self.guard_live()?;
        if self.phase != Phase::Computing {
            return Err(LifecycleError::PhaseForbids { phase: self.phase });
        }
        let cutoff = self
            .cutoff
            .ok_or(LifecycleError::PhaseForbids { phase: self.phase })?;
        if now_epoch > self.attempt_deadline() {
            self.tick(now_epoch);
            return Ok(self.phase);
        }
        self.phase = if bound_root == cutoff.root {
            Phase::Aborted(AbortClass::RelationRefused { class_code })
        } else {
            Phase::Aborted(AbortClass::ResultUnbound)
        };
        Ok(self.phase)
    }

    /// Present a verified equivocation against this cutoff.
    pub fn present_equivocation(
        &mut self,
        proof: &EquivocationProof,
    ) -> Result<Phase, LifecycleError> {
        self.guard_live()?;
        if proof.domain != self.domain {
            return Err(LifecycleError::EquivocationDomainMismatch);
        }
        let verdict = verify_equivocation(proof).map_err(LifecycleError::EquivocationInvalid)?;
        self.repudiated = vec![
            CutoffRoot {
                domain: self.domain,
                leaf_count: proof.left.leaf_count,
                root: proof.left.root,
            },
            CutoffRoot {
                domain: self.domain,
                leaf_count: proof.right.leaf_count,
                root: proof.right.root,
            },
        ];
        self.phase = Phase::Aborted(AbortClass::Equivocation {
            verdict_digest: verdict.digest,
        });
        Ok(self.phase)
    }

    /// Advance the clock and apply whatever deadline has passed.
    ///
    /// Frozen order: an open batch fails on the seal deadline; a sealed batch
    /// fails on the availability deadline before the compute deadline, so an
    /// unrecoverable payload is named as such rather than as a slow computation.
    pub fn tick(&mut self, now_epoch: u64) -> Phase {
        if self.phase.is_terminal() {
            return self.phase;
        }
        match self.phase {
            Phase::Open => {
                if now_epoch > self.deadline(self.timeouts.seal) {
                    self.phase = Phase::Aborted(AbortClass::CutoffRootWithheld);
                }
            }
            Phase::Sealed => {
                if now_epoch > self.deadline(self.timeouts.availability)
                    && let Some((seq, _)) = self.first_unavailable()
                {
                    self.phase = Phase::Aborted(AbortClass::InputWithheld { seq });
                } else if now_epoch > self.attempt_deadline() {
                    self.phase = self.timeout_phase();
                }
            }
            Phase::Computing => {
                if now_epoch > self.attempt_deadline() {
                    self.phase = self.timeout_phase();
                }
            }
            Phase::Aborted(_) | Phase::Settled { .. } => {}
        }
        self.phase
    }

    /// Start another compute attempt after a retryable abort.
    pub fn resume(&mut self, now_epoch: u64) -> Result<Phase, LifecycleError> {
        match self.phase {
            Phase::Aborted(class) if class.is_retryable() => {
                self.attempts += 1;
                self.phase = Phase::Computing;
                let _ = now_epoch;
                Ok(self.phase)
            }
            phase => Err(LifecycleError::NotResumable { phase }),
        }
    }

    /// Claim a refund into `ledger`.
    pub fn claim_refund(
        &self,
        ledger: &mut ReserveLedger,
        entitlement: &Entitlement<'_>,
    ) -> Result<u64, RefundError> {
        let class = match self.phase {
            Phase::Aborted(class) if class.is_terminal() => class,
            phase => return Err(RefundError::PhaseNotRefundable { phase }),
        };
        let consequence = class.consequence();
        let nullifier = match (consequence, entitlement) {
            (Consequence::RefundEveryEscrowedSubmission, Entitlement::Escrowed { nullifier }) => {
                *nullifier
            }
            (Consequence::RefundEveryAdmittedRecord, Entitlement::Included(receipt)) => {
                let cutoff = self
                    .cutoff
                    .ok_or(RefundError::WrongEntitlement { consequence })?;
                verify_receipt(&cutoff, receipt).map_err(RefundError::Receipt)?;
                receipt.record.nullifier
            }
            (
                Consequence::RefundUnderEitherRepudiatedRoot,
                Entitlement::IncludedUnderRepudiatedRoot { statement, receipt },
            ) => {
                if !statement.is_well_formed() {
                    return Err(RefundError::MalformedStatement);
                }
                let cutoff = self
                    .repudiated
                    .iter()
                    .find(|candidate| {
                        candidate.root == statement.root
                            && candidate.leaf_count == statement.leaf_count
                    })
                    .copied()
                    .ok_or(RefundError::RootNotRepudiated)?;
                verify_receipt(&cutoff, receipt).map_err(RefundError::Receipt)?;
                receipt.record.nullifier
            }
            _ => return Err(RefundError::WrongEntitlement { consequence }),
        };
        ledger.resolve(nullifier, Disposition::Refunded)
    }

    /// Release one reservation to the settlement relation.
    ///
    /// Only a settled batch may do this, and settlement itself is a different
    /// relation that this model does not implement.
    pub fn release_to_settlement(
        &self,
        ledger: &mut ReserveLedger,
        nullifier: [u8; 32],
    ) -> Result<u64, RefundError> {
        match self.phase {
            Phase::Settled { .. } => ledger.resolve(nullifier, Disposition::Settled),
            phase => Err(RefundError::PhaseNotSettled { phase }),
        }
    }

    fn deadline(&self, offset: u64) -> u64 {
        self.domain.cutoff_epoch.saturating_add(offset)
    }

    fn attempt_deadline(&self) -> u64 {
        self.deadline(
            self.timeouts
                .compute
                .saturating_mul(u64::from(self.attempts) + 1),
        )
    }

    fn timeout_phase(&self) -> Phase {
        if self.attempts < self.timeouts.compute_retries {
            Phase::Aborted(AbortClass::ComputeTimeout {
                attempts: self.attempts + 1,
            })
        } else {
            Phase::Aborted(AbortClass::ComputeExhausted {
                attempts: self.attempts + 1,
            })
        }
    }

    fn guard_live(&self) -> Result<(), LifecycleError> {
        if self.phase.is_terminal() {
            Err(LifecycleError::Terminal { phase: self.phase })
        } else {
            Ok(())
        }
    }
}
