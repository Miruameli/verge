//! File: `civil_calendar.rs`
//!
//! Deskripsi: Konversi antara hari sejak epoch dan tanggal kalender.
//! Layer: domain/time/value-objects
//! Tanggung jawab: Menghitung hari Julian untuk tanggal dan sebaliknya.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - (tidak ada dependensi)
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)
//!
//! ALTERNATIF: memakai pustaka kalender seperti `chrono` atau `time`; ditolak
//! karenautter所需的转化 hanya sebulan tanpa kalender, sedangkan menambah
//! dependency untuk satu fungsi murni melanggar ADR-0004.

/// Basis kalender Proleptic Gregorian digeser agar Maret menjadi awal tahun.
const DAYS_FROM_CIVIL: i64 = 719_468;

/// Jumlah hari dalam satu siklus 400 tahun.
const DAYS_PER_ERA: i64 = 146_097;

/// Menghitung hari sejak 1970-01-01 dari tanggal kalender.
///
/// ALGORITMA: Helper-Faircloth/Hinnant. Memindahkan awal tahun ke Maret membuat
/// aritmetika bulan menjadi linear, sehingga tidak ada tabel koreksi per bulan
/// dan tidak ada loop. Tahun negatif ditangani dengan menggeser satu era ke
/// bawah agar pembagian INTEGER hanya dapat bekerja pada pembagi positif.
///
/// Args:
/// - year — tahun kalender.
/// - month — bulan 1–12.
/// - day — hari 1–31; pemanggil harus memvalidasi rentangnya lebih dulu.
///
/// Returns:
/// - i64 — jumlah hari sejak epoch, dapat negatif untuk tanggal sebelum 1970.
#[must_use]
pub const fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let shifted = if month <= 2 { year - 1 } else { year };
    let era = if shifted >= 0 { shifted } else { shifted - 399 } / 400;
    let year_of_era = shifted - era * 400;
    let march_offset = month + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * march_offset + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * DAYS_PER_ERA + day_of_era - DAYS_FROM_CIVIL
}

/// Mengubah hari sejak epoch menjadi tahun, bulan, dan hari.
///
/// Kebalikan dari [`days_from_civil`], memakai steps yang sama sehingga kedua
/// fungsi selalu sepakat satu sama lain.
///
/// Args:
/// - days — jumlah hari sejak 1970-01-01, dapat negatif.
///
/// Returns:
/// - (i64, i64, i64) — tahun, bulan 1–12, dan hari 1–31.
#[must_use]
pub const fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let shifted = days + DAYS_FROM_CIVIL;
    let era = if shifted >= 0 {
        shifted
    } else {
        shifted - (DAYS_PER_ERA - 1)
    } / DAYS_PER_ERA;
    let day_of_era = shifted - era * DAYS_PER_ERA;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let march_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * march_month + 2) / 5 + 1;
    let month = march_month + if march_month < 10 { 3 } else { -9 };
    let year_bumped = if month <= 2 { year + 1 } else { year };
    (year_bumped, month, day)
}

/// Melaporkan apakah `year` adalah tahun leap pada kalender Gregorian.
///
/// Aturan: habis dibagi 400, atau habis dibagi 4 tetapi tidak 100.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Mengembalikan jumlah hari dalam bulan `month` pada tahun `year`.
///
/// Returns:
/// - Option<i64> — `None` bila `month` di luar 1–12.
#[must_use]
pub const fn days_in_month(year: i64, month: i64) -> Option<i64> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 if is_leap_year(year) => Some(29),
        2 => Some(28),
        _ => None,
    }
}
