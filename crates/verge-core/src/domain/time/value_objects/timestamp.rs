//! File: `timestamp.rs`
//!
//! Deskripsi: Nilai waktu UTC dalam milidetik sejak epoch Unix.
//! Layer: domain/time/value-objects
//! Tanggung jawab: Membungkus waktu commit agar perbandingan tidak bergantung
//!   pada integer telanjang, serta mengurai dan mencetak waktu tersebut.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `civil_calendar.rs`, `timestamp_parsing.rs`
//!   - `shared/exceptions/verge_error.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use crate::domain::time::value_objects::civil_calendar::civil_from_days;
use crate::domain::time::value_objects::timestamp_parsing::{parse_rfc3339, parse_unix_ms};
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Prefiks unix milidetik agar bentuk angka tidak tertukar dengan commit hex.
const UNIX_PREFIX: char = '@';

/// Jumlah milidetik dalam satu detik.
const MILLIS_PER_SECOND: i64 = 1000;

/// Jumlah detik dalam satu hari.
const SECONDS_PER_DAY: i64 = 86_400;

/// Waktu sebagai milidetik sejak epoch Unix dalam UTC.
///
/// Invariants:
/// - Tidak ada zona waktu tersimpan: nilainya selalu UTC absolut, sehingga
///   perbandingan antar commit tidak bergantung pada mesin pembaca.
///
/// Immutability: penuh — nilai ini disalin, bukan diubah.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp {
    /// Milidetik sejak 1970-01-01T00:00:00Z.
    unix_ms: i64,
}

impl Timestamp {
    /// Membuat waktu dari milidetik Unix yang sudah diketahui benar.
    ///
    /// KENAPA `const fn`: `system_clock` membungkus hasil `SystemTime` tanpa
    /// alokasi maupun jalur error.
    #[must_use]
    pub const fn from_unix_ms(unix_ms: i64) -> Self {
        Self { unix_ms }
    }

    /// Mengembalikan milidetik sejak epoch Unix.
    #[must_use]
    pub const fn unix_ms(self) -> i64 {
        self.unix_ms
    }

    /// Mengurai teks waktu dari pengguna menjadi waktu UTC.
    ///
    /// Bentuk yang diterima: RFC 3339 dengan offset `Z` (opsional pecahan
    /// detik), dan unix milidetik didahului `@`.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`InvalidTimestamp`](VergeError::InvalidTimestamp) untuk
    /// bentuk yang tidak dikenal, tanggal yang tidak ada (mis. 30 Februari),
    /// jam di luar 0–23, pecahan di luar tiga digit, dan offset selain nol.
    ///
    /// Example:
    /// ```
    /// use verge_core::domain::time::value_objects::timestamp::Timestamp;
    ///
    /// let epoch = Timestamp::parse("1970-01-01T00:00:00Z").unwrap();
    /// assert_eq!(epoch.unix_ms(), 0);
    /// assert_eq!(Timestamp::parse("@42").unwrap().unix_ms(), 42);
    /// assert_eq!(epoch.to_rfc3339(), "1970-01-01T00:00:00.000Z");
    /// assert!(Timestamp::parse("2026-10-01T10:00:00+07:00").is_err());
    /// ```
    pub fn parse(text: &str) -> Result<Self> {
        let trimmed = text.trim();
        let unix_ms = match trimmed.strip_prefix(UNIX_PREFIX) {
            Some(digits) => parse_unix_ms(digits),
            None => parse_rfc3339(trimmed),
        };
        unix_ms.map(Self::from_unix_ms).ok_or_else(|| invalid(text))
    }

    /// Mengembalikan representasi RFC 3339 UTC dengan presisi milidetik.
    ///
    /// Keluaran ini dicetak `verge log` sehingga waktu yang ditampilkan
    /// pengguna dapat langsung disalin ke `--as-of` tanpa konversi manual.
    #[must_use]
    pub fn to_rfc3339(self) -> String {
        let seconds = self.unix_ms.div_euclid(MILLIS_PER_SECOND);
        let millis = self.unix_ms.rem_euclid(MILLIS_PER_SECOND);
        let (year, month, day) = civil_from_days(seconds.div_euclid(SECONDS_PER_DAY));
        let sod = seconds.rem_euclid(SECONDS_PER_DAY);
        format!(
            "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{millis:03}Z",
            sod / 3600,
            (sod % 3600) / 60,
            sod % 60,
        )
    }
}

/// Mengembalikan error galat yang menyebut masukan aslinya.
///
/// Menyebut teks yang gagal di-parse penting karena nama zona waktu, panjang
/// digit, dan karakter tak lazim semuanya harus menghasilkan pesan yang sama.
fn invalid(text: &str) -> VergeError {
    VergeError::InvalidTimestamp(text.to_owned())
}
