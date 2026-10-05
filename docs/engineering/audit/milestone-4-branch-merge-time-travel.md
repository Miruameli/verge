# Audit: Milestone 4 — branch, merge, and time travel

## 2026-10-03 — Milestone 4: O(1) branch, three-way merge, and `AS OF`

| Field  | Value                                                              |
| ------ | ------------------------------------------------------------------ |
| Time   | 2026-10-03                                                         |
| Action | Merge PR #23 (branch), #24 (merge), #26 (time-travel + tag) into `main` |
| Actor  | Miruameli                                                          |
| Reason | Close issue #22 and #25; the "O(1) branching", "3-way merge", and "time-travel query" promises are unmet without all three |
| Related | Issue #22, #25, PR #23, #24, #26, ADR-0007, ADR-0008, ADR-0009       |
| Impact | `verge branch`, `verge merge`, `verge query --as-of`, and `verge tag` work; time travel can be answered without searching for a commit id |
| Rollback | `git revert` on each merge commit; old blocks stay readable because they are content-addressed |

### Evidence

- 306 tests passed after review; `clippy -D warnings` and `cargo fmt` clean; CI
  green on all three PRs (`format`, `lint`, `test`, `audit`, `secrets`).
- Branch smoke run: `verge branch create eksperimen` adds a pointer without
  adding a data block; `branch switch` writes the complete `HEAD`.
- Merge smoke run: `verge merge eksperimen --table users --strategy ours`
  reports `merged eksperimen into main at 66ed2fc47af2 (3 rows, strategy ours)`.
- Time travel smoke run: `tag create q3` → `query --as-of q3` returns the
  contents from when the tag was created; `query --as-of 2026-10-01T00:00:00Z`
  before the first commit is rejected with a message naming the oldest commit
  time.

### Defects found during review and fixed in the same PR

1. `looks_like_time` treated any text with a `-` as the 5th character as a time,
   so the successfully created branch `2026-q1-report` could no longer be read
   by `show` or `query`. Only the `@` prefix and a `YYYY-MM-DD` prefix are now
   routed to the time parser.
2. `tag create` was not atomic: read-then-write followed by `fs::rename`, which
   overwrites the pointer, so two processes could take turns moving the tag. It
   now uses `create_new`.
3. The 10,000 commit limit reported the traversal limit as "oldest commit" even
   though older commits exist. It now reports `SearchLimitReached`.
4. A pointer name longer than 255 bytes reached `fs::write` and surfaced as a
   raw I/O error. It is now rejected as `InvalidName`.

### Process findings

- `FileTagPointer` never had an adapter test at all even though every use case
  test uses `FakeWorld`. The adapter test was added in PR #26.
- Proof that the new tests really capture behavior was obtained by deliberate
  code mutation: removing the table filter in `instant_commit_lookup` made two
  `instant_table_filter_tests` fail.

### Technical debt recorded

- `verge query --as-of <TAG>` still gives an `invalid reference` message when the
  tag points at another table's commit (issue #31).
- The SQL query engine does not exist yet; issue #30 specifies a lexer and a
  simple SELECT parser.