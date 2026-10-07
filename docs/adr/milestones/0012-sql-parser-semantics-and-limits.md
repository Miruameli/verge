# ADR-0012: SQL parser semantics and parser limits (M5)

## Status

Proposed — M5 Part 1 (lexer and parser).

## Context

M4 introduced `verge query --table <NAME> --as-of <WHEN>` which reads an entire
table snapshot at one revision. The product promises "SQL standard + Verge
extensions," but no SQL parser exists. Issue #30 (M5) requests a first increment:
a lexer and parser for simple `SELECT` statements with `AS OF`.

Two design questions must be answered before code is written:

1. **AS OF in SQL form.** ADR-0008 defines `AS OF <WHEN>` for the flag-based
   `--as-of` parameter and documents the accepted forms (RFC 3339 UTC,
   `@<unix_ms>`, `refs/tags/<name>`, `refs/heads/<name>`, bare branch/tag name).
   The SQL syntax must reuse the same resolution pipeline without creating a
   second code path, so the same `resolve_revision` entry point handles both.

2. **Parser limits.** Without boundaries a malformed or deliberately hostile
   query can consume unbounded memory or stack. The parser must state its limits
   explicitly.

## Decision

### The SQL grammar accepted by Part 1

```
query      = "SELECT" select_list "FROM" table_ref as_of_clause
select_list = "*" | column_ref ("," column_ref)*
table_ref  = identifier
as_of_clause = ("AS" "OF" when_value)?
when_value = string_literal | parameter | identifier
```

- Keywords are case-insensitive (`SELECT` == `select`).
- `AS OF` is two separate keyword tokens; the parser composes them into one
  clause.
- A string literal is single-quoted; two consecutive single quotes (`''`) inside
  a string are an escaped quote, not a string terminator.
- A parameter is `@` followed by digits (e.g. `@1767225600000`); it is passed
  through to `resolve_revision` unchanged.
- The `AS OF` value is captured verbatim as a `String` and dispatched to the
  existing revision resolver — no SQL-specific timestamp parsing is introduced.

### The WHERE clause

```
condition  = disjunction
disjunction = conjunction ("OR" conjunction)*
conjunction = comparison  ("AND" comparison )*
comparison  = primary comp_op value
comp_op    = "=" | "!=" | "<>" | "<" | "<=" | ">" | ">="
value      = string_literal | number | identifier
primary    = column_ref | value
```

- Precedence: `OR` < `AND` < comparison. No parentheses in Part 1.
- A bare identifier in a value position that is not a reserved keyword is treated
  as a column reference (useful for `WHERE col = other_col`, though only
  literal comparisons are required by Part 1).

### Parser limits

| Limit | Value | Rationale |
|-------|-------|-----------|
| Max tokens | 8 192 | matches a generous SQL line; catches runaway input early |
| Max identifier length | 64 bytes | same as `MAX_LEN` for `TableName` (ADR-0010) |
| Max WHERE nesting depth | 16 | chained comparisons; deep nesting is almost always a mistake |
| Max columns in select list | 256 | a table with more than 256 columns is a schema smell |

Exceeding any limit returns a parser error that names the limit, never a panic.

### AS OF resolution

- `AS OF` is optional; when omitted, the query resolves against `HEAD` on the
  active branch, exactly as `resolve_revision` already handles `HEAD`.
- The `when_value` text is forwarded to `resolve_revision` with no transformation,
  so every form in ADR-0008 is automatically supported.
- The 10 000 commit scan bound (ADR-0011) applies unchanged; `SearchLimitReached`
  and `NoCommitAtInstant` are returned as-is.

## Alternatives Considered

- **Reuse an external SQL parser crate** — adds a dependency; the grammar is small
  enough to write by hand, and the parser must emit Verge-specific error text.
- **Parse AS OF into a timestamp at parse time** — would duplicate ADR-0008
  validation logic; routing through `resolve_revision` keeps one code path.
- **Support parentheses and NOT in WHERE** — deferred. Precedence without
  parentheses is deterministic and sufficient for Part 1.

## Consequences

- The SQL parser is self-contained in `domain/sql/` with no new crate
  dependency.
- `AS OF` in SQL and `--as-of` on the CLI produce identical revision resolution.
- Future SQL features (joins, GROUP BY, subqueries) extend the grammar without
  changing the existing `AS OF` or resolution path.
- The parser limits are enforced by the parser itself; exceeding them never causes
  a panic or unbounded allocation.

## Justification

Requirements:
- a reproducible, deterministic SQL parser that never panics on bad input,
- `AS OF` resolution consistent with ADR-0008 and ADR-0011,
- explicit, documented limits on parsing resources.

This design meets all three with hand-written code in the domain layer, reusing
the existing revision resolver and table reader.

## Date: 2026-10-08
## Author: Miruameli
## Review Date: 2026-12-08
