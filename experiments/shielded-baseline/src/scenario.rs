//! The end-to-end session: submitters, log, cutoff, executor, delivery.
//!
//! One `Session` drives the whole composed pipeline with no step left as a
//! boolean:
//!
//! ```text
//! submitters --prepare--> sealed payloads + admission requests + escrow
//!            --admit-->   AdmissionLog (append-only, envelope only)
//!            --seal_padded--> CutoffRoot (leaf count == capacity, constant)
//!            --observe_cutoff--> BatchMachine (Open -> Sealed)
//!            --report_availability--> threshold met, or input-withheld
//!            --begin_compute--> Computing
//! executor   --assemble--> BatchInput from the committed positions
//!            --evaluate--> the module's Clear evaluator
//!            --publish--> ShieldedReceipt + DeliveryCommitment + sealed outputs
//!            --deliver_result--> Settled, or ResultUnbound
//! owners     --audit--> findings, or a transferable dispute object
//! ```
//!
//! The session is deterministic and offline: no clock, no network, no
//! randomness. Epochs are integers the caller supplies, exactly as upstream.

use degg_inclusion_availability::lifecycle::{BatchMachine, Phase, ReserveLedger, Timeouts};
use degg_inclusion_availability::log::{
    AdmissionAck, AdmissionLog, AdmissionRefusal, CutoffRoot, DomainDefect, InclusionReceipt,
    LogDomain, SealRefusal,
};
use degg_relation_ir::batch::{
    BatchInput, BoundaryStatements, OrderWitness, RequestedMode, Side, SlotInput,
};
use degg_relation_ir::lower::{
    ClearEvaluator, LoweringTarget, Outcome, OwnerOutput, lower, required_reservation,
};
use degg_relation_ir::module::{RelationModule, dark_fba_n4_k4_q15_v0};

use crate::executor::{Assembly, AssemblyRefusal, Executor, ExecutorInputs, ShieldedRun, Tamper};
use crate::owner::{OwnedPosition, OwnerEvidence};
use crate::receipt::DeliveryOpening;
use crate::roles::CredentialRegistry;
use crate::seal::{PlainOrder, SealedPayload};
use crate::submit::{Submission, SubmitDefect, prepare};

/// Batch identifier every scenario in this crate uses.
pub const BATCH_ID: u64 = 7;
/// Market identifier every scenario in this crate uses.
pub const MARKET_ID: u64 = 9;
/// Cutoff epoch every scenario in this crate uses.
pub const CUTOFF_EPOCH: u64 = 10;

/// One order in a scenario's book, before sealing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BookOrder {
    /// Owner index.
    pub owner: u8,
    /// Side of the book.
    pub side: Side,
    /// Limit tick index.
    pub limit_tick: u8,
    /// Quantity in base atoms.
    pub quantity: u64,
    /// Reservation, or `None` for exactly the worst-case obligation.
    pub reserved: Option<u64>,
    /// Arrival time, or `None` for the cutoff epoch.
    pub arrived_at: Option<u64>,
    /// Nullifier, or `None` for a per-position default.
    pub nullifier: Option<u64>,
}

impl BookOrder {
    /// An order reserving exactly its worst-case obligation, arriving at the
    /// cutoff, with the default nullifier for its position.
    #[must_use]
    pub fn exact(owner: u8, side: Side, limit_tick: u8, quantity: u64) -> Self {
        Self {
            owner,
            side,
            limit_tick,
            quantity,
            reserved: None,
            arrived_at: None,
            nullifier: None,
        }
    }

    fn plain(&self, index: usize, module: &RelationModule) -> PlainOrder {
        let required = required_reservation(
            &module.params.tick_prices,
            self.side,
            self.limit_tick,
            self.quantity,
        )
        .unwrap_or(0);
        PlainOrder {
            batch_id: BATCH_ID,
            market_id: MARKET_ID,
            owner: self.owner,
            side: self.side,
            limit_tick: self.limit_tick,
            quantity: self.quantity,
            reserved: self.reserved.unwrap_or(required),
            nullifier: self
                .nullifier
                .unwrap_or_else(|| 101 + u64::try_from(index).unwrap_or(0)),
            arrived_at: self.arrived_at.unwrap_or(CUTOFF_EPOCH),
        }
    }
}

/// A named book to run end to end.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scenario {
    /// Transcript name.
    pub name: &'static str,
    /// The book, in submission order.
    pub orders: Vec<BookOrder>,
}

impl Scenario {
    /// A scenario over the frozen relation's fixed batch, market, and cutoff.
    #[must_use]
    pub fn new(name: &'static str, orders: Vec<BookOrder>) -> Self {
        Self { name, orders }
    }
}

