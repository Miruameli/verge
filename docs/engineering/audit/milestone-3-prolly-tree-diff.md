# Audit: Milestone 3 — prolly tree and row-level diff

## 2026-10-03 — Milestone 3: prolly tree and row-level diff

| Field  | Value                                                              |
| ------ | ------------------------------------------------------------------ |
| Time   | 2026-10-03                                                         |
| Action | Merge PR #19 (prolly tree + diff) and PR #20 (version 0.2.0), tag `v0.2.0` |
| Actor  | Miruameli                                                          |
| Reason | Close issue #18 and remove the one-block-per-table limit in ADR-0005 |
| Related | Issue #18, PR #19, PR #20, ADR-0006                                |
| Impact | Unchanged rows share blocks; `verge diff` shows changes per row     |
| Rollback | `git revert` on the merge commit; old blocks stay readable because they are content-addressed |

### Evidence

- 181 tests passed; `clippy -D warnings` and `cargo fmt` clean; CI green on both PRs.
- Binary smoke run: `verge diff HEAD~1..HEAD --table users` produces
  `~ 3 ,citra,surabaya -> ,citra,sidoarjo` and `+ 4 ,sari,medan`.
- Measured dedup: adding one row adds exactly 3 blocks (leaf, root, commit
  object); the header and the other leaves reuse the same blocks as the previous
  commit.
- `verge diff HEAD..HEAD` prints `no changes`.

### Format change that breaks compatibility with old data

| Field  | Value                                                              |
| ------ | ------------------------------------------------------------------ |
| Time   | 2026-10-03                                                         |
| Action | Replace the raw block snapshot with a prolly tree                 |
| Actor  | Miruameli                                                          |
| Reason | Per-row dedup and diff require row partitioning                   |
| Related | ADR-0006, CHANGELOG 0.2.0                                           |
| Impact | A snapshot from version 0.1.0 cannot be read as a tree           |
| Rollback | `git revert` does not restore data; tables must be re-imported   |

Tables from a repository at version 0.1.0 must be re-imported (`verge import`
then `verge commit`) before `verge log`, `verge show`, or `verge diff` are used.
This limit is recorded as a breaking change in the CHANGELOG and ADR-0006.

### Findings fixed during review

1. `HEADER_MARKER` was not imported in the decoding module, so its arm became a
   binding pattern that captured every marker; every node was read as a header.
   Found by a test, not by inspection.
2. The `build_plan` doctest expected two leaves for data that only fits in one
   leaf.
3. Three files exceeded 150 lines and two folders exceeded 5 files; they were
   split by responsibility without `#[allow]`.
4. Revision resolution only accepted 64 hex digits even though `verge log` prints
   12; a `HEAD~N` resolver and an ambiguous prefix with detection were added.

### Technical debt recorded

- Simple CSV table format without quoting; commas inside values are misread.
- Memory load during import is proportional to the table size.
- Three-way merge and SQL query do not exist yet (M4 and M5).