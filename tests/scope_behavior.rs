use signal_domain::{z2VNHr, z2VNQR, z2VRXn, z2VUtf, z2VYDN, z2VZQa, z2Vefc, z2Vetb};

fn root_all() -> z2Vefc {
    z2Vefc::z2VTcf
}

fn health_body() -> z2Vefc {
    z2Vefc::z2VcfC(z2Vetb::z2VabW)
}

fn programming_all() -> z2Vefc {
    z2Vefc::z2VM7z(z2VYDN::z2VSAm(z2VRXn::z2Vdwx(z2VNHr::z2VdAM)))
}

fn programming_type_systems() -> z2Vefc {
    z2Vefc::z2VM7z(z2VYDN::z2VSAm(z2VRXn::z2Vdwx(z2VNHr::z2VPym)))
}

fn programming_compilation() -> z2Vefc {
    z2Vefc::z2VM7z(z2VYDN::z2VSAm(z2VRXn::z2Vdwx(z2VNHr::z2VdbW)))
}

#[test]
fn containment_follows_the_generated_tree_without_a_mirror_taxonomy() {
    let root = root_all().into_scope();
    let programming = programming_all().into_scope();
    let type_systems = programming_type_systems().into_scope();
    let compilation = programming_compilation().into_scope();
    let health = health_body().into_scope();

    assert!(root.is_all());
    assert!(root.contains_scope(&type_systems));
    assert!(programming.contains_scope(&type_systems));
    assert!(!type_systems.contains_scope(&programming));
    assert!(!type_systems.contains_scope(&compilation));
    assert!(!programming.contains_scope(&health));
    assert!(programming.overlaps_scope(&type_systems));
    assert!(!programming.overlaps_scope(&health));
}

#[test]
fn scope_domain_and_collection_matching_preserve_wildcard_semantics() {
    let programming = programming_all().into_scope();
    assert!(programming.matches_domain(&programming_type_systems()));
    assert!(programming.matches_domain(&root_all()));
    assert!(!programming.matches_domain(&health_body()));

    let scopes = z2VZQa::new(vec![programming_type_systems().into_scope()]);
    assert!(scopes.matches_any_domain(&[programming_type_systems()]));
    assert!(!scopes.matches_any_domain(&[health_body()]));

    let set = z2VUtf::new(vec![z2VNQR::from(health_body())]);
    assert!(set.matches_domain(&health_body()));
    assert!(!set.matches_domain(&programming_compilation()));
}
