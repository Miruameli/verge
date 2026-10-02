//! File: `cli_usage.rs`
//!
//! Deskripsi: Teks bantuan dan versi CLI.
//! Layer: config
//! Tanggung jawab: Menyediakan satu sumber teks untuk `--help` dan `--version`.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - (tidak ada dependensi eksternal)
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!   - #8 (Milestone 2)
//!   - #18 (Milestone 3)
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
  verge --help                                    Show this message
  verge --version                                 Show the version

Examples:
  verge import users.csv --table users
  verge commit --table users --message \"add users\" --author ana
  verge log --table users --limit 5
  verge show HEAD --table users > users-2026.csv
  verge diff 67f9a3fa6f9a39b1fafe6fa621ab246b5367c2ea2bac05dbdab7f043e82ccfde..HEAD --table users
";
