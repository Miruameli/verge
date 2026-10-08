# ADR-0013: SQL planner, inline AS OF, and `commits()` table-valued function (M5 Part 2)

## Status

Proposed — M5 Part 2 (SQL planner and Verge extensions).

## Context

The SQL engine from M5 Part 1 (ADR-0012) parses `SELECT ... FROM <table> [AS OF <WHEN>] [WHERE <expr>]`
into an AST (`SelectStatement`) and executes it in one pass: the executor
resolves column names to indices row-by-row. Two extensions are needed:

1. **A planner.** Column-name resolution and validation should happen once,
   before execution, so the executor can use pre-resolved indices and the
   `WHERE` expression is validated for every row, not just the first.
2. **Inline `AS OF`.** The SQL `AS OF` clause should take precedence over the
   CLI `--as-of` flag. Currently the SQL syntax cannot express a revision: the
   `as_of` field in the AST is parsed but the use case always uses the CLI
   `--as-of` value. This makes it impossible to read an old snapshot of one
   table while querying another in the same command.
3. **`commits()` table-valued function.** Users want to query the commit
   history itself as a table — `SELECT * FROM commits()` — with columns
   `commit_id, parent_ids, table, author, timestamp, message`, subject to
   `WHERE` filtering and `AS OF` resolution.

## Decision

### 1. Separation of planning and execution

A new `domain/sql/planner/` module converts a `SelectStatement` AST into an
`ExecutionPlan`:

```
SelectStatement  ──plan_select()──→  ExecutionPlan
                                       select_indices: Vec<usize>
                                       filter: Option<&Expr>
                                       as_of: Option<String>
                                       is_table_function: bool
```

The planner:
- Resolves column names in the `SELECT` list to indices against the table header,
  returning `PlanError::ColumnNotFound` when a column does not exist. This
  validation happens once, before execution.
- Carries the `WHERE` expression by reference — it is evaluated per-row by the
  executor's `eval_expr`.
- Preserves `as_of` and `is_table_function` from the AST so the use case can
  branch on them.

`PlanError` converts to `SqlError` via `From`, and `SqlError` converts to
`VergeError` via `From`, so a `?` in the use case chains all three without
explicit mapping.

### 2. Inline `AS OF` precedence

In `sql_query` use case:

```rust
let as_of = stmt.as_of.as_deref().unwrap_or(&input.as_of);
```

SQL `AS OF` takes precedence; the CLI `--as-of` is only a fallback. This means
`SELECT ... FROM users AS OF 'HEAD~1'` reads `users` as it was at the parent
of `HEAD`, regardless of `--as-of HEAD`.

A test (`as_of_inline_mengalahkan_cli`) commits two snapshots on `users` and
asserts that the inline `AS OF 'HEAD~1'` returns the first snapshot's data even
when the CLI passes `--as-of HEAD`.

### 3. `commits()` table-valued function

When `is_table_function` is true and the table name is `commits`, the use case
does not read the table from the block store. Instead it builds rows from the
first-parent commit chain starting at the resolved revision:

- **Virtual header:** `commit_id,parent_ids,table,author,timestamp,message`
  (6 columns, defined in `COMMIT_HEADERS`).
- **`commit_id`:** the `CommitId` hex string, stored as the `TableRow` key.
- **`parent_ids`:** first-parent chain IDs joined with `;`. Comma is the
  column separator in Verge CSV (ADR-0006, no quoting), so `;` is used between
  parent IDs. Only the immediate first parent is followed (first-parent chain).
- **`table`:** the `TableName` stored in the commit.
- **`author`:** the commit author string.
- **`timestamp`:** the commit timestamp in Unix milliseconds.
- **`message`:** the commit summary. This is the **last** column, so it may
  contain commas without ambiguity (ADR-0006 no-quoting format).

The traversal is bounded by `MAX_COMMIT_SCAN` (`const`, value `10_000`) per
ADR-0011. Traversal starts at the resolved revision (e.g. `HEAD`, `@<ts>`,
`refs/tags/<name>`) and follows the first-parent chain newest-first. No sort is
applied — the natural traversal order (newest → oldest) is the output order.

`WHERE` filtering works on `commits()` the same as on regular tables: the
planner resolves the column indices and the executor evaluates `eval_expr`
against each row.

### 4. Plan reuse on existing `WHERE` tests

Existing `WHERE` tests (`where_komparasi`, `where_dengan_and`,
`where_dengan_or`) are unchanged — they exercise the same `WHERE` evaluation
path but through the new planner + `execute_plan` flow instead of the old
`execute_query`.

## Alternatives Considered

- **Resolve columns at execution time (as Part 1 did).** Rejected: per-row
  name lookup is wasted work and defers validation to the first row, so a
  typo surfaces as a row-0 error rather than a pre-execution failure.
- **Parse `AS OF` into a timestamp at parse time.** Rejected in ADR-0012; the
  value is forwarded verbatim to `resolve_revision` so every ADR-0008 form
  (`HEAD~1`, `refs/tags/...`, `@<unix_ms>`, RFC 3339) is supported without
  a second code path.
- **Parent IDs as a separate column type.** Rejected: the CSV format
  (ADR-0006) is delimiter-based with no quoting. Using `;` as the separator
  within `parent_ids` and keeping `message` as the last column preserves
  column count and order.
- **Sort commit rows by hash.** Rejected: chronological (first-parent) order
   is more useful than hash-descending order for a `commits()` log.

## Consequences

- The executor no longer resolves column names; that responsibility lives
  entirely in the planner. `execute_query` (the old path) is kept for
  backward-compatible direct use but `sql_query` use case now uses
  `execute_plan`.
- `AS OF` precedence is now SQL-first: a user who passes `--as-of HEAD` on the
  CLI but writes `AS OF 'HEAD~1'` in SQL gets the SQL revision.
- `commits()` is read-only: it traverses the commit chain but never mutates it.
  It is bounded by the 10 000-commit scan limit (ADR-0011).
- Three Rust source files were split to stay under the 150 SLOC gate:
  `executor.rs` → `executor.rs` + `eval.rs`, `parser/mod.rs` →
  `parser/mod.rs` + `clauses.rs`, `sql_query_tests.rs` →
  `sql_query_tests.rs` + `sql_commits_tests.rs`.

## Justification

Requirements:
- Pre-validate column references before execution so typos fail fast.
- Allow SQL `AS OF` to override the CLI `--as-of` so a single command can read
  multiple tables at different revisions.
- Expose commit history as a queryable table with standard `WHERE` filtering.
- Keep every source file under the 150 SLOC gate.

This design meets all four with a thin planner layer, a use-case-level
branch for `commits()`, and file splits that respect the structure gate.

## Date: 2026-10-08
## Author: Miruameli
## Review Date: 2026-12-08
