//! File: `row_strategy_tests.rs`
//!
//! Deskripsi: Test aturan konflik dan strategi resolusi merge.
//! Layer: domain/merge/tests
//! Tanggung jawab: Membuktikan setiap strategi dan kasus konflik per baris.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/merge/{merge_strategy,row_merge}.rs`
//!   - `domain/tree/table_codec.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use crate::domain::merge::merge_strategy::MergeStrategy;
use crate::domain::merge::rows::row_merge::merge_rows;
use crate::domain::tree::table_codec::TableRows;
use crate::domain::tree::value_objects::table_row::TableRow;

/// Bentuk hasil merge agar assertion test tetap pendek.
#[derive(Debug, PartialEq, Eq)]
struct Outcome {
    /// Jumlah baris hasil gabungan.
    rows: usize,
    /// Kunci konflik yang tersisa.
    conflicts: Vec<String>,
}

/// Mengurai tabel fixture menjadi baris terurut.
fn rows(text: &[u8]) -> Vec<TableRow> {
    TableRows::parse(text)
        .expect("tabel fixture valid")
        .rows()
        .to_vec()
}

/// Menjalankan merge dengan `ours_is_newer` sesuai argumen.
fn outcome(
    base: &[u8],
    ours: &[u8],
    theirs: &[u8],
    strategy: MergeStrategy,
    ours_newer: bool,
) -> Outcome {
    let merged = merge_rows(
        &rows(base),
        &rows(ours),
        &rows(theirs),
        strategy,
        ours_newer,
    );
    Outcome {
        rows: merged.rows.len(),
        conflicts: merged.conflicts.iter().map(|c| c.key.clone()).collect(),
    }
}

/// Menjalankan merge dengan branch aktif dianggap lebih baru.
fn manual(base: &[u8], ours: &[u8], theirs: &[u8]) -> Outcome {
    outcome(base, ours, theirs, MergeStrategy::Manual, true)
}

#[test]
fn perubahan_satu_sisi_diterima_tanpa_konflik() {
    let result = manual(
        b"id,name\n1,ana\n",
        b"id,name\n1,ana\n2,budi\n",
        b"id,name\n1,ana\n",
    );

    assert_eq!(
        result,
        Outcome {
            rows: 2,
            conflicts: vec![]
        }
    );
}

#[test]
fn perubahan_berdua_pada_kunci_berbeda_digabung_bersamaan() {
    let result = manual(
        b"id,name\n1,ana\n2,budi\n",
        b"id,name\n1,ana\n2,budi\n3,citra\n",
        b"id,name\n1,ana\n2,budi\n4,dimas\n",
    );

    assert_eq!(
        result,
        Outcome {
            rows: 4,
            conflicts: vec![]
        }
    );
}

#[test]
fn perubahan_berdua_pada_nilai_sama_bertabrakan() {
    let result = manual(
        b"id,name\n1,ana\n",
        b"id,name\n1,citra\n",
        b"id,name\n1,budi\n",
    );

    assert_eq!(
        result,
        Outcome {
            rows: 0,
            conflicts: vec!["1".to_owned()]
        }
    );
}

#[test]
fn strategi_ours_mempertahankan_nilai_branch_aktif() {
    let result = outcome(
        b"id,name\n1,ana\n",
        b"id,name\n1,citra\n",
        b"id,name\n1,budi\n",
        MergeStrategy::Ours,
        true,
    );

    assert_eq!(
        result,
        Outcome {
            rows: 1,
            conflicts: vec![]
        }
    );
}

#[test]
fn strategi_theirs_mengambil_nilai_branch_yang_digabung() {
    let result = outcome(
        b"id,name\n1,ana\n",
        b"id,name\n1,citra\n",
        b"id,name\n1,budi\n",
        MergeStrategy::Theirs,
        false,
    );

    assert_eq!(
        result,
        Outcome {
            rows: 1,
            conflicts: vec![]
        }
    );
}

#[test]
fn last_write_wins_memilih_sisi_lama_bila_theirs_lebih_baru() {
    let result = outcome(
        b"id,name\n1,ana\n",
        b"id,name\n1,citra\n",
        b"id,name\n1,budi\n",
        MergeStrategy::LastWriteWins,
        false,
    );

    assert_eq!(
        result,
        Outcome {
            rows: 1,
            conflicts: vec![]
        }
    );
}
