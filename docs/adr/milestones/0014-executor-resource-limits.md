# ADR-0014: Executor Resource Limits

## Status
Accepted

## Date: 2026-10-08
## Author: Miruameli
## Review Date: 2026-10-15

## Context

Issue #90 (M5 Part 3) requires the SQL executor to enforce memory and time
limits to prevent resource exhaustion from large table scans or unbounded
`commits()` traversal.  Without bounds, a query over a large table or a
deep commit history could consume unbounded memory or run indefinitely.

Key constraints:
- Rust has no garbage collector; memory must be tracked explicitly.
- The executor processes rows one at a time in a loop — this is the natural
  checkpoint for both memory and time checks.
- `commits()` traversal is bounded by `MAX_COMMIT_SCAN` (ADR-0011, 10,000)
  but still needs a scan-budget for memory and wall-time.

## Decision

Introduce a `ScanBudget` struct in `domain/sql/executor.rs` that tracks:
1. **Memory budget** — accumulated byte count of output rows written.  When
   the accumulated output exceeds the limit (default 64 MiB), execution
   stops early with `VergeError::QueryResourceLimit`.
2. **Time budget** — wall-clock deadline (default 10 seconds from creation).
   When `Instant::now()` exceeds the deadline, execution stops early with
   `VergeError::QueryResourceLimit`.

The `ScanBudget` is created in the `sql_query` use case with default limits
and passed by reference to `execute_plan` and `build_commit_rows`.  Both
check the budget after processing each row/commit.

A new `VergeError::QueryResourceLimit` variant carries `limit_type`
("memory" or "time") and a human-readable `detail` string.

### Default limits
| Resource | Default | Rationale |
|----------|---------|-----------|
| Memory   | 64 MiB  | Issue #90 default; large enough for typical queries |
| Time     | 10 s    | Issue #90 default; bounds worst-case execution |

## Alternatives Considered

- **External timeout wrapper (e.g. `tokio::time::timeout`):** Rejected because
  the project currently uses no async runtime for SQL queries.  Adding tokio
  solely for query timeouts would be disproportionate.
- **Counting input rows only:** Rejected — memory is proportional to output
  size (projection + WHERE filtering can both grow and shrink the result).
  Tracking written bytes is a direct proxy for actual memory use.
- **Separate `budget.rs` file in `domain/sql/`:** Rejected because
  `domain/sql/` already has 5 direct files (`ast.rs`, `eval.rs`,
  `executor.rs`, `mod.rs`, `token.rs`).  Adding a sixth would violate the
  ≤5-files-per-folder rule.  `ScanBudget` is collocated with `executor.rs`
  since it is an execution concern.

## Consequences

- Query execution is now bounded by memory and time, preventing
  resource-exhaustion denial of service.
- Users see a clear error message when limits are exceeded.
- The budget is checked per-row, adding negligible overhead (one `Instant`
  check + one comparison per row).
- `executor.rs` grows but stays within the 150 SLOC limit after file
  reorganization.
