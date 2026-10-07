//! File: `mod.rs`
//!
//! Deskripsi: Domain SQL query engine — lexer, parser, AST, dan eksekusi.
//! Layer: domain/sql
//! Tanggung jawab: Mendeklarasikan submodule SQL yang dipakai engine query.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `token.rs`, `lexer/mod.rs`, `ast.rs`, `executor.rs`, `parser/`
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!
//! Related ADR:
//!   - ADR-0012 (SQL parser semantics and limits)

pub mod ast;
pub mod executor;
pub mod lexer;
pub mod parser;
pub mod token;