/// Why a session could not be opened or driven.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionError {
    /// The log domain is not usable.
    Domain(
        /// The defect.
        DomainDefect,
    ),
    /// A submission could not be prepared.
    Submit(
        /// The defect.
        SubmitDefect,
    ),
    /// The cutoff could not be sealed.
    Seal(
        /// The refusal.
        SealRefusal,
    ),
    /// The lifecycle machine refused a transition.
    Lifecycle(
        /// The error, rendered as its phase name where one exists.
        &'static str,
    ),
    /// The executor could not assemble a witness.
    Assembly(
        /// The refusal.
        AssemblyRefusal,
    ),
    /// An inclusion receipt was unavailable for a committed position.
    ReceiptUnavailable {
        /// The position.
        seq: u32,
    },
}

/// One driven session, after the cutoff is sealed and availability is reported.
#[derive(Clone, Debug)]
pub struct Session {
    /// The frozen log domain.
    pub domain: LogDomain,
    /// The relation module the executor runs.
    pub module: RelationModule,
    /// The named executor.
    pub executor: Executor,
    /// The enrolment table.
    pub registry: CredentialRegistry,
    /// Every submission, in submission order.
    pub submissions: Vec<Submission>,
    /// The acknowledgement for each admitted submission, aligned with
    /// [`Session::admitted`].
    pub acks: Vec<AdmissionAck>,
    /// Indices into [`Session::submissions`] of the admitted submissions, in
    /// admission order.
    pub admitted: Vec<usize>,
    /// Submissions the log refused, with the refusal.
    pub refused: Vec<(usize, AdmissionRefusal)>,
    /// The published cutoff root.
    pub cutoff: CutoffRoot,
    /// One inclusion receipt per committed position.
    pub receipts: Vec<InclusionReceipt>,
    /// The reserve ledger.
    pub ledger: ReserveLedger,
    /// The lifecycle machine.
    pub machine: BatchMachine,
    /// The deadlines in force.
    pub timeouts: Timeouts,
    log: AdmissionLog,
}

/// One published run, with everything an owner needs to audit it.
#[derive(Clone, Debug)]
pub struct Run {
    /// The executor's published run.
    pub run: ShieldedRun,
    /// The assembled batch, executor-visible.
    pub assembly: Assembly,
    /// One delivery opening per committed position.
    pub openings: Vec<DeliveryOpening>,
    /// Each owner's local output, opened with that owner's delivery key.
    pub delivered: Vec<Option<OwnerOutput>>,
    /// The lifecycle phase after the result was delivered.
    pub phase: Phase,
}

impl Session {
    /// Open a session, admit every submission, seal the padded cutoff, observe
    /// it, and report full availability for every committed position.
    pub fn open(scenario: &Scenario, now_epoch: u64) -> Result<Self, SessionError> {
        let module = dark_fba_n4_k4_q15_v0();
        let registry = CredentialRegistry::enrol(module.params.owners, &[0x5a; 32]);
        let (executor, capability) =
            Executor::commission("degg-named-executor/v0", &[0xe0; 32], registry.clone());
        let domain = LogDomain::dark_fba_v0(BATCH_ID, MARKET_ID, CUTOFF_EPOCH);
        let mut log = AdmissionLog::open(domain).map_err(SessionError::Domain)?;
        let mut ledger = ReserveLedger::new();

        let mut submissions = Vec::with_capacity(scenario.orders.len());
        for (index, order) in scenario.orders.iter().enumerate() {
            let plain = order.plain(index, &module);
            submissions.push(
                prepare(&capability, &registry, &domain, plain).map_err(SessionError::Submit)?,
            );
        }

        let mut acks = Vec::new();
        let mut admitted = Vec::new();
        let mut refused = Vec::new();
        for (index, submission) in submissions.iter().enumerate() {
            ledger.escrow(submission.request.nullifier, submission.escrow);
            match log.admit(&submission.request, submission.request.arrival_epoch) {
                Ok(ack) => {
                    acks.push(ack);
                    admitted.push(index);
                }
                Err(refusal) => refused.push((index, refusal)),
            }
        }

        let cutoff = log.seal_padded(now_epoch).map_err(SessionError::Seal)?;
        let mut receipts = Vec::new();
        for seq in 0..u32::try_from(cutoff.leaf_count).unwrap_or(u32::MAX) {
            receipts.push(
                log.receipt(seq)
                    .ok_or(SessionError::ReceiptUnavailable { seq })?,
            );
        }

        let timeouts = Timeouts::dark_fba_v0();
        let mut machine = BatchMachine::new(domain, timeouts);
        machine
            .observe_cutoff(cutoff, now_epoch)
            .map_err(|_| SessionError::Lifecycle("observe_cutoff refused"))?;
        for seq in 0..u32::try_from(cutoff.leaf_count).unwrap_or(u32::MAX) {
            machine
                .report_availability(seq, domain.availability_shares)
                .map_err(|_| SessionError::Lifecycle("report_availability refused"))?;
        }

        Ok(Self {
            domain,
            module,
            executor,
            registry,
            submissions,
            acks,
            admitted,
            refused,
            cutoff,
            receipts,
            ledger,
            machine,
            timeouts,
            log,
        })
    }

    /// The admission log, for tests that need the holder's own view.
    #[must_use]
    pub fn log(&self) -> &AdmissionLog {
        &self.log
    }

