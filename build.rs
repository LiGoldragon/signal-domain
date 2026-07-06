use std::{env, path::PathBuf};

use schema_rust::build::{CargoSchemaMetadata, GenerationDriver, GenerationPlan, ModuleEmission};

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
        println!("cargo:rerun-if-changed=src/schema/domain.rs");

        let plan = GenerationPlan::new(&self.crate_root, "signal-domain", "0.1.0")
            .with_module(ModuleEmission::declaration_module("domain"));

        GenerationDriver::new(plan)
            .generate()
            .expect("generate signal-domain schema artifacts")
            .write_or_check("SIGNAL_DOMAIN_UPDATE_SCHEMA_ARTIFACTS")
            .expect("checked-in signal-domain schema artifacts are fresh");
        CargoSchemaMetadata::new("signal-domain").emit_schema_directory(&self.crate_root);
    }
}
