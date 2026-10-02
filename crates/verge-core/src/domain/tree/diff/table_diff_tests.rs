//! File: `table_diff_tests.rs`
//!
//! Deskripsi: Test perbandingan dua tabel.
//! Layer: domain/tree/diff
//! Tanggung jawab: Membuktikan perubahan terurut dan lengkap.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `table_diff.rs`
//! Related issues: #18 (Milestone 3)
//! Related ADR: ADR-0006 (Prolly tree untuk tabel)

use crate::domain::tree::diff::row_change::RowChange;
use crate::domain::tree::diff::table_diff::TableDiff;

#[test]
fn tabel_identik_tidak_punya_perubahan() {
    let diff = TableDiff::between(b"id,name\n1,ana\n2,budi\n", b"id,name\n1,ana\n2,budi\n");

    assert!(
        diff.is_empty(),
        "tabel identik tidak boleh menghasilkan perubahan"
    );
}

#[test]
fn urutan_berbeda_tetap_dianggap_identik() {
    let diff = TableDiff::between(b"id,name\n1,ana\n2,budi\n", b"id,name\n2,budi\n1,ana\n");

    assert!(diff.is_empty(), "urutan file tidak memengaruhi isi tabel");
}

#[test]
fn baris_baru_ditandai_ditambahkan_dan_terhapus_ditandai_dihapus() {
    let diff = TableDiff::between(b"id,name\n1,ana\n3,citra\n", b"id,name\n1,ana\n2,budi\n");

    assert_eq!(diff.changes.len(), 2);
    assert_eq!(diff.changes[0].key(), "2");
    assert!(matches!(diff.changes[0], RowChange::Added { .. }));
    assert_eq!(diff.changes[1].key(), "3");
    assert!(matches!(diff.changes[1], RowChange::Removed { .. }));
}

#[test]
fn nilai_berubah_ditandai_dengan_nilai_lama_dan_baru() {
    let diff = TableDiff::between(b"id,name\n1,ana\n", b"id,name\n1,budi\n");

    assert_eq!(
        diff.changes,
        [RowChange::Modified {
            key: "1".to_owned(),
            before: b",ana".to_vec(),
            after: b",budi".to_vec(),
        }]
    );
}

#[test]
fn seluruh_kolom_setelah_kunci_dibandingkan_bukan_hanya_kolom_pertama() {
    let diff = TableDiff::between(b"id,a,b\n1,x,y\n", b"id,a,b\n1,x,z\n");

    assert_eq!(
        diff.changes[0],
        RowChange::Modified {
            key: "1".to_owned(),
            before: b",x,y".to_vec(),
            after: b",x,z".to_vec(),
        },
        "perubahan pada kolom ketiga harus terdeteksi"
    );
}

#[test]
fn perubahan_tambahan_dan_penghapusan_tercampur_terurut_menurut_kunci() {
    let diff = TableDiff::between(
        b"id,name\n5,lima\n9,sembilan\n",
        b"id,name\n1,satu\n5,lima\n6,enam\n",
    );

    let keys: Vec<&str> = diff.changes.iter().map(RowChange::key).collect();
    assert_eq!(
        keys,
        ["1", "6", "9"],
        "kunci harus naik meski jenisnya bercampur"
    );
    assert!(matches!(diff.changes[0], RowChange::Added { .. }));
    assert!(matches!(diff.changes[1], RowChange::Added { .. }));
    assert!(matches!(diff.changes[2], RowChange::Removed { .. }));
}

#[test]
fn tabel_kosong_ke_tabel_isi_menandai_seluruh_baris_ditambahkan() {
    let diff = TableDiff::between(b"id,name\n", b"id,name\n1,ana\n2,budi\n");

    assert_eq!(diff.changes.len(), 2);
    assert!(diff
        .changes
        .iter()
        .all(|change| matches!(change, RowChange::Added { .. })));
}

#[test]
fn tabel_isi_ke_kosong_menandai_seluruh_baris_dihapus() {
    let diff = TableDiff::between(b"id,name\n1,ana\n", b"id,name\n");

    assert_eq!(diff.changes.len(), 1);
    assert!(matches!(diff.changes[0], RowChange::Removed { .. }));
}

#[test]
fn tabel_rusak_diperlakukan_sebagai_kosong_tanpa_henti() {
    let diff = TableDiff::between(b"id,name\nbaris rusak\n", b"id,name\n1,ana\n");

    assert_eq!(diff.changes.len(), 1);
    assert_eq!(diff.changes[0].key(), "1");
}
