# Audit: compliance with the modularization rules

## 2026-10-04 — Measuring compliance and repairing violations

| Field    | Value                                                                                          |
| -------- | ---------------------------------------------------------------------------------------------- |
| Time     | 2026-10-04                                                                                     |
| Action   | Measure the whole repository against the modularization rules, then fix the four real violations |
| Actor    | Miruameli                                                                                      |
| Reason   | The modularization rules are only useful when they are measured; until now the folder limit, imports, and layer direction were only kept on the side |
| Related  | Issue #33, ADR-0003 (7-layer architecture), PR #34                                            |
| Impact   | No behavior change; 310 tests passed before and after                                         |
| Rollback | `git revert` the commit on PR #34; no data format changes                                    |

### How it was measured

Every number below is computed from the contents of the repository, not from
memory:

- SLOC is counted as lines that are neither empty nor comments (`//`, `///`,
  `//!`).
- Direct files and direct subfolders are counted per folder, not accumulated
  over paths.
- Layer direction is computed from every `use crate::…` and `use super::…`
  statement, mapped to its target layer.
- Dependency cycles are searched for by DFS traversal over the module graph.

### Before and after

| Rule                                                | Before | After      |
| --------------------------------------------------- | ------ | ---------- |
| Files > 150 SLOC                                    | 0      | 0          |
| Folder > 5 direct files                             | 1      | 0          |
| Folder > 5 subfolders                               | 1      | 1 (intentional) |
| Functions > 50 lines                                | 0      | 0          |
| TODO/FIXME/HACK without an issue number             | 0      | 0          |
| Module dependency cycles                            | 0      | 0          |
| Layer `domain` importing `infrastructure`           | 2      | 0          |
| `#[allow]` attributes without a reason              | 1      | 0          |
| Tests                                               | 306    | 310        |

### Violations that were fixed

1. **`application/version-control/tests/` had six direct files.** Three
   revision-resolution tests were moved to `tests/revision/`, following the
   pattern already used by `tests/tagging/`.
2. **`domain/merge/tests/` imported the filesystem adapter.** `merge_base_tests.rs`
   and `commit_chain_fixture.rs` used `FileCommitRepository` and
   `FileBlockStore`; both were moved to `crates/verge-core/tests/merge/`
   as integration tests. This removes the last remaining layer inversion:
   `domain` now imports only `shared`.
3. **`#[allow(clippy::too_many_arguments)]` on `merge_writer.rs`.** Nine
   arguments were collapsed into seven with no attribute at all: the header and
   the rows merge into a single `TableRows`, and the two merge ends are taken
   from the `MergeSides` that already existed. Behavior did not change because
   both merge sides still become the parent commit in the same order.
4. **The CLI dispatcher imported ten commands.** The six table commands are now
   handled by the group router in
   `commands/table-versioning/dispatch.rs`, so `cli_dispatcher.rs`
   imports five group commands and one table router. The list of table command
   names lives in a single constant with the `is_table_command` function, so a
   registered name cannot possibly lack a `match` arm.
5. **One 57-line integration function.** `versioning.rs` moves the repository,
   graph, and two-branch setup into the `skenario_branch_dan_tag()` helper.
   Snapshot equality before and after the move is proven by the same test.

### Exceptions that still apply

- **`domain/` has seven subfolders.** `commit`, `ident`, `merge`, `storage`,
  `table`, `time`, and `tree` are mutually independent contexts; merging them
  only adds folder depth without reducing the number of concepts. The project
  owner sets the per-folder file-count limit as a target, not an absolute
  number, and the reasoning is recorded in `docs/architecture.md`.
- **`shared/` uses a name that is on the generic-name list.** Layer 6 of the
  7-layer architecture is named `shared/`; renaming it to `common/` or
  `cross-cutting/` adds no clarity. Its contents are not a utility pile: only
  the result kernel and error types.
- **`shared` and `config` import `domain`.** `VergeError` carries `Digest`
  as error content; `config` validates `TableName`, `BlockId`, and
  `HexText` while mapping paths. Both touch only pure value objects.

### Tests that were added

- `table_command_tests.rs` proves that every table command name is recognized,
  that other group commands are not recognized, and that the error message
  names a command that actually exists. This test catches a real class of bug: a
  name added to the list without a `match` arm would end up reported as
  `unknown table command`.

### Process notes

- Manual inspection and reader review did not find these violations; every
  finding came from measurement. Indicators like "this folder is getting
  crowded" only became visible after the directories were counted.
- Two of the five violations (numbers 3 and 4) surfaced because of measurement,
  not because of a failing test: neither changes behavior and all tests passed
  before and after.

---

## 2026-10-04 — Enforcing the rules with an automatic gate

| Field    | Value                                                                                                    |
| -------- | -------------------------------------------------------------------------------------------------------- |
| Time     | 2026-10-04                                                                                               |
| Action   | Turn the manual measurement into a CI job `structure` that rejects PRs                                   |
| Actor    | Miruameli                                                                                                |
| Reason   | The measurement from the previous audit lived only in a conversation; no artifact preserved its results   |
| Related  | Issue #39, Issue #41, PR #40                                                                              |
| Impact   | No behavior change; 313 tests passed; the new gate adds one status check on `main`                       |
| Rollback | Remove the `structure` job from `.github/workflows/ci.yml`; no production code changes                   |

