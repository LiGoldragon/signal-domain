//! Rust-owned carrier for the provisional `ScopeOf<T>` Interface shape.

/// Structural behavior required of a value that can represent a scope.
///
/// Implementations compare the encoded variants directly. They never recover
/// textual names or construct a second taxonomy beside the generated one.
pub trait ScopeValue {
    /// Whether this value contains `candidate` in its structural subtree.
    fn contains_value(&self, candidate: &Self) -> bool;

    /// Whether this value is the `All` node for its structural level.
    fn is_all_value(&self) -> bool {
        false
    }
}

/// A scope expressed by a value in the scoped domain tree.
///
/// The wrapper is Rust behavior/representation chosen for the explicit
/// `ScopeOf` shape seat; it is not a second declaration of any Interface type.
#[derive(Clone, Debug, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct Scope<T>(T);

impl<T> Scope<T> {
    /// Construct the scope represented by `value`.
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    /// Borrow the value that represents this scope.
    pub const fn value(&self) -> &T {
        &self.0
    }

    /// Recover the represented value.
    pub fn into_value(self) -> T {
        self.0
    }
}

impl<T: ScopeValue> Scope<T> {
    /// Whether this scope contains `candidate`.
    pub fn contains_scope(&self, candidate: &Self) -> bool {
        self.0.contains_value(&candidate.0)
    }

    /// Whether this scope contains the exact domain value or one of its
    /// structural descendants.
    pub fn matches_value(&self, candidate: &T) -> bool {
        self.0.contains_value(candidate)
    }

    /// Whether this is the root `All` scope.
    pub fn is_all(&self) -> bool {
        self.0.is_all_value()
    }
}

/// Associates a domain tree with its scope carrier.
pub trait DomainTreeContract: ScopeValue + Sized {
    /// Scope type for this domain tree.
    type ScopeTree: ScopeTreeContract<DomainTree = Self>;
}

/// Associates a scope carrier with its domain tree.
pub trait ScopeTreeContract: Sized {
    /// Domain tree represented by this scope.
    type DomainTree: DomainTreeContract<ScopeTree = Self>;
}

/// Total, consuming conversion from a domain value into its scope.
pub trait ScopeOf: DomainTreeContract {
    /// Wrap this exact domain value as a scope without cloning or translation.
    fn scope_of(self) -> Self::ScopeTree;
}

/// Directional structural containment between scopes.
pub trait ScopeContainment: ScopeTreeContract {
    /// Whether this scope contains `candidate`.
    fn contains_scope(&self, candidate: &Self) -> bool;
}

/// Symmetric structural overlap between scopes.
pub trait ScopeOverlap: ScopeTreeContract {
    /// Whether either scope contains the other.
    fn overlaps_scope(&self, other: &Self) -> bool;
}

/// Scope-to-scope filtering behavior.
pub trait ScopeFiltering: ScopeTreeContract {
    /// Whether `candidate` is selected by this scope.
    fn matches_scope(&self, candidate: &Self) -> bool;
}

/// Scope-to-domain matching behavior.
pub trait ScopeDomainMatching: ScopeTreeContract {
    /// Whether this scope selects `domain`.
    fn matches_domain(&self, domain: &Self::DomainTree) -> bool;
}
