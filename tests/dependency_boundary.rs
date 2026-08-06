use std::process::Command;

#[test]
fn runtime_tree_contains_only_the_domain_crate() {
    let output = Command::new("cargo")
        .args([
            "tree",
            "--locked",
            "--edges",
            "normal",
            "--no-default-features",
        ])
        .output()
        .expect("run runtime dependency query");
    assert!(output.status.success(), "status: {:?}", output.status);
    let tree = String::from_utf8(output.stdout).expect("runtime dependency tree");
    for forbidden in [
        "core-ethos",
        "schema-rust",
        "sema-translator",
        "schema-language",
        "dotos",
        "rkyv",
    ] {
        assert!(
            !tree.contains(forbidden),
            "runtime tree must not contain build-time crate {forbidden}:\n{tree}"
        );
    }
}

#[test]
fn build_tree_is_exact_and_single_world() {
    let manifest = include_str!("../Cargo.toml");
    let lockfile = include_str!("../Cargo.lock");
    let build_script = include_str!("../build.rs");
    for floating in ["branch =", "branch=", "path+"] {
        assert!(!manifest.contains(floating) && !lockfile.contains(floating));
    }
    assert_eq!(
        lockfile.matches("name = \"sema-translator\"").count(),
        1,
        "one Sema authority package must serve the strict build"
    );
    assert_eq!(
        lockfile.matches("name = \"schema-rust\"").count(),
        1,
        "one bootstrap Rust projection package must serve the strict build"
    );
    assert!(manifest.contains("9e36587c85bd69357e9042729ba2df0052799756"));
    assert!(build_script.contains("CargoEthosSourceMetadata"));
    assert!(build_script.contains("publish_owned_source_directory"));
    assert!(
        !lockfile.contains("name = \"schema-language\""),
        "the deleted pre-bootstrap schema world must not enter the build graph"
    );
    assert!(
        !lockfile.contains("name = \"sema-engine\""),
        "bootstrap generation must not lock the runtime Sema engine"
    );

    let core = Command::new("cargo")
        .args([
            "tree",
            "--locked",
            "--edges",
            "normal,build",
            "-i",
            "core-ethos",
        ])
        .output()
        .expect("run core-ethos source query");
    assert!(core.status.success(), "status: {:?}", core.status);
    let core = String::from_utf8(core.stdout).expect("core-ethos source tree");
    assert_eq!(
        core.lines()
            .filter(|line| line.starts_with("core-ethos v"))
            .count(),
        1,
        "one core-ethos package world must serve the strict build:\n{core}"
    );

    let build = Command::new("cargo")
        .args(["tree", "--locked", "--edges", "normal,build"])
        .output()
        .expect("run build dependency query");
    assert!(build.status.success(), "status: {:?}", build.status);
    let build = String::from_utf8(build.stdout).expect("build dependency tree");
    for required in ["schema-rust", "sema-translator", "core-nomos", "rust-logos"] {
        assert!(build.contains(required), "missing {required}:\n{build}");
    }
    assert!(
        !build.contains("sema-engine"),
        "bootstrap generation must not pull the runtime Sema engine:\n{build}"
    );
}
