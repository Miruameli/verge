# Verge Architecture

This document explains how the code is laid out and why. Decisions specific to
each detail live in `docs/adr/fondasi/` (foundation) and `docs/adr/milestones/`
(per milestone); action notes and evidence live in `docs/engineering/audit/`.

## Layers

```
Layer 1  domain/          Entities, value objects, ports (no I/O)
Layer 2  application/     Use cases that orchestrate ports
Layer 3  infrastructure/  Port implementations (local filesystem)
Layer 4  presentation/    (next milestone: HTTP/Web UI)
Layer 5  interfaces/      Entry points: CLI, jobs, event handlers
Layer 6  shared/          Kernel and cross-layer exceptions
Layer 7  config/          Layout constants and configuration
```

Dependency rules — one direction, no cycles:

```
interfaces  ──► application ──► domain
infrastructure ──────────────► domain
config, shared ──────────────► all layers
domain ──► shared (kernel and exceptions only)
```

This direction is the result of automated measurement of every `use` in the repo,
not a preference. Measurement result as of 2026-10-04:

| Layer  | Layers imported                                       |
| ------ | ----------------------------------------------------- |
| domain | `shared`                                               |
| application | `domain`, `config`, `shared`                       |
| infrastructure | `domain`, `config`, `shared`                  |
| interfaces | `config`, `shared` (plus its own `cli` module)     |
| shared | `domain` (`Digest` in the error variant only)         |
| config | `domain` (`TableName`, `BlockId`, `HexText` only)     |

The last two rows are deliberate exceptions: `shared/exceptions` uses `Digest` as
the error payload and `config/*` validates `TableName`, `BlockId`, and `HexText`
when mapping paths. Both touch only pure value objects, without I/O and without
services, and moving them would only copy types. What remains forbidden: `domain`
importing `application`, `infrastructure`, `interfaces`, or `config` — including
in test files. That is why the merge base test that uses the filesystem adapter
was moved to `crates/verge-core/tests/merge/`.

There are no module dependency cycles: the `use` graph across the whole repo was
analyzed without finding a single cycle. Inter-crate dependencies are also
trivial, `verge-cli` depends only on `verge-core`, and `verge-core` does not
depend on any other crate in the workspace.

`domain` must not know anything about the filesystem, the network, or the on-disk
format. All I/O contracts are expressed as traits (ports) that are implemented in
`infrastructure`. That is what lets an S3/GCS backend arrive later without
touching the business logic.

## Folder conventions

- Folders use kebab-case (`value-objects/`, `file-system/`); module names use
  snake_case through the `#[path]` attribute.
- The limit of **5 direct files** and **5 subfolders** per folder is a target, not
  an absolute number: deviations are accepted when each folder content is a
  separate context that cannot be merged without blurring responsibilities. Today
  `domain/` holds seven contexts (commit, ident, merge, storage, table, time,
  tree) and stays easier to navigate than if forced to be merged.
- The limit of **150 lines per file** remains binding; files that need to be
  larger are split by responsibility (example: `commit_graph.rs` +
  `commit_history.rs`).
- Every folder has `mod.rs` as its single module declaration point.
- `docs/adr/` uses numbered subfolders (`fondasi/`, `milestones/`), not one flat
  folder: the numbered registry stays globally ordered (0001–0009), while the
  subfolders keep each folder under five files. Moving old ADRs into subfolders
  does not change ADR content (it is FORBIDDEN to change old ADRs; that rule
  applies to content, not to file location).
- `docs/engineering/audit/` uses one file per entry with `audit.md` as the index;
  the audit grows over time, so one flat file would quickly pass the line limit.

## Code map

