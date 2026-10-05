# ADR-0006: Prolly tree for tables

## Status

Accepted

## Context

Milestone 2 stored table contents as one full block per table (ADR-0005). The
consequence is honest but undesirable: unchanged data is not written twice, but a
one-byte change rewrites the whole table, and diff can only compare whole blocks,
so it yields no per-row change information.

The product promises "row-level diff" and "branching without copying data". Both
demand a structure that divides a table into small parts that can be compared and
shared.

## Decision

1. Table contents are parsed into a header and rows; the first column becomes the
   row key.
2. Rows are sorted by key, and duplicate keys take the last row
   (`last-write-wins`), so the order is deterministic and independent of file
   order.
3. Rows are partitioned into leaf nodes with a 4 KiB limit; internal nodes point
   to their children. The root always holds the header node as its first child.
4. The identity of every node is the SHA-256 of its canonical encoding, so nodes
   with equal contents always share one block: changing a single row rewrites
   only the leaf that holds that row.
5. `diff` compares two tables by a merge walk over the sorted keys, so its result
   is complete and stably ordered.

## Table Format Limits

A table is read as simple CSV: the first line is the header, the column separator
is a comma, and a line must have at least two columns. Table contents that do not
satisfy the rules are rejected with a line number, not silently skipped.

## Alternatives Considered

- **Per-row block chunking without a tree** — gives partial dedup, but provides no
  structure for query, merge, and range lookup.
- **Full deltas between commits** — a delta must be verified against its base; if
  the base is lost, the audit trail is damaged too.
- **Balanced B-tree** — gives faster lookup, but proving the correctness of
  balancing is far more work than is needed now.
- **Protobuf/Avro for rows** — adds a dependency and schema version
  compatibility; the canonical format of our own is already enough.

## Consequences

- Unchanged rows are not rewritten; leaf blocks are reused across commits. This
  limit removes the technical debt recorded in ADR-0005.
- `verge diff` produces per-row changes with a stable order.
- Snapshots written in version 0.1.0 are raw blocks and **cannot** be read as a
  tree; `verge show` and `verge log` on an old repository must be imported again.
  This is a recorded format change in the CHANGELOG.
- Memory use at commit time stays proportional to table size, because the whole
  table is parsed in memory before it is partitioned.

## Justification

Requirements:

- row-level dedup so a small change does not rewrite the table,
- per-row diff that can be verified,
- a format that can be read back without external state.

This solution meets all three with the same object (the content-addressed block)
and without adding a new storage concept.

## Date

2026-10-03

## Author

Miruameli

## Review Date

2026-11-03
