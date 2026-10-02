//! File: mod.rs
//!
//! Deskripsi: Layer `shared` —Concerns lintas layer (kernel + exception).
//! Layer: shared
//! Tanggung jawab: Mendeklarasikan modul bersama yang boleh dipakai semua layer.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - (tidak ada dependensi eksternal)
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

pub mod exceptions;
pub mod kernel;
