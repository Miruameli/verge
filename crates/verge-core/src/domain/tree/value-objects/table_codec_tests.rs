//! File: `table_codec_tests.rs`
//!
//! Deskripsi: Test konversi isi tabel menjadi baris terurut.
//! Layer: domain/tree/value-objects
//! Tanggung jawab: Membuktikan penguraian, pengurutan, dan round-trip tanpa kehilangan data.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `table_codec.rs`, `../nodes/tree_node.rs`
//! Related issues: #18 (Milestone 3)
//! Related ADR: ADR-0006 (Prolly tree untuk tabel)

use crate::domain::tree::table_codec::TableRows;
use crate::domain::tree::tree_builder::build_plan;
use crate::shared::exceptions::verge_error::VergeError;

#[test]
fn baris_diurutkan_bukan_urutan_file() {
    let rows = TableRows::parse(b"id,name\n3,citra\n1,ana\n2,budi\n").unwrap();

    let keys: Vec<&[u8]> = rows.rows().iter().map(|row| row.key().as_bytes()).collect();
    assert_eq!(keys, [b"1".as_slice(), b"2".as_slice(), b"3".as_slice()]);
    assert_eq!(rows.header(), b"id,name");
}

#[test]
fn isi_tabel_dapat_disusun_kembali_tanpa_baris_hilang() {
    let raw = b"id,name,city\n2,budi,bandung\n1,ana,jakarta\n";
    let rows = TableRows::parse(raw).unwrap();

    assert_eq!(
        rows.to_bytes(),
        b"id,name,city\n1,ana,jakarta\n2,budi,bandung\n"
    );
}

#[test]
fn round_trip_dua_kali_tetap_konsisten() {
    let raw = b"id,name\n1,ana\n2,budi\n";
    let once = TableRows::parse(raw).unwrap().to_bytes();
    let twice = TableRows::parse(&once).unwrap().to_bytes();

    assert_eq!(once, twice);
    assert_eq!(once, raw);
}

#[test]
fn kunci_ganda_mengambil_baris_terakhir() {
    let rows = TableRows::parse(b"id,name\n1,awal\n1,akhir\n").unwrap();

    assert_eq!(rows.len(), 1);
    assert_eq!(rows.get(0).unwrap().value(), b",akhir");
}

#[test]
fn tabel_tanpa_baris_data_tetap_menyimpan_header() {
    let rows = TableRows::parse(b"id,name\n").unwrap();

    assert!(rows.is_empty());
    assert_eq!(rows.header(), b"id,name");
    assert_eq!(rows.to_bytes(), b"id,name\n");
}

#[test]
fn baris_tanpa_pemisah_kolom_ditolak_dengan_nomor_baris() {
    let error = TableRows::parse(b"id,name\n1\n").unwrap_err();

    assert!(
        matches!(error, VergeError::MalformedTable { line: 2, .. }),
        "error yang diharapkan: baris 2 malformed, didapat: {error:?}"
    );
}

#[test]
fn baris_tanpa_kolom_nilai_ditolak() {
    let error = TableRows::parse(b"id,name\n1,\n").unwrap_err();

    assert!(matches!(error, VergeError::MalformedTable { line: 2, .. }));
}

#[test]
fn baris_dengan_kunci_kosong_ditolak() {
    let error = TableRows::parse(b"id,name\n,ana\n").unwrap_err();

    assert!(
        matches!(error, VergeError::MalformedTable { line: 2, .. }),
        "baris tanpa kunci harus ditolak"
    );
}

#[test]
fn nilai_baris_menyimpan_seluruh_kolom_setelah_kunci() {
    let rows = TableRows::parse(b"id,a,b\n7,x,y\n").unwrap();
    let row = rows.get(0).unwrap();

    assert_eq!(row.key().as_bytes(), b"7");
    assert_eq!(row.value(), b",x,y");
}
#[test]
fn daun_dan_internal_menyusun_ulang_tabel_dari_baris_yang_sama() {
    let raw = b"id,name\n1,ana\n2,budi\n3,citra\n";
    let rows = TableRows::parse(raw).expect("tabel valid");
    let plan = build_plan(&rows).expect("plan berhasil");
    let mut rebuilt = rows.header().to_vec();
    rebuilt.push(b'\n');
    for node in plan.nodes.iter().filter(|node| node.is_leaf()) {
        for row in node.rows() {
            rebuilt.extend_from_slice(row.key().as_bytes());
            rebuilt.extend_from_slice(row.value());
            rebuilt.push(b'\n');
        }
    }

    assert_eq!(
        rebuilt, raw,
        "penyusunan ulang daun harus mengembalikan tabel asli"
    );
}
