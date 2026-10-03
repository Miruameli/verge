//! File: `merge_report_printer.rs`
//!
//! Deskripsi: Pencetak ringkasan dan konflik hasil `verge merge`.
//! Layer: interfaces/cli/commands/merging
//! Tanggung jawab: Mengubah `MergeReport` menjadi output yang dapat dibaca mesin.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `dtos/merging/merge_report.rs`
//!   - `domain/merge/rows/row_conflict.rs`
//!   - `domain/ident/value_objects/digest_text.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use verge_core::application::version_control::dtos::merging::merge_report::MergeReport;
use verge_core::domain::ident::value_objects::digest_text::HexText;
use verge_core::domain::merge::rows::row_conflict::RowConflict;

/// Jumlah awalan hex yang dicetak pada merge dan konflik.
const ID_PREFIX_LEN: usize = 12;

/// Mencetak ringkasan merge; konflik ditulis ke stderr karena hasil merge
/// tidak boleh dibaca berhasil dari stdout saja.
pub(super) fn print_report(report: &MergeReport) {
    match report.commit {
        Some(commit) => println!(
            "merged {} into {} at {} ({} rows, strategy {})",
            report.source,
            report.current,
            short(&commit.to_hex()),
            report.rows,
            report.strategy.label()
        ),
        None => eprintln!(
            "merge cancelled: {} conflict(s) on strategy {}",
            report.conflicts.len(),
            report.strategy.label()
        ),
    }
    for conflict in &report.conflicts {
        print_conflict(conflict);
    }
}

/// Mencetak satu konflik sebagai tiga sisi nilai yang dapat dibandingkan.
fn print_conflict(conflict: &RowConflict) {
    eprintln!(
        "  {} base={} ours={} theirs={}",
        conflict.key,
        text(conflict.base_or_empty()),
        text(conflict.ours_or_empty()),
        text(conflict.theirs_or_empty())
    );
}

/// Memotong hex ke awalan yang cukup untuk dicetak.
///
/// KENAPA: 48 bit membuat id tetap unik untuk repo berskala nyata, sehingga
/// id yang dicetak masih bisa dicari lewat `verge log` tanpa memenuhi baris
/// ringkasan dengan hex penuh.
fn short(hex: &str) -> String {
    hex.chars().take(ID_PREFIX_LEN).collect()
}

/// Mengubah byte nilai baris menjadi teks yang aman dicetak.
///
/// KENAPA: nilai baris disimpan sebagai byte mentah, dan `from_utf8_lossy`
/// membuat satu baris rusak tetap tercetak sebagai pengganti daripada membuat
/// seluruh merge gagal tanpa memberi tahu baris mana yang bermasalah.
fn text(value: &[u8]) -> String {
    String::from_utf8_lossy(value).into_owned()
}
