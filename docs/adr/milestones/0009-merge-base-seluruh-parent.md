# ADR-0009: Merge base follows all parents

## Status

Accepted — applied in v0.3.0 (PR #24).

## Context

ADR-0007 established that the merge base is computed on the `first-parent` chain
of both branches, on the grounds that commits created by the engine are always
linear.

Practice showed that assumption to be wrong as soon as a merge commit has two
parents. A merge result stores `theirs` as its second parent, so `theirs` has
already become an ancestor of the active branch. When that branch is merged
again, the `first-parent` walk never finds `theirs` as an ancestor, the base
falls back to the earliest commit, and the whole table counts as changed — the
second merge writes a commit whose contents are identical to the `theirs` side
while still counting as a new change.

## Decision

The merge base is computed over **all parents** of both ends of the branches:
the base is the nearest commit that is an ancestor of both `ours` and `theirs`,
without restricting the kind of parent that is walked.

As a consequence, a source branch whose every commit is already an ancestor of the
active branch is rejected with `AlreadyMerged` instead of producing an empty merge
commit.

## Alternatives Considered

- **Stay on `first-parent`.** Rejected: it produces a pointless merge on the
  second merge and every merge after that, and a base that is too early makes
  every row look changed.
- **Reject the second merge explicitly without going through a base.** Rejected:
  a correct base is still needed when other commits were inserted between the
  two, so an ordering-based rejection is not reliable.
- **Store an ancestor index per commit.** Rejected: the index must be updated on
  every commit, while the merge base is read only during a merge — the cost of
  the index is not proportional to a lookup that rarely happens.

## Consequences

- A second merge on the same branch no longer writes an empty commit.
- `AlreadyMerged` becomes an actionable error instead of a silent merge.
- Every decision in ADR-0007 apart from the "Merge base" section still applies.

## Date: 2026-10-03
## Author: Miruameli
## Review Date: 2026-12-03
