# Roadmap

Targets are estimates and may change; major changes require a new ADR.

## M1 — Content-addressed storage and commit graph (done)

- Immutable block store with deduplication and atomic writes.
- Immutable commits with an identifier derived from the canonical encoding.
- Branch as a movable pointer, tag as an immutable pointer.
- `verge init` to create a repository.
- Full quality gate: format, lint, test, dependency audit, secret detection.

## M2 — Table data versioning (done)

- Table content is stored as a content-addressed block; the pointer
  `tables/<nama>/working` stores only the digest, so data never exists in two
  places.
- Commits and branch pointers are stored as blocks that are verified again when
  read; manipulated bytes are rejected.
- `verge import`, `verge commit`, `verge log`, and `verge show` for time-travel
  reads.
- Limitation accepted and recorded in ADR-0005: one commit stores one full block
  per table; per-row deduplication waits for the prolly tree.

## M3 — Prolly tree and diff (done)

- Prolly tree for sorted, deterministic table rows; unchanged leaves are reused
  across commits.
- Row-level diff between two commits, including the columns that changed in the
  same row.
- `verge diff <rev-a>..<rev-b>` with stable output, plus `HEAD~N` resolution and
  commit prefixes.

## M4 — Branch, merge, and time travel (done)

- O(1) branch `create`/`switch`/`list`/`delete` without copying data — done.
- Three-way merge with a base taken from merge-base — done.
- Resolution strategies: manual, ours, theirs, last-write-wins — done; a custom
  resolver follows together with the query API.
- Query `AS OF <commit | tag | timestamp>` — done.

## M5 — Query engine

- Standard SQL parser and planner plus the Verge extensions.
- WASM UDFs and Python UDFs without restarting the server.
- Executor with memory and time limits.

## M6 — Server, SDK, and Web UI

- Server with gRPC, HTTP, and the PostgreSQL wire protocol.
- Python (pyo3) and Rust SDKs.
- Web UI for the commit graph, diff viewer, and conflict resolver.

## Cross-milestone

- S3 and GCS object storage backends.
- Ownership and garbage collection based on reachability.
- Releases with checksum, SBOM, and cross-platform artifacts.
