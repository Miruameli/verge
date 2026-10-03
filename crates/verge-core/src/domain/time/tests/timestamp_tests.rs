//! File: `timestamp_tests.rs`
//!
//! Deskripsi: Test parsing dan pencetakan nilai waktu.
//! Layer: domain/time/tests
//! Tanggung jawab: Membuktikan konversi kalender dan milidetik tidak salah.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/time/value_objects/timestamp.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use crate::domain::time::value_objects::timestamp::Timestamp;

/// Mengurai `text` lalu mengembalikannya sebagai milidetik.
fn millis(text: &str) -> i64 {
    Timestamp::parse(text).expect("waktu harus valid").unix_ms()
}

#[test]
fn epoch_diparse_sebagai_nol() {
    assert_eq!(millis("1970-01-01T00:00:00Z"), 0);
}

#[test]
fn unix_milidetik_diparse_apa_adanya() {
    assert_eq!(millis("@1767225600000"), 1_767_225_600_000);
    assert_eq!(millis("@0"), 0);
}

#[test]
fn spasi_di_tepi_diabaikan() {
    assert_eq!(millis("  1970-01-01T00:00:00Z  "), 0);
}

#[test]
fn pecahan_kolom_dan_detik_konsisten() {
    let base = millis("1970-01-01T00:00:00Z");
    assert_eq!(millis("1970-01-01T00:00:00.5Z"), base + 500);
    assert_eq!(millis("1970-01-01T00:00:00.050Z"), base + 50);
    assert_eq!(millis("1970-01-01T00:00:01Z"), base + 1000);
}

#[test]
fn hari_leap_diperlakukan_benar() {
    // 2024-02-29 hanya ada pada tahun leap; 2025-02-29 tidak boleh diterima.
    assert!(Timestamp::parse("2024-02-29T00:00:00Z").is_ok());
    assert!(Timestamp::parse("2025-02-29T00:00:00Z").is_err());
    // 1900 bukan tahun leap (habis dibagi 100), 2000 adalah (habis dibagi 400).
    assert!(Timestamp::parse("1900-02-29T00:00:00Z").is_err());
    assert!(Timestamp::parse("2000-02-29T00:00:00Z").is_ok());
}

#[test]
fn tanggal_sebelum_epoch_tetap_bernilai_negatif() {
    assert_eq!(millis("1969-12-31T23:59:59Z"), -1000);
}

#[test]
fn offset_bukan_nol_ditolak() {
    // Zona waktu lokal membuat hasil audit bergantung pada mesin pembaca.
    assert!(Timestamp::parse("2026-10-01T10:00:00+07:00").is_err());
    assert!(Timestamp::parse("2026-10-01T10:00:00-05:00").is_err());
    assert!(Timestamp::parse("2026-10-01T10:00:00").is_err());
}

#[test]
fn rentang_tanggal_tidak_sah_ditolak() {
    assert!(Timestamp::parse("2026-13-01T00:00:00Z").is_err());
    assert!(Timestamp::parse("2026-00-01T00:00:00Z").is_err());
    assert!(Timestamp::parse("2026-10-32T00:00:00Z").is_err());
    assert!(Timestamp::parse("2026-04-31T00:00:00Z").is_err());
}

#[test]
fn rentang_waktu_tidak_sah_ditolak() {
    assert!(Timestamp::parse("1970-01-01T24:00:00Z").is_err());
    assert!(Timestamp::parse("1970-01-01T00:60:00Z").is_err());
    assert!(Timestamp::parse("1970-01-01T00:00:60Z").is_err());
}

#[test]
fn pecahan_lebih_dari_tiga_digit_ditolak() {
    // 10:00:00.1234Z akan tercetak ulang sebagai .123Z — nilai berbeda.
    assert!(Timestamp::parse("1970-01-01T00:00:00.1234Z").is_err());
    assert!(Timestamp::parse("1970-01-01T00:00:00.Z").is_err());
}

#[test]
fn panjang_komponen_tetap_dipakai() {
    assert!(Timestamp::parse("2026-1-01T00:00:00Z").is_err());
    assert!(Timestamp::parse("26-10-01T00:00:00Z").is_err());
    assert!(Timestamp::parse("2026-10-01T0:00:00Z").is_err());
}

#[test]
fn unix_milidetik_bukan_angka_ditolak() {
    assert!(Timestamp::parse("@").is_err());
    assert!(Timestamp::parse("@12a").is_err());
    assert!(Timestamp::parse("@-1").is_err());
}

#[test]
fn teks_asing_tidak_terbaca_sebagai_waktu() {
    assert!(Timestamp::parse("HEAD").is_err());
    assert!(Timestamp::parse("main").is_err());
    assert!(Timestamp::parse("").is_err());
    assert!(Timestamp::parse("2026-10-01 10:00:00Z").is_err());
}

#[test]
fn pencetakan_bolak_balik_menjaga_nilai() {
    for text in [
        "1970-01-01T00:00:00.000Z",
        "2026-10-01T10:00:00.123Z",
        "2000-02-29T23:59:59.999Z",
        "1969-12-31T23:59:59.001Z",
    ] {
        let stamp = Timestamp::parse(text).expect("waktu harus valid");
        assert_eq!(stamp.to_rfc3339(), text, "bolak-balik {text}");
    }
}

#[test]
fn pencetakan_menampilkan_presisi_milidetik() {
    let stamp = Timestamp::from_unix_ms(1_767_225_600_123);
    assert_eq!(stamp.to_rfc3339(), "2026-01-01T00:00:00.123Z");
}

#[test]
fn nilai_sebelum_epoch_dicetak_dengan_tanda_negatif_di_depan() {
    // `rem_euclid` menjaga pecahan tetap positif walau detik bernilai negatif.
    assert_eq!(
        Timestamp::from_unix_ms(-1).to_rfc3339(),
        "1969-12-31T23:59:59.999Z"
    );
}
