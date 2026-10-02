//! File: `staged_table.rs`
//!
//! Deskripsi: DTO hasil penyimpanan data kerja tabel.
//! Layer: application/version-control/dtos
//! Tanggung jawab: Membawa akar tree data kerja kepada pemanggil.
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
//!   - #18 (Milestone 3)
//!
//! Related ADR:
//!   - ADR-0006 (Prolly tree untuk tabel)

use crate::domain::storage::value_objects::block_id::BlockId;
use crate::domain::table::value_objects::table_name::TableName;

/// Data kerja tabel yang sudah tersimpan sebagai tree prolly.
///
/// KENAPA: yang dibawa adalah akar tree, bukan isi tabel. Isi tabel tersebar
/// pada beberapa blok sehingga satu digest tidak lagi dapat mewakili tabel;
/// pointer hanya menunjuk akar, sedangkan jumlah byte dan baris tetap
/// dilaporkan sebagai ringkasan.
///
/// Immutability: penuh; nilai ini hanya dibaca oleh lapisan antarmuka.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagedTable {
    /// Tabel yang datanya disimpan.
    pub table: TableName,
    /// Node akar tree tabel yang menjadi data kerja.
    pub root: BlockId,
    /// Total byte seluruh node tree yang ditulis.
    pub bytes: usize,
    /// Jumlah baris data tabel.
    pub rows: usize,
}
