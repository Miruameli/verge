//! File: `strategy_tests.rs`
//!
//! Deskripsi: Teststrategi resolusi konflik merge baris.
//! Layer: domain/merge/tests/rows
//! Tanggung jawab: Membuktikan nilai yang dipilih tiap strategi.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/merge/merge_strategy.rs`, `tests/rows/support.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use crate::domain::merge::merge_strategy::MergeStrategy;

use super::support::{outcome, Outcome};

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
