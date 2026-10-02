//! File: `table_name_tests.rs`
//!
//! Deskripsi: Test validasi nama tabel.
//! Layer: domain/table/value-objects
//! Tanggung jawab: Men membuktikan allowlist menolak path traversal.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `table_name.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use crate::domain::table::value_objects::table_name::TableName;

#[test]
fn menerima_nama_mendek_dan_berseparator() {
    assert_eq!(TableName::parse("users").unwrap().as_str(), "users");
    assert_eq!(
        TableName::parse("users_2026-q1").unwrap().as_str(),
        "users_2026-q1"
    );
    assert_eq!(TableName::parse("t2").unwrap().to_string(), "t2");
}

#[test]
fn menolak_path_traversal_dan_nama_kosong() {
    for raw in [
        "", "..", "../etc", "a/b", ".hidden", "-lead", "Users", "a b", "a\\b",
    ] {
        assert!(
            TableName::parse(raw).is_err(),
            "`{raw}` seharusnya ditolak sebagai nama tabel"
        );
    }
}

#[test]
fn menolak_nama_lebih_dari_batas_panjang() {
    let raw = "a".repeat(65);
    assert!(TableName::parse(&raw).is_err());
    let max = "a".repeat(64);
    assert!(TableName::parse(&max).is_ok());
}

#[test]
fn menolak_karakter_kontrol() {
    assert!(TableName::parse("user\nname").is_err());
    assert!(TableName::parse("user\0name").is_err());
}