    /// The sealed payload the executor received for each committed position.
    #[must_use]
    pub fn payloads(&self) -> std::collections::BTreeMap<u32, SealedPayload> {
        self.admitted
            .iter()
            .zip(&self.acks)
            .map(|(index, ack)| (ack.seq, self.submissions[*index].sealed))
            .collect()
    }

    /// The committed position of one submission, if it was admitted.
    #[must_use]
    pub fn seq_of(&self, submission_index: usize) -> Option<u32> {
        self.admitted
            .iter()
            .position(|index| *index == submission_index)
            .map(|at| self.acks[at].seq)
    }

    /// Assemble and evaluate, then deliver the result to the lifecycle machine.
    pub fn compute(&mut self, tamper: &Tamper, now_epoch: u64) -> Result<Run, SessionError> {
        self.machine
            .begin_compute(now_epoch)
            .map_err(|_| SessionError::Lifecycle("begin_compute refused"))?;
        let inputs = ExecutorInputs {
            cutoff: self.cutoff,
            receipts: self.receipts.clone(),
            payloads: self.payloads(),
            known_verdicts: Vec::new(),
        };
        let assembly = self
            .executor
            .assemble(&inputs, &self.ledger, &self.machine, tamper)
            .map_err(SessionError::Assembly)?;
        let run = self.executor.execute(&assembly, tamper);
        let openings = (0..u32::try_from(self.cutoff.leaf_count).unwrap_or(u32::MAX))
            .filter_map(|seq| run.delivery.open(seq))
            .collect();
        let delivered = (0..self.module.params.owners)
            .map(|owner| {
                run.deliveries
                    .iter()
                    .find(|sealed| sealed.owner() == owner)
                    .and_then(|sealed| sealed.open(&self.executor.delivery_key(owner)).ok())
            })
            .collect();
        let phase = self
            .machine
            .deliver_result(
                run.receipt.cutoff.root,
                run.receipt.outcome_digest,
                now_epoch,
            )
            .map_err(|_| SessionError::Lifecycle("deliver_result refused"))?;
        Ok(Run {
            run,
            assembly,
            openings,
            delivered,
            phase,
        })
    }

    /// One owner's evidence bundle for an audit.
    #[must_use]
    pub fn owner_evidence<'a>(&'a self, run: &'a Run, owner: u8) -> OwnerEvidence<'a> {
        let mut positions = Vec::new();
        for (at, index) in self.admitted.iter().enumerate() {
            let submission = &self.submissions[*index];
            if submission.plain.owner != owner {
                continue;
            }
            let seq = self.acks[at].seq;
            positions.push(OwnedPosition {
                submission,
                inclusion: &self.receipts[usize::try_from(seq).unwrap_or(usize::MAX)],
                opening: run.openings.iter().find(|opening| opening.seq == seq),
            });
        }
        OwnerEvidence {
            owner,
            positions,
            cutoff: &self.cutoff,
            delivered: run
                .delivered
                .get(usize::from(owner))
                .and_then(Option::as_ref),
        }
    }

    /// The Clear reference batch, built from the submitters' own plaintexts.
    ///
    /// This is the differential's left-hand side. It is assembled from what
    /// the *submitters* hold, in admission order, with every boundary and
    /// per-slot statement satisfied — never from the executor's assembly. The
    /// evaluator is the same one, and deliberately so: a Shielded run *is* the
    /// module's Clear evaluator executed by one named process, so what the
    /// differential compares is the composed assembly path, not the relation
    /// semantics. Those were compared elsewhere, over 2,116,916 cases by
    /// `experiments/relation-ir` and 300,436,169 by the two-oracle run.
    #[must_use]
    pub fn clear_reference_batch(&self) -> BatchInput {
        let slots = usize::from(self.module.params.slots);
        let mut inputs = vec![SlotInput::Empty; slots];
        for (at, index) in self.admitted.iter().enumerate() {
            let plain = self.submissions[*index].plain;
            let seq = usize::try_from(self.acks[at].seq).unwrap_or(usize::MAX);
            if seq >= slots {
                continue;
            }
            inputs[seq] = SlotInput::Occupied(OrderWitness {
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
        }
        BatchInput {
            requested_mode: RequestedMode::Clear,
            batch_id: BATCH_ID,
            market_id: MARKET_ID,
            cutoff: CUTOFF_EPOCH,
            accepted_input_root: self.cutoff.root,
            boundary: BoundaryStatements::SATISFIED,
            slots: inputs,
        }
    }

    /// Evaluate the Clear reference batch.
    #[must_use]
    pub fn clear_reference(&self) -> Outcome {
        Self::clear_evaluator(&self.module).evaluate(&self.clear_reference_batch())
    }

    /// The Clear evaluator for one module.
    #[must_use]
    pub fn clear_evaluator(module: &RelationModule) -> ClearEvaluator {
        lower(module, LoweringTarget::Clear).expect("the frozen module lowers")
    }
}
