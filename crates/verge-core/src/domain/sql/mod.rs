//! File: `mod.rs`
//!
//! Deskripsi: Domain SQL query engine — lexer, parser, AST, planner, dan eksekusi.
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
//!   - `core/token.rs`, `core/ast.rs`, `core/executor.rs`, `core/eval.rs`
//!   - `lexer/mod.rs`, `parser/mod.rs`, `planner/mod.rs`
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!   - #92 (M5 Part 2: planner dan Verge extensions)
//!   - #96 (domain/sql split)
//!
//! Related ADR:
//!   - ADR-0012 (SQL parser semantics and limits)
//!
//! Pipeline structure:
//!   - core/ → token, ast, eval, executor (tipe & eksekusi inti)
//!   - lexer/ → tokenizer
//!   - parser/ → parser + AST builder
//!   - planner/ → query planner

// Core SQL types
#[path = "core/ast/mod.rs"]
pub mod ast;
#[path = "core/eval/mod.rs"]
pub mod eval;
#[path = "core/executor/mod.rs"]
pub mod executor;
#[path = "core/token/mod.rs"]
pub mod token;

// Pipeline components
pub mod lexer;
pub mod parser;
pub mod planner;
