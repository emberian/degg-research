//! The relation module: the whole relation as one canonical data value.
//!
//! [`dark_fba_n4_k4_q15_v0`] is the frozen instance. Its digest names the
//! relation identity: changing any field, including the admission-check order,
//! produces a different digest and therefore a different relation.

use crate::canon::{Canonical, Sink};
use crate::policy::{AdmissionPolicy, frozen_v0_check_order};
use crate::receipt::{ReceiptDomain, ReceiptShape, ShapeStatus};
use crate::refusal::{REFUSAL_CLASSES, RefusalClass};
use crate::ty::{FieldDecl, PortDecl, Ty, Visibility};

/// Module identity fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleIdentity {
    /// Full relation identifier, namespace/parameters/version.
    pub relation: String,
    /// Relation namespace.
    pub namespace: String,
    /// Semantic version.
    pub version: String,
    /// Canonical encoder version for every byte string this module names.
    pub encoder: String,
    /// Repository-relative paths of the specification documents this module
    /// restates. Recorded as paths, not content digests: the documents gain
    /// dated addenda, and the frozen semantic content is this module's own
    /// data, whose digest is the identity anchor.
    pub spec_sources: Vec<String>,
}

impl Canonical for ModuleIdentity {
    fn tag(&self) -> &'static str {
        "ir/module-identity"
    }
    fn body(&self, sink: &mut Sink) {
        sink.str(&self.relation);
        sink.str(&self.namespace);
        sink.str(&self.version);
        sink.str(&self.encoder);
        sink.count(self.spec_sources.len());
        for source in &self.spec_sources {
            sink.str(source);
        }
    }
}

/// Frozen numeric parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Params {
    /// Number of distinct owners.
    pub owners: u8,
    /// Padded slot capacity.
    pub slots: u8,
    /// Quote atoms per base atom, indexed by tick.
    pub tick_prices: Vec<u64>,
    /// Smallest admissible quantity for an occupied slot.
    pub quantity_floor: u64,
    /// Largest admissible quantity for an occupied slot.
    pub quantity_ceiling: u64,
    /// The fee policy.
    pub fee_policy: FeePolicy,
}

impl Params {
    /// Number of price ticks.
    pub fn ticks(&self) -> usize {
        self.tick_prices.len()
    }
}

impl Canonical for Params {
    fn tag(&self) -> &'static str {
        "ir/params"
    }
    fn body(&self, sink: &mut Sink) {
        sink.u8(self.owners);
        sink.u8(self.slots);
        sink.count(self.tick_prices.len());
        for price in &self.tick_prices {
            sink.u64(*price);
        }
        sink.u64(self.quantity_floor);
        sink.u64(self.quantity_ceiling);
        sink.nested(&self.fee_policy);
    }
}

/// The fee policy. v0 is exactly zero fees.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeePolicy {
    /// No fees anywhere; conservation is exact with zero named outflows.
    Zero,
}

impl Canonical for FeePolicy {
    fn tag(&self) -> &'static str {
        "ir/fee-policy"
    }
    fn body(&self, sink: &mut Sink) {
        sink.u32(match self {
            Self::Zero => 0,
        });
    }
}

/// The price objective.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PriceObjective {
    /// Maximize matched volume.
    MaximizeVolume,
}

/// The price tie rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PriceTie {
    /// Ties select the lowest tick index.
    LowestTick,
}

/// The clearing rule as data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClearingSpec {
    /// Objective over the volume curve.
    pub objective: PriceObjective,
    /// Tie rule among maximizing ticks.
    pub tie: PriceTie,
}

impl Canonical for ClearingSpec {
    fn tag(&self) -> &'static str {
        "ir/clearing-spec"
    }
    fn body(&self, sink: &mut Sink) {
        sink.u32(match self.objective {
            PriceObjective::MaximizeVolume => 0,
        });
        sink.u32(match self.tie {
            PriceTie::LowestTick => 0,
        });
    }
}

/// The allocation method.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AllocationMethod {
    /// Exact largest-remainder pro-rata over every eligible order per side.
    LargestRemainder,
}

/// The residual tie rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResidualTie {
    /// Equal remainders resolve to the earliest canonical slot index.
    EarliestCanonicalSlot,
}

/// The allocation rule as data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AllocationSpec {
    /// Apportionment method.
    pub method: AllocationMethod,
    /// Residual tie rule.
    pub residual_tie: ResidualTie,
}

impl Canonical for AllocationSpec {
    fn tag(&self) -> &'static str {
        "ir/allocation-spec"
    }
    fn body(&self, sink: &mut Sink) {
        sink.u32(match self.method {
            AllocationMethod::LargestRemainder => 0,
        });
        sink.u32(match self.residual_tie {
            ResidualTie::EarliestCanonicalSlot => 0,
        });
    }
}

