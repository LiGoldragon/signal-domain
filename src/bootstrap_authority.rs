//! Production authority path for the Signal domain Interface.
//!
//! The domain component owns this declared bootstrap metadata. Callers receive
//! only the resulting authorized transaction and Rust projection capability;
//! they cannot re-create its catalog, authority proof, receipt, seats, or
//! sealed Rust vocabulary.

use std::collections::BTreeMap;

use core_ethos::bootstrap::{
    BootstrapCatalog, BootstrapGrammarIdentities, BootstrapPriorIdentities,
    BootstrapPriorVocabulary, BootstrapVersionPolicy, CanonicalIdentityOrder, EthosKind,
    EthosVersion, IdentitySchema, IdentitySchemaCatalog, InterfaceRole, NomosSchema, SchemaRole,
    TextualMetadataRecord, TextualMetadataSnapshot, TextualProjectionAddress,
};
use name_table::LocalEncodedId;
use rust_logos::{RustLogos, RustTypePath, RustTypePathResolver};
pub use sema_translator::bootstrap::AuthorizedBootstrap;
use sema_translator::bootstrap::{
    BootstrapAuthorityContract, SealedRustVocabulary, authorize_bootstrap,
};
use signal_sema_translator::{VocabularyEncodedId, VocabularyRoot};

use crate::bootstrap_manifest::{self as manifest, AuthoritySeat, DeclarationSeat};

const MODULE_PATH: &[&str] = &["signal_domain", "domain"];

/// Authorize the component's declared domain Interface source.
pub fn assemble(source: &str) -> AuthorizedBootstrap {
    let catalog = bootstrap_catalog();
    let (approved_metadata, canonical_identities) = approved_metadata(&catalog);
    authorize_bootstrap(
        source,
        BootstrapAuthorityContract::new(
            manifest::AUTHORITY_IDENTITY,
            manifest::AUTHORITY_REVISION,
            BootstrapGrammarIdentities {
                document: universal(manifest::GRAMMAR_DOCUMENT_LOCAL),
                syntax: universal(manifest::GRAMMAR_SYNTAX_LOCAL),
            },
            catalog,
            approved_metadata,
            canonical_identities,
            BTreeMap::new(),
        ),
    )
    .expect("assemble authority-approved domain Interface transaction")
}

/// Authorize the checked-in canonical domain Interface source.
pub fn domain_bootstrap() -> AuthorizedBootstrap {
    assemble(include_str!("../schema/domain.schema"))
}

/// Obtain Rust projection capability through the component's production path.
pub fn rust_logos() -> RustLogos {
    RustLogos::from_authority(&SealedRustVocabulary::bootstrap())
        .expect("authority releases the bootstrap Rust vocabulary")
}

pub fn universal(local: u16) -> VocabularyEncodedId {
    VocabularyEncodedId::new(VocabularyRoot::Universal, vec![LocalEncodedId::new(local)])
        .expect("manifest seats are nonempty Universal identities")
}

pub fn declaration(spelling: &str) -> VocabularyEncodedId {
    let seat = manifest::DECLARATION_SEATS
        .iter()
        .find(|seat| seat.owner_local.is_none() && seat.spelling == spelling)
        .unwrap_or_else(|| panic!("missing top-level authority seat {spelling}"));
    universal(seat.local)
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
    .expect("manifest seats satisfy bootstrap prior relationships");
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
    after.extend(manifest::DECLARATION_SEATS.iter().map(declaration_record));
    (
        TextualMetadataSnapshot::new(after)
            .expect("manifest declaration projection addresses are exact"),
        manifest::DECLARATION_SEATS
            .iter()
            .map(|seat| (universal(seat.local), seat.canonical.to_be_bytes().to_vec()))
            .collect(),
    )
}

pub struct DomainRustTypePaths(BTreeMap<VocabularyEncodedId, RustTypePath>);

impl DomainRustTypePaths {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for DomainRustTypePaths {
    fn default() -> Self {
        Self(BTreeMap::from([
            (
                universal(manifest::VECTOR_SEAT.local),
                RustTypePath::try_new(vec!["Vec".to_owned()])
                    .expect("Vec is an explicit Rust type path"),
            ),
            (
                universal(manifest::SCOPEOF_SEAT.local),
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
