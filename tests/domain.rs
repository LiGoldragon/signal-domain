use signal_domain::{Domain, DomainScope};

#[test]
fn shared_domain_data_round_trips_as_a_portable_archive() {
    let domain = Domain::Art(signal_domain::ArtDomain::Music);
    let scope = DomainScope {
        domain: domain.clone(),
    };
    let domain_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&domain).expect("archive domain");
    let scope_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&scope).expect("archive domain scope");
    assert_eq!(
        rkyv::from_bytes::<Domain, rkyv::rancor::Error>(&domain_bytes).expect("restore domain"),
        domain
    );
    assert_eq!(
        rkyv::from_bytes::<DomainScope, rkyv::rancor::Error>(&scope_bytes)
            .expect("restore domain scope"),
        scope
    );
}

#[cfg(feature = "datom")]
#[test]
fn shared_domain_data_round_trips_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let scope = DomainScope {
        domain: Domain::Art(signal_domain::ArtDomain::Music),
    };
    let text = scope.clone().datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<DomainScope>::from(text);
    assert_eq!(
        pending
            .actualize(&mut Budget {
                remaining: 1024,
                reader: ReaderBudget { remaining: 1024 },
                depth: 0,
                maximum_depth: 1024,
            })
            .expect("restore scope"),
        scope
    );
}
