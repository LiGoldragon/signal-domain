//! Bootstrap generation for the domain Interface through the unified
//! authority pipeline.
//!
//! This build script authorizes `schema/domain.schema` through
//! `SemaBootstrapAuthority`, generates canonical Ethos source and Rust
//! projections through `BootstrapGeneration`, and installs them atomically
//! through `CommitBootstrap`.
//!
// psyche-grasp: slightly-reviewed (2026-08-07)

use std::{collections::BTreeMap, env, fs, path::PathBuf};

use name_table::{EncodedName, NameView, TextualMetadata};
use rust_logos::{RustLogos, RustTypePath, RustTypePathResolver};
use schema_rust::{
    bootstrap::{BootstrapGeneration, CommitBootstrap},
    build::CargoEthosSourceMetadata,
};
use sema_translator::bootstrap::{AuthorityNameView, SemaBootstrapAuthority, SourcePlacement};

fn main() {
    DomainBuild::from_environment().run();
}

struct DomainBuild {
    crate_root: PathBuf,
}

impl DomainBuild {
    fn from_environment() -> Self {
        Self {
            crate_root: PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir set")),
        }
    }

    fn run(&self) {
        println!("cargo:rerun-if-changed=schema/domain.schema");
        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-changed=src/schema/domain/generated.rs");

        let source_path = self.crate_root.join("schema/domain.schema");
        let rust_path = self.crate_root.join("src/schema/domain/generated.rs");
        let source = fs::read_to_string(&source_path).expect("read domain Interface source");

        let placement = SourcePlacement::new(
            vec!["signal_domain".to_owned(), "domain".to_owned()],
            vec![
                "signal_domain".to_owned(),
                "domain".to_owned(),
                "domain.schema".to_owned(),
            ],
        );

        let mut authority =
            SemaBootstrapAuthority::new().expect("empty authority owns its seed");
        authority
            .admit_domain_shape("ScopeOf", 1)
            .expect("authority admits the domain ScopeOf shape constructor");
        let assembly = authority
            .authorize(&source, placement)
            .expect("assemble authority-approved domain Interface transaction");
        let rust = RustLogos::new();
        let type_paths = DomainRustTypePaths::from_name_view(assembly.name_view());

        let generated = BootstrapGeneration::new(
            &assembly,
            &rust,
            &type_paths,
            &[],
            &source_path,
            &rust_path,
        )
        .generate()
        .expect("project domain Interface from the verified transaction");

        CommitBootstrap::single(generated)
            .write_or_check("SIGNAL_DOMAIN_UPDATE_INTERFACE_ARTIFACTS")
            .expect("checked-in domain Interface source and Rust projection are fresh");

        CargoEthosSourceMetadata::new("signal-domain")
            .publish_owned_source_directory(self.crate_root.join("schema"));
    }
}

/// Resolves external Rust type paths by looking up textual names through the
/// sealed authority name view.
struct DomainRustTypePaths<'a> {
    name_view: &'a AuthorityNameView,
    overrides: BTreeMap<&'static str, RustTypePath>,
}

impl<'a> DomainRustTypePaths<'a> {
    fn from_name_view(name_view: &'a AuthorityNameView) -> Self {
        let overrides = BTreeMap::from([
            (
                "ScopeOf",
                RustTypePath::try_new(vec!["crate".to_owned(), "Scope".to_owned()])
                    .expect("Scope is an explicit Rust type path"),
            ),
        ]);
        Self {
            name_view,
            overrides,
        }
    }
}

impl RustTypePathResolver for DomainRustTypePaths<'_> {
    fn resolve_type_path(&self, encoded_name: &EncodedName) -> Option<&RustTypePath> {
        let metadata: &TextualMetadata = self.name_view.textual_metadata(encoded_name)?;
        self.overrides.get(metadata.textual_name().as_str())
    }
}
