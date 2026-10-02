//! File: result.rs
//!
//! Deskripsi: Alias `Result` tunggal untuk seluruh engine.
//! Layer: shared/kernel
//! Tanggung jawab: Menyediakan alias hasil dengan parameter error opsional.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `shared/exceptions/verge_error.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

use crate::shared::exceptions::verge_error::VergeError;

/// Alias hasil untuk operasi engine.
///
/// Tipe error adalah parameter opsional sehingga call site yang membutuhkan
/// error spesifik tetap bisa menuliskannya secara eksplisit, tanpa mengubah
/// signature yang sudah dipakai layer lain.
///
/// Example:
/// ```
/// use verge_core::Result;
///
/// fn hitung(nilai: i32) -> Result<i32> {
///     Ok(nilai * 2)
/// }
///
/// assert_eq!(hitung(21).expect("double"), 42);
/// ```
pub type Result<T, E = VergeError> = core::result::Result<T, E>;
