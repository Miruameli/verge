# Audit Trail

Record of significant actions on this repository: time, action, reason, actor,
related issue or PR, impact, and rollback procedure.

Each entry lives in its own file so the folder does not pile up and every entry
is easy to trace. This index is the single entry point; a new entry adds a file
under `audit/` and one table row below.

## Entry index

| Entry                                  | Contents                                                       |
| -------------------------------------- | -------------------------------------------------------------- |
| [`audit/fondasi-m1.md`](audit/fondasi-m1.md) | Repository bootstrap, Milestone 1 merge, branch protection |
| [`audit/milestone-2-versioning-tabel.md`](audit/milestone-2-versioning-tabel.md) | Import, commit, log, and time-travel read           |
| [`audit/milestone-3-prolly-tree-diff.md`](audit/milestone-3-prolly-tree-diff.md) | Prolly tree, `verge diff`, and version 0.2.0         |
| [`audit/milestone-4-branch-merge-time-travel.md`](audit/milestone-4-branch-merge-time-travel.md) | O(1) branch, three-way merge, `AS OF`, and immutable tag |
| [`audit/rilis/v0.1.0.md`](audit/rilis/v0.1.0.md) | First release, four binaries, checksum, SBOM           |
| [`audit/rilis/v0.2.0.md`](audit/rilis/v0.2.0.md) | Milestone 3 release, recorded retrospectively        |
| [`audit/rilis/v0.3.0.md`](audit/rilis/v0.3.0.md) | Milestone 4 release and its proof                |
| [`audit/rilis/v0.4.0.md`](audit/rilis/v0.4.0.md) | First registry release: core + CLI 0.4.0, byte-identical |
| [`audit/audit-kepatuhan-mandate.md`](audit/audit-kepatuhan-mandate.md) | Measurement of the modularization rules and the five fixes |

## Entry writing rules

- Every entry states time, action, actor, reason, related issue/PR, impact,
  evidence, and rollback.
- Evidence is concrete: test counts, quality gate results, and binary smoke runs.
  Claims without evidence are not written as fact.
- Entries are only added, never rewritten: mistakes are corrected in a new entry
  that names the old one.