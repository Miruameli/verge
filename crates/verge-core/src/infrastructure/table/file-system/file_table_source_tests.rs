//! File: `file_table_source_tests.rs`
//!
//! Deskripsi: Test integrasi `FileTableSource` pada berkas sementara.
//! Layer: infrastructure/table/file-system
//! Tanggung jawab: Membuktikan isi dibaca apa adanya dan batas ukuran ditegakkan.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `file_table_source.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use std::fs::{self, File};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use super::file_table_source::FileTableSource;
use crate::domain::table::ports::table_source::{TableSource, MAX_TABLE_BYTES};
use crate::shared::exceptions::verge_error::VergeError;

/// Direktori sementara yang unik untuk satu pengujian.
fn scratch(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    let dir = std::env::temp_dir().join(format!("verge-src-{name}-{}-{nanos}", std::process::id()));
    fs::create_dir_all(&dir).expect("buat direktori sementara");
    dir
}

/// Menulis `bytes` ke berkas di direktori sementara dan mengembalikan path-nya.
fn source_file(name: &str, bytes: &[u8]) -> PathBuf {
    let path = scratch(name).join("users.csv");
    fs::write(&path, bytes).expect("tulis sumber");
    path
}

#[test]
fn isi_berkas_dibaca_tanpa_normalisasi() {
    let source = FileTableSource;
    let raw = b"id,name\r\n1,ana\r\n2,budi\r\n";
    let path = source_file("mentah", raw);

    assert_eq!(source.read_all(&path).expect("baca tabel"), raw);
    drop(fs::remove_dir_all(path.parent().expect("path punya induk")));
}

#[test]
fn berkas_lebih_besar_dari_batas_ditolak_tanpa_dibaca() {
    let source = FileTableSource;
    let path = scratch("besar").join("users.csv");
    // KONTEKS: `set_len` membuat berkas sparse di Linux, jadi berkas 256 MiB
    // tidak pernah benar-benar memakai disk; yang diuji adalah penolakan awal.
    File::create(&path)
        .expect("buat sumber")
        .set_len(MAX_TABLE_BYTES as u64 + 1)
        .expect("set panjang sparse");
    let error = source
        .read_all(&path)
        .expect_err("berkas melebihi batas wajib ditolak");
    assert!(
        matches!(error, VergeError::Io(_)),
        "ditolak sebagai Io, bukan {error:?}"
    );
    assert!(
        error.to_string().contains(&MAX_TABLE_BYTES.to_string()),
        "pesan menyebut batas ukuran: {error}"
    );
    drop(fs::remove_dir_all(path.parent().expect("path punya induk")));
}

#[test]
fn berkas_yang_tidak_ada_melaporkan_error_io() {
    let source = FileTableSource;
    let path = scratch("hilang").join("tidak-ada.csv");

    assert!(matches!(source.read_all(&path), Err(VergeError::Io(_))));
    drop(fs::remove_dir_all(path.parent().expect("path punya induk")));
}
