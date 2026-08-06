use std::{collections::BTreeMap, env, fs, path::PathBuf};

use core_ethos::bootstrap::{
    BootstrapCatalog, BootstrapGrammarIdentities, BootstrapPriorIdentities,
    BootstrapPriorVocabulary, BootstrapVersionPolicy, CanonicalIdentityOrder, EthosKind,
    EthosVersion, IdentitySchema, IdentitySchemaCatalog, InterfaceRole, NomosSchema, SchemaRole,
    TextualMetadataRecord, TextualMetadataSnapshot, TextualProjectionAddress,
};
use name_table::LocalEncodedId;
use rust_logos::{RustLogos, RustTypePath, RustTypePathResolver};
use schema_rust::{bootstrap::BootstrapInterfaceGeneration, build::CargoEthosSourceMetadata};
use sema_translator::bootstrap::{
    BootstrapAuthorityContract, SealedRustVocabulary, authorize_bootstrap,
};
use signal_sema_translator::{VocabularyEncodedId, VocabularyRoot};

#[path = "src/bootstrap_manifest.rs"]
mod bootstrap_manifest;

use bootstrap_manifest::{AuthoritySeat, DeclarationSeat};

const MODULE_PATH: &[&str] = &["signal_domain", "domain"];

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
        println!("cargo:rerun-if-changed=src/schema/domain/generated.rs");

        let source_path = self.crate_root.join("schema/domain.schema");
        let rust_path = self.crate_root.join("src/schema/domain/generated.rs");
        let source = fs::read_to_string(&source_path).expect("read domain Interface source");
        let catalog = bootstrap_catalog();
        let (approved_metadata, canonical_identities) = approved_metadata(&catalog);
        let assembly = authorize_bootstrap(
            &source,
            BootstrapAuthorityContract::new(
                bootstrap_manifest::AUTHORITY_IDENTITY,
                bootstrap_manifest::AUTHORITY_REVISION,
                BootstrapGrammarIdentities {
                    document: universal(bootstrap_manifest::GRAMMAR_DOCUMENT_LOCAL),
                    syntax: universal(bootstrap_manifest::GRAMMAR_SYNTAX_LOCAL),
                },
                catalog,
                approved_metadata,
                canonical_identities,
                BTreeMap::new(),
            ),
        )
        .expect("assemble authority-approved domain Interface transaction");
        let rust = rust_logos();
        let type_paths = DomainRustTypePaths::new();

        BootstrapInterfaceGeneration::new(&assembly, &rust, &type_paths, &source_path, &rust_path)
            .generate()
            .expect("project domain Interface from the verified transaction")
            .write_or_check("SIGNAL_DOMAIN_UPDATE_INTERFACE_ARTIFACTS")
            .expect("checked-in domain Interface source and Rust projection are fresh");
        CargoEthosSourceMetadata::new("signal-domain")
            .publish_owned_source_directory(self.crate_root.join("schema"));
    }
}

fn universal(local: u16) -> VocabularyEncodedId {
    VocabularyEncodedId::new(VocabularyRoot::Universal, vec![LocalEncodedId::new(local)])
        .expect("manifest seats are nonempty Universal identities")
}

fn metadata_record(
    spelling: &str,
    identity: VocabularyEncodedId,
    owner: Option<VocabularyEncodedId>,
) -> TextualMetadataRecord {
    TextualMetadataRecord {
        address: TextualProjectionAddress {
            module_path: MODULE_PATH.iter().map(|part| (*part).to_owned()).collect(),
            lexical_owner: owner,
            visible_name: spelling.to_owned(),
        },
        encoded_name: identity,
    }
}

fn fixed_specifications() -> Vec<(AuthoritySeat, Vec<SchemaRole>)> {
    use bootstrap_manifest as manifest;

    vec![
        (
            manifest::INTERFACE_SEAT,
            vec![SchemaRole::FileKind(EthosKind::Interface)],
        ),
        (
            manifest::NEXUS_SEAT,
            vec![SchemaRole::FileKind(EthosKind::Nexus)],
        ),
        (
            manifest::SEMA_SEAT,
            vec![SchemaRole::FileKind(EthosKind::Sema)],
        ),
        (
            manifest::INPUT_SEAT,
            vec![SchemaRole::InterfaceRole(InterfaceRole::Input)],
        ),
        (
            manifest::OUTPUT_SEAT,
            vec![SchemaRole::InterfaceRole(InterfaceRole::Output)],
        ),
        (
            manifest::REFUSAL_SEAT,
            vec![SchemaRole::InterfaceRole(InterfaceRole::Refusal)],
        ),
        (
            manifest::STRING_SEAT,
            vec![SchemaRole::Nominal { persistent: true }],
        ),
        (
            manifest::INTEGER_SEAT,
            vec![SchemaRole::Nominal { persistent: true }],
        ),
        (
            manifest::BOOLEAN_SEAT,
            vec![SchemaRole::Nominal { persistent: true }],
        ),
        (
            manifest::UNIT_SEAT,
            vec![SchemaRole::Nominal { persistent: true }],
        ),
        (manifest::VECTOR_SEAT, vec![SchemaRole::Shape { arity: 1 }]),
        (manifest::OPTION_SEAT, vec![SchemaRole::Shape { arity: 1 }]),
        (manifest::MAP_SEAT, vec![SchemaRole::Shape { arity: 2 }]),
        (manifest::RESULT_SEAT, vec![SchemaRole::Shape { arity: 2 }]),
        (
            manifest::STREAM_SEAT,
            vec![
                SchemaRole::Shape { arity: 1 },
                SchemaRole::Nomos(NomosSchema::StreamInitiation { arity: 2 }),
            ],
        ),
        (
            manifest::STREAMIDENTITY_SEAT,
            vec![SchemaRole::Shape { arity: 1 }],
        ),
        (manifest::SCOPEOF_SEAT, vec![SchemaRole::Shape { arity: 1 }]),
    ]
}

