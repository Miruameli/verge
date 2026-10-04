//! File: `cli_usage.rs`
//!
//! Deskripsi: Teks bantuan dan versi CLI.
//! Layer: config
//! Tanggung jawab: Menyediakan satu sumber teks untuk `--help` dan `--version`.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-04
//! Version: 0.4.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - (tidak ada dependensi eksternal)
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!   - #8 (Milestone 2)
//!   - #18 (Milestone 3)
//!   - #22 (Milestone 4)
//!   - #31 (Tabel tag pada pesan galat)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

/// Versi binary, diambil dari `Cargo.toml` crate `verge-cli`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Teks bantuan yang dicetak untuk `--help` dan saat perintah tidak dikenal.
pub const USAGE: &str = "\
verge — a versioned database: Git for your data

Usage:
  verge init [PATH]                              Create an empty repository at PATH (default: .)
  verge import <PATH> --table <NAME>             Stage a table's data from PATH for the next commit
  verge commit --table <NAME> --message <MSG>    Record the staged data as a new commit
                 [--author <NAME>]               Default author comes from $VERGE_AUTHOR
  verge log [--table <NAME>] [--limit <N>]       Show commit history for a table (default: 20 entries)
  verge show <REVISION> --table <NAME>           Print a table's data at REVISION (HEAD, a commit id, or a branch)
  verge diff <FROM>..<TO> --table <NAME>         Show row changes between two revisions of a table
                                               (both sides accept HEAD, a branch, or a commit id)
  verge branch list                               List branches and their head commits
  verge branch create <NAME>                      Create a branch at the current head
  verge branch switch <NAME>                      Switch HEAD to an existing branch
  verge branch delete <NAME>                      Delete a branch pointer (data is kept)
  verge merge <BRANCH> --table <NAME>              Three-way merge a branch into the current one
                 [--strategy <NAME>]               manual (default), ours, theirs, last-write-wins
                 [--message <MSG>] [--author <NAME>]
  verge query --table <NAME> --as-of <WHEN>        Print a table as it was at a point in time
                 <WHEN> is an RFC 3339 UTC timestamp (2026-10-01T10:00:00Z),
                 unix milliseconds (@1767225600000), a tag, or a commit id
  verge tag create <NAME> [--revision <REV>]      Tag a commit; tags are immutable
  verge tag list                                  List tags with the commit and table each points at
  verge tag delete <NAME>                         Delete a tag (data is kept)
  verge --help                                    Show this message
  verge --version                                 Show the version

Examples:
  verge import users.csv --table users
  verge commit --table users --message \"add users\" --author ana
  verge log --table users --limit 5
  verge show HEAD --table users > users-2026.csv
  verge diff 67f9a3fa6f9a39b1fafe6fa621ab246b5367c2ea2bac05dbdab7f043e82ccfde..HEAD --table users
  verge query --table users --as-of 2026-10-01T10:00:00Z
  verge tag create q2-report --revision HEAD
  verge branch create eksperimen
  verge branch switch eksperimen
  verge merge eksperimen --table users --strategy ours --author ana
";
