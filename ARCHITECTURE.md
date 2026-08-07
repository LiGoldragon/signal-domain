# Architecture

The authored Ethos text is the domain Interface's retained source of intent.
Its durable authority transaction and Rust projection are intentionally pending:

```text
Interface text
    │
    ▼
hqu.30 durable CommitBootstrap
    │
    ▼
authority-owned transaction + installed Rust projection
```

`schema/domain.schema` contains the provisional `Interface.{1 0 0}` envelope
and all 41 taxonomy declarations. The previous component-owned identity seats,
canonical bytes, and generated Rust came from a condemned caller authority and
were removed rather than translated into a new compatibility identity scheme.
Current Sema authority minting is intentionally in-memory only, so this source
cannot have a reproducible checked-in Rust projection before hqu.30.

The generated Rust is absent, and the crate consequently does not compile.
That explicit break prevents stale generated names or a handwritten duplicate
from masquerading as the authority product. hqu.30 restores the live Interface
only by atomically installing a durable authority result and its behavior.

Daemons, storage, and policy belong to their owning repositories.
