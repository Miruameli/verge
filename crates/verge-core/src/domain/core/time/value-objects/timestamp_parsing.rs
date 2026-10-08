//! File: `timestamp_parsing.rs`
//!
//! Deskripsi: Penguraian teks waktu pengguna menjadi milidetik Unix.
//! Layer: domain/time/value-objects
//! Tanggung jawab: Menerima hanya RFC 3339 UTC dan unix milidetik, serta
//!   menolak bentuk lain dengan alasan yang dapat ditelusuri.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `civil_calendar.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)
//!
//! ALTERNATIF: memakai parser tanggal pihak ketiga; ditolak karena format yang
//! diterima sengaja sangat sempit sehingga fungsi ini lebih kecil daripada
//! dependency tersebut (ADR-0004).

use crate::domain::time::value_objects::civil_calendar::{days_from_civil, days_in_month};

/// Jumlah detik dalam satu hari.
const SECONDS_PER_DAY: i64 = 86_400;

/// Jumlah milidetik dalam satu detik.
const MILLIS_PER_SECOND: i64 = 1000;

/// Mengurai unix milidetik setelah prefiks `@` sebagai digit desimal.
///
/// ALTERNATIF: menerima `-123`; ditolak karena unix milidetik negatif tidak
/// pernah dihasilkan commit sehingga hanya menambah cabang tanpa guna.
pub(crate) fn parse_unix_ms(digits: &str) -> Option<i64> {
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    digits.parse().ok()
}

/// Mengurai RFC 3339 UTC `YYYY-MM-DDThh:mm:ss[.fff]Z` menjadi milidetik.
///
/// Offset selain nol ditolak: konversinya membutuhkan basis data zona waktu
/// yang tidak dimiliki engine, dan menerima waktu lokal membuat hasil audit
/// bergantung pada mesin pembaca.
pub(crate) fn parse_rfc3339(text: &str) -> Option<i64> {
    let (date, time) = text.strip_suffix('Z')?.split_once('T')?;
    let (year, month, day) = split_date(date)?;
    let (hour, minute, second, millis) = split_time(time)?;
    if day < 1 || day > days_in_month(year, month)? || hour > 23 || minute > 59 || second > 59 {
        return None;
    }
    let seconds =
        days_from_civil(year, month, day) * SECONDS_PER_DAY + hour * 3600 + minute * 60 + second;
    Some(seconds * MILLIS_PER_SECOND + millis)
}

/// Memecah bagian tanggal `YYYY-MM-DD` menjadi tahun, bulan, dan hari.
fn split_date(date: &str) -> Option<(i64, i64, i64)> {
    let mut parts = date.split('-');
    let year = parse_fixed(parts.next()?, 4)?;
    let month = parse_fixed(parts.next()?, 2)?;
    let day = parse_fixed(parts.next()?, 2)?;
    (parts.next().is_none()).then_some((year, month, day))
}

/// Memecah bagian waktu `hh:mm:ss[.fff]` menjadi jam, menit, detik, milidetik.
fn split_time(time: &str) -> Option<(i64, i64, i64, i64)> {
    let (clock, millis) = match time.split_once('.') {
        Some((clock, fraction)) => (clock, parse_millis(fraction)?),
        None => (time, 0),
    };
    let mut parts = clock.split(':');
    let hour = parse_fixed(parts.next()?, 2)?;
    let minute = parse_fixed(parts.next()?, 2)?;
    let second = parse_fixed(parts.next()?, 2)?;
    (parts.next().is_none()).then_some((hour, minute, second, millis))
}

/// Mengurai `text` sebagai digit desimal sepanjang tepat `len` karakter.
///
/// Panjang diperiksa lebih dulu supaya `2026-1-1` tidak diterima sebagai
/// tanggal yang valid format maupun tidak valid secara diam-diam.
fn parse_fixed(text: &str, len: usize) -> Option<i64> {
    (text.len() == len && text.bytes().all(|byte| byte.is_ascii_digit()))
        .then(|| text.parse().ok())
        .flatten()
}

/// Mengurai pecahan detik sepanjang satu sampai tiga digit menjadi milidetik.
///
/// ALTERNATIF: menerima pecahan panjang lalu memotongnya; ditolak karena
/// `10:00:00.1234Z` akan tercetak ulang sebagai `10:00:00.123Z`, yaitu nilai
/// berbeda dari yang diminta pengguna.
fn parse_millis(fraction: &str) -> Option<i64> {
    if !(1..=3).contains(&fraction.len()) || !fraction.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    format!("{fraction:0<3}").parse().ok()
}
