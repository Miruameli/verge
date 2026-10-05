# Audit: Milestone 2 — table data versioning

## 2026-10-03 — Milestone 2: table data versioning

| Field  | Value                                                                 |
| ------ | --------------------------------------------------------------------- |
| Time   | 2026-10-03                                                            |
| Action | Merge PR #9 into `main`: import, commit, log, and time-travel read   |
| Actor  | Miruameli                                                             |
| Reason | Close issue #8; table data can only be versioned after this          |
| Related | Issue #8, PR #9, ADR-0005                                             |
| Impact | `verge show <commit>` can read table contents from an old commit      |
| Rollback | `git revert` on the merge commit; immutable blocks already written are not damaged |

### Evidence

- 106 tests passed: 68 unit, 18 E2E CLI, 4 integration, 16 doctest.
- Release binary smoke run: `init → import → commit → commit → log → show <old>`
  produces different contents per revision.
- Three stages with identical content add exactly one block on disk.
- Commit bytes modified outside Verge are rejected when read
  (`VergeError::MalformedCommit`).
- CI green: format, lint, test, RustSec audit, and gitleaks.

### Decisions made

- Table content is stored as a single content-addressed block;
  `tables/<nama>/working` stores only the digest. The reason and its limits are
  recorded in ADR-0005.
- `Commit` carries the table name so history can be filtered per table.
- The CLI becomes the composition root that assembles adapters, the system
  clock, and use cases.