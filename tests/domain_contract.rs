use signal_domain::{
    DataLeaf, DataLeafScope, Domain, DomainScope, EngineeringLeaf, EngineeringLeafScope,
    ScopeContainment, ScopeDomainMatching, ScopeFiltering, ScopeOf, ScopeOverlap, ScopeSet,
    Software, SoftwareScope, Technology, TechnologyScope,
};

fn schema_domain() -> Domain {
    Domain::Technology(Technology::Software(Software::Data(
        DataLeaf::SchemaEvolution,
    )))
}

fn schema_scope() -> DomainScope {
    DomainScope::Technology(TechnologyScope::Software(SoftwareScope::Data(
        DataLeafScope::SchemaEvolution,
    )))
}

fn architecture_domain() -> Domain {
    Domain::Technology(Technology::Software(Software::Engineering(
        EngineeringLeaf::Architecture,
    )))
}

fn architecture_scope() -> DomainScope {
    DomainScope::Technology(TechnologyScope::Software(SoftwareScope::Engineering(
        EngineeringLeafScope::Architecture,
    )))
}

fn engineering_all_domain() -> Domain {
    Domain::Technology(Technology::Software(Software::Engineering(
        EngineeringLeaf::All,
    )))
}

fn engineering_all_scope() -> DomainScope {
    DomainScope::Technology(TechnologyScope::Software(SoftwareScope::Engineering(
        EngineeringLeafScope::All,
    )))
}

#[test]
fn domain_round_trips_through_rkyv() {
    let domain = schema_domain();
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&domain).expect("archive domain");
    let decoded = rkyv::from_bytes::<Domain, rkyv::rancor::Error>(&bytes).expect("decode domain");

    assert_eq!(decoded, domain);
}

#[test]
fn all_domain_round_trips_through_rkyv() {
    let domain = Domain::All;
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&domain).expect("archive all domain");
    let decoded =
        rkyv::from_bytes::<Domain, rkyv::rancor::Error>(&bytes).expect("decode all domain");

    assert_eq!(decoded, domain);
}

#[test]
fn scope_of_projection_is_total_and_structure_preserving() {
    // [assumption A4 — total structural projection]
    // [assumption A18 — nested All]
    assert_eq!(
        <Domain as ScopeOf>::scope_of(&Domain::All),
        DomainScope::All
    );
    assert_eq!(
        <Domain as ScopeOf>::scope_of(&architecture_domain()),
        architecture_scope()
    );
    assert_eq!(
        <Domain as ScopeOf>::scope_of(&schema_domain()),
        schema_scope()
    );
    assert_eq!(
        <Domain as ScopeOf>::scope_of(&engineering_all_domain()),
        engineering_all_scope()
    );
}

#[test]
fn scope_containment_matches_reference_matrix() {
    // [assumption A5 — API attachment of root All]
    // [assumption A6 — containment against All]
    // [assumption A7 — containment reflexivity]
    // [assumption A8 — ancestor containment]
    // [assumption A9 — reverse containment]
    // [assumption A10 — unrelated containment]
    // [assumption A18 — nested All]
    let all = DomainScope::All;
    let architecture = architecture_scope();
    let engineering = engineering_all_scope();
    let schema = schema_scope();

    assert!(<DomainScope as ScopeContainment>::contains_scope(
        &all,
        &architecture
    ));
    assert!(!<DomainScope as ScopeContainment>::contains_scope(
        &architecture,
        &all
    ));
    assert!(<DomainScope as ScopeContainment>::contains_scope(
        &architecture,
        &architecture
    ));
    assert!(<DomainScope as ScopeContainment>::contains_scope(
        &engineering,
        &architecture
    ));
    assert!(!<DomainScope as ScopeContainment>::contains_scope(
        &architecture,
        &engineering
    ));
    assert!(!<DomainScope as ScopeContainment>::contains_scope(
        &schema,
        &architecture
    ));
    assert!(!<DomainScope as ScopeContainment>::contains_scope(
        &architecture,
        &schema
    ));
    assert!(!<DomainScope as ScopeContainment>::contains_scope(
        &engineering,
        &schema
    ));
}

#[test]
fn scope_overlap_matches_reference_matrix() {
    // [assumption A5 — API attachment of root All]
    // [assumption A7 — containment reflexivity]
    // [assumption A8 — ancestor containment]
    // [assumption A10 — unrelated containment]
    // [assumption A11 — overlap symmetry]
    let all = DomainScope::All;
    let architecture = architecture_scope();
    let engineering = engineering_all_scope();
    let schema = schema_scope();

    assert!(<DomainScope as ScopeOverlap>::overlaps_scope(
        &all,
        &architecture
    ));
    assert!(<DomainScope as ScopeOverlap>::overlaps_scope(
        &architecture,
        &all
    ));
    assert!(<DomainScope as ScopeOverlap>::overlaps_scope(
        &architecture,
        &architecture
    ));
    assert!(<DomainScope as ScopeOverlap>::overlaps_scope(
        &engineering,
        &architecture
    ));
    assert!(<DomainScope as ScopeOverlap>::overlaps_scope(
        &architecture,
        &engineering
    ));
    assert!(!<DomainScope as ScopeOverlap>::overlaps_scope(
        &schema,
        &architecture
    ));
    assert!(!<DomainScope as ScopeOverlap>::overlaps_scope(
        &architecture,
        &schema
    ));
}

