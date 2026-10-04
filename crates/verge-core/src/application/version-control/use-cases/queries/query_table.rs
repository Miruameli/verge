//! File: `query_table.rs`
//!
//! Deskripsi: Use case query tabel pada satu titik waktu.
//! Layer: application/version-control/use-cases/queries
//! Tanggung jawab: Menyelesaikan `--as-of` lalu membaca isi tabel pada commit itu.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-04
//! Version: 0.4.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `application/version-control/dtos/snapshot_content.rs`
//!   - `application/version-control/revision_resolver.rs`
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!   - #31 (Tabel tag pada pesan galat)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

use crate::application::version_control::dtos::snapshot_content::SnapshotContent;
use crate::application::version_control::revision_resolver::resolve_revision;
use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::commit::repositories::ports::tag_pointer::TagPointer;
use crate::domain::storage::ports::block_store::Store;
use crate::domain::table::value_objects::table_name::TableName;
use crate::domain::tree::table_reader::read_table;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Masukan query tabel pada satu titik waktu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryTableInput {
    /// Tabel yang isinya dibaca.
    pub table: TableName,
    /// Titik waktu: RFC 3339 UTC, `@<unix_ms>`, nama tag, atau commit id.
    pub as_of: String,
}

/// Membaca isi tabel pada commit yang ditunjuk `as_of`.
///
/// Satu jalur untuk tiga bentuk `--as-of`; tidak ada perintah terpisah untuk
/// waktu maupun tag, sehingga hasil yang sama tidak bisa diperoleh lewat
/// resolution yang berbeda.
///
/// Args:
/// - input — tabel dan titik waktu.
/// - refs — port pointer branch.
/// - tags — port pointer tag.
/// - commits — port penyimpanan objek commit.
/// - store — port object store.
///
/// Returns:
/// - Ok(SnapshotContent) — isi tabel pada titik waktu tersebut.
///
/// # Errors
///
/// Mengembalikan error dari [`resolve_revision`] termasuk
/// [`NoCommitAtInstant`](VergeError::NoCommitAtInstant) bila tidak ada commit
/// pada waktu itu,
/// [`CommitBelongsToOtherTable`](VergeError::CommitBelongsToOtherTable) bila
/// revisi menunjuk commit tabel lain, dan
/// [`BlockNotFound`](VergeError::BlockNotFound) bila blok tabel hilang.
///
/// Example:
/// ```no_run
/// use verge_core::application::version_control::use_cases::queries::query_table::{
///     query_table, QueryTableInput,
/// };
/// use verge_core::domain::table::value_objects::table_name::TableName;
///
/// fn run(
///     refs: &dyn verge_core::RefPointer,
///     tags: &dyn verge_core::TagPointer,
///     commits: &dyn verge_core::CommitRepository,
///     store: &dyn verge_core::Store,
/// ) -> verge_core::Result<()> {
///     let input = QueryTableInput {
///         table: TableName::parse("users")?,
///         as_of: "2026-10-01T10:00:00Z".to_owned(),
///     };
///     let content = query_table(&input, refs, tags, commits, store)?;
///     println!("{} bytes pada {}", content.bytes.len(), content.commit);
///     Ok(())
/// }
/// ```
pub fn query_table(
    input: &QueryTableInput,
    refs: &dyn RefPointer,
    tags: &dyn TagPointer,
    commits: &dyn CommitRepository,
    store: &dyn Store,
) -> Result<SnapshotContent> {
    let id = resolve_revision(&input.as_of, refs, tags, commits, Some(&input.table))?;
    let commit = commits.load(&id)?;
    if commit.table() != &input.table {
        return Err(VergeError::CommitBelongsToOtherTable {
            revision: input.as_of.clone(),
            commit_table: commit.table().clone(),
            requested: input.table.clone(),
        });
    }
    let rows = read_table(commit.tree(), store)?;
    Ok(SnapshotContent {
        commit: id,
        bytes: rows.to_bytes(),
    })
}
