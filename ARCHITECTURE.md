# Architecture

`signal-domain` owns only the shared domain taxonomy contract. It exposes the generated `Domain`, `DomainScope`, `DomainScopes`, `ScopeSet`, and related leaf enums directly from the crate root.

The schema file is copied from `signal-spirit` without taxonomy cleanup so existing enum order and layout remain stable for downstream extraction work. The build script runs `schema-rust` in checked-artifact mode to keep `src/schema/domain.rs` aligned with `schema/domain.schema`.

Runtime components, daemons, storage owners, policy decisions, and Spirit/Mind behavior belong in their owning repositories, not in this crate.
