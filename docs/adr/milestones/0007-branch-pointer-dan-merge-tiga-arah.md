# ADR-0007: Branch as a pointer and three-way merge

## Status

Accepted — rolled out in stages. Branch (O(1) pointer) in v0.3.0; three-way
merge follows in the same release.

## Context

At v0.2.0 the repository had only one branch: `HEAD` pointed to `main` and the
branch pointer was never rewritten. The product promise "every experiment is a
branch" could not be kept, because there was nowhere else to experiment.

Two constraints must be met:

1. **Branching must be O(1).** Copying data per branch makes the repository size
   grow in proportion to the number of branches and experiments, which conflicts
   with the content-addressed model already built in ADR-0002 and ADR-0006.
2. **Merge must be auditable.** A merge result must be traceable back to its
   origins: base, ours side, and theirs side.

## Decision

### Branch = pointer

A branch is stored as a single file containing 64 hex characters in
`refs/heads/<nama>`. Creating a branch writes only one small file; no table block
is copied or rewritten. Deleting a branch removes only the pointer, not the
block, so commits that have already been shared can still be read by identifier
and the audit trail stays intact.

### Branch name limits

A branch name becomes a file name, so it is validated as one path segment that is
safe on every platform: not empty, not starting with a dot, containing no path
separators, no Windows forbidden characters (`<>:"|?*`), no control characters,
not ending in a dot or a space, and not a reserved device name (`con`, `nul`,
`com1`, …). The module `domain/commit/value-objects/branch_name_policy.rs` stores
this policy as a pure function so it can be tested without `Result`.

### Merge base (superseded by ADR-0009)

The merge base is the first commit that is an ancestor of both ends of the
branches, computed by walking the `first-parent` chain of each. This follows the
linear history that `verge commit` uses; if commits ever have two parents from a
non-merge process, the walk must be extended to the whole DAG and a "nearest
base" policy would need a decision of its own.

NOTE: this section no longer applies. Practice showed that the `first-parent`
walk yields a base that is too early as soon as a merge commit has two parents;
its replacement decision lives in ADR-0009.

### Conflicts

Rows are merged by key. A side that is unchanged from the base never becomes a
conflict. A conflict occurs only when `ours` and `theirs` both change the same
value from the same base value. Deleting a row that the other side changed is
treated as a conflict, not silently ignored.

### Resolution strategy

| Strategy           | Behavior                                                   |
| ------------------ | ---------------------------------------------------------- |
| `manual`           | prints the conflict list and exits without writing a commit |
| `ours`             | keeps the value of the active branch                       |
| `theirs`           | takes the value of the branch being merged                 |
| `last-write-wins`  | takes the side with the newest `timestamp_unix_ms`         |

## Alternatives Considered

- **Copy tables per branch.** Rejected: it violates O(1) and duplicates storage.
- **Branch as an entry inside a single refs file.** Rejected: one shared file
  means two opposing operations lock the same file; one file per branch keeps
  `create` and `delete` from getting in each other's way.
- **Two-way merge without a base.** Rejected: without a base it is impossible to
  distinguish "changed on one side" from "changed on both sides", so every
  difference becomes a false conflict.
- **Full deltas between commits as a replacement for the tree.** Rejected in
  ADR-0006: a delta must be verified against its base, and losing the base
  damages the audit trail.

## Consequences

- Branch operations stay measurably O(1): creating a branch adds no data blocks
  at all.
- `HEAD` is no longer just a branch name; `switch` rewrites `HEAD` atomically
  with a temporary file name that is never read as a branch.
- A deleted branch can no longer serve as a name reference, so a user must keep
  the commit identifier if they want to refer to that commit later.
- Merge requires reading three versions of a table; a conflicted merge writes
  nothing, so the repo remains recoverable.

## Justification

The promise "Git for your data" is only proven if branch and merge really work;
without both, the finished diff feature can only compare linear history.

## Date
## Author
## Review Date: 2026-12-03
