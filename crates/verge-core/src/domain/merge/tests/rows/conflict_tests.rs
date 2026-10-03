//! File: `conflict_tests.rs`
//!
//! Deskripsi: Test aturan konflik baris saat merge tiga arah.
//! Layer: domain/merge/tests/rows
//! Tanggung jawab: Membuktikan kapan dua sisi dianggap bertabrakan.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/merge/tests/rows/support.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use super::support::{manual, Outcome};

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
fn penghapusan_bertabrakan_dengan_perubahan_menjadi_konflik() {
    let result = manual(b"id,name\n1,ana\n", b"id,name\n", b"id,name\n1,budi\n");

    assert_eq!(
        result,
        Outcome {
            rows: 0,
            conflicts: vec!["1".to_owned()]
        }
    );
}

#[test]
fn penghapusan_di_satu_saja_diterima() {
    let result = manual(
        b"id,name\n1,ana\n2,budi\n",
        b"id,name\n1,ana\n",
        b"id,name\n1,ana\n2,budi\n",
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
fn baris_baru_dengan_kunci_sama_di_kedua_sisi_dikonflikkan() {
    let result = manual(b"id,name\n", b"id,name\n1,citra\n", b"id,name\n1,budi\n");

    assert_eq!(
        result,
        Outcome {
            rows: 0,
            conflicts: vec!["1".to_owned()]
        }
    );
}
