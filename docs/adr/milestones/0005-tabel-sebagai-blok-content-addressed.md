# ADR-0005: Table as a content-addressed block and on-disk pointer

## Status

Accepted

## Context

Milestone 1 proved immutable blocks and the commit graph, but there was no data
that could be versioned. A `verge show <commit>` call on an old commit can only
be proven if the table contents are genuinely stored and can be read back from
disk.

The main question is where table data lives: as an ordinary working file outside
the block store, or as an immutable object inside the block store.

## Decision

1. Table contents are stored as a single content-addressed block in `objects/`.
2. The file `tables/<nama>/working` holds only the working block digest (64
   hexadecimal characters plus a newline). That file is a pointer, not data.
3. `Commit` is stored as a block too; the stored bytes are the same canonical
   encoding as the bytes that are hashed into `CommitId`, so a reader can verify
   integrity every time it loads a commit.
4. The branch pointer (`refs/heads/<nama>`) holds a `CommitId` in the same
   format; `HEAD` points to a branch name.
5. The CLI is the composition root: it wires the filesystem adapter, the system
   clock, and the use cases, then passes them as dependencies.

## Accepted Limitations

- One commit stores one full block for that table. Two commits with identical
  contents use the same block (no duplication), but a change as small as one byte
  rewrites the whole table block.
- Consequently, row-based `diff` and `merge` are not yet possible; both depend on
  the prolly tree structure planned for Milestone 3.
- Table size is bounded by process memory when the commit is created.

## Alternatives Considered

- **Prolly tree right away** — gives row-level dedup, but requires a tree
  structure and an insertion algorithm that have not been proven yet. Building
  them without evidence of correctness carries high risk.
- **Table data outside the block store** — duplicates storage, breaks content
  addressing, and allows the working copy to diverge from what was recorded.
- **External database for tables** — moves the source of truth outside the
  repository and closes off the audit trail.

## Consequences

- `verge log` and `verge show` can be proven with real data.
- Unchanged data is never written twice.
- A commit whose bytes are manipulated on disk is rejected on read.
- Memory use at commit time is proportional to table size; the prolly tree in
  Milestone 3 removes this limitation.

## Justification

Requirements:

- a verifiable audit trail,
- time-travel read on old commits,
- dedup without physical duplication.

This solution meets all three with the same mechanism as ordinary blocks, adds no
new concept, and documents its limitations as they are.

## Date

2026-10-03

## Author

Miruameli

## Review Date

2026-11-03
