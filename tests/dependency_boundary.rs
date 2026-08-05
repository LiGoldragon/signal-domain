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
        "80c7b17f7ad3cf547d2624c6a243e5de5f85c9f3",
        "36a83e8d49989605c1fb9c7be265ddf10a752c31",
        "f0ce3aaddddb9fb9b8d2bcc8548a22efe7578cc0",
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

    assert!(
        manifest.contains("version = \"=0.8.17\""),
        "the archive ABI dependency must pin rkyv 0.8.17 exactly"
    );
    assert!(
        lockfile.contains("name = \"rkyv\"\nversion = \"0.8.17\""),
        "the lockfile must resolve rkyv 0.8.17"
    );

    let dotos_tree = Command::new("cargo")
        .args(["tree", "--locked", "--all-features", "-i", "dotos"])
        .output()
        .expect("run Dotos source-identity query");
    assert!(
        dotos_tree.status.success(),
        "status: {:?}",
        dotos_tree.status
    );
    let dotos_tree = String::from_utf8(dotos_tree.stdout).expect("Dotos source identity");
    assert!(
        dotos_tree.contains(
            "dotos v0.10.0 (https://github.com/LiGoldragon/dotos.git?rev=80c7b17f7ad3cf547d2624c6a243e5de5f85c9f3#80c7b17f)"
        ),
        "Dotos must resolve from the exact current source:\n{dotos_tree}"
    );
    assert!(
        !dotos_tree.contains("?rev=80c7b17f7ad3#"),
        "Dotos must refuse the short-revision source identity:\n{dotos_tree}"
    );
}
