//! File: `staged_table.rs`
//!
//! Deskripsi: DTO hasil penyimpanan data kerja tabel.
//! Layer: application/version-control/dtos
//! Tanggung jawab: Membawa identitas blok data kerja kepada pemanggil.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `domain/table/value-objects/table_name.rs`
//!   - `domain/storage/value-objects/block_id.rs`
//!
//! Related issues:
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0005 (Tabel sebagai blok content-addressed)

use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::table::value_objects::table_name::TableName;

/// Data kerja tabel yang sudah tersimpan sebagai blok immutable.
///
/// Immutability: penuh; nilai ini hanya dibaca oleh lapisan antarmuka.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagedTable {
    /// Tabel yang datanya disimpan.
    pub table: TableName,
    /// Blok yang memuat isi tabel.
    pub block: BlockId,
    /// Jumlah byte isi tabel.
    pub bytes: usize,
}
