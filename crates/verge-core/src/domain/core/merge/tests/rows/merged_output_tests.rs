//! File: `merged_output_tests.rs`
//!
//! Deskripsi: Test bentuk tabel hasil merge tanpa konflik.
//! Layer: domain/merge/tests/rows
//! Tanggung jawab: Membuktikan penggabungan perubahan sepihak dan urutan baris.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/merge/row_merge.rs`, `tests/rows/support.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use crate::domain::merge::merge_strategy::MergeStrategy;
use crate::domain::merge::rows::row_merge::merge_rows;

use super::support::{manual, rows, Outcome};

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
fn baris_hasil_merge_terurut_ulang_menurut_kunci() {
    let merged = merge_rows(
        &rows(b"id,name\n"),
        &rows(b"id,name\n5,lima\n1,satu\n"),
        &rows(b"id,name\n"),
        MergeStrategy::Manual,
        true,
    );

    let keys: Vec<Vec<u8>> = merged
        .rows
        .iter()
        .map(|row| row.key().as_bytes().to_vec())
        .collect();
    assert_eq!(keys, vec![b"1".to_vec(), b"5".to_vec()]);
}