### What is enforced

| Rule                                              | Limit                        |
| ------------------------------------------------- | ---------------------------- |
| SLOC per `.rs` file                              | 150                          |
| Direct files per folder                           | 5                            |
| Subfolders per folder                             | 5, or 10 for a root layer    |
| Required header fields per `.rs` file             | 11, each exactly once        |
| `TODO`/`FIXME`/`HACK` without an issue reference  | 0                            |
| Characters outside the approved punctuation list  | 0                            |

### How the gate verifies itself

The new structure gate was tested by injecting eight classes of damage into a
copy of the repository, then running the gate over that copy:

| Injected damage                     | Result |
| ----------------------------------- | ------ |
| `License:` removed                  | caught |
| `Version:` duplicated               | caught |
| `Related issues:` duplicated        | caught |
| `Related ADR:` removed              | caught |
| Cyrillic homoglyph in `KENAPA`      | caught |
| Stray CJK in a comment              | caught |
| `TODO` without an issue reference   | caught |
| SLOC 155                            | caught |

After all eight damages were restored, the gate passed again with exit 0.
Without this step the gate is only claimed to work, not proven to.

### Known limitations

**Two header forms** are still alive side by side: the canonical form with one
field per line (234 files) and the compact form using `·` as the separator
(32 files). The gate accepts both as long as the compact form still exists;
normalization is recorded in Issue #41.

Two automatic normalization attempts in the same session **failed and were not
committed**: the `·`-based splitter broke values that contain commas, so the
`Dependencies` line contained `` `, ` `` instead of module names. The lesson
taken: header normalization is not one-shot work; the value of each field
must be taken from the file itself and verified before and after.

### Process notes

- The structure gate commit briefly landed on the local branch
  `chore/header-konsisten`, which was created for the cancelled header
  normalization experiment, instead of on the branch `ci/structure-gate`. As a
  result PR #40 briefly showed no changes at all. The fix: PR #38 was merged
  first, the gate commit was cherry-picked onto `main`, then the feature branch
  was force-pushed with `--force-with-lease`. `--force-push` is used only on the
  feature branch; `main` is never force-pushed.
- Two automatic header normalization attempts failed and were reverted via
  `git checkout -- crates` before they could be committed. The cause was the
  `·`-based splitter breaking values that contain commas. Header normalization
  is not one-shot work; it needs value checks before and after, as recorded
  under Known limitations above.

---

## 2026-10-04 — Extending gate coverage to every tracked file

| Field    | Value                                                                                          |
| -------- | ---------------------------------------------------------------------------------------------- |
| Time     | 2026-10-04                                                                                     |
| Action   | Close two gate coverage gaps: non-Latin letters outside `crates/`, and the folder limit outside `crates/` |
| Actor    | Miruameli                                                                                      |
| Reason   | The gate exists to enforce the rules, but two rules did not apply outside `crates/`, so only half of the repository was monitored |
| Related  | Issue #43, PR #44                                                                              |
| Impact   | No product behavior change; 313 tests passed; the gate scans 310 tracked text files             |
| Rollback | Revert `git checkout` on PR #44; no data format changes                                      |

### The two gaps, and how they were proven

| Gap                                                    | How it was proven                                               |
| ------------------------------------------------------ | --------------------------------------------------------------- |
| The non-ASCII rule only scanned `crates/**/*.rs`       | One Cyrillic line was injected into `docs/roadmap.md`; the gate still exited 0 |
| The folder-limit rule only scanned `crates/`           | The four gate modules were moved back to the `.github/scripts/` root, the folder reached seven direct files, the gate still exited 0 |

Both were closed in PR #44. Non-Latin letters are now checked across every
tracked text file using a Latin-letter whitelist; the folder limit now also uses
`.github/scripts/` as a root, and the root itself is counted because
`rglob("*")` returns only descendants.

### Mistakes that nearly passed as "passed"

| Mistake                                                                        | Consequence                                        |
| ------------------------------------------------------------------------------ | -------------------------------------------------- |
| The probe injected the ASCII text `U+041A` instead of a Cyrillic letter       | The gate "passed" without testing anything          |
| The probe used `git checkout -- .`, which restores from the index, not from `HEAD` | One case's damage leaked into the next case; three cases reported the wrong location |
| The `text_rules.py` file itself contained two Hangul letters and was not tracked by git | The gate let it through precisely when the new gate was created |
| The first version of the folder-limit extension did not count the root          | Seven files in `.github/scripts` still passed       |

All four are defects in the verification tool, not in the code it was checking.
Every one was found because the probe was run and its output was read, not
because the automatic check itself ran. The lesson taken: a newly created gate
must be tested by injecting damage, and the probe itself must be checked for
whether it really injects what it claims.

### Consequences that were applied

- `tracked_text_files` uses `--others --exclude-standard`, so new files that
  are not yet tracked are checked too.
- `sys.dont_write_bytecode` is set in the gate so that a read-only process does
  not write `__pycache__/` into the repository.
- `check-structure.py` was split into four modules in `.github/scripts/structure/`;
  the main file briefly rose to 184 SLOC and is now 115.
- Those four modules were moved so that `.github/scripts/` does not exceed the
  limit of five direct files.
