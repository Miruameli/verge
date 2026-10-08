//! File: `commits_tvf.rs`
//!
//! Deskripsi: Table-valued function `commits()` — membangun baris virtual
//!   dari sejarah commit first-parent.
//! Layer: application/version-control/use-cases/queries
//! Tanggung jawab: Traversal history commit, pembatasan scan, dan
//!   konstruksi `TableRows` virtual untuk `SELECT * FROM commits()`.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `infrastructure/query/budget.rs` (`ScanBudget`)
//!   - `domain/commit/repositories/ports/commit_repository.rs`
//!   - `domain/tree/table_codec.rs`, `domain/tree/value-objects/{row_key,table_row}.rs`
//!   - `shared/exceptions/verge_error.rs`
//!   - `shared/kernel/result.rs`
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!   - #90 (M5 Part 3: resource limits)
//!   - #92 (M5 Part 2: `commits()` TVF)
//!
//! Related ADR:
//!   - ADR-0006 (CSV format tanpa quoting)
//!   - ADR-0011 (Batas 10.000 commit time-travel)
//!   - ADR-0014 (Executor resource limits)

use crate::domain::commit::repositories::ports::commit_repository::CommitRepository;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::tree::table_codec::TableRows;
use crate::domain::tree::value_objects::row_key::RowKey;
use crate::domain::tree::value_objects::table_row::TableRow;
use crate::infrastructure::query::ScanBudget;
use crate::shared::exceptions::verge_error::VergeError;
use crate::shared::kernel::result::Result;

/// Batas maksimal commit yang ditelusuri oleh `commits()`.
pub(super) const MAX_COMMIT_SCAN: usize = 10_000;

/// Header virtual untuk table-valued function `commits()`.
pub(super) const COMMIT_HEADERS: [&str; 6] = [
    "commit_id",
    "parent_ids",
    "table",
    "author",
    "timestamp",
    "message",
];

/// Memastikan `COMMIT_HEADERS` tersedia untuk planner.
pub(super) fn commit_headers() -> Vec<String> {
    COMMIT_HEADERS.iter().map(ToString::to_string).collect()
}

/// Menelusuri first-parent dari `start` dan membangun `TableRows` virtual
/// untuk table-valued function `commits()`.
///
/// Kolom: `commit_id`, `parent_ids`, `table`, `author`, `timestamp`, `message`.
/// Traversal terbatas oleh [`MAX_COMMIT_SCAN`] (ADR-0011) dan [`ScanBudget`].
///
/// # Errors
///
/// Mengembalikan error bila commit tidak dapat dimuat, atau batas sumber
/// daya (memori/waktu) terlampaui.
pub(super) fn build_commit_rows(
    start: &CommitId,
    commits: &dyn CommitRepository,
    budget: &mut ScanBudget,
) -> Result<TableRows> {
    let mut rows = Vec::new();
    let mut cursor: Option<CommitId> = Some(*start);
    let mut seen = 0;
    while let Some(id) = cursor {
        if seen >= MAX_COMMIT_SCAN {
            break;
        }
        budget.check_time()?;
        let commit = commits.load(&id)?;
        let parents = commit
            .parents()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(";");
        let value = format!(
            ",{parents},{table},{author},{ts},{msg}",
            parents = parents,
            table = commit.table().as_str(),
            author = commit.author(),
            ts = commit.timestamp_unix_ms(),
            msg = commit.summary(),
        );
        budget.track_memory(value.len())?;
        let Some(row) = TableRow::new(
            RowKey::new(commit.id().to_string().into_bytes()),
            value.into_bytes(),
        ) else {
            return Err(VergeError::NothingToCommit("commit_id kosong".to_string()));
        };
        rows.push(row);
        seen += 1;
        cursor = commit.parents().first().copied();
    }
    rows.sort_by(|a, b| b.key().cmp(a.key()));
    Ok(TableRows::from_parts(
        COMMIT_HEADERS.join(",").into_bytes(),
        rows,
    ))
}
