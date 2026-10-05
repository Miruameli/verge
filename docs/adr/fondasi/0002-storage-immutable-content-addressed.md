# ADR-0002: Immutable content-addressed storage

## Status

Accepted

## Context

Verge's core promise is a built-in audit trail and branching without copying
data. Both require objects that never change and names that can be verified
without trusting the storage layer.

## Decision

1. Every storage object (block, tree, commit) is immutable once written.
2. An object's name is the SHA-256 of its canonical encoding.
3. Blocks are stored in a fan-out layout `objects/ab/cd/<hex>` so that a single
   directory never holds thousands of entries.
4. Writes use a temporary file, `fsync`, then `rename`, so a block is never
   visible half-written.
5. A branch is a moving pointer to a commit; a tag is an immutable pointer.

## Alternatives Considered

- **Full snapshot per branch** — simple, but it copies data and directly
  contradicts the "O(1) branching" claim.
- **Copy-on-write per table** — strong, but complex and wasteful when many
  branches are active.
- **Object store without content addressing** — requires external metadata for
  integrity, so the audit trail cannot be verified on its own.

## Consequences

- Data identity can be verified at any time: re-hashing the content must
  produce the same name.
- Automatic deduplication for snapshots whose content did not change.
- Unreachable blocks are not deleted yet; reachability-based block collection
  is planned separately once the query engine is ready.

## Justification

Content addressing is the only design that provides an audit trail,
deduplication, and cheap branching at the same time, without a separate
metadata table.

## Date

2026-10-03

## Author

Miruamel
