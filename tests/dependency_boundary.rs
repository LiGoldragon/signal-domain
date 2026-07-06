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
        "nota ",
        "schema ",
    ] {
        assert!(
            !tree.contains(forbidden_crate),
            "default dependency tree must not contain {forbidden_crate}:\n{tree}"
        );
    }
}

#[test]
fn nota_text_feature_only_adds_canonical_text_projection() {
    let output = Command::new("cargo")
        .args([
            "tree",
            "--edges",
            "normal",
            "--no-default-features",
            "--features",
            "nota-text",
        ])
        .output()
        .expect("run cargo tree");

    assert!(output.status.success(), "status: {:?}", output.status);
    let tree = String::from_utf8(output.stdout).expect("dependency tree");

    assert!(
        tree.contains("nota"),
        "nota-text should opt into nota:\n{tree}"
    );
    assert!(
        tree.contains("schema"),
        "nota-text should opt into schema:\n{tree}"
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
            "nota-text dependency tree must not contain {forbidden_crate}:\n{tree}"
        );
    }
}
