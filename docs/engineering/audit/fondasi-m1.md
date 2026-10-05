# Audit: foundation and Milestone 1

## 2026-10-03 — Repository initialization from zero

| Field  | Value                                                             |
| ------ | ----------------------------------------------------------------- |
| Time   | 2026-10-03                                                        |
| Action | Rebuild the workspace from nothing: engine, CLI, docs, and CI     |
| Actor  | Miruameli (`gh` account verified via `gh auth status`)            |
| Reason | The product specification demands a native versioning foundation and an audit trail with no technical debt |
| Related | Issue #1 (Milestone 1)                                            |
| Impact | 40 tests green, `clippy -D warnings` clean, `cargo fmt` clean    |
| Rollback | `git revert` on the bootstrap commit; no user data is affected  |

## 2026-10-03 — Milestone 1 merge and branch protection

| Field  | Value                                                                 |
| ------ | --------------------------------------------------------------------- |
| Time   | 2026-10-03                                                            |
| Action | Merge PR #2 (squash) into `main` and enable branch protection         |
| Actor  | Miruameli                                                             |
| Reason | Close issue #1 and make `main` a branch that only changes through a PR |
| Related | PR #2, issue #1, issue #6                                              |
| Impact | `main` holds the engine foundation; every quality gate must be green before merge |
| Rollback | `git revert 40bdb34`; branch protection can be changed via repository settings |

### Compliance notes

- Required status checks on `main`: `format`, `lint`, `test`, `audit`, `secrets`.
  The branch must be up to date before merge; force push and branch deletion are
  disabled; protection also applies to admins.
- `required_approving_review_count` is 0 because GitHub does not allow
  self-approval. The self-review checklist on each PR replaces review from
  someone else until there is a second contributor.
- The first bootstrap commit to `main` (`4a961d4`) is an empty commit with no
  code, created only so that the `main` branch exists before a PR can be made.
  All code enters through a PR.

### Milestone 1 definition of done

- [x] 40 tests pass, `clippy -D warnings` clean, `cargo fmt` clean.
- [x] `cargo audit` (RustSec) clean.
- [x] `gitleaks` clean in pre-commit and CI.
- [x] CLI smoke run proven: `verge init` creates the layout, a second init is rejected.
- [x] PR #2 reviewed and merged; issue #1 closed.
- [x] `main` is protected by branch protection.

## Technical debt recorded at foundation time

- Per-row deduplication and `verge diff` wait for the prolly tree in Milestone 3.
- The `v0.1.0` release still waits for the cross-platform binary workflow,
  checksums, and SBOM (issue #6).