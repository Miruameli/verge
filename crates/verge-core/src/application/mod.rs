//! File: mod.rs
//!
//! Deskripsi: Layer `application` — use case yang mengorkestrasi port.
//! Layer: application
//! Tanggung jawab: Mendeklarasikan use case engine tanpa detail I/O.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - application/repository-bootstrap/mod.rs
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

#[path = "repository-bootstrap/mod.rs"]
pub mod repository_bootstrap;

#[path = "version-control/mod.rs"]
pub mod version_control;
