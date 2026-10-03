//! File: `mod.rs`
//!
//! Deskripsi: Pointer ref pada filesystem lokal.
//! Layer: infrastructure/commit/file-system/refs
//! Tanggung jawab: Mendeklarasikan implementasi branch dan tag beserta helper
//!   I/O pointer yang keduanya pakai.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `file_ref_pointer.rs`, `file_tag_pointer.rs`, `pointer_dir.rs`,
//!     `pointer_file.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

pub mod file_ref_pointer;
pub mod file_tag_pointer;
pub mod pointer_dir;
pub mod pointer_file;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
