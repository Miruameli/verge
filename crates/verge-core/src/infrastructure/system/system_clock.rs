//! File: `system_clock.rs`
//!
//! Deskripsi: Sumber waktu engine dari jam sistem.
//! Layer: infrastructure/system
//! Tanggung jawab: Menyediakan waktu saat ini sebagai milidetik sejak epoch Unix.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - (tidak ada dependensi internal)
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

use std::time::{SystemTime, UNIX_EPOCH};

/// Mengembalikan waktu sekarang dalam milidetik sejak epoch Unix.
///
/// Returns:
/// - i64 — jumlah milidetik sejak 1970-01-01T00:00:00Z. Nilai 0 diberikan
///   bila jam sistem berada sebelum epoch, sehingga penulisan commit tidak
///   pernah gagal hanya karena penyesuaian jam.
///
/// Example:
/// ```
/// use verge_core::infrastructure::system::system_clock::now_unix_ms;
///
/// assert!(now_unix_ms() > 0);
/// ```
///
/// Performance: satu syscall jam; tidak ada alokasi.
/// Thread-safe: ya (tanpa state bersama).
#[must_use]
pub fn now_unix_ms() -> i64 {
    let Ok(elapsed) = SystemTime::now().duration_since(UNIX_EPOCH) else {
        return 0;
    };
    i64::try_from(elapsed.as_millis()).unwrap_or(i64::MAX)
}
