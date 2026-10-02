//! File: mod.rs
//!
//! Deskripsi: Layer `infrastructure` — implementasi port di dunia nyata.
//! Layer: infrastructure
//! Tanggung jawab: Menyediakan implementasi konkret untuk port domain.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - infrastructure/commit/mod.rs
//!   - infrastructure/table/mod.rs
//!   - infrastructure/system/mod.rs
//!   - infrastructure/storage/mod.rs
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!   - #8 (Milestone 2)
//!
//! Related ADR:
//!   - ADR-0003 (Arsitektur 7-layer)

#[path = "commit/mod.rs"]
pub mod commit;
#[path = "storage/mod.rs"]
pub mod storage;
#[path = "system/mod.rs"]
pub mod system;
#[path = "table/mod.rs"]
pub mod table;
