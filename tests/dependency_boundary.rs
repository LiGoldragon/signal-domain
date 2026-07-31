use std::process::Command;

#[test]
fn default_dependency_tree_has_no_runtime_or_text_crates() {
    let output = Command::new("cargo")
        .args(["tree", "--edges", "normal", "--no-default-features"])
        .output()
        .expect("run cargo tree");

    assert!(output.status.success(), "status: {:?}", output.status);
    let tree = String::from_utf8(output.stdout).expect("dependency tree");

    for forbidden_crate in [
        "signal-spirit",
        "signal-mind",
        "spirit",
        "mind",
        "kameo",
        "tokio",
        "redb",
        "dotos ",
        "schema-language",
    ] {
        assert!(
            !tree.contains(forbidden_crate),
            "default dependency tree must not contain {forbidden_crate}:\n{tree}"
        );
    }
}

#[test]
fn dotos_text_feature_only_adds_canonical_text_projection() {
    let output = Command::new("cargo")
        .args([
            "tree",
            "--edges",
            "normal",
            "--no-default-features",
            "--features",
            "dotos-text",
        ])
        .output()
        .expect("run cargo tree");

    assert!(output.status.success(), "status: {:?}", output.status);
    let tree = String::from_utf8(output.stdout).expect("dependency tree");

    assert!(
        tree.contains("dotos"),
        "dotos-text should opt into dotos:\n{tree}"
    );
    assert!(
        tree.contains("schema"),
        "dotos-text should opt into schema:\n{tree}"
    );
    for forbidden_crate in [
        "signal-spirit",
        "signal-mind",
        "spirit",
        "mind",
        "kameo",
        "tokio",
        "redb",
    ] {
        assert!(
            !tree.contains(forbidden_crate),
            "dotos-text dependency tree must not contain {forbidden_crate}:\n{tree}"
        );
    }
}

#[test]
fn schema_family_is_exact_and_single_world() {
    let output = Command::new("cargo")
        .args(["tree", "--locked", "--all-features", "--duplicates"])
        .output()
        .expect("run duplicate dependency query");

    assert!(output.status.success(), "status: {:?}", output.status);
    assert!(
        output.stdout.is_empty(),
        "schema family must not contain duplicate package worlds:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );

    let manifest = include_str!("../Cargo.toml");
    let lockfile = include_str!("../Cargo.lock");
    for revision in [
        "80c7b17f7ad3",
        "c966e0ce30bb",
        "6179a7cf1394083244dd3f3e1d2709f9ec08f7db",
    ] {
        assert!(
            manifest.contains(revision),
            "manifest must pin exact schema-family revision {revision}"
        );
        assert!(
            lockfile.contains(revision),
            "lockfile must resolve exact schema-family revision {revision}"
        );
    }
    for floating_source in ["branch =", "branch=", "path+"] {
        assert!(
            !manifest.contains(floating_source) && !lockfile.contains(floating_source),
            "schema family must not contain floating or path source {floating_source}"
        );
    }
}
