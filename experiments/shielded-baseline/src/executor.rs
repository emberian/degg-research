//! The named executor: assembly, derivation, evaluation, publication.
//!
//! This is where the composition happens, and it happens in one place: the
//! executor never invents an input. It walks the committed positions of the
//! cutoff root in order, opens the payload each position commits to, and
//! builds the relation's [`BatchInput`] from what it finds. The canonical slot
//! index of `DARK_FBA_RELATION.md` section 5 *is* the log's `seq`, so the
//! residual-allocation rank of section 7 is fixed by the commitment rather
//! than by the executor.
//!
//! ## Booleans replaced by objects
//!
//! `DARK_FBA_RELATION.md` section 3 says plainly that the four boundary
//! statements are booleans supplied by the executor and are not proofs. Under
//! this composition each of the four is *derived* from an object, and
//! [`DerivedBoundary`] records which object:
//!
//! | Port | Derived from |
//! |---|---|
//! | `admission-log-final` | the observed [`CutoffRoot`] and the [`BatchMachine`] phase, so a withheld root is `cutoff-root-withheld`, not a `false` |
//! | `root-binds-slots` | one verified [`InclusionReceipt`] per position, plus the payload commitment opening at each occupied one |
//! | `no-conflicting-root` | the absence of a verified [`EquivocationVerdict`] for this log domain |
//! | `payloads-available` | the machine's availability reports against the domain threshold |
//!
//! Three of the four per-slot statements are derived the same way:
//! `included-under-root` from that slot's inclusion receipt,
//! `authorized` from the credential registry against the record's committed
//! `submitter`, and `eligible` from that credential being enrolled at all.
//! The fourth, `custody-bound`, is derived from the reserve ledger holding at
//! least the plaintext's reservation under that record's nullifier — which is
//! the weakest of the four, because the ledger is a map from nullifier to
//! integer and is bound to no external balance.
//!
//! None of that makes the statements *proofs*. Each is now the executor's
//! honest evaluation of a predicate over objects a third party could check if
//! it had them; the executor still evaluates it, alone, and publishes only the
//! result.

use std::collections::BTreeMap;

use degg_inclusion_availability::equivocation::EquivocationVerdict;
use degg_inclusion_availability::lifecycle::{BatchMachine, Phase, ReserveLedger};
use degg_inclusion_availability::log::{
    AdmissionRecord, CutoffRoot, InclusionReceipt, ReceiptDefect, verify_receipt,
};
use degg_relation_ir::batch::{
    BatchInput, BoundaryStatements, OrderWitness, RequestedMode, SlotInput,
};
use degg_relation_ir::canon::Canonical;
use degg_relation_ir::lower::{
    ClearEvaluator, ClearedTick, LoweringTarget, Outcome, PublicOutcome, lower,
};
use degg_relation_ir::module::{RelationModule, dark_fba_n4_k4_q15_v0};
use degg_relation_ir::receipt::ReceiptStatus;

use crate::receipt::{
    CutoffBinding, DeliveryCommitment, DeliveryEntry, ShieldedReceipt, SlotOutcome,
};
use crate::roles::{
    CredentialRegistry, ExecutorId, ExecutorKey, OwnerDeliveryKey, SealingCapability,
};
use crate::seal::{PlainOrder, SealDefect, SealedLocalOutput, SealedPayload};

/// One derived boundary or per-slot statement, with the object behind it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DerivedFact {
    /// The relation port this fact instantiates.
    pub port: &'static str,
    /// Whether the statement holds.
    pub holds: bool,
    /// The object the executor derived it from.
    pub object: &'static str,
}

/// The four boundary statements, derived.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DerivedBoundary {
    /// The statements as the relation consumes them.
    pub statements: BoundaryStatements,
    /// One record per statement, in the relation's frozen port order.
    pub facts: [DerivedFact; 4],
}

/// What the executor holds at compute time.
#[derive(Clone, Debug)]
pub struct ExecutorInputs {
    /// The published cutoff root.
    pub cutoff: CutoffRoot,
    /// One inclusion receipt per committed position, in position order.
    pub receipts: Vec<InclusionReceipt>,
    /// The sealed payload the executor received for each occupied position.
    pub payloads: BTreeMap<u32, SealedPayload>,
    /// Verified root-equivocation verdicts the executor knows about.
    pub known_verdicts: Vec<EquivocationVerdict>,
}

