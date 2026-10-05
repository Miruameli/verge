# ADR-0008: Time-travel `AS OF` and immutable tags

## Status

Accepted — applied in v0.3.0.

## Context

v0.2.0 could already read table contents at a given revision
(`verge show <REVISION>`), but only if the user knew the branch name, `HEAD~N`,
or a hex prefix. Two product advantages were still unmet:

1. **Time-travel query.** Answering "the state of the table at 2026-10-01
   10:00" required the user to record a `commit id` by hand — exactly what
   should be automated for auditing and debugging.
2. **Tags.** The `.verge/refs/tags/` namespace is created by `init` but never
   contains anything, so there is no way to give a name to a point in time.

Three technical challenges must be decided first:

1. **Commits are not ordered on the same branch.** After a merge, `HEAD` points
   to the merge commit with the newest timestamp, but commits from the branch
   that was merged can carry timestamps much older than that. Choosing "the
   first commit found that is older than the requested time" during a random
   lookup returns a different answer depending on storage order.
2. **One instant can match several commits.** Two different commits can have the
   same millisecond, so "the newest" is not always unique.
3. **Local time is ambiguous.** `2026-10-01 10:00` without a time zone means
   different things on different machines, so audit results are not
   reproducible.

## Decision

### `AS OF <TIMESTAMP>` selects the newest commit on the first-parent chain

`AS OF T` selects the first commit on the `first-parent` chain of the active
branch that satisfies `timestamp_unix_ms <= T`. Because the walk always starts at
`HEAD` and stops at the first commit that meets the bound, the result **does not
depend on storage order or on which branch is read first**.

The `first-parent` chain is chosen rather than the whole DAG, for this reason:
the `first-parent` chain is the sequence of states that were actually active on
that branch. Commits from a branch that has already been merged are not "the
state of this branch at that time" — they only became part of the branch when
the merge commit was created.

### The tie-break is defined, not random

When several commits meet the bound with the same millisecond, the one closest to
`HEAD` is chosen. This rule is repeatable: the same input on the same repository
always yields the same commit.

### UTC only

The accepted forms:

| Form                           | Example                    |
| ------------------------------ | -------------------------- |
| RFC 3339 with a `Z` offset     | `2026-10-01T10:00:00Z`     |
| RFC 3339 with milliseconds    | `2026-10-01T10:00:00.123Z` |
| Unix milliseconds prefixed `@` | `@1767225600000`           |

Any non-zero offset is rejected. Converting an offset requires a time zone table
that the engine does not have, and accepting local time makes audit results
dependent on the reading machine. What is rejected is the time zone, not the
precision: a time in milliseconds since the epoch is an integer that does not
depend on the reader.

### Immutable tags

A tag is a pointer like a branch, with two differences:

1. `verge tag create` **rejects** a name that already exists. A tag is never
   moved silently to another commit, because a tag that changes means a change
   in the meaning of the audit trail: a report that cites `tag q2-report` must
   keep pointing to the same state.
2. `verge tag delete` removes the pointer, not the block, so the data can still
   be read through the commit-id.

Tags use the same `branch_name_policy` because both become file names in
`refs/`.

### `--as-of` accepts three forms

One flag for three forms, so there is no separate resolution path:

| Input                 | Handling                                    |
| --------------------- | ------------------------------------------- |
| `2026-10-01T10:00:00Z` | parsed as a timestamp                       |
| `@1767225600000`     | parsed as a timestamp                       |
| `refs/tags/<nama>`   | an explicit tag, no name ambiguity           |
| `refs/heads/<nama>`  | an explicit branch                          |
| any other name        | a branch if it exists, a tag if not — ambiguous if both exist |

The `refs/` prefix is available precisely so that ambiguity can be resolved
explicitly by the user, rather than guessed by the resolver.

### One table per invocation

`--as-of` applies to a single table, just like `verge show`. Answering "the state
of all tables" means having one instant be consistent across all tables, which is
a different problem and deserves its own decision.

## Alternatives Considered

- **Search the whole DAG instead of `first-parent`.** Rejected: commits from a
  branch that has already been merged would appear as a "state" at a time before
  they were merged, so `AS OF` would report a state that was never active on that
  branch.
- **Store a per-time commit index.** Rejected: the index must be updated on every
  commit and can be corrupted, while `first-parent` already gives an answer with
  a clear bound.
- **Mutable tags with `tag create --force`.** Rejected: a label that can change
  makes a report that cites `tag q2-report` point to a different state from the
  one that was ever read. If it is really needed, the user creates a new tag.
- **Accept a time offset and convert to UTC.** Rejected: it requires a time zone
  database inside the engine; that cost is larger than the value gained for a
  local CLI that stores time in Unix milliseconds.
- **Timestamp from `SystemTime::now()` without validation.** Rejected: `AS OF`
  accepts user input, so it must be validated before use. Values greater than
  `i64::MAX/1000` are rejected so that the conversion to milliseconds cannot
  overflow.

## Consequences

- `AS OF` becomes fully deterministic: the same input on the same repository
  always yields the same commit.
- Commits with identical timestamps cannot be told apart through `AS OF`; an
  explicit tag (`refs/tags/<nama>`) is the way out, and that is exactly its
  purpose.
- `verge tag list` must show the commit that a tag points to, so the user can
  be sure the tag is not misleading.
- Tags cannot be used as branch names, so the explicit `refs/` prefix stays
  available even though it is not required in normal use.

## Amendment to ADR-0007

The "Merge base" section of ADR-0007 has been superseded by ADR-0009: the merge
base now follows all parents, not only the `first-parent` chain. That decision is
recorded as its own ADR so that ADR-0007 is no longer read as the specification
of the merge base in force.

## Justification

Time travel without `AS OF` means the user has to look up a `commit id` by hand
every time they want to know a past state — and that is exactly the check done
most often during an audit. Immutable tags close the second gap: giving a name
to a point in time without the risk of a label silently changing meaning.

## Date: 2026-10-03
## Author: Miruameli
## Review Date: 2026-12-03
