use crate::{Domain, DomainScope};

// [assumption A1 — contract decomposition and names]
// [assumption A2 — trait ownership]
pub trait DomainTreeContract: Sized {
    type ScopeTree: ScopeTreeContract<DomainTree = Self>;
}

pub trait ScopeTreeContract: Sized {
    type DomainTree: DomainTreeContract<ScopeTree = Self>;
}

pub trait ScopeOf: DomainTreeContract {
    fn scope_of(&self) -> Self::ScopeTree;
}

// [assumption A3 — coherent relation operands]
pub trait ScopeContainment: ScopeTreeContract {
    fn contains_scope(&self, candidate: &Self) -> bool;
}

pub trait ScopeOverlap: ScopeTreeContract {
    fn overlaps_scope(&self, other: &Self) -> bool;
}

pub trait ScopeFiltering: ScopeTreeContract {
    fn matches_scope(&self, candidate: &Self) -> bool;
}

pub trait ScopeDomainMatching: ScopeTreeContract {
    fn matches_domain(&self, domain: &Self::DomainTree) -> bool;
}

impl DomainTreeContract for Domain {
    type ScopeTree = DomainScope;
}

impl ScopeTreeContract for DomainScope {
    type DomainTree = Domain;
}

impl ScopeOf for Domain {
    fn scope_of(&self) -> Self::ScopeTree {
        // [assumption A4 — total structural projection]
        self.clone().into()
    }
}

impl ScopeContainment for DomainScope {
    fn contains_scope(&self, candidate: &Self) -> bool {
        // [assumption A5 — API attachment of root All]
        // [assumption A6 — containment against All]
        // [assumption A7 — containment reflexivity]
        // [assumption A8 — ancestor containment]
        // [assumption A9 — reverse containment]
        // [assumption A10 — unrelated containment]
        // [assumption A18 — nested All]
        if self.is_all() {
            return true;
        }
        DomainScope::contains_scope(self, candidate)
    }
}

impl ScopeOverlap for DomainScope {
    fn overlaps_scope(&self, other: &Self) -> bool {
        // [assumption A5 — API attachment of root All]
        // [assumption A11 — overlap symmetry]
        <DomainScope as ScopeContainment>::contains_scope(self, other)
            || <DomainScope as ScopeContainment>::contains_scope(other, self)
    }
}

impl ScopeFiltering for DomainScope {
    fn matches_scope(&self, candidate: &Self) -> bool {
        // [assumption A5 — API attachment of root All]
        // [assumption A12 — filtering direction]
        <DomainScope as ScopeContainment>::contains_scope(self, candidate)
    }
}

impl ScopeDomainMatching for DomainScope {
    fn matches_domain(&self, domain: &Self::DomainTree) -> bool {
        // [assumption A17 — domain-side All]
        if domain.is_all() {
            return true;
        }

        // [assumption A5 — API attachment of root All]
        // [assumption A13 — exact scope-domain match]
        // [assumption A14 — ancestor scope-domain match]
        // [assumption A15 — reverse scope-domain match]
        // [assumption A16 — unrelated scope-domain match]
        // [assumption A18 — nested All]
        let candidate = <Domain as ScopeOf>::scope_of(domain);
        <DomainScope as ScopeContainment>::contains_scope(self, &candidate)
    }
}
