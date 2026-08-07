# signal-domain

`signal-domain` owns Signal's domain taxonomy as a bootstrap Protos Interface.
Its canonical textual projection is [`schema/domain.schema`](schema/domain.schema):
an `Interface.{1 0 0}` header, empty textual imports, and the four Interface
body positions `{Inputs Outputs Refusals Types}`. The complete taxonomy lives
in `Types`.

The former caller-owned authority manifest, mint seats, canonical-order bytes,
and generated Rust projection were removed. A fresh Sema authority intentionally
mints opaque names in memory; until hqu.30 owns durable `CommitBootstrap`
installation, it cannot safely reproduce or freshness-check a checked-in Rust
artifact. The authored Ethos source is retained unchanged as that future input.

The crate intentionally does not compile while the generated projection is
absent. hqu.30 must restore compilation by durably committing authority state,
installing the matching generated Rust, and then binding behavior to that single
installed projection. Runtime components, daemons, storage, and policy remain
outside this crate.
