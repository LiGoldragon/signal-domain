# signal-domain

`signal-domain` owns Signal's domain taxonomy as a bootstrap Protos Interface.
Its canonical textual projection is [`schema/domain.schema`](schema/domain.schema):
an `Interface.{1 0 0}` header, empty textual imports, and the four Interface
body positions `{Inputs Outputs Refusals Types}`. The complete taxonomy lives
in `Types`.

[`src/bootstrap_manifest.rs`](src/bootstrap_manifest.rs) records the naming
authority's explicit opaque identity seats and separate canonical-order bytes.
Those order bytes preserve the conceptual taxonomy in the human-facing Ethos
projection. The build assembles that exact authority-approved transaction and uses the strict
`schema-rust` Interface lane to check the canonical source and the compiled
Rust projection together. Rust identifiers encode the complete identities;
this crate does not recreate source spellings as aliases.

The small `Scope<T>` carrier and scope-relation implementations are handwritten
Rust behavior outside the current Interface declaration stage. Runtime
components, daemons, storage, and policy remain outside this crate.
