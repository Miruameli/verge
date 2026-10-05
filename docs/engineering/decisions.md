# Decision Log

Lighter-weight notes than ADRs. Decisions that change the architecture are
recorded in full in `docs/adr/`.

## 2026-10-03 — Start from zero with a 7-layer architecture

**Decision:** the repository was rebuilt from nothing with a 7-layer structure
from the very first commit, instead of being patched once the first feature was
done.

**Reason:** the cost of moving layer structures grows quickly over time, because
every new feature is bound to touch the wrong folder. The modularization rules
have to apply from the start.

**Alternative:** a flat module layout per crate with naming conventions.

**Consequence:** more small files from the beginning; navigation must be
disciplined.

## 2026-10-03 — `Result` uses a default error parameter

**Decision:** the engine uses `pub type Result<T, E = VergeError>`, the CLI uses
`pub type Result<T, E = anyhow::Error>`.

**Reason:** the library keeps strictly typed errors, while the application
boundary needs dynamic errors that carry context. Both still provide an escape
hatch through the second parameter.

**Consequence:** library call sites can write `Result<T>` without repeating the
error name, and tests can write it explicitly.

## 2026-10-03 — Repository bootstrap uses ports, not `std::fs`

**Decision:** the `initialize_repository` use case talks only to the
`MetadataWriter` and `BlockStoreFactory` ports.

**Reason:** the use case can be tested without a filesystem, including rollback
scenarios and I/O failures that are hard to reproduce on a real disk.

**Consequence:** there is one extra trait in the domain, but all I/O operations
stay in infrastructure.