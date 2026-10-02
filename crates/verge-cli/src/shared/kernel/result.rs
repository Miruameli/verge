//! File: result.rs
//!
//! Deskripsi: Alias `Result` untuk lapisan CLI.
//! Layer: shared/kernel
//! Tanggung jawab: Menyatukan penanganan error adapter menjadi satu tipe.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - anyhow (error dinamis dengan context)
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

/// Alias hasil untuk adapter CLI.
///
/// Tipe error adalah parameter opsional: default `anyhow::Error` dipakai karena
/// lapisan ini adalah batas aplikasi, di mana context error dinamis lebih berguna
/// daripada error bertipe ketat. Test dapat menuliskannya secara eksplisit.
pub type Result<T, E = anyhow::Error> = std::result::Result<T, E>;
