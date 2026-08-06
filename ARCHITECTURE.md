# Architecture

The authority-approved encoded form is the domain Interface's semantic center.
The checked Ethos text and Rust are projections of one verified transaction:

```text
explicit authority seats + Interface text
                    │
                    ▼
       VerifiedBootstrapAssembly
                    │
                    ▼
        bootstrap Nomos → WholeLogos
                    │
                    ▼
       strict encoded-name Rust projection
```

`schema/domain.schema` contains the provisional `Interface.{1 0 0}` envelope
and all 41 taxonomy declarations. `src/bootstrap_manifest.rs` contains the
producer-owned identity seats; identities are not computed from spelling,
position, content, Rust, or the source parser. Separate explicit canonical
bytes preserve the conceptual declaration and variant order in Ethos. The
build refuses missing, unused, or altered seats through the production
`sema-translator` authority boundary and checks both canonical projections for
freshness.

The generated Rust is the live Interface representation. There is no legacy
six-slot decoder, `TrueSchema` conversion, human-name alias layer, or parallel
Rust declaration universe. `src/schema/domain/behavior.rs` is reserved for
handwritten behavior over those generated identities. `Scope<T>` is the
explicit Rust realization of the authority-cataloged `ScopeOf` shape used by
the Interface; it does not duplicate an Interface declaration.

Daemons, storage, and policy belong to their owning repositories.
