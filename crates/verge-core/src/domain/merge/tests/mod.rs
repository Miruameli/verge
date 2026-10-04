//! File: `mod.rs`
//!
//! Deskripsi: Titik masuk test aturan merge.
//! Layer: domain/merge
//! Tanggung jawab: Mendaftarkan test resolusi baris merge. Test merge base
//!   yang memakai adapter penyimpanan berada di `tests/merge/` karena domain
//!   tidak boleh bergantung pada infrastruktur.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-04
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

mod rows;
