//! File: `row_key_tests.rs`
//!
//! Deskripsi: Test value object kunci baris.
//! Layer: domain/tree/value-objects
//! Tanggung jawab: Membuktikan urutan kunci deterministik.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `row_key.rs`
//! Related issues: #18 (Milestone 3)
//! Related ADR: ADR-0006 (Prolly tree untuk tabel)

use crate::domain::tree::value_objects::row_key::RowKey;

#[test]
fn kunci_kosong_dapat_dikenali() {
    assert!(RowKey::new(Vec::new()).is_empty());
    assert!(!RowKey::new(b"id".to_vec()).is_empty());
}

#[test]
fn kunci_non_utf8_tetap_dapat_ditampilkan_tanpa_panik() {
    let key = RowKey::new(vec![0xff, 0xfe]);
    assert_eq!(key.as_bytes(), &[0xff, 0xfe]);
    assert!(!key.to_text().is_empty());
}

#[test]
fn kunci_dengan_nilai_sama_berhasil_dibandingkan() {
    assert_eq!(RowKey::new(b"id-1".to_vec()), RowKey::new(b"id-1".to_vec()));
    assert_ne!(RowKey::new(b"id-1".to_vec()), RowKey::new(b"id-2".to_vec()));
}
