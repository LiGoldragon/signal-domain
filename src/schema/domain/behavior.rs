use crate::scope::{
    DomainTreeContract, Scope, ScopeContainment, ScopeDomainMatching, ScopeFiltering, ScopeOf,
    ScopeOverlap, ScopeTreeContract, ScopeValue,
};

macro_rules! exact_scope_values {
    ($($value:ty),+ $(,)?) => {
        $(
            impl ScopeValue for $value {
                fn contains_value(&self, candidate: &Self) -> bool {
                    std::mem::discriminant(self) == std::mem::discriminant(candidate)
                }
            }
        )+
    };
}

macro_rules! scope_values_with_all {
    ($(($value:ty, $all:path)),+ $(,)?) => {
        $(
            impl ScopeValue for $value {
                fn contains_value(&self, candidate: &Self) -> bool {
                    matches!(self, $all)
                        || std::mem::discriminant(self) == std::mem::discriminant(candidate)
                }

                fn is_all_value(&self) -> bool {
                    matches!(self, $all)
                }
            }
        )+
    };
}

// All-unit levels compare their encoded variants directly. This keeps the
// taxonomy singular: there is no handwritten mirror of the 369 variants.
exact_scope_values!(
    z2Vetb, z2VSfA, z2VdJT, z2VW7d, z2Vd44, z2VMFi, z2VXTW, z2VTbU, z2Vcwy, z2VUUJ, z2VZgf,
    z2VacB, z2VUoi, z2VU57, z2VU74, z2VT9a, z2Vefz, z2VaNs, z2Vdqc, z2VRd6, z2VQsy, z2Vd5x,
    z2VX82,
);

scope_values_with_all!(
    (z2VMQR, z2VMQR::z2VTRx),
    (z2VNHr, z2VNHr::z2VdAM),
    (z2VcT9, z2VcT9::z2Ve89),
    (z2VPzQ, z2VPzQ::z2VVRt),
    (z2VaYC, z2VaYC::z2Vb7s),
    (z2VdPi, z2VdPi::z2VcjX),
    (z2VNs2, z2VNs2::z2VMKR),
    (z2VWfG, z2VWfG::z2VV74),
    (z2VXVk, z2VXVk::z2VdB8),
    (z2Vb1K, z2Vb1K::z2VNH6),
    (z2VXd9, z2VXd9::z2VUuV),
    (z2VdTi, z2VdTi::z2VMmz),
);

impl ScopeValue for z2VYDN {
    fn contains_value(&self, candidate: &Self) -> bool {
        match (self, candidate) {
            (Self::z2VYvf(left), Self::z2VYvf(right)) => left.contains_value(right),
            (Self::z2VSAm(left), Self::z2VSAm(right)) => left.contains_value(right),
            _ => false,
        }
    }
}

impl ScopeValue for z2VRXn {
    fn contains_value(&self, candidate: &Self) -> bool {
        match (self, candidate) {
            (Self::z2Vdwx(left), Self::z2Vdwx(right)) => left.contains_value(right),
            (Self::z2Vcan(left), Self::z2Vcan(right)) => left.contains_value(right),
            (Self::z2Vbie(left), Self::z2Vbie(right)) => left.contains_value(right),
            (Self::z2VXQN(left), Self::z2VXQN(right)) => left.contains_value(right),
            (Self::z2VRSY(left), Self::z2VRSY(right)) => left.contains_value(right),
            (Self::z2VXMU(left), Self::z2VXMU(right)) => left.contains_value(right),
            (Self::z2VdbC(left), Self::z2VdbC(right)) => left.contains_value(right),
            (Self::z2VQ4o(left), Self::z2VQ4o(right)) => left.contains_value(right),
            (Self::z2VeeD(left), Self::z2VeeD(right)) => left.contains_value(right),
            (Self::z2VYxD(left), Self::z2VYxD(right)) => left.contains_value(right),
            (Self::z2VNjS(left), Self::z2VNjS(right)) => left.contains_value(right),
            (Self::z2VWB4, Self::z2VWB4) => true,
            _ => false,
        }
    }
}

impl ScopeValue for z2Vefc {
    fn contains_value(&self, candidate: &Self) -> bool {
        if self.is_all_value() {
            return true;
        }

        match (self, candidate) {
            (Self::z2VcfC(left), Self::z2VcfC(right)) => left.contains_value(right),
            (Self::z2VQi1(left), Self::z2VQi1(right)) => left.contains_value(right),
            (Self::z2Vde5(left), Self::z2Vde5(right)) => left.contains_value(right),
            (Self::z2VYYE(left), Self::z2VYYE(right)) => left.contains_value(right),
            (Self::z2VQvx(left), Self::z2VQvx(right)) => left.contains_value(right),
            (Self::z2VN6x(left), Self::z2VN6x(right)) => left.contains_value(right),
            (Self::z2VZCp(left), Self::z2VZCp(right)) => left.contains_value(right),
            (Self::z2VRrb(left), Self::z2VRrb(right)) => left.contains_value(right),
            (Self::z2Vcb2(left), Self::z2Vcb2(right)) => left.contains_value(right),
            (Self::z2VYRf(left), Self::z2VYRf(right)) => left.contains_value(right),
            (Self::z2VZoY(left), Self::z2VZoY(right)) => left.contains_value(right),
            (Self::z2VZnN(left), Self::z2VZnN(right)) => left.contains_value(right),
            (Self::z2VSk1(left), Self::z2VSk1(right)) => left.contains_value(right),
            (Self::z2Vcsv(left), Self::z2Vcsv(right)) => left.contains_value(right),
            (Self::z2VQsA(left), Self::z2VQsA(right)) => left.contains_value(right),
            (Self::z2VNdy(left), Self::z2VNdy(right)) => left.contains_value(right),
            (Self::z2Vaqc(left), Self::z2Vaqc(right)) => left.contains_value(right),
            (Self::z2VP6j(left), Self::z2VP6j(right)) => left.contains_value(right),
            (Self::z2VZdc(left), Self::z2VZdc(right)) => left.contains_value(right),
            (Self::z2VYHB(left), Self::z2VYHB(right)) => left.contains_value(right),
            (Self::z2Vd2R(left), Self::z2Vd2R(right)) => left.contains_value(right),
            (Self::z2VVd2(left), Self::z2VVd2(right)) => left.contains_value(right),
            (Self::z2VSih(left), Self::z2VSih(right)) => left.contains_value(right),
            (Self::z2VM7z(left), Self::z2VM7z(right)) => left.contains_value(right),
            _ => false,
        }
    }

