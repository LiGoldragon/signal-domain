use signal_domain::{
    DataLeaf, Domain, DomainScope, EngineeringLeaf, ScopeSet, Software, Technology,
};

fn schema_domain() -> Domain {
    Domain::Technology(Technology::Software(Software::Data(
        DataLeaf::SchemaEvolution,
    )))
}

fn architecture_scope() -> DomainScope {
    DomainScope::Technology(signal_domain::TechnologyScope::Software(
        signal_domain::SoftwareScope::Engineering(
            signal_domain::EngineeringLeafScope::Architecture,
        ),
    ))
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
fn domain_scope_matches_its_domain() {
    let domain = schema_domain();
    let scope = DomainScope::from(domain.clone());

    assert!(scope.contains_domain(&domain));
    assert!(domain.matches_scope(&scope));
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
fn all_domain_scope_matches_any_domain() {
    let concrete_domain = schema_domain();
    let all_domain = Domain::All;
    let all_scope = DomainScope::All;
    let all_scopes = signal_domain::DomainScopes::new(vec![all_scope.clone()]);
    let concrete_domains = vec![concrete_domain.clone()];

    assert_eq!(DomainScope::from(all_domain.clone()), all_scope);
    assert!(all_scope.matches_domain(&concrete_domain));
    assert!(concrete_domain.matches_scope(&all_scope));
    assert!(all_scope.matches_scope(&architecture_scope()));
    assert!(architecture_scope().matches_scope(&all_scope));
    assert!(architecture_scope().matches_domain(&all_domain));
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

#[cfg(feature = "nota-text")]
#[test]
fn domain_round_trips_through_nota_text() {
    use nota::{NotaEncode, NotaSource};

    let domain = schema_domain();
    let rendered = domain.to_nota();
    let decoded = NotaSource::new(&rendered)
        .parse::<Domain>()
        .expect("decode domain NOTA");

    assert_eq!(rendered, "(Technology (Software (Data SchemaEvolution)))");
    assert_eq!(decoded, domain);
}

#[cfg(feature = "nota-text")]
#[test]
fn all_domain_round_trips_through_nota_text() {
    use nota::{NotaEncode, NotaSource};

    let domain = Domain::All;
    let rendered = domain.to_nota();
    let decoded = NotaSource::new(&rendered)
        .parse::<Domain>()
        .expect("decode all domain NOTA");

    assert_eq!(rendered, "All");
    assert_eq!(decoded, domain);
}

#[cfg(feature = "nota-text")]
#[test]
fn scope_collection_round_trips_through_nota_text() {
    use nota::{NotaEncode, NotaSource};

    let scopes = signal_domain::DomainScopes::new(vec![DomainScope::from(schema_domain())]);
    let rendered = scopes.to_nota();
    let decoded = NotaSource::new(&rendered)
        .parse::<signal_domain::DomainScopes>()
        .expect("decode domain scopes NOTA");

    assert_eq!(decoded, scopes);
}
