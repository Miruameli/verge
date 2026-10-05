# Audit: crates.io backfill preparation

## 2026-10-05 — Backfill scope corrected from empirical packaging tests

| Field  | Value                                                                 |
| ------ | --------------------------------------------------------------------- |
| Time   | 2026-10-05                                                            |
| Action | Tested packaging at each historical tag; corrected the release runbook |
| Actor  | Miruameli                                                             |
| Reason | The runbook sent operators to a step that can never succeed           |
| Related | Issue #48, PR #49, issue #55, PR #56                                  |
| Impact | Backfill scope is `verge-core` only, with the reason recorded          |
| Rollback | Documentation only; `git revert` restores the previous wording         |

### What was tested

`cargo package -p verge-cli` in a clean worktree at each tag:

| Tag | Result |
| --- | ------ |
| `v0.1.0` | `error: all dependencies must have a version specified when packaging.` |
| `v0.2.0` | same |
| `v0.3.0` | same |

All three manifests carry `verge-core = { path = "../verge-core" }` with no
`version` field. The fix exists only on `main`; it was never on a tag.

### What was wrong before

The runbook claimed the CLI failure was a registry-ordering problem that
"resolves itself once step 1 completes". That claim was not tested when it was
written, and it is false.

Two independent blockers were conflated:

1. **Manifest defect.** A path dependency without a `version` cannot be
   packaged. Cargo rejects it before contacting any registry.
2. **Registry absence.** Visible only after blocker 1 is patched, as
   `no matching package named 'verge-core' found`.

Proving them separate required the `--offline` flag. With the registry
unreachable entirely, the unpatched manifest still fails with the blocker 1
message, and the patched manifest then fails with the blocker 2 message. A
claim about registry ordering cannot survive a run where the registry is
provably absent.

### Decision

Backfill `verge-core` only. `verge-cli` ships from `0.4.0`, published from
`main`. The rejected alternative was patching the manifest during publish,
which would publish a package whose contents differ from its tag and so break
the one property the backfill exists to provide.

Stated limitation: `cargo add verge-cli --version 0.3.0` will not resolve.

Third limitation, and the one that is permanent rather than merely
unavoidable: the backfilled pages carry **no README, no `keywords` and no
`categories`**. Those fields were added to the workspace manifest after the
three tags were cut, and a crates.io version cannot be re-uploaded. So
`0.1.0`-`0.3.0` are permanently less discoverable than `0.4.0` onwards, and
that is the price of byte-identical content. Accepted deliberately: the
backfill exists so historical versions are installable by exact version, and
discoverability is recoverable from `0.4.0`.

This is recorded next to the byte-identical claim on purpose. Read alone, that
claim sounds like a pure guarantee; it is a trade, and the cost is named here
and in the runbook rather than left for a user to discover on the crate page.

### Evidence

- `verge-core` packages cleanly at all three tags: 109, 141 and 216 files.
- The `README.md` missing from each crate directory is not a defect. Cargo
  copies the workspace-root `README.md` into the package; the `main` package
  archive contains it, byte-identical to the root file. Confirmed by
  extracting the archive rather than trusting the exit code.
- No code was changed. Structure gate, 313 tests and `cargo fmt --check` all
  pass.