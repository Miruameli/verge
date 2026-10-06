# Changelog

All notable changes to this project are recorded in this file.
The format follows [Keep a Changelog](https://keepachangelog.com/id/1.1.0/),
and versioning follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] — 2026-10-06

### Changed

- `verge tag list` prints the table column for every tag, taken from the commit
  the tag points to. `TagEntry` changed from a three-element tuple to a named
  struct `TagEntry { name, commit, table, summary }`.
- The error raised when a revision points to a commit of another table now uses
  the `CommitBelongsToOtherTable` variant, which names the revision, the table
  the commit belongs to, and the requested table; previously `verge query`,
  `verge show`, `verge diff`, and `verge merge` reported `invalid reference`, so
  a valid tag was also mistaken for a broken one.
- The toolchain pin moved from Rust 1.82 to 1.98 (language edition stays 2021);
  see ADR-0010. Local gates and CI now run the same compiler.
- `AS OF` and hex-prefix search now document their 10,000-commit traversal
  bound; see ADR-0011. No behaviour changed.

### Migration

- `TagEntry` consumers: replace tuple destructuring with field access. The old
  `let (name, commit, summary) = entry;` becomes
  `let (name, commit, summary) = (entry.name, entry.commit, entry.summary);`
  and the new `entry.table` field names the table the tagged commit belongs
  to. This is the only breaking API change in 0.4.0.
- `VergeError::CommitBelongsToOtherTable` needs no migration: the enum is
  `#[non_exhaustive]`, so matching on a new variant was never exhaustive and
  existing `match` arms keep compiling. Callers that want the better message
  only need to stop mapping it to `invalid reference`.
- `verge-cli` is published to crates.io for the first time at 0.4.0
  (`cargo install verge-cli --version 0.4.0 --locked`); the binary stays named
  `verge`.

### Fixed

- `verge show` now has an end-to-end test for a revision that points to a commit
  of another table, in line with the existing `query`, `diff`, and `merge`
  tests. Without that test, the table message fix on `show` can silently
  regress.
- The documentation of the `CommitBelongsToOtherTable.revision` field was
  generalized: the field accepts user-supplied revision text (`query`, `show`,
  `diff`) and hex commit ids (`merge`), while the old documentation mentioned
  only the former and therefore contradicted its callers.

### Added

- A `structure` gate in CI, run through `.github/scripts/structure/check-structure.py`,
  enforcing rules that were previously only measured by hand: 150 SLOC per
  file, 5 direct files per folder, 11 mandatory header fields on every `.rs`
  file (each exactly once), `TODO`/`FIXME`/`HACK` without an issue reference,
  and characters outside the approved punctuation list. The last rule catches a
  class of damage invisible to `cargo`: in PR #35 a header comment lost
  `License`, a field was duplicated, and a bullet disappeared without a single
  gate failing.

- The `structure` gate now also enforces the non-Latin letter rule on **all**
  tracked text files, not just `crates/**/*.rs`. Cyrillic, Greek, Thai, CJK,
  and fullwidth letters in `.md`, `.yml`, `.toml`, or `.py` previously passed
  the gate with exit 0. The rule uses a Latin letter allowlist, so legitimate
  typography such as `—` and `→` is still accepted, and accented Latin letters
  such as `José` stay legitimate.
- The `structure` gate now scans `.github/scripts/` as well, not only `crates/`.
  The four gate modules were moved to `.github/scripts/structure/` so that
  `.github/scripts/` stays within the five direct file limit — a violation the
  gate could not previously catch because its root was not scanned.


## [0.3.0] — 2026-10-04

### Added

- A `Timestamp` time value (RFC 3339 UTC and `@<unix_ms>`) with pure calendar
  conversion and no external dependency; used by `verge query` and `verge log`.
- CLI `verge query --table <NAME> --as-of <WHEN>`: `WHEN` may be RFC 3339 UTC
  (`2026-10-01T10:00:00Z`, millisecond option `.123Z`), unix milliseconds
  (`@1767225600000`), a tag name, `refs/tags/<nama>`, `refs/heads/<nama>`, a
  full commit id, a 12–63 character hex prefix, `HEAD`, or `HEAD~N`. Non-zero
  offsets are rejected so the audit stays reproducible.
- CLI `verge tag create <NAME> [--revision <REV>]`, `verge tag list`, and
  `verge tag delete <NAME>`; tags are immutable (`TagAlreadyExists`) and point
  to a single commit under `refs/tags/` without copying blocks.
- `verge log` now prints `123456789abc 2026-10-01T10:00:00.000Z author summary`
  so the printed time can be copied straight into `--as-of`.
- The `TagPointer` port and the `FileTagPointer` adapter; `revision_resolver`
  now uses `TagPointer`, so `verge show` and `verge diff` also accept tags and
  timestamps.
