//! File: lib.rs
//!
//! Deskripsi: Crate root `verge-core` — mesin database versioned.
//! Layer: crate root (peta layer)
//! Tanggung jawab: Mendaftarkan seluruh layer dan mengekspor API publik.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - sha2 (perhitungan SHA-256)
//!
//! Related issues:
//!   - #1 (Milestone 1: content-addressed block store + commit graph)
//!
//! Related ADR:
//!   - ADR-0001 (Rust sebagai core, toolchain=bawaan)
//!   - ADR-0002 (Storage immutable content-addressed)
//!   - ADR-0003 (Arsitektur 7-layer)
//!
//! Peta layer (lihat docs/architecture.md):
//! - `domain/`        — entitas, value object, port (tanpa I/O)
//! - `application/`   — use case yang mengorkestrasi port
//! - `infrastructure/`— implementasi port di filesystem lokal
//! - `shared/`        — kernel dan exception lintas layer
//! - `config/`        — konstanta layout repository dan storage
//!
//! Layer `presentation/` dan `interfaces/` belum berisi kode: keduanya akan
//! lahir bersama perintah CLI/server di milestone berikutnya, dan tidak ada
//! file placeholder di repository ini.

#[path = "application/mod.rs"]
pub mod application;
#[path = "config/mod.rs"]
pub mod config;
#[path = "domain/mod.rs"]
pub mod domain;
#[path = "infrastructure/mod.rs"]
pub mod infrastructure;
#[path = "shared/mod.rs"]
pub mod shared;

pub use domain::commit::entities::commit::Commit;
pub use domain::commit::repositories::commit_graph::CommitGraph;
pub use domain::commit::repositories::ports::commit_repository::CommitRepository;
pub use domain::commit::repositories::ports::ref_pointer::RefPointer;
pub use domain::commit::value_objects::commit_id::CommitId;
pub use domain::commit::value_objects::commit_ref::Ref;
pub use domain::ident::value_objects::digest::Digest;
pub use domain::storage::ports::block_store::Store;
pub use domain::storage::ports::block_store_factory::BlockStoreFactory;
pub use domain::storage::ports::metadata_writer::MetadataWriter;
pub use domain::storage::value_objects::block_id::BlockId;
pub use domain::storage::value_objects::put_outcome::PutOutcome;
pub use domain::table::ports::table_workspace::TableWorkspace;
pub use domain::table::value_objects::table_name::TableName;
pub use infrastructure::storage::file_system::file_block_store::FileBlockStore;
pub use infrastructure::storage::file_system::file_store_factory::FileStoreFactory;
pub use infrastructure::storage::file_system::local_file_system::LocalFileSystem;
pub use shared::exceptions::verge_error::VergeError;
pub use shared::kernel::result::Result;