fn bootstrap_catalog() -> BootstrapCatalog {
    use bootstrap_manifest as manifest;

    let specifications = fixed_specifications();
    let metadata = TextualMetadataSnapshot::new(
        specifications
            .iter()
            .map(|(seat, _)| metadata_record(seat.spelling, universal(seat.local), None))
            .collect(),
    )
    .expect("manifest fixed textual metadata is exact");
    let schemas = IdentitySchemaCatalog::new(
        specifications
            .iter()
            .map(|(seat, roles)| {
                IdentitySchema::new(universal(seat.local), roles.clone())
                    .expect("manifest fixed schema roles are admitted")
            })
            .collect(),
    )
    .expect("manifest fixed identities are unique");
    let priors = BootstrapPriorVocabulary::new(
        BootstrapPriorIdentities {
            interface_kind: universal(manifest::INTERFACE_SEAT.local),
            nexus_kind: universal(manifest::NEXUS_SEAT.local),
            sema_kind: universal(manifest::SEMA_SEAT.local),
            input_role: universal(manifest::INPUT_SEAT.local),
            output_role: universal(manifest::OUTPUT_SEAT.local),
            refusal_role: universal(manifest::REFUSAL_SEAT.local),
            string_type: universal(manifest::STRING_SEAT.local),
            integer_type: universal(manifest::INTEGER_SEAT.local),
            boolean_type: universal(manifest::BOOLEAN_SEAT.local),
            unit_type: universal(manifest::UNIT_SEAT.local),
            vector_shape: universal(manifest::VECTOR_SEAT.local),
            option_shape: universal(manifest::OPTION_SEAT.local),
            map_shape: universal(manifest::MAP_SEAT.local),
            result_shape: universal(manifest::RESULT_SEAT.local),
            stream_nomos: universal(manifest::STREAM_SEAT.local),
            stream_shape: universal(manifest::STREAM_SEAT.local),
            stream_identity_shape: universal(manifest::STREAMIDENTITY_SEAT.local),
        },
        &schemas,
        &metadata,
    )
    .expect("manifest seats satisfy the bootstrap prior relationships");
    let canonical_order = CanonicalIdentityOrder::new(
        specifications
            .iter()
            .map(|(seat, _)| (universal(seat.local), seat.canonical.to_be_bytes().to_vec())),
    )
    .expect("manifest fixed canonical bytes are unique");

    BootstrapCatalog::new(
        MODULE_PATH.iter().map(|part| (*part).to_owned()).collect(),
        metadata,
        schemas,
        priors,
        BootstrapVersionPolicy::exact(EthosVersion::new(1, 0, 0)),
        canonical_order,
    )
    .expect("domain bootstrap catalog is complete")
}

fn declaration_record(seat: &DeclarationSeat) -> TextualMetadataRecord {
    metadata_record(
        seat.spelling,
        universal(seat.local),
        seat.owner_local.map(universal),
    )
}

fn approved_metadata(
    catalog: &BootstrapCatalog,
) -> (
    TextualMetadataSnapshot,
    BTreeMap<VocabularyEncodedId, Vec<u8>>,
) {
    let mut after = catalog.metadata().records().to_vec();
    after.extend(
        bootstrap_manifest::DECLARATION_SEATS
            .iter()
            .map(declaration_record),
    );
    (
        TextualMetadataSnapshot::new(after)
            .expect("manifest declaration projection addresses are exact"),
        bootstrap_manifest::DECLARATION_SEATS
            .iter()
            .map(|seat| (universal(seat.local), seat.canonical.to_be_bytes().to_vec()))
            .collect(),
    )
}

fn rust_logos() -> RustLogos {
    RustLogos::from_authority(&SealedRustVocabulary::bootstrap())
        .expect("authority releases the bootstrap Rust vocabulary")
}

struct DomainRustTypePaths(BTreeMap<VocabularyEncodedId, RustTypePath>);

impl DomainRustTypePaths {
    fn new() -> Self {
        Self(BTreeMap::from([
            (
                universal(bootstrap_manifest::VECTOR_SEAT.local),
                RustTypePath::try_new(vec!["Vec".to_owned()])
                    .expect("Vec is an explicit Rust type path"),
            ),
            (
                universal(bootstrap_manifest::SCOPEOF_SEAT.local),
                RustTypePath::try_new(vec!["crate".to_owned(), "Scope".to_owned()])
                    .expect("Scope is an explicit Rust type path"),
            ),
        ]))
    }
}

impl RustTypePathResolver for DomainRustTypePaths {
    fn resolve_type_path(&self, encoded_id: &VocabularyEncodedId) -> Option<&RustTypePath> {
        self.0.get(encoded_id)
    }
}