- CLI `verge branch create|switch|list|delete`: O(1) branch pointers under
  `refs/heads/` without copying blocks, with cross-platform name validation
  (rejecting forbidden Windows characters and reserved device names).
- CLI `verge merge <BRANCH> --table <NAME>`: three-way merge against the merge
  base of the nearest common ancestor (all parents, not only first-parent), with
  the `manual` (default), `ours`, `theirs`, and `last-write-wins` strategies; a
  merge that already exists is rejected with `AlreadyMerged`.
- ADR-0008: `AS OF` semantics on the first-parent chain and immutable tags.
- ADR-0009: the merge base follows all parents; the "Merge base" section of
  ADR-0007 is marked superseded.

### Changed

- `resolve_revision` takes an `Option<&TableName>` so that
  `--as-of <TIMESTAMP>` can be resolved for a specific table; non-time callers
  pass `None`.
- `read_snapshot` and `diff_tables` take an additional `&dyn TagPointer`.
- `infrastructure/commit/file-system` is split into `file-system/refs/` for
  branch/tag pointers and `file-system/tests/` for the commit store.
- Files and folders that exceeded the modularization limits are split by
  responsibility: `use-cases/refs/` for branch/tag pointers,
  `domain/merge/rows/row_lookup.rs`, `merge_report_printer.rs` in the CLI, and
  `docs/engineering/audit/` one file per entry.

### Fixed

- Branch or tag names that are not date-shaped are no longer rejected as
  timestamps: `verge show 2026-q1-report` and
  `verge query --as-of 2026-q1-report` read that name as a pointer again.
- `verge tag create` no longer relies on an "already exists" check followed by
  a write; the tag pointer is created with `create_new`, so two processes cannot
  take turns overwriting the same tag.
- An `AS OF` beyond 10.000 commits now reports the actual search limit
  (`SearchLimitReached`) instead of naming the wrong oldest commit time.
- Branch and tag names longer than 255 bytes are rejected as `InvalidName`
  before touching disk, instead of failing as an I/O error.

## [0.2.0] — 2026-10-03

### Added

- CLI `verge diff <FROM>..<TO> --table <NAME>` that prints row-level changes
  (`+` added, `-` removed, `~` changed) and `no changes` when the tables are
  identical.

### Changed

- The table working data (`tables/<nama>/working`) now points at the prolly tree
  root digest instead of the table content block; `FileTableWorkspace::new` only
  takes the layout.

## [0.1.0] — 2026-10-03

First release: binaries for linux (x86_64, aarch64), macOS (arm64), and Windows
(x86_64), each with `SHA256SUMS` and a CycloneDX SBOM.

### Added

- `table` subdomain: a validated `TableName` (allowlist `[a-z0-9_-]`, at most 64
  characters) so a table name cannot escape `.verge`.
- Two-way commit codec: `commit_encoding` writes canonical bytes,
  `commit_decoding` re-verifies the digest and rejects bytes manipulated on
  disk.
- The `CommitRepository`, `RefPointer`, `TableWorkspace`, and `TableSource`
  ports.
- The `stage_table`, `record_commit`, `read_history`, and `read_snapshot` use
  cases.
- Filesystem adapters: `FileCommitRepository`, `FileRefPointer`,
  `FileTableWorkspace`, `FileTableSource`, and the `now_unix_ms` system clock.
- CLI `verge import`, `verge commit`, `verge log`, and `verge show` with
  time-travel read on old commits.
- ADR-0005: tables stored as content-addressed blocks, along with the
  limitations accepted before the prolly tree arrived.

### Changed

- `Commit` now carries the table name, so history can be filtered per table.
- Commit field accessors are split into `commit_fields.rs` so the file stays
  under the 150-line limit without relaxing visibility.

### Milestone 1 — storage and versioning foundation

Shipped earlier in the same version:

- A two-crate Rust workspace: `verge-core` (engine) and `verge-cli` (the `verge`
  binary).
- A content-addressed block store (`FileBlockStore`) with deduplication, a
  two-level fan-out layout, and atomic writes (temp file + fsync + rename).
- Immutable commits with length-prefixed canonical encoding, so identifiers are
  derived deterministically from the commit content.
- A commit graph with branches (movable pointers), tags (immutable pointers),
  ancestry checks on diamonds, and first-parent traversal.
- The `initialize_repository` use case, with rollback when a creation step fails.
- CLI `verge init`, `--help`, and `--version`.
- A 7-layer architecture with ports in the domain and implementations in
  infrastructure.
- Quality gates: `rustfmt`, `clippy` (pedantic, `-D warnings`), `cargo test`,
  `gitleaks`, `cargo audit`, and dependabot.


[Unreleased]: https://github.com/Miruameli/verge/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/Miruameli/verge/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/Miruameli/verge/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/Miruameli/verge/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Miruameli/verge/releases/tag/v0.1.0
