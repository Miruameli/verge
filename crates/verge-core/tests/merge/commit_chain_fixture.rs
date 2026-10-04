//! File: `commit_chain_fixture.rs`
//!
//! Deskripsi: Pembuat rantai commit untuk test merge base.
//! Layer: tests (integration)
//! Tanggung jawab: Menyimpan rantai commit linear di repository sementara.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-04
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `infrastructure/commit/file-system/file_commit_repository.rs`
//!
//! Related issues:
//!   - #22 (Milestone 4)
//!
//! Related ADR:
//!   - ADR-0007 (Branch sebagai pointer dan merge tiga arah)

use std::path::Path;

use verge_core::config::repository_layout::RepositoryLayout;
use verge_core::domain::commit::entities::commit::Commit;
use verge_core::domain::commit::repositories::ports::commit_repository::CommitRepository;
use verge_core::domain::commit::value_objects::commit_id::CommitId;
use verge_core::domain::ident::value_objects::digest::Digest;
use verge_core::domain::table::value_objects::table_name::TableName;
use verge_core::infrastructure::commit::file_system::file_commit_repository::FileCommitRepository;
use verge_core::infrastructure::storage::file_system::file_block_store::FileBlockStore;

/// Table yang dipakai seluruh rantai commit pada fixture.
const TABLE: &str = "users";

/// Menyimpan rantai commit linear lalu mengembalikan commit terakhirnya.
///
/// Args:
/// - dir — direktori sementara repository.
/// - labels — penanda tiap commit; dipakai sebagai pesan sehingga digest tiap
///   commit berbeda meski tabelnya sama.
///
/// Returns:
/// - Vec<CommitId> — seluruh commit dalam rantai, dari akar ke ujung.
///
/// # Panics
///
/// Test fixture boleh panik bila storage sementara tidak dapat disiapkan; test
/// yang gagal karena lingkungan, bukan karena logika merge.
pub fn commit_chain(dir: &Path, labels: &[&str]) -> Vec<CommitId> {
    let layout = RepositoryLayout::under(dir);
    std::fs::create_dir_all(layout.heads()).expect("buat direktori heads");
    let store = FileBlockStore::open(layout.objects()).expect("buka store");
    let commits = FileCommitRepository::new(store);
    let table = TableName::parse(TABLE).expect("nama tabel fixture");
    let mut tip: Option<CommitId> = None;
    let mut chain = Vec::with_capacity(labels.len());

    for (position, label) in labels.iter().enumerate() {
        let parents = tip.map(|parent| vec![parent]).unwrap_or_default();
        let commit = Commit::new(
            parents,
            Digest::of(label.as_bytes()),
            &table,
            "ana",
            format!("feat: {label}"),
            i64::try_from(position).expect("posisi muat dalam i64"),
        );
        commits.save(&commit).expect("simpan commit");
        tip = Some(commit.id());
        chain.push(commit.id());
    }

    assert!(!chain.is_empty(), "rantai commit tidak boleh kosong");
    chain
}
