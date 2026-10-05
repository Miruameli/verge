//! File: `time_travel_cli.rs`
//!
//! Deskripsi: Test end-to-end time-travel dan tag.
//! Layer: interfaces/cli/tests
//! Tanggung jawab: Mendaftarkan modul test time-travel sebagai satu target.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Related issues:
//!   - #25 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0008 (Time-travel AS OF dan tag immutable)

// KENAPA: folder mengikuti kebab-case seperti `merging/` dan `versioning/`,
//         nama modul tetap snake_case agar `#[path]` tidak perlu ditulis ulang.
#[path = "time-travel/mod.rs"]
mod time_travel;
