//! File: `merge_branch.rs`
//!
//! Deskripsi: Use case merge branch ke branch aktif.
//! Layer: application/version-control/use-cases/merging
//! Tanggung jawab: Menghitung merge base, menggabungkan baris, menulis commit.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/merge/**`, `domain/tree/**`, `domain/commit/**`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use crate::application::version_control::dtos::merging::merge_report::MergeReport;
use crate::application::version_control::use_cases::merging::merge_reader::read_sides;
use crate::application::version_control::use_cases::merging::merge_writer::write_merge_commit;
use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::repositories::ports::ref_pointer::RefPointer;
use crate::domain::merge::merge_base::find_merge_base;
use crate::domain::merge::merge_strategy::MergeStrategy;
use crate::domain::merge::rows::row_merge::merge_rows;
use crate::domain::storage::ports::block_store::Store;
use crate::domain::table::value_objects::table_name::TableName;
use crate::domain::tree::table_codec::TableRows;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Masukan merge branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeBranchInput {
    /// Branch yang datanya digabung ke branch aktif.
    pub source: String,
    /// Tabel yang digabung.
    pub table: TableName,
    /// Strategi resolusi konflik.
    pub strategy: MergeStrategy,
    /// Pesan commit merge.
    pub message: String,
    /// Author yang tercatat pada commit merge.
    pub author: String,
    /// Waktu commit merge dalam milidetik sejak epoch Unix.
    pub timestamp_unix_ms: i64,
}

/// Menggabungkan branch `input.source` ke branch aktif.
///
/// KONTEKS: merge menulis satu commit dengan dua parent sehingga asal kedua sisi
/// tetap dapat ditelusuri; KENAPA: menimpa commit branch aktif seperti commit
/// biasa menghapus bukti bahwa tabel pernah punya dua versi.
///
/// Args:
/// - input — branch sumber, tabel, strategi, dan metadata commit.
/// - refs — port pointer branch.
/// - commits — port penyimpanan objek commit.
/// - store — port object store blok.
///
/// Returns:
/// - Ok(MergeReport) — commit merge, atau daftar konflik bila strategi manual
///   menemukan konflik sehingga tidak ada yang ditulis.
///
/// # Errors
///
/// Mengembalikan [`NothingToMerge`](VergeError::NothingToMerge) bila branch
/// sumber belum punya commit atau sudah menunjuk commit yang sama,
/// [`UnrelatedHistories`](VergeError::UnrelatedHistories) bila kedua branch tidak
/// punya leluhur bersama, [`InvalidRef`](VergeError::InvalidRef) bila salah satu
/// commit bukan untuk tabel tersebut, serta error I/O dari port.
pub fn merge_branch(
    input: &MergeBranchInput,
    refs: &dyn RefPointer,
    commits: &dyn CommitRepository,
    store: &dyn Store,
) -> Result<MergeReport> {
    let current = refs.head_branch()?;
    let theirs = refs
        .resolve(&input.source)?
        .ok_or_else(|| VergeError::NothingToMerge(input.source.clone()))?;
    let ours = refs
        .resolve(&current)?
        .ok_or_else(|| VergeError::HeadUnborn(current.clone()))?;
    if ours == theirs {
        return Err(VergeError::NothingToMerge(input.source.clone()));
    }
    // KONTEKS: bila merge base adalah ujung branch sumber, semua commit sumber
    // sudah ada di branch aktif; KENAPA: tanpa pemeriksaan ini merge kedua akan
    // menulis commit merge sia-sia setiap kali perintah dijalankan.
    let base = find_merge_base(ours, theirs, commits)?
        .ok_or_else(|| VergeError::UnrelatedHistories(input.source.clone()))?;
    if base == theirs {
        return Err(VergeError::AlreadyMerged(input.source.clone()));
    }
    let sides = read_sides(base, ours, theirs, commits, store, &input.table)?;

    let merged = merge_rows(
        sides.base.rows(),
        sides.ours.rows(),
        sides.theirs.rows(),
        input.strategy,
        sides.ours_commit.timestamp_unix_ms() >= sides.theirs_commit.timestamp_unix_ms(),
    );
    let mut report = MergeReport {
        current: current.clone(),
        source: input.source.clone(),
        strategy: input.strategy,
        base: Some(sides.base_commit.id()),
        commit: None,
        rows: merged.rows.len(),
        conflicts: merged.conflicts,
    };
    if input.strategy == MergeStrategy::Manual && report.has_conflicts() {
        return Ok(report);
    }

    report.commit = Some(write_merge_commit(
        input,
        &current,
        &sides,
        &TableRows::from_parts(sides.ours.header().to_vec(), merged.rows),
        refs,
        commits,
        store,
    )?);
    Ok(report)
}