/// One whole relation module.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RelationModule {
    /// Identity fields.
    pub identity: ModuleIdentity,
    /// Frozen numeric parameters.
    pub params: Params,
    /// Input ports with visibility annotations.
    pub inputs: Vec<PortDecl>,
    /// Output ports with visibility annotations.
    pub outputs: Vec<PortDecl>,
    /// Admission-check priority as data.
    pub admission: AdmissionPolicy,
    /// Clearing rule.
    pub clearing: ClearingSpec,
    /// Allocation rule.
    pub allocation: AllocationSpec,
    /// The full refusal-class vocabulary in canonical order.
    pub refusal_classes: Vec<RefusalClass>,
    /// Receipt shapes, with emission status.
    pub receipts: Vec<ReceiptShape>,
}

impl Canonical for RelationModule {
    fn tag(&self) -> &'static str {
        "ir/relation-module"
    }
    fn body(&self, sink: &mut Sink) {
        sink.nested(&self.identity);
        sink.nested(&self.params);
        sink.count(self.inputs.len());
        for port in &self.inputs {
            sink.nested(port);
        }
        sink.count(self.outputs.len());
        for port in &self.outputs {
            sink.nested(port);
        }
        sink.nested(&self.admission);
        sink.nested(&self.clearing);
        sink.nested(&self.allocation);
        sink.count(self.refusal_classes.len());
        for class in &self.refusal_classes {
            sink.nested(class);
        }
        sink.count(self.receipts.len());
        for shape in &self.receipts {
            sink.nested(shape);
        }
    }
}

fn order_witness_ty() -> Ty {
    let statements = [
        ("authorized", "authorization statement"),
        ("eligible", "eligibility statement"),
        ("included-under-root", "exact-inclusion statement"),
        ("custody-bound", "custody-binding statement"),
    ];
    let mut fields = vec![
        FieldDecl::new("batch-id", Visibility::PrivateToOwner, Ty::U64),
        FieldDecl::new("market-id", Visibility::PrivateToOwner, Ty::U64),
        FieldDecl::new("owner", Visibility::PrivateToOwner, Ty::OwnerIndex),
        FieldDecl::new(
            "side",
            Visibility::PrivateToOwner,
            Ty::EnumTag {
                name: "side".to_owned(),
                variants: vec!["buy".to_owned(), "sell".to_owned()],
            },
        ),
        FieldDecl::new("limit-tick", Visibility::PrivateToOwner, Ty::TickIndex),
        FieldDecl::new(
            "quantity",
            Visibility::PrivateToOwner,
            Ty::QuantityInRange {
                floor: 1,
                ceiling: 15,
            },
        ),
        FieldDecl::new("reserved", Visibility::PrivateToOwner, Ty::U64),
        FieldDecl::new("nullifier", Visibility::PrivateToOwner, Ty::U64),
        FieldDecl::new("arrived-at", Visibility::PrivateToOwner, Ty::U64),
    ];
    for (name, _) in statements {
        fields.push(FieldDecl::new(name, Visibility::Executor, Ty::Bool));
    }
    Ty::Record {
        name: "order-witness".to_owned(),
        fields,
    }
}

fn cleared_tick_ty() -> Ty {
    Ty::EnumTag {
        name: "cleared-tick".to_owned(),
        variants: vec![
            "no-trade".to_owned(),
            "tick-0".to_owned(),
            "tick-1".to_owned(),
            "tick-2".to_owned(),
            "tick-3".to_owned(),
        ],
    }
}

fn owner_output_ty() -> Ty {
    Ty::Record {
        name: "owner-output".to_owned(),
        fields: vec![
            FieldDecl::new("owner", Visibility::PrivateToOwner, Ty::OwnerIndex),
            FieldDecl::new("bought", Visibility::PrivateToOwner, Ty::U64),
            FieldDecl::new("sold", Visibility::PrivateToOwner, Ty::U64),
            FieldDecl::new("base-delta", Visibility::PrivateToOwner, Ty::I64),
            FieldDecl::new("quote-delta", Visibility::PrivateToOwner, Ty::I64),
            FieldDecl::new(
                "released-base-reservation",
                Visibility::PrivateToOwner,
                Ty::U64,
            ),
            FieldDecl::new(
                "released-quote-reservation",
                Visibility::PrivateToOwner,
                Ty::U64,
            ),
            FieldDecl::new(
                "owned-slot-fills",
                Visibility::PrivateToOwner,
                Ty::Array {
                    element: Box::new(Ty::PaddedOption {
                        element: Box::new(Ty::U64),
                    }),
                    length: 4,
                },
            ),
        ],
    }
}