/// Why assembly refused before any evaluation.
///
/// These are structural: the executor could not build a witness at all. They
/// are disjoint from the relation's section 4.1 classes, which describe a
/// witness that exists and is inadmissible.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssemblyRefusal {
    /// The receipt vector does not cover exactly the committed positions.
    PositionCountMismatch {
        /// Receipts supplied.
        supplied: usize,
        /// Positions the cutoff root commits.
        committed: u64,
    },
    /// A receipt claims a position other than its index in the vector.
    PositionOutOfOrder {
        /// Index in the supplied vector.
        index: u32,
        /// Position the record claims.
        claimed: u32,
    },
    /// No payload was supplied for an occupied position.
    MissingPayload {
        /// The position.
        seq: u32,
    },
    /// The supplied payload does not open the commitment at this position.
    PayloadCommitmentMismatch {
        /// The position.
        seq: u32,
    },
    /// The payload did not decode to a canonical order.
    Payload {
        /// The position.
        seq: u32,
        /// The seal defect.
        defect: SealDefect,
    },
    /// The committed leaf count exceeds the relation module's slot capacity.
    CapacityMismatch {
        /// Positions committed.
        committed: u64,
        /// Slots the module declares.
        capacity: u8,
    },
}

/// A deliberate adversary handle: what a dishonest executor does to the batch
/// it assembles.
///
/// The honest interface is [`Tamper::None`]. Every other variant exists so the
/// detection tests can build a genuinely dishonest run rather than assert
/// against a mock, exactly as
/// [`degg_inclusion_availability::log::AdmissionLog::adversarially_append_after_cutoff`]
/// does upstream.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamper {
    /// Assemble honestly.
    None,
    /// Evaluate a different order at one committed position.
    SubstitutePlaintext {
        /// The position substituted.
        seq: u32,
        /// The order the executor pretends was committed there.
        plain: PlainOrder,
    },
    /// Treat one occupied position as an empty slot.
    OmitPosition {
        /// The position dropped.
        seq: u32,
    },
    /// Publish a delivery entry that the executor's own evaluation does not
    /// support.
    ///
    /// Distinct from a substitution: the assembled batch and the public result
    /// are the honest ones, and only the *committed local effect* is a lie.
    /// This is the dishonesty the fill-admissibility half of
    /// [`crate::owner::audit`] exists for, and the only route to a fill above
    /// an owner's own quantity: the reserve ledger and rule 18 together bound
    /// a substituted position's worst-case *obligation* by what was escrowed,
    /// so [`Tamper::SubstitutePlaintext`] can trade quantity against limit
    /// tick inside that budget but cannot buy an owner more than it paid for.
    ForgeSlotOutcome {
        /// The position whose entry is forged.
        seq: u32,
        /// The entry published there.
        outcome: SlotOutcome,
    },
}

/// One assembled batch, as the executor holds it.
///
/// `plains` is executor-visible order content and never crosses the public
/// boundary; it is `pub` because the executor role is inside this crate and
/// [`crate::SHIELDED_VISIBILITY_DISCLOSURE`] says what that means.
#[derive(Clone, Debug)]
pub struct Assembly {
    /// The relation's batch input.
    pub batch: BatchInput,
    /// The plaintext at each committed position, or `None` for a position the
    /// executor treated as empty.
    pub plains: Vec<Option<PlainOrder>>,
    /// The committed record at each position.
    pub records: Vec<AdmissionRecord>,
    /// The derived boundary statements.
    pub boundary: DerivedBoundary,
    /// The cutoff binding this assembly is against.
    pub cutoff: CutoffBinding,
}

/// One published Shielded run.
#[derive(Clone, Debug)]
pub struct ShieldedRun {
    /// The full outcome, including every private fill and owner output.
    ///
    /// Executor-visible. Publishing this object *is* publishing the book.
    pub outcome: Outcome,
    /// The public projection of the outcome.
    pub public: PublicOutcome,
    /// The published computation receipt.
    pub receipt: ShieldedReceipt,
    /// The per-position delivery commitment.
    pub delivery: DeliveryCommitment,
    /// One sealed local output per owner, on a settled run; empty on a refusal.
    pub deliveries: Vec<SealedLocalOutput>,
}

/// The named executor.
#[derive(Clone, Debug)]
pub struct Executor {
    id: ExecutorId,
    key: ExecutorKey,
    registry: CredentialRegistry,
    module: RelationModule,
    module_digest: [u8; 32],
    evaluator: ClearEvaluator,
}

