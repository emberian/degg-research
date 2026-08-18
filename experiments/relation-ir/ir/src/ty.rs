//! The IR's type table and visibility labels, as data.
//!
//! A visibility label is a typed annotation, not an enforcement mechanism. The
//! only lowering in this crate is Clear, and under it the executing process
//! sees every value regardless of label; see [`crate::lower`].

use crate::canon::{Canonical, Sink};

/// Who may learn a value in the module's target mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Visibility {
    /// Anyone may learn the value.
    Public,
    /// Only the owning participant may learn the value.
    PrivateToOwner,
    /// The named executor supplies or may learn the value.
    Executor,
}

impl Visibility {
    /// Canonical enum code.
    pub fn code(self) -> u32 {
        match self {
            Self::Public => 0,
            Self::PrivateToOwner => 1,
            Self::Executor => 2,
        }
    }
}

impl Canonical for Visibility {
    fn tag(&self) -> &'static str {
        "ir/visibility"
    }
    fn body(&self, sink: &mut Sink) {
        sink.u32(self.code());
    }
}

/// One type in the IR's bounded type vocabulary.
///
/// Every domain is statically bounded; there is no unbounded integer, dynamic
/// vector, or recursion through values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ty {
    /// A boolean.
    Bool,
    /// Unsigned 8-bit integer.
    U8,
    /// Unsigned 32-bit integer.
    U32,
    /// Unsigned 64-bit integer.
    U64,
    /// Signed 64-bit two's-complement integer, exact and non-modular.
    I64,
    /// A 32-byte digest.
    Digest256,
    /// An owner index below the module's owner count.
    OwnerIndex,
    /// A tick index below the module's tick count.
    TickIndex,
    /// A slot index below the module's slot capacity.
    SlotIndex,
    /// A quantity within an inclusive frozen range.
    QuantityInRange {
        /// Smallest admissible value.
        floor: u64,
        /// Largest admissible value.
        ceiling: u64,
    },
    /// A closed enumeration named by tag strings.
    EnumTag {
        /// Enumeration name.
        name: String,
        /// Variant tags in canonical order.
        variants: Vec<String>,
    },
    /// A fixed-length array.
    Array {
        /// Element type.
        element: Box<Ty>,
        /// Frozen length.
        length: u32,
    },
    /// A padded optional: canonical emptiness or one element, with no latent
    /// fields in the empty case.
    PaddedOption {
        /// Element type.
        element: Box<Ty>,
    },
    /// A named record with ordered, visibility-annotated fields.
    Record {
        /// Record name.
        name: String,
        /// Fields in canonical order.
        fields: Vec<FieldDecl>,
    },
}

impl Ty {
    fn code(&self) -> u32 {
        match self {
            Self::Bool => 0,
            Self::U8 => 1,
            Self::U32 => 2,
            Self::U64 => 3,
            Self::I64 => 4,
            Self::Digest256 => 5,
            Self::OwnerIndex => 6,
            Self::TickIndex => 7,
            Self::SlotIndex => 8,
            Self::QuantityInRange { .. } => 9,
            Self::EnumTag { .. } => 10,
            Self::Array { .. } => 11,
            Self::PaddedOption { .. } => 12,
            Self::Record { .. } => 13,
        }
    }
}

impl Canonical for Ty {
    fn tag(&self) -> &'static str {
        "ir/ty"
    }
    fn body(&self, sink: &mut Sink) {
        sink.u32(self.code());
        match self {
            Self::Bool
            | Self::U8
            | Self::U32
            | Self::U64
            | Self::I64
            | Self::Digest256
            | Self::OwnerIndex
            | Self::TickIndex
            | Self::SlotIndex => {}
            Self::QuantityInRange { floor, ceiling } => {
                sink.u64(*floor);
                sink.u64(*ceiling);
            }
            Self::EnumTag { name, variants } => {
                sink.str(name);
                sink.count(variants.len());
                for variant in variants {
                    sink.str(variant);
                }
            }
            Self::Array { element, length } => {
                sink.nested(element.as_ref());
                sink.u32(*length);
            }
            Self::PaddedOption { element } => {
                sink.nested(element.as_ref());
            }
            Self::Record { name, fields } => {
                sink.str(name);
                sink.count(fields.len());
                for field in fields {
                    sink.nested(field);
                }
            }
        }
    }
}

/// One named, visibility-annotated field of a record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldDecl {
    /// Field name.
    pub name: String,
    /// Who may learn the field in the target mode.
    pub visibility: Visibility,
    /// Field type.
    pub ty: Ty,
}

impl FieldDecl {
    /// Convenience constructor.
    pub fn new(name: &str, visibility: Visibility, ty: Ty) -> Self {
        Self {
            name: name.to_owned(),
            visibility,
            ty,
        }
    }
}

impl Canonical for FieldDecl {
    fn tag(&self) -> &'static str {
        "ir/field"
    }
    fn body(&self, sink: &mut Sink) {
        sink.str(&self.name);
        sink.nested(&self.visibility);
        sink.nested(&self.ty);
    }
}

/// One input or output port of a relation module.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PortDecl {
    /// Port name.
    pub name: String,
    /// Who may learn the port's value in the target mode.
    pub visibility: Visibility,
    /// Port type.
    pub ty: Ty,
}

impl PortDecl {
    /// Convenience constructor.
    pub fn new(name: &str, visibility: Visibility, ty: Ty) -> Self {
        Self {
            name: name.to_owned(),
            visibility,
            ty,
        }
    }
}

impl Canonical for PortDecl {
    fn tag(&self) -> &'static str {
        "ir/port"
    }
    fn body(&self, sink: &mut Sink) {
        sink.str(&self.name);
        sink.nested(&self.visibility);
        sink.nested(&self.ty);
    }
}