    fn is_all_value(&self) -> bool {
        matches!(self, Self::z2VTcf)
    }
}

impl DomainTreeContract for z2Vefc {
    type ScopeTree = z2VNQR;
}

impl ScopeTreeContract for z2VNQR {
    type DomainTree = z2Vefc;
}

impl ScopeOf for z2Vefc {
    fn scope_of(self) -> Self::ScopeTree {
        self.into()
    }
}

impl From<z2Vefc> for z2VNQR {
    fn from(domain: z2Vefc) -> Self {
        Self::new(Scope::new(domain))
    }
}

impl ScopeContainment for z2VNQR {
    fn contains_scope(&self, candidate: &Self) -> bool {
        self.payload().contains_scope(candidate.payload())
    }
}

impl ScopeOverlap for z2VNQR {
    fn overlaps_scope(&self, other: &Self) -> bool {
        ScopeContainment::contains_scope(self, other)
            || ScopeContainment::contains_scope(other, self)
    }
}

impl ScopeFiltering for z2VNQR {
    fn matches_scope(&self, candidate: &Self) -> bool {
        ScopeContainment::contains_scope(self, candidate)
    }
}

impl ScopeDomainMatching for z2VNQR {
    fn matches_domain(&self, domain: &Self::DomainTree) -> bool {
        domain.is_all_value() || self.payload().matches_value(domain)
    }
}

impl z2Vefc {
    /// Move this domain value into its generated `ScopeOf` carrier.
    pub fn into_scope(self) -> z2VNQR {
        ScopeOf::scope_of(self)
    }

    /// Whether this domain value is selected by `scope`.
    pub fn matches_scope(&self, scope_: &z2VNQR) -> bool {
        ScopeDomainMatching::matches_domain(scope_, self)
    }

    /// Whether this is the root `All` domain value.
    pub fn is_all(&self) -> bool {
        self.is_all_value()
    }
}

impl z2VNQR {
    /// Borrow the encoded domain value represented by this scope.
    pub fn domain(&self) -> &z2Vefc {
        self.payload().value()
    }

    /// Recover the encoded domain value represented by this scope.
    pub fn into_domain(self) -> z2Vefc {
        self.into_payload().into_value()
    }

    /// Whether this scope contains `candidate`.
    pub fn contains_scope(&self, candidate: &Self) -> bool {
        ScopeContainment::contains_scope(self, candidate)
    }

    /// Whether this scope overlaps `other`.
    pub fn overlaps_scope(&self, other: &Self) -> bool {
        ScopeOverlap::overlaps_scope(self, other)
    }

    /// Whether this scope selects `candidate`.
    pub fn matches_scope(&self, candidate: &Self) -> bool {
        ScopeFiltering::matches_scope(self, candidate)
    }

    /// Whether this scope selects `domain`.
    pub fn matches_domain(&self, domain: &z2Vefc) -> bool {
        ScopeDomainMatching::matches_domain(self, domain)
    }

    /// Whether this is the root `All` scope.
    pub fn is_all(&self) -> bool {
        self.payload().is_all()
    }
}

impl z2VZQa {
    /// Iterate over the generated scope payload.
    pub fn iter(&self) -> impl Iterator<Item = &z2VNQR> {
        self.payload().iter()
    }

    /// Whether any scope selects any candidate domain.
    pub fn matches_any_domain(&self, domains: &[z2Vefc]) -> bool {
        self.iter()
            .any(|scope_| domains.iter().any(|domain| scope_.matches_domain(domain)))
    }
}

impl z2VUtf {
    /// Iterate over the generated scope payload.
    pub fn iter(&self) -> impl Iterator<Item = &z2VNQR> {
        self.payload().iter()
    }

    /// Whether any scope selects `domain`.
    pub fn matches_domain(&self, domain: &z2Vefc) -> bool {
        self.iter().any(|scope_| scope_.matches_domain(domain))
    }

    /// Whether any scope selects any candidate domain.
    pub fn matches_any_domain(&self, domains: &[z2Vefc]) -> bool {
        domains.iter().any(|domain| self.matches_domain(domain))
    }
}
