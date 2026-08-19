//! Exact bounded semantics for a proposed confidential energy-dispatch relation.
//!
//! This crate is an offline **Clear** semantic oracle and a direct-recomputation
//! verifier. It contains no encryption, FHE, vFHE, MPC, proof system, network,
//! clock, custody, or delivery channel. The proposed Dark target is the relation
//! plus the frozen leakage surface in [`surface`], not this implementation.
//!
//! The relation is deliberately tiny and nonconvex. Three padded providers plan
//! three periods on two buses. Minimum output and an implicit on/off choice mean
//! a generic LP dual is not an optimality certificate. [`oracle`] therefore
//! enumerates the bounded trajectory domain, and [`verify::verify_candidate`]
//! recomputes that oracle. This is useful ground truth, but it is not a succinct
//! proof and is not cheaper than doing the relevant search again.

pub mod canonical;
pub mod model;
pub mod oracle;
pub mod sha256;
pub mod surface;
pub mod transcript;
pub mod verify;

pub use model::{
    BUSES, CostSegment, DispatchRequest, PERIODS, PROVIDERS, ProviderInput, PublicDomain,
    RELATION_ID, Witness,
};
pub use oracle::{CandidateBundle, Execution, Plan, PrivateDelivery, solve_clear};
pub use verify::{VerificationError, verify_candidate};
