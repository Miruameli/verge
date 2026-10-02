//! File: `storage_layout.rs`
//!
//! Deskripsi: Konstanta layout on-disk untuk blok content-addressed.
//! Layer: config
//! Tanggung jawab: Menetapkan fan-out direktori dan cara menyusun path blok.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - domain/ident/value-objects/digest_text.rs
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use std::path::{Path, PathBuf};

use crate::domain::ident::value_objects::digest_text::HexText;
use crate::domain::storage::value_objects::block_id::BlockId;

/// Jumlah karakter hex per tingkat fan-out.
const FANOUT: usize = 2;

/// Aturan penempatan blok di disk.
pub struct StorageLayout;

impl StorageLayout {
    /// Menyusun path blok di bawah `root`.
    ///
    /// Args:
    /// - root — direktori objek.
    /// - id — identifier blok.
    ///
    /// Returns:
    /// - `PathBuf` — `root/<ab>/<cd>/<hex>`.
    ///
    /// Example:
    /// ```
    /// use verge_core::domain::storage::value_objects::block_id::BlockId;
    /// use verge_core::config::storage_layout::StorageLayout;
    ///
    /// let id = BlockId::of(b"abc");
    /// let hex = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
    /// let path = StorageLayout::block_path(std::path::Path::new("/repo/objects"), &id);
    /// assert!(path.ends_with(format!("ba/78/{hex}")));
    /// ```
    #[must_use]
    pub fn block_path(root: &Path, id: &BlockId) -> PathBuf {
        let hex = id.to_hex();
        root.join(&hex[..FANOUT])
            .join(&hex[FANOUT..2 * FANOUT])
            .join(hex)
    }
}