fn receipt_shapes() -> Vec<ReceiptShape> {
    let digest = |name: &str| PortDecl::new(name, Visibility::Public, Ty::Digest256);
    vec![
        ReceiptShape {
            domain: ReceiptDomain::SourceAdmission,
            status: ShapeStatus::DeclaredOnly,
            fields: vec![],
        },
        ReceiptShape {
            domain: ReceiptDomain::Computation,
            status: ShapeStatus::EmittedByClearLowering,
            fields: vec![
                digest("module-digest"),
                digest("input-digest"),
                digest("outcome-digest"),
                PortDecl::new(
                    "status",
                    Visibility::Public,
                    Ty::EnumTag {
                        name: "receipt-status".to_owned(),
                        variants: vec!["settled".to_owned(), "refused".to_owned()],
                    },
                ),
            ],
        },
        ReceiptShape {
            domain: ReceiptDomain::ProofVerification,
            status: ShapeStatus::DeclaredOnly,
            fields: vec![],
        },
        ReceiptShape {
            domain: ReceiptDomain::OutputDelivery,
            status: ShapeStatus::EmittedByClearLowering,
            fields: vec![
                PortDecl::new("owner", Visibility::Executor, Ty::OwnerIndex),
                PortDecl::new("module-digest", Visibility::Executor, Ty::Digest256),
                PortDecl::new("input-digest", Visibility::Executor, Ty::Digest256),
                PortDecl::new("output-digest", Visibility::Executor, Ty::Digest256),
            ],
        },
        ReceiptShape {
            domain: ReceiptDomain::Settlement,
            status: ShapeStatus::DeclaredOnly,
            fields: vec![],
        },
        ReceiptShape {
            domain: ReceiptDomain::Disclosure,
            status: ShapeStatus::DeclaredOnly,
            fields: vec![],
        },
        ReceiptShape {
            domain: ReceiptDomain::Retention,
            status: ShapeStatus::DeclaredOnly,
            fields: vec![],
        },
    ]
}

/// The frozen IR instance for `dark-fba/n4-k4-q15/v0`.
pub fn dark_fba_n4_k4_q15_v0() -> RelationModule {
    RelationModule {
        identity: ModuleIdentity {
            relation: "dark-fba/n4-k4-q15/v0".to_owned(),
            namespace: "dark-fba".to_owned(),
            version: "v0".to_owned(),
            encoder: "degg-cbe/v1".to_owned(),
            spec_sources: vec![
                "relations/CLEARING_V0.md".to_owned(),
                "docs/research/DARK_FBA_RELATION.md".to_owned(),
            ],
        },
        params: Params {
            owners: 4,
            slots: 4,
            tick_prices: vec![1, 2, 3, 4],
            quantity_floor: 1,
            quantity_ceiling: 15,
            fee_policy: FeePolicy::Zero,
        },
        inputs: vec![
            PortDecl::new("batch-id", Visibility::Public, Ty::U64),
            PortDecl::new("market-id", Visibility::Public, Ty::U64),
            PortDecl::new("cutoff", Visibility::Public, Ty::U64),
            PortDecl::new("accepted-input-root", Visibility::Public, Ty::Digest256),
            PortDecl::new(
                "requested-mode",
                Visibility::Public,
                Ty::EnumTag {
                    name: "requested-mode".to_owned(),
                    variants: vec![
                        "clear".to_owned(),
                        "shielded-single-executor".to_owned(),
                        "dark-target".to_owned(),
                    ],
                },
            ),
            PortDecl::new("admission-log-final", Visibility::Executor, Ty::Bool),
            PortDecl::new("root-binds-slots", Visibility::Executor, Ty::Bool),
            PortDecl::new("no-conflicting-root", Visibility::Executor, Ty::Bool),
            PortDecl::new("payloads-available", Visibility::Executor, Ty::Bool),
            PortDecl::new(
                "slots",
                Visibility::PrivateToOwner,
                Ty::Array {
                    element: Box::new(Ty::PaddedOption {
                        element: Box::new(order_witness_ty()),
                    }),
                    length: 4,
                },
            ),
        ],
        outputs: vec![
            PortDecl::new(
                "public-result",
                Visibility::Public,
                Ty::Record {
                    name: "public-result".to_owned(),
                    fields: vec![
                        FieldDecl::new("batch-id", Visibility::Public, Ty::U64),
                        FieldDecl::new("market-id", Visibility::Public, Ty::U64),
                        FieldDecl::new("accepted-input-root", Visibility::Public, Ty::Digest256),
                        FieldDecl::new("tick", Visibility::Public, cleared_tick_ty()),
                        FieldDecl::new("volume", Visibility::Public, Ty::U64),
                    ],
                },
            ),
            PortDecl::new(
                "owner-outputs",
                Visibility::PrivateToOwner,
                Ty::Array {
                    element: Box::new(owner_output_ty()),
                    length: 4,
                },
            ),
            PortDecl::new(
                "refusal-class",
                Visibility::Public,
                Ty::EnumTag {
                    name: "refusal-class".to_owned(),
                    variants: REFUSAL_CLASSES
                        .iter()
                        .map(|class| class.tag().to_owned())
                        .collect(),
                },
            ),
        ],
        admission: frozen_v0_check_order(),
        clearing: ClearingSpec {
            objective: PriceObjective::MaximizeVolume,
            tie: PriceTie::LowestTick,
        },
        allocation: AllocationSpec {
            method: AllocationMethod::LargestRemainder,
            residual_tie: ResidualTie::EarliestCanonicalSlot,
        },
        refusal_classes: REFUSAL_CLASSES.to_vec(),
        receipts: receipt_shapes(),
    }
}