| Path                                  | Responsibility                                             |
| ------------------------------------- | ---------------------------------------------------------- |
| `domain/ident/value-objects/`         | SHA-256 Digest and its hex representation               |
| `domain/commit/entities/`             | The `Commit` entity and its field readers                  |
| `domain/commit/codec/`                | Canonical encoding and decoding that verifies the digest    |
| `domain/commit/value-objects/`        | `CommitId`, `Ref` (branch/tag/commit)                      |
| `domain/commit/repositories/`         | `CommitGraph`, history traversal, and commit/ref/tag ports  |
| `domain/time/value-objects/`          | UTC `Timestamp`, civil calendar, and RFC 3339 parsing       |
| `domain/table/`                       | `TableName` and table working-data ports                    |
| `domain/tree/`                        | `TableRows`, node codec, builder, reader, and diff          |
| `domain/tree/nodes/`                  | `TreeNode` (header, leaf, internal) and its codec           |
| `domain/merge/`                       | Resolution strategy (`manual`/`ours`/`theirs`/`last-write-wins`), merge base, and row merging |
| `domain/storage/ports/`               | `Store`, `BlockStoreFactory`, `MetadataWriter`              |
| `application/repository-bootstrap/`   | Repository creation use case                                |
| `application/version-control/`        | `revision_target`, `revision_resolver`, `revision_walk`, `instant_commit_lookup` |
| `application/version-control/use-cases/refs/branching/` | Branch create, switch, list, delete use cases |
| `application/version-control/use-cases/merging/`   | Merge use case: read the sides, write the merge commit     |
| `application/version-control/use-cases/refs/tagging/`   | Tag create, list, delete use cases (immutable)     |
| `application/version-control/use-cases/queries/`   | `query --as-of <WHEN>` use case (RFC 3339/`@ms`/tag/commit) |
| `application/version-control/tests/`  | Cross-use-case tests: instant lookup (`revision/` for revision resolution, `tagging/` for tags) |
| `crates/verge-core/tests/merge/`      | Merge base integration tests using the real filesystem adapter |
| `infrastructure/storage/file-system/` | `FileBlockStore`, factory, local metadata writer            |
| `infrastructure/commit/file-system/refs/` | Branch/tag pointers: `FileRefPointer`, `FileTagPointer` |
| `infrastructure/commit/file-system/`  | Commit object storage (`FileCommitRepository`)              |
| `infrastructure/table/file-system/`   | Tree root pointer and table source reading                  |
| `infrastructure/system/`              | System clock for commit timestamps                          |
| `config/`                             | Repository layout and block paths                           |
| `verge-cli/interfaces/cli/`           | Dispatcher: `init`, `branch`, `merge`, `tag`, help, version |
| `verge-cli/interfaces/cli/commands/table-versioning/dispatch.rs` | Table command router: `import`, `commit`, `log`, `show`, `diff`, `query` |

## Enforced invariants

1. An object that has already been written never changes; an object name = the
   hash of its content.
2. A branch is a movable pointer; a tag is an immutable pointer.
3. A commit enters the graph only when all of its parents already exist.
4. Block writes are atomic: temp file + fsync + rename.
5. Repository bootstrap is all-or-nothing: any failure deletes the trace that has
   already been created.
6. Table content lives only inside the block store; files in `.verge/tables/` are
   tree root digest pointers, not data.
7. A commit loaded from disk is verified again against its digest, so byte
   manipulation outside Verge is detected at read time.
8. Table names are validated before any path is touched: allowlist `[a-z0-9_-]`,
   at most 64 characters, no path separator.
9. Branch and tag pointer names are validated before any path is touched: not
   empty, not starting with a dot, at most 255 bytes, no path separator, and not
   a reserved Windows device name.
10. A tag is never overwritten: the pointer is written with `create_new`, so two
    processes that create a tag with the same name cannot take turns writing the
    same pointer.
11. A tag stores only a commit id, not a table name. The table of a tag is read
    from the commit it points to; an error caused by a different table names both
    tables instead of reporting a broken reference.

## Parents Model

A commit uses an explicit parent order. The first element is the *first parent* —
the branch where the commit was created — and it is used when traversing the log.
The rest are additional parents from merges, so the DAG topology can be
reconstructed exactly.
