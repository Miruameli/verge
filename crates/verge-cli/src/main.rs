//! File: main.rs
//!
//! Deskripsi: Entry point binary `verge`.
//! Layer: interfaces/cli (entry point)
//! Tanggung jawab: Menjalankan dispatcher dan memetakan error ke exit code.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `interfaces/cli/cli_dispatcher.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)
//!
//! Catatan: berkas ini adalah root binary yang diminta Cargo sehingga tetap
//! berada di `src/`; seluruh logika perintah ada di `interfaces/cli/`.

use std::process::ExitCode;

#[path = "interfaces/mod.rs"]
mod interfaces;

#[path = "shared/mod.rs"]
mod shared;

/// Menjalankan CLI dan mengembalikan exit code.
///
/// Returns:
/// - ExitCode — `SUCCESS` bila perintah selesai, `FAILURE` bila ada error yang
///   sudah dicetak ke stderr.

#[path = "config/mod.rs"]
mod config;
fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match interfaces::cli::cli_dispatcher::dispatch(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("verge: {error:#}");
            ExitCode::FAILURE
        }
    }
}
