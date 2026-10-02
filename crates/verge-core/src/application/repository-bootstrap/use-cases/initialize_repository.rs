//! File: `initialize_repository.rs`
//!
//! Deskripsi: Use case pembuatan repository Verge baru.
//! Layer: application/repository-bootstrap/use-cases
//! Tanggung jawab: Mengorkestrasi pembuatan layout repository lewat port.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - application/repository-bootstrap/dtos/initialized_repository.rs
//!   - `domain/storage/ports/{block_store_factory,metadata_writer}.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

use crate::application::repository_bootstrap::dtos::initialized_repository::InitializedRepository;
use crate::config::repository_layout::{RepositoryLayout, HEAD_MAIN};
use crate::domain::storage::ports::block_store_factory::BlockStoreFactory;
use crate::domain::storage::ports::metadata_writer::MetadataWriter;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Membuat repository kosong beserta object store-nya.
///
/// Args:
/// - layout — lokasi repository pada folder kerja pengguna.
/// - `store_factory` — port pembuka object store.
/// - writer — port penulis metadata repository.
///
/// Returns:
/// - Ok(InitializedRepository) — repository siap dipakai bila semua langkah sukses.
///
/// # Errors
///
/// Mengembalikan [`RepositoryAlreadyExists`](crate::VergeError::RepositoryAlreadyExists)
/// bila `.verge` sudah ada, atau error I/O bila langkah pembuatan gagal. Saat
/// langkah gagal, direktori yang sudah dibuat dihapus (rollback) sebelum error
/// dikembalikan.
///
/// Example (tidak dijalankan otomatis karena menulis ke filesystem):
/// ```no_run
/// use verge_core::application::repository_bootstrap::use_cases::initialize_repository::initialize_repository;
/// use verge_core::config::repository_layout::RepositoryLayout;
/// use verge_core::infrastructure::storage::file_system::file_store_factory::FileStoreFactory;
/// use verge_core::infrastructure::storage::file_system::local_file_system::LocalFileSystem;
///
/// let workspace = std::env::temp_dir().join("verge-doc-example");
/// let layout = RepositoryLayout::under(&workspace);
/// let repo = initialize_repository(&layout, &FileStoreFactory, &LocalFileSystem)
///     .expect("bootstrap repository");
/// assert!(layout.heads().is_dir());
/// assert!(repo.store().root().is_dir());
/// ```
pub fn initialize_repository<F, W>(
    layout: &RepositoryLayout,
    store_factory: &F,
    writer: &W,
) -> Result<InitializedRepository<F::Store>>
where
    F: BlockStoreFactory,
    W: MetadataWriter,
{
    if writer.exists(&layout.root) {
        return Err(VergeError::RepositoryAlreadyExists(layout.root.clone()));
    }
    let created = create_layout(layout, writer);
    if let Err(error) = created {
        rollback(layout, writer);
        return Err(error);
    }
    let store = match store_factory.open(&layout.objects()) {
        Ok(store) => store,
        Err(error) => {
            rollback(layout, writer);
            return Err(error);
        }
    };
    Ok(InitializedRepository::new(layout.clone(), store))
}

/// Membuat seluruh direktori dan berkas `HEAD` milik repository baru.
fn create_layout<W: MetadataWriter>(layout: &RepositoryLayout, writer: &W) -> Result<()> {
    writer.create_dir_all(&layout.heads())?;
    writer.create_dir_all(&layout.tags())?;
    writer.write(&layout.head_file(), HEAD_MAIN.as_bytes())?;
    Ok(())
}

/// Menghapus jejak repository yang gagal dibuat agar folder kerja bersih kembali.
fn rollback<W: MetadataWriter>(layout: &RepositoryLayout, writer: &W) {
    drop(writer.remove_dir_all(&layout.root));
}
