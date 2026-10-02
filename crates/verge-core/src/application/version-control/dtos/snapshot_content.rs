//! File: `snapshot_content.rs`
//!
//! Deskripsi: DTO isi tabel pada satu commit.
//! Layer: application/version-control/dtos
//! Tanggung jawab: Mengembalikan byte tabel tepat seperti saat commit dibuat.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/commit/value-objects/commit_id.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use crate::domain::commit::value_objects::commit_id::CommitId;

/// Isi tabel pada commit tertentu, apa adanya tanpa normalisasi.
///
/// Immutability: penuh; byte adalah salinan dari blok yang tersimpan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotContent {
    /// Commit yang dibaca datanya.
    pub commit: CommitId,
    /// Isi tabel pada commit tersebut.
    pub bytes: Vec<u8>,
}
