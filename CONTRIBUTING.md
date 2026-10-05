# Contributing to Verge

Thank you for reading. This document describes the workflow that must be followed.

## Prerequisites

- A Rust toolchain matching `rust-toolchain.toml` (installed through `rustup`).
- `gitleaks` for local secret detection.
- The `gh` CLI for all GitHub operations.

## Mandatory workflow

1. **Issue first.** Every code change starts from an issue that explains the
   problem, the impact, and the priority. A PR without a closing issue is
   rejected.
2. **A separate branch** off `main`:

   | Type     | Prefix      | Example                   |
   | -------- | ----------- | ------------------------- |
   | Feature  | `feat/`     | `feat/prolly-tree`       |
   | Fix      | `fix/`      | `fix/atomic-block-write` |
   | Refactor | `refactor/` | `refactor/commit-graph`  |
   | Docs     | `docs/`     | `docs/architecture`       |
   | Chores   | `chore/`    | `chore/dependabot`        |

3. **Conventional Commits** with an imperative subject of at most 72 characters:

   ```text
   feat(storage): write blocks atomically with rename

   A block that is only partially written can be read as corrupted data.
   Rename after fsync makes the block visible only when it is whole.
   ```

4. **Local quality gates** — all of them must be green:

   ```bash
   cargo fmt --all --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all
   gitleaks detect
   python3 .github/scripts/structure/check-structure.py
   ```

5. **Self-review** before opening the PR: read the diff again as if you were
   someone else, and check for dead code, duplication, and tests that only
   exercise wiring.
6. **The PR** contains Problem, Approach, Alternatives, Risks/Rollback, and
   Verification. Include `Closes #<issue>`.
7. **Merge** only after CI is green and the PR checklist is complete.

## Code quality rules

- The **150-line-per-file** limit is binding. The **5 direct files** and
  **5 subfolders** per folder are targets: a deviation is accepted when every
  item in the folder is a separate context, and the reason is written in
  [`docs/architecture.md`](docs/architecture.md).
- Every file has a header comment: `File`, `Deskripsi`, `Layer`,
  `Tanggung jawab`, `Author`, `Created`, `Modified`, `Version`, `License`,
  `Dependencies`, `Related issues`, `Related ADR`.
- Every public function, struct, and trait has a doc comment that states the
  arguments, return, errors, and an example where relevant.
- `TODO`/`FIXME`/`HACK` must include an issue number; without one the PR is
  rejected.
- Code must not cross layers directly: presentation touches application,
  application and infrastructure touch the domain, never the other way around.

## Tests

- Unit tests for domain logic, integration tests for the end-to-end storage
  path, and E2E tests for CLI behavior through the real binary.
- Tests must be deterministic: no dependency on the network, the system clock,
  or execution order. Temporary directories use a unique name per process.

## ADR

Significant architectural decisions are written as a new ADR in `docs/adr/`.
Old ADRs are never edited; a correction is a new ADR that supersedes it.

## Reporting vulnerabilities

Do not open a public issue for a vulnerability. Follow [`SECURITY.md`](SECURITY.md).
