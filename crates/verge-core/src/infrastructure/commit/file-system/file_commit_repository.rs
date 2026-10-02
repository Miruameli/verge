//! File: `file_commit_repository.rs`
//!
//! Deskripsi: Implementasi `CommitRepository` di atas block store.
//! Layer: infrastructure/commit/file-system
//! Tanggung jawab: Menyimpan objek commit sebagai blok dan memverifikasi digest saat memuat.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/repositories/ports/commit_repository.rs`
//!   - `infrastructure/storage/file-system/file_block_store.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use crate::domain::commit::codec::commit_decoding::decode;
use crate::domain::commit::entities::commit::Commit;
use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::storage::ports::block_store::Store;
use crate::domain::storage::value_objects::put_outcome::PutOutcome;
use crate::infrastructure::storage::file_system::file_block_store::FileBlockStore;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Repository objek commit yang berbagi block store dengan data tabel.
///
/// Commit tidak memakai format berkas khusus: encoding kanoniknya ditulis lewat
/// `Store::put`, sehingga nama blok otomatis sama dengan `CommitId` dan data
/// tabel tidak pernah diduplikasi di tempat lain.
#[derive(Debug, Clone)]
pub struct FileCommitRepository {
    /// Store yang memegang seluruh blok repository.
    store: FileBlockStore,
}

impl FileCommitRepository {
    /// Membuat repository commit di atas `store`.
    ///
    /// Args:
    /// - store — block store yang sama dengan dipakai data tabel.
    ///
    /// Returns:
    /// - Self — repository tanpa state turunan; seluruh state ada di `store`.
    #[must_use]
    pub fn new(store: FileBlockStore) -> Self {
        Self { store }
    }
}

impl CommitRepository for FileCommitRepository {
    /// Menyimpan `commit` sebagai blok content-addressed.
    ///
    /// Returns:
    /// - Ok(PutOutcome) — identifier blok; sama dengan [`Commit::id`] karena
    ///   store meng-hash byte yang sama dengan yang membentuk identifier commit.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O bila blok tidak dapat ditulis secara durable.
    fn save(&self, commit: &Commit) -> Result<PutOutcome> {
        self.store.put(&commit.encode())
    }

    /// Memuat commit lalu memverifikasi ulang digest-nya.
    ///
    /// # Errors
    ///
    /// Mengembalikan [`CommitNotFound`](VergeError::CommitNotFound) bila objek
    /// tidak ada dan
    /// [`MalformedCommit`](VergeError::MalformedCommit) bila byte yang tersimpan
    /// tidak kanonik atau digestnya tidak cocok dengan nama blok yang diminta.
    fn load(&self, id: &CommitId) -> Result<Commit> {
        let bytes = self.read_object(id)?;
        let commit = decode(&bytes)?;
        // KONTEKS: `decode` sudah memastikan byte kanonik, tetapi belum
        // memastikan byte itu milik blok yang sedang dibaca.
        // KENAPA: pointer yang dimanipulasi akan menunjuk commit lain; tanpa
        // pemeriksaan ini riwayat bisa menampilkan commit yang tidak pernah
        // ditulis ke branch tersebut.
        // ALTERNATIF: mempercayai nama blok langsung ditolak karena storage
        // bisa rusak dan isi blok bisa diganti tanpa mengubah nama berkasnya.
        if commit.id() != *id {
            return Err(VergeError::MalformedCommit {
                reason: "digest does not match the block name",
            });
        }
        Ok(commit)
    }
}

impl FileCommitRepository {
    /// Membaca byte objek commit, menormalkan kegagalan blok hilang.
    ///
    /// # Errors
    ///
    /// Mengembalikan error I/O selain blok yang memang tidak ada.
    fn read_object(&self, id: &CommitId) -> Result<Vec<u8>> {
        self.store.get(id).map_err(|error| match error {
            VergeError::BlockNotFound { .. } => VergeError::CommitNotFound(*id),
            other => other,
        })
    }
}
