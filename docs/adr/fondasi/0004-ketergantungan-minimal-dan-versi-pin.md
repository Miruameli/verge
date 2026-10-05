# ADR-0004: Minimal dependencies and pinned versions

## Status

Accepted

## Context

For a database, every dependency is part of the attack surface and part of the
maintenance load. The specification also demands a self-hostable project with
no vendor lock-in.

## Decision

1. `verge-core` uses only one external dependency: `sha2` (SHA-256 from
   RustCrypto).
2. `verge-cli` uses `anyhow` as the application error boundary.
3. Versions are pinned in `Cargo.toml` and `Cargo.lock`, which are committed, so
   builds are reproducible.
4. `cargo audit` (the RustSec advisory database) and dependabot must be green on
   every PR.

## Alternatives Considered

- **A hand-written hashing implementation (fewer than 100 lines)** — avoids a
  dependency, but re-implements a risky cryptographic primitive and contradicts
  the principle of never writing your own cryptographic algorithms.
- **BLAKE3** — faster, not yet as established as SHA-256 for audit purposes, and
  it adds one more dependency.
- **No lockfile** — builds are not reproducible and dependency auditing becomes
  impossible.

## Consequences

- The engine's attack surface stays small and easy to audit.
- Dependency upgrades must be considered deliberately; dependabot helps keep
  that task visible.
- Hashing is limited to SHA-256; for integrity and audit purposes this is
  sufficient.

## Justification

Quality and security always win over speed. One already-audited dependency is
far cheaper than several that may not be safe.

## Date

2026-10-03

## Author

Miruamel
