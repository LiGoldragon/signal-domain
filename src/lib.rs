//! Signal's authored domain Interface source.
//!
//! `schema/domain.schema` is retained as the future durable-authority input.
//! Its Rust projection is intentionally absent until hqu.30 can commit
//! authority state and install one durable projection atomically.

mod scope;

pub mod schema;

pub use crate::schema::domain::*;
pub use crate::scope::{
    DomainTreeContract, Scope, ScopeContainment, ScopeDomainMatching, ScopeFiltering, ScopeOf,
    ScopeOverlap, ScopeTreeContract, ScopeValue,
};

/// Canonical textual projection of the domain Interface.
pub const DOMAIN_INTERFACE_SOURCE: &str = include_str!("../schema/domain.schema");
