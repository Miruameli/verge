# ADR-0011: Bound of 10,000 commits on time-travel and prefix search traversals

## Status

Accepted

## Context

`AS OF` walks the `first-parent` chain without an index
(`find_commit_at_or_before` in
`crates/verge-core/src/application/version-control/instant_commit_lookup.rs`).
On a repository with a long history that walk is unbounded work: every step
loads one commit block, and a timestamp older than anything in the table would
scan the whole chain before answering "not found". The same holds for
hex-prefix search over the active chain
(`find_by_prefix` in `revision_walk.rs`).

ADR-0008 specifies the `first-parent` rule, the tie-break, UTC-only input,
and immutable tags, but says nothing about a traversal bound. Issue #30 cites
ADR-0008 as the authority for "rantai first-parent dan batas 10.000 commit",
which is wrong on the second half: the number exists only in code
(`const MAX_SCAN: usize = 10_000` in both files named above), in the
`SearchLimitReached` error variant, and in one audit narrative
(`docs/engineering/audit/milestone-4-branch-merge-time-travel.md`). Anyone
reasoning from the ADRs alone would assume the scan is unbounded.

There are three bound sites, not one:

1. `find_commit_at_or_before` — stops after `MAX_SCAN` commits and returns
   `VergeError::SearchLimitReached { requested, boundary }`. `boundary` is the
   timestamp of the last commit actually examined, deliberately **not** the
   oldest commit in the repository: reporting `oldest` at that point would
   state something the walk never established (see the `KONTEKS` comment at the
   return site).
2. `find_by_prefix` — stops after `MAX_SCAN` commits by breaking out of the
   loop, so a prefix with no match inside the window surfaces as ordinary
   `InvalidRef`, not as a limit error.
3. `walk_back` (`HEAD~N`) — has **no** bound; depth is given explicitly by the
   user, so each step is requested work rather than open-ended search.

## Decision

1. The bound stays at `10_000` scanned commits for both `find_commit_at_or_before`
   and `find_by_prefix`. It bounds cost, not semantics: hitting the edge
   surfaces as an error, never as a silently wrong answer.
2. `SearchLimitReached` keeps the `{ requested, boundary }` shape. `boundary`
   tells the caller how far the walk got, so a retry with a nearer timestamp
   or an explicit tag (`refs/tags/<nama>`) is actionable.
3. `find_by_prefix` keeps the break-with-`InvalidRef` behaviour: a prefix miss
   inside the window is indistinguishable from a genuine miss, and inventing a
   separate "prefix limit" error would leak search-window internals into
   reference resolution. The window is documented here so the behaviour is not
   a surprise.
4. `walk_back` stays unbounded: `HEAD~N` with explicit `N` needs no guard; a
   depth beyond history already returns `InvalidRef`.
5. Raising the bound means changing `MAX_SCAN` in **both** files (the constant
   is duplicated, one per traversal) and re-baselining the performance
   expectation below. No index, no configuration flag: a flag would let two
   callers disagree about what "the state at time T" means.

Performance expectation: each scanned step is one commit-block load plus a
timestamp comparison — linear in the number of commits walked, constant
memory. The bound therefore caps `AS OF` latency at roughly 10,000 block
loads in the worst case.

## Alternatives Considered

- **Unbounded scan.** Rejected: a timestamp older than any commit in the table
  would walk the entire history on every call, turning a typo in a year into
  a full-chain read.
- **A per-time commit index.** Rejected already by ADR-0008 for the same
  reasons; the bound is the cheaper half of that decision and does not
  contradict it.
- **One shared constant or a config flag.** Rejected: the two traversals have
  different miss semantics (explicit limit error vs. ordinary `InvalidRef`),
  so sharing a name would suggest a shared behaviour that does not exist. A
  flag would split the meaning of identical queries across callers.
- **Editing ADR-0008 to add the bound.** Rejected: ADRs are immutable per
  `CONTRIBUTING.md`. This record amends ADR-0008 without touching it, the
  same pattern ADR-0008 itself uses for ADR-0007.

## Consequences

- `AS OF` has a documented horizon: tables whose history exceeds 10,000
  commits on the `first-parent` chain need an explicit tag or commit id for
  older instants.
- Issue #30 must cite this ADR (not ADR-0008) for the bound; the `first-parent`
  citation stays with ADR-0008.
- Future work that indexes commits by time supersedes this ADR with a new
  record; it does not edit this one.

## Amendment to ADR-0008

ADR-0008 remains the specification of the `first-parent` rule, tie-break, UTC
handling, and tag immutability. This ADR adds the traversal bound that
ADR-0008 never stated. Neither record is edited by the other.

## Justification

A bound that exists only in code is a silent contract: users discover it as an
error they never agreed to, and contributors reasoning from the ADRs assume
unbounded scans. Writing it down makes the horizon part of the product
semantics, explains what each caller sees at the edge, and names the exact
price of moving it.

## Date: 2026-10-06
## Author: Miruameli
## Review Date: 2026-12-06
