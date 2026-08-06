//! Signal's domain Interface, represented by its authority-seated encoded names.
//!
//! `schema/domain.schema` is the canonical textual projection. The compiled
//! Rust representation is a strict projection of the same verified bootstrap
//! transaction; handwritten Rust owns only behavior outside that Interface.

pub mod bootstrap_manifest;
mod scope;

pub mod schema;

pub use crate::schema::domain::*;
pub use crate::scope::{
    DomainTreeContract, Scope, ScopeContainment, ScopeDomainMatching, ScopeFiltering, ScopeOf,
    ScopeOverlap, ScopeTreeContract, ScopeValue,
};

/// Canonical textual projection of the domain Interface.
pub const DOMAIN_INTERFACE_SOURCE: &str = include_str!("../schema/domain.schema");

/// Checked-in strict Rust projection of the authority-verified Interface.
pub const DOMAIN_INTERFACE_RUST: &str = include_str!("schema/domain/generated.rs");
