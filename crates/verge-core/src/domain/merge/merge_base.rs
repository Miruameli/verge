//! File: `merge_base.rs`
//!
//! Deskripsi: Pencarian merge base antara dua branch.
//! Layer: domain/merge
//! Tanggung jawab: Menentukan leluhur commit terdekat yang dipakai merge.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/repositories/ports/commit_repository.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use std::collections::HashSet;

use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::shared::kernel::result::Result;

/// Batas panjang rantai yang boleh ditelusuri.
///
/// KONTEKS: repository dapat di-chain tanpa batas oleh commit yang dibuat pengguna; KENAPA:
/// batas membuat merge berhenti dengan error jelas, bukan berjalan tanpa akhir;
/// ALTERNATIF: tanpa batas, satu merge pada repository rusak bisa menghabiskan
/// waktu proses tanpa henti.
const MAX_ANCESTRY_DEPTH: usize = 100_000;

/// Menentukan merge base antara ujung `ours` dan ujung `theirs`.
///
/// KONTEKS: penelusuran mengikuti seluruh parent, bukan hanya `first-parent`; KENAPA:
/// commit merge punya dua parent sehingga `theirs` bisa sudah menjadi leluhur
/// branch aktif lewat parent kedua, dan penelusuran `first-parent` akan
/// menganggapnya belum pernah digabung lalu menulis commit merge sia-sia.
///
/// Args:
/// - ours — ujung branch aktif.
/// - theirs — ujung branch yang digabung.
/// - commits — port pembacaan objek commit.
///
/// Returns:
/// - Ok(Some(CommitId)) — leluhur terdekat yang dipakai sebagai base.
/// - Ok(None) — kedua branch tidak punya leluhur bersama.
///
/// # Errors
///
/// Mengembalikan error dari port bila objek commit gagal dibaca.
pub fn find_merge_base(
    ours: CommitId,
    theirs: CommitId,
    commits: &dyn CommitRepository,
) -> Result<Option<CommitId>> {
    if ours == theirs {
        return Ok(Some(ours));
    }
    let ancestors: HashSet<CommitId> = ancestors_of(ours, commits)?.into_iter().collect();
    for candidate in ancestors_of(theirs, commits)? {
        if ancestors.contains(&candidate) {
            return Ok(Some(candidate));
        }
    }
    Ok(None)
}

/// Mengumpulkan `tip` dan seluruh leluhurnya sampai habis atau melebihi batas.
///
/// KONTEKS: breadth-first per commit sehingga base yang ditemukan adalah yang
/// terdekat pada urutan penelusuran branch sumber.
///
/// # Errors
///
/// Mengembalikan error dari port bila objek commit gagal dibaca.
fn ancestors_of(tip: CommitId, commits: &dyn CommitRepository) -> Result<Vec<CommitId>> {
    let mut chain = vec![tip];
    let mut index = 0;
    while index < chain.len() {
        if chain.len() > MAX_ANCESTRY_DEPTH {
            break;
        }
        let parents = commits.load(&chain[index])?.parents().to_vec();
        for parent in parents {
            if !chain.contains(&parent) {
                chain.push(parent);
            }
        }
        index += 1;
    }
    Ok(chain)
}
