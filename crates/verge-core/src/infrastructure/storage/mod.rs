//! File: mod.rs
//!
//! Deskripsi: Adapter storage di filesystem lokal.
//! Layer: infrastructure/storage
//! Tanggung jawab: Mendeklarasikan implementasi backend filesystem.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - infrastructure/storage/file-system/mod.rs
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

#[path = "file-system/mod.rs"]
pub mod file_system;
