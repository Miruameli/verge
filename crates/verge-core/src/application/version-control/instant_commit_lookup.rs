//! File: `instant_commit_lookup.rs`
//!
//! Deskripsi: Pencarian commit pada atau sebelum waktu tertentu.
//! Layer: application/version-control
//! Tanggung jawab: Memilih commit terbaru pada rantai first-parent yang tidak
//!   melewati waktu yang diminta, untuk satu tabel.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `domain/commit/repositories/ports/*.rs`,
//!   `domain/table/value_objects/table_name.rs`
//! Related issues: #25 (Milestone 4)
//! Related ADR: ADR-0008 (Time-travel AS OF dan tag immutable)

use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::table::value_objects::table_name::TableName;
use crate::domain::time::value_objects::timestamp::Timestamp;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Batas commit yang diperiksa saat mencari waktu.
///
/// KENAPA: penelusuran mengikuti rantai first-parent tanpa indeks; batas ini
/// menjaga biaya tetap wajar dan menjaga batas pencarian ini ketara bagi pengguna
/// alih-alih diam-diam mengembalikan hasil yang mungkin salah.
const MAX_SCAN: usize = 10_000;

/// Outcome pencarian commit pada satu waktu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstantLookup {
    /// Commit ditemukan beserta waktu commitnya.
    Found {
        /// Commit yang dipilih.
        commit: CommitId,
        /// Waktu commit terpilih.
        timestamp: Timestamp,
    },
}

/// Mencari commit terbaru untuk `table` pada atau sebelum `at`.
///
/// KONTEKS: penelusuran dimulai dari `head` lalu berjalan pada rantai
/// first-parent; KENAPA: rantai itu adalah urutan keadaan yang benar-benar
/// aktif pada branch, sedangkan commit dari branch yang sudah digabung bukan
/// "keadaan branch ini" pada waktu itu. Karena penelusuran dari `head` ke
/// belakang, commit pertama yang memenuhi batas adalah yang paling dekat
/// dengan `head`, sehingga hasil tidak bergantung pada urutan penyimpanan.
///
/// Args:
/// - head — ujung branch tempat penelusuran dimulai.
/// - table — tabel yang ditanyakan.
/// - at — batas waktu atas; commit dengan waktu yang sama tetap dipakai.
/// - commits — port penyimpanan objek commit.
///
/// Returns:
/// - `InstantLookup` — commit yang dipilih beserta waktunya.
///
/// # Errors
///
/// - [`NoCommitAtInstant`](VergeError::NoCommitAtInstant) bila tidak ada
///   commit tabel tersebut pada atau sebelum `at`; pesan galat menyebut waktu
///   commit tertua yang ditemukan agar pengguna tahu apa yang tersedia.
pub fn find_commit_at_or_before(
    head: CommitId,
    table: &TableName,
    at: Timestamp,
    commits: &dyn CommitRepository,
) -> Result<InstantLookup> {
    let mut cursor = Some(head);
    let mut scanned = 0_usize;
    let mut oldest: Option<Timestamp> = None;
    while let Some(current) = cursor {
        if scanned == MAX_SCAN {
            break;
        }
        let commit = commits.load(&current)?;
        let stamp = Timestamp::from_unix_ms(commit.timestamp_unix_ms());
        oldest = Some(oldest.map_or(stamp, |kept: Timestamp| kept.min(stamp)));
        if commit.table() == table && stamp <= at {
            return Ok(InstantLookup::Found {
                commit: current,
                timestamp: stamp,
            });
        }
        cursor = commit.parents().first().copied();
        scanned += 1;
    }
    Err(VergeError::NoCommitAtInstant {
        requested: at.to_rfc3339(),
        oldest: oldest.map_or_else(|| "none".to_owned(), Timestamp::to_rfc3339),
    })
}
