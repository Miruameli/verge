//! File: mod.rs
//!
//! Deskripsi: Use case bootstrap repository.
//! Layer: application/repository-bootstrap
//! Tanggung jawab: Mendeklarasikan use case dan DTO bootstrap repository.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - dtos/mod.rs, use-cases/mod.rs
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

#[path = "dtos/mod.rs"]
pub mod dtos;

#[path = "use-cases/mod.rs"]
pub mod use_cases;
