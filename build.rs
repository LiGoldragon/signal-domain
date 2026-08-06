use std::{env, fs, path::PathBuf};

use schema_rust::{bootstrap::BootstrapInterfaceGeneration, build::CargoEthosSourceMetadata};

#[allow(dead_code)]
#[path = "src/bootstrap_authority.rs"]
mod bootstrap_authority;
#[path = "src/bootstrap_manifest.rs"]
mod bootstrap_manifest;

fn main() {
    SchemaBuild::from_environment().run();
}

struct SchemaBuild {
    crate_root: PathBuf,
}

impl SchemaBuild {
    fn from_environment() -> Self {
        Self {
            crate_root: PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir set")),
        }
    }

    fn run(&self) {
        println!("cargo:rerun-if-changed=schema/domain.schema");
        println!("cargo:rerun-if-changed=src/bootstrap_manifest.rs");
        println!("cargo:rerun-if-changed=src/bootstrap_authority.rs");
        println!("cargo:rerun-if-changed=src/schema/domain/generated.rs");

        let source_path = self.crate_root.join("schema/domain.schema");
        let rust_path = self.crate_root.join("src/schema/domain/generated.rs");
        let source = fs::read_to_string(&source_path).expect("read domain Interface source");
        let assembly = bootstrap_authority::assemble(&source);
        let rust = bootstrap_authority::rust_logos();
        let type_paths = bootstrap_authority::DomainRustTypePaths::default();

        BootstrapInterfaceGeneration::new(&assembly, &rust, &type_paths, &source_path, &rust_path)
            .generate()
            .expect("project domain Interface from the verified transaction")
            .write_or_check("SIGNAL_DOMAIN_UPDATE_INTERFACE_ARTIFACTS")
            .expect("checked-in domain Interface source and Rust projection are fresh");
        CargoEthosSourceMetadata::new("signal-domain")
            .publish_owned_source_directory(self.crate_root.join("schema"));
    }
}
