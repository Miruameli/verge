# Verge — Git for your data

Verge is a versioned database: every change is a **commit**, every experiment is
a **branch**, and data collaboration is settled with **diff** and **merge**.
No more manual audit trails or database copies for experiments.

```
Verge repository = Prolly tree (immutable blocks) + Commit graph (DAG)
Branch          = one pointer to a commit, not a copy of the data
Time travel     = read state through a commit, a tag, or a recorded time
```

## Status

Milestone 4 — branch, three-way merge, and time-travel `AS OF` are done. What
works today:

| Capability                    | Status                                 |
| ----------------------------- | -------------------------------------- |
| Repository bootstrap          | Done — `verge init`                    |
| Block store content-addressed | Done — deduplication + atomic write    |
| Commit graph (branch/tag)     | Done — movable pointer, no data copied |
| Table commits                 | Done — `verge import` + `verge commit` |
| Commit history                | Done — `verge log`                     |
| Time-travel read              | Done — `verge show <commit>`           |
| Row-level diff                | Done — `verge diff <FROM>..<TO>`       |
| 3-way merge                   | Done — `verge merge`                   |
| Query `AS OF`                 | Done — `verge query --as-of <WHEN>`    |
| Immutable tags                | Done — `verge tag create/list/delete`  |
| SQL + Verge extensions        | Planned — M5                           |

Latest release: [`v0.3.0`](https://github.com/Miruameli/verge/releases/tag/v0.3.0) —
binaries for linux (x86_64, aarch64), macOS (arm64), and Windows (x86_64),
each with `SHA256SUMS` and a CycloneDX SBOM per target.

Full roadmap: [`docs/roadmap.md`](docs/roadmap.md).

## Quick Start

```bash
cargo build --release
cd /tmp/contoh
printf 'id,name\n1,ana\n' > users.csv

verge init
verge import users.csv --table users
verge commit --table users --message "feat: seed users" --author ana

printf 'id,name\n1,ana\n2,budi\n' > users.csv
verge import users.csv --table users
verge commit --table users --message "feat: add budi" --author budi

verge log --table users
verge show HEAD --table users
verge diff 67f9a3fa6f9a39b1fafe6fa621ab246b5367c2ea2bac05dbdab7f043e82ccfde..HEAD --table users
```

Output of `verge log --table users`:

```
5d2ec22025b8 budi feat: add budi
67f9a3fa6f9a ana  feat: seed users
```

Table content at the old commit is still readable without restoring the working
copy:

```bash
verge show 67f9a3fa6f9a39b1fafe6fa621ab246b5367c2ea2bac05dbdab7f043e82ccfde --table users
# id,name
# 1,ana
```

Changes between two revisions are read row by row, without restoring the
working copy:

```
+ 2 ,budi
```

`verge branch` manages branches without copying data:

```
$ verge branch create eksperimen
created eksperimen at e48c017fe0c8e0dd8eda332c96bde7c7274f7b6d651e4c77c28c69c6d48954d2
$ verge branch switch eksperimen
switched main -> eksperimen
$ verge branch list
* eksperimen           e48c017fe0c8
  main                 e48c017fe0c8
```

Creating a branch adds no data blocks at all: only one pointer file under
`.verge/refs/heads/` is written. Deleting a branch only removes the pointer;
blocks and commits remain readable through their identifiers.

`verge merge` merges branches using a three-way merge:

```
$ verge branch create eksperimen && verge branch switch eksperimen
$ # ... different commits in eksperimen ...
$ verge branch switch main
$ # ... different commits in main ...
$ verge merge eksperimen --table users --author ana
merged eksperimen into main at 09e000a35de0 (4 rows, strategy manual)
```

Rows that changed on only one side are taken from that side. Rows changed on
both sides with different values become conflicts; the `manual` strategy prints
all three sides and cancels the merge, while `ours`, `theirs`, and
`last-write-wins` resolve it.

`verge diff` shows row-level changes in a stable order:

```
~ 3 ,citra,surabaya -> ,citra,sidoarjo
+ 4 ,sari,medan
```

A single changed row rewrites only the tree leaf that holds that row; every
other row and the header reuse the same blocks as the previous commit.

`verge query --as-of` answers "the state of the table at a given time" without
looking up commit ids by hand:

```
$ verge tag create q3 --revision HEAD
tagged q3 at HEAD
$ verge query --table users --as-of q3
id,name
1,ana
$ verge query --table users --as-of 2026-10-01T10:00:00Z
id,name
1,ana
2,budi
```

Tags are immutable: `verge tag create` rejects a name that already exists, so a
report that mentions `q3` always points to the same state. `verge tag list`
prints the name, commit, and tables that each tag owns, so a tag never has to be
guessed before it is used as an `--as-of`.

The resulting repository layout:

```
.verge
├── HEAD              # ref: refs/heads/main
├── objects/          # immutable blocks: objects/ab/cd/<sha256>
├── refs/
│   ├── heads/        # branch pointers (movable)
│   └── tags/         # tag pointers (immutable)
└── tables/<nama>/working   # root tree digest, not the data itself
```

## Repository

```
crates/
├── verge-core/       # engine: domain, application, infrastructure, config
└── verge-cli/        # command-line interface (`verge`)
docs/
├── architecture.md   # layer map and dependency rules
├── adr/              # architecture decisions per foundation and milestone
└── engineering/      # audit trail (one file per entry) and decision log
```

The architecture uses 7 standard layers; dependencies may only flow inward:

```
interfaces → application → domain
infrastructure ────────────► domain
config, shared ────────────► all layers
```

Details and the reasons behind them: [`docs/architecture.md`](docs/architecture.md).

## Design Principles

0. **Tables as a prolly tree.** Table content is split into rowed leaves;
   unchanged rows share blocks across commits, so a small change does not
   rewrite the whole table.
1. **Immutable and content-addressed.** Objects never change after they are
   written; their name is the SHA-256 hash of their content, so they can be
   verified without trusting storage.
2. **O(1) branching.** A branch is a pointer; creating or switching branches
   never copies data.
3. **Deterministic.** Canonical encoding is guaranteed, so identical content
   always yields identical identifiers — a prerequisite for verifiable
   deduplication and merge.
4. **Minimal dependencies.** `verge-core` uses a single external dependency
   (`sha2`). Without that, the audit trail and the attack surface grow as well.

## Development

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
gitleaks detect
```

Every command above must be green before a PR is merged; CI enforces it.

## Contributing

Follow [`CONTRIBUTING.md`](CONTRIBUTING.md): every change needs an issue, a
separate branch, and a PR that passes every quality gate.
## License

Apache-2.0 — see [`LICENSE`](LICENSE).