impl Executor {
    /// Commission the named executor for the frozen relation module.
    ///
    /// The evaluator is `lower(module, LoweringTarget::Clear)`. It is the
    /// Clear evaluator, run by one named process: that is what
    /// [`degg_relation_ir::lower::LoweringRefusal::ShieldedBackendAbsent`]
    /// says a Shielded run is, and this crate does not pretend otherwise.
    #[must_use]
    pub fn commission(
        label: &str,
        seed: &[u8; 32],
        registry: CredentialRegistry,
    ) -> (Self, SealingCapability) {
        let module = dark_fba_n4_k4_q15_v0();
        let module_digest = module.digest();
        let evaluator = lower(&module, LoweringTarget::Clear).expect("the frozen module lowers");
        let (id, key) = ExecutorKey::commission(label, seed);
        let capability = key.grant_sealing();
        (
            Self {
                id,
                key,
                registry,
                module,
                module_digest,
                evaluator,
            },
            capability,
        )
    }

    /// The executor's public identity.
    #[must_use]
    pub fn id(&self) -> ExecutorId {
        self.id
    }

    /// The relation identity this executor runs.
    #[must_use]
    pub fn module_digest(&self) -> [u8; 32] {
        self.module_digest
    }

    /// The relation module this executor runs.
    #[must_use]
    pub fn module(&self) -> &RelationModule {
        &self.module
    }

    /// The delivery key one owner needs to open its own local output.
    #[must_use]
    pub fn delivery_key(&self, owner: u8) -> OwnerDeliveryKey {
        self.key.grant_delivery(owner)
    }

    /// Open one sealed payload. The capability that defines Shielded.
    pub fn open(&self, sealed: &SealedPayload) -> Result<PlainOrder, SealDefect> {
        sealed.open(&self.key)
    }

    /// Assemble the batch from the committed log positions.
    pub fn assemble(
        &self,
        inputs: &ExecutorInputs,
        ledger: &ReserveLedger,
        machine: &BatchMachine,
        tamper: &Tamper,
    ) -> Result<Assembly, AssemblyRefusal> {
        let cutoff = &inputs.cutoff;
        let capacity = self.module.params.slots;
        if cutoff.leaf_count != u64::from(capacity) {
            return Err(AssemblyRefusal::CapacityMismatch {
                committed: cutoff.leaf_count,
                capacity,
            });
        }
        if inputs.receipts.len() as u64 != cutoff.leaf_count {
            return Err(AssemblyRefusal::PositionCountMismatch {
                supplied: inputs.receipts.len(),
                committed: cutoff.leaf_count,
            });
        }

        let domain = cutoff.domain;
        let mut slots = Vec::with_capacity(inputs.receipts.len());
        let mut plains = Vec::with_capacity(inputs.receipts.len());
        let mut records = Vec::with_capacity(inputs.receipts.len());
        let mut every_position_binds = true;

        for (index, receipt) in inputs.receipts.iter().enumerate() {
            let seq = u32::try_from(index).expect("capacity is bounded by u32");
            if receipt.record.seq != seq {
                return Err(AssemblyRefusal::PositionOutOfOrder {
                    index: seq,
                    claimed: receipt.record.seq,
                });
            }
            records.push(receipt.record);
            let included: Result<(), ReceiptDefect> = verify_receipt(cutoff, receipt);
            if included.is_err() {
                every_position_binds = false;
            }
            let dropped =
                matches!(tamper, Tamper::OmitPosition { seq: dropped } if *dropped == seq);
            if receipt.record.is_padding(&domain) || dropped {
                slots.push(SlotInput::Empty);
                plains.push(None);
                continue;
            }
            let substituted = match tamper {
                Tamper::SubstitutePlaintext { seq: at, plain } if *at == seq => Some(*plain),
                _ => None,
            };
            let plain = match substituted {
                Some(plain) => plain,
                None => {
                    let sealed = inputs
                        .payloads
                        .get(&seq)
                        .ok_or(AssemblyRefusal::MissingPayload { seq })?;
                    if sealed.commitment() != receipt.record.payload_commitment {
                        return Err(AssemblyRefusal::PayloadCommitmentMismatch { seq });
                    }
                    self.open(sealed)
                        .map_err(|defect| AssemblyRefusal::Payload { seq, defect })?
                }
            };
            let enrolled = self.registry.owner_of(&receipt.record.submitter);
            let witness = OrderWitness {
                batch_id: plain.batch_id,
                market_id: plain.market_id,
                owner: plain.owner,
                side: plain.side,
                limit_tick: plain.limit_tick,
                quantity: plain.quantity,
                reserved: plain.reserved,
                nullifier: plain.nullifier,
                arrived_at: plain.arrived_at,
                authorized: enrolled == Some(plain.owner),
                eligible: enrolled.is_some(),
                included_under_root: included.is_ok(),
                custody_bound: ledger
                    .amount(&receipt.record.nullifier)
                    .is_some_and(|amount| amount >= plain.reserved),
            };
            slots.push(SlotInput::Occupied(witness));
            plains.push(Some(plain));
        }

        let boundary = derive_boundary(inputs, machine, every_position_binds);
        let batch = BatchInput {
            requested_mode: RequestedMode::ShieldedSingleExecutor,
            batch_id: domain.batch,
            market_id: domain.market,
            cutoff: domain.cutoff_epoch,
            accepted_input_root: cutoff.root,
            boundary: boundary.statements,
            slots,
        };
        Ok(Assembly {
            batch,
            plains,
            records,
            boundary,
            cutoff: CutoffBinding::of(cutoff),
        })
    }

