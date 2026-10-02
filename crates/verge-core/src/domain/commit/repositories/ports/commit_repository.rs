//! File: `commit_repository.rs`
//!
//! Deskripsi: Port penyimpanan objek commit.
//! Layer: domain/commit/repositories/ports
//! Tanggung jawab: Menyimpan dan memuat commit sebagai blok terverifikasi.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/entities/commit.rs`
//!   - `domain/storage/ports/block_store.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use crate::domain::commit::entities::commit::Commit;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::storage::value_objects::put_outcome::PutOutcome;
use crate::shared::kernel::result::Result;

/// Penyimpanan objek commit yang dapat diverifikasi sendiri.
///
/// Byte yang disimpan adalah encoding kanonik commit, sehingga nama blok sama
/// dengan `CommitId`. Implementasi wajib memverifikasi ulang digest saat memuat:
/// storage yang rusak atau dimanipulasi tidak boleh menghasilkan commit yang
/// dipercaya begitu saja.
pub trait CommitRepository {
    /// Menyimpan `commit` dan mengembalikan hasil penulisannya.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila commit tidak dapat ditulis secara durable.
    fn save(&self, commit: &Commit) -> Result<PutOutcome>;

    /// Memuat commit dengan identifier `id`.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`CommitNotFound`](crate::VergeError::CommitNotFound) bila
    /// objek tidak ada dan
    /// [`MalformedCommit`](crate::VergeError::MalformedCommit) bila byte yang
    /// tersimpan bukan encoding kanonik dengan digest yang cocok.
    fn load(&self, id: &CommitId) -> Result<Commit>;
}

impl<T: CommitRepository + ?Sized> CommitRepository for &T {
    fn save(&self, commit: &Commit) -> Result<PutOutcome> {
        (**self).save(commit)
    }

    fn load(&self, id: &CommitId) -> Result<Commit> {
        (**self).load(id)
    }
}