#[test]
fn scope_filtering_matches_reference_matrix() {
    // [assumption A5 — API attachment of root All]
    // [assumption A6 — containment against All]
    // [assumption A7 — containment reflexivity]
    // [assumption A8 — ancestor containment]
    // [assumption A9 — reverse containment]
    // [assumption A10 — unrelated containment]
    // [assumption A12 — filtering direction]
    let all = DomainScope::All;
    let architecture = architecture_scope();
    let engineering = engineering_all_scope();
    let schema = schema_scope();

    assert!(<DomainScope as ScopeFiltering>::matches_scope(
        &all,
        &architecture
    ));
    assert!(!<DomainScope as ScopeFiltering>::matches_scope(
        &architecture,
        &all
    ));
    assert!(<DomainScope as ScopeFiltering>::matches_scope(
        &architecture,
        &architecture
    ));
    assert!(<DomainScope as ScopeFiltering>::matches_scope(
        &engineering,
        &architecture
    ));
    assert!(!<DomainScope as ScopeFiltering>::matches_scope(
        &architecture,
        &engineering
    ));
    assert!(!<DomainScope as ScopeFiltering>::matches_scope(
        &schema,
        &architecture
    ));
    assert!(!<DomainScope as ScopeFiltering>::matches_scope(
        &architecture,
        &schema
    ));
}

#[test]
fn scope_domain_matching_matches_reference_matrix() {
    // [assumption A5 — API attachment of root All]
    // [assumption A13 — exact scope-domain match]
    // [assumption A14 — ancestor scope-domain match]
    // [assumption A15 — reverse scope-domain match]
    // [assumption A16 — unrelated scope-domain match]
    // [assumption A17 — domain-side All]
    // [assumption A18 — nested All]
    let all = DomainScope::All;
    let architecture = architecture_scope();
    let engineering = engineering_all_scope();
    let schema = schema_scope();

    assert!(<DomainScope as ScopeDomainMatching>::matches_domain(
        &all,
        &architecture_domain()
    ));
    assert!(<DomainScope as ScopeDomainMatching>::matches_domain(
        &all,
        &Domain::All
    ));
    assert!(<DomainScope as ScopeDomainMatching>::matches_domain(
        &architecture,
        &Domain::All
    ));
    assert!(<DomainScope as ScopeDomainMatching>::matches_domain(
        &architecture,
        &architecture_domain()
    ));
    assert!(<DomainScope as ScopeDomainMatching>::matches_domain(
        &engineering,
        &architecture_domain()
    ));
    assert!(!<DomainScope as ScopeDomainMatching>::matches_domain(
        &architecture,
        &engineering_all_domain()
    ));
    assert!(!<DomainScope as ScopeDomainMatching>::matches_domain(
        &schema,
        &architecture_domain()
    ));
    assert!(!<DomainScope as ScopeDomainMatching>::matches_domain(
        &engineering,
        &schema_domain()
    ));
}

#[test]
fn domain_scopes_match_any_domain() {
    let domains = signal_domain::DomainScopes::new(vec![DomainScope::Technology(
        signal_domain::TechnologyScope::Software(signal_domain::SoftwareScope::Data(
            signal_domain::DataLeafScope::All,
        )),
    )]);
    let entry_domains = vec![schema_domain()];

    assert!(domains.matches_any_domain(&entry_domains));
}

#[test]
fn all_domain_scope_collection_matches_any_domain() {
    let concrete_domain = schema_domain();
    let all_scope = DomainScope::All;
    let all_scopes = signal_domain::DomainScopes::new(vec![all_scope]);
    let concrete_domains = vec![concrete_domain.clone()];

    assert!(all_scopes.matches_any_domain(&concrete_domains));
}

#[test]
fn scope_set_matches_any_domain() {
    let scope_set = ScopeSet::new(vec![architecture_scope()]);
    let architecture = Domain::Technology(Technology::Software(Software::Engineering(
        EngineeringLeaf::Architecture,
    )));
    let domains = vec![architecture];

    assert!(scope_set.matches_any_domain(&domains));
}

#[cfg(feature = "dotos-text")]
#[test]
fn domain_round_trips_through_dotos_text() {
    use dotos::{DotosEncode, DotosSource};

    let domain = schema_domain();
    let rendered = domain.to_dotos();
    let decoded = DotosSource::new(&rendered)
        .parse::<Domain>()
        .expect("decode domain DOTOS");

    assert_eq!(rendered, "Technology.Software.Data.SchemaEvolution");
    assert_eq!(decoded, domain);
}

#[cfg(feature = "dotos-text")]
#[test]
fn all_domain_round_trips_through_dotos_text() {
    use dotos::{DotosEncode, DotosSource};

    let domain = Domain::All;
    let rendered = domain.to_dotos();
    let decoded = DotosSource::new(&rendered)
        .parse::<Domain>()
        .expect("decode all domain DOTOS");

    assert_eq!(rendered, "All");
    assert_eq!(decoded, domain);
}

#[cfg(feature = "dotos-text")]
#[test]
fn scope_collection_round_trips_through_dotos_text() {
    use dotos::{DotosEncode, DotosSource};

    let scopes = signal_domain::DomainScopes::new(vec![DomainScope::from(schema_domain())]);
    let rendered = scopes.to_dotos();
    let decoded = DotosSource::new(&rendered)
        .parse::<signal_domain::DomainScopes>()
        .expect("decode domain scopes DOTOS");

    assert_eq!(decoded, scopes);
}