    /// Evaluate an assembled batch and publish the run.
    #[must_use]
    pub fn execute(&self, assembly: &Assembly, tamper: &Tamper) -> ShieldedRun {
        let outcome = self.evaluator.evaluate(&assembly.batch);
        let public = outcome.public();
        let (entries, deliveries, status): (Vec<DeliveryEntry>, _, _) = match &outcome {
            Outcome::Settled(settled) => {
                let price = match settled.public.tick {
                    ClearedTick::NoTrade => 0,
                    ClearedTick::Tick(tick) => self.module.params.tick_prices[usize::from(tick)],
                };
                let entries = assembly
                    .plains
                    .iter()
                    .enumerate()
                    .map(|(index, plain)| {
                        let seq = u32::try_from(index).expect("capacity is bounded by u32");
                        match plain {
                            None => DeliveryEntry::NoLocalOutput,
                            Some(plain) => {
                                match SlotOutcome::derive(seq, plain, price, settled.fills[index]) {
                                    Some(outcome) => DeliveryEntry::Produced(outcome),
                                    // Unreachable for an outcome this evaluator
                                    // produced: its conservation audit already
                                    // refuses a fill above the quantity or a
                                    // reservation that cannot cover it.
                                    None => DeliveryEntry::NoLocalOutput,
                                }
                            }
                        }
                    })
                    .collect();
                let deliveries = settled
                    .owners
                    .iter()
                    .map(|owner| {
                        SealedLocalOutput::seal(&self.delivery_key(owner.owner), owner.clone())
                    })
                    .collect();
                (entries, deliveries, ReceiptStatus::Settled)
            }
            Outcome::Refused(refused) => (
                vec![DeliveryEntry::NoLocalOutput; assembly.plains.len()],
                Vec::new(),
                ReceiptStatus::Refused(refused.refusal.class),
            ),
        };
        let mut entries = entries;
        if let Tamper::ForgeSlotOutcome { seq, outcome } = tamper
            && let Some(slot) = entries.get_mut(usize::try_from(*seq).unwrap_or(usize::MAX))
        {
            *slot = DeliveryEntry::Produced(*outcome);
        }
        let delivery = DeliveryCommitment::build(&assembly.cutoff, &self.module_digest, entries);
        let receipt = ShieldedReceipt::new(
            self.id,
            self.module_digest,
            assembly.cutoff,
            assembly.batch.digest(),
            public.digest(),
            delivery.root(),
            status,
        );
        ShieldedRun {
            outcome,
            public,
            receipt,
            delivery,
            deliveries,
        }
    }
}

fn derive_boundary(
    inputs: &ExecutorInputs,
    machine: &BatchMachine,
    every_position_binds: bool,
) -> DerivedBoundary {
    let domain_digest = inputs.cutoff.domain.digest();
    let log_final = matches!(machine.phase(), Phase::Sealed | Phase::Computing)
        && machine.cutoff() == Some(inputs.cutoff);
    let no_conflicting_root = !inputs
        .known_verdicts
        .iter()
        .any(|verdict| verdict.domain_digest == domain_digest);
    let payloads_available = machine.first_unavailable().is_none();
    DerivedBoundary {
        statements: BoundaryStatements {
            log_final,
            root_binds_slots: every_position_binds,
            no_conflicting_root,
            payloads_available,
        },
        facts: [
            DerivedFact {
                port: "admission-log-final",
                holds: log_final,
                object: "observed CutoffRoot and BatchMachine phase",
            },
            DerivedFact {
                port: "root-binds-slots",
                holds: every_position_binds,
                object: "one verified InclusionReceipt per committed position",
            },
            DerivedFact {
                port: "no-conflicting-root",
                holds: no_conflicting_root,
                object: "absence of a verified EquivocationVerdict for this domain",
            },
            DerivedFact {
                port: "payloads-available",
                holds: payloads_available,
                object: "availability reports against the domain threshold",
            },
        ],
    }
}
