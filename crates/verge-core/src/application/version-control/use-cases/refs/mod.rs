//! File: `mod.rs`
//!
//! Deskripsi: Titik masuk use case pointer ref.
//! Layer: application/version-control/use-cases/refs
//! Tanggung jawab: Mendaftarkan use case branch (bergerak) dan tag (immutable).
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.3.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `branching/`, `tagging/`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

pub mod branching;
pub mod tagging;
