//! File: `budget_tests.rs`
//!
//! Deskripsi: Unit test untuk `ScanBudget` — memory limit, timeout, unlimited.
//! Layer: infrastructure/query/tests
//! Tanggung jawab: Membuktikan resource limits berfungsi dengan benar.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Related issues:
//!     - #90 (M5 Part 3: executor resource limits)

//! Related ADR:
//!     - ADR-0014 (Executor resource limits)

use std::time::Duration;

use crate::infrastructure::query::budget::ScanBudget;
use crate::shared::exceptions::verge_error::VergeError;

#[test]
fn memory_limit_terlampaui_mengembalikan_error() {
    let mut budget = ScanBudget::with_limits(10, Duration::from_secs(10));
    assert!(budget.track_memory(5).is_ok());
    let err = budget.track_memory(10).unwrap_err();
    assert!(matches!(
        err,
        VergeError::QueryResourceLimit {
            limit_type: "memory",
            ..
        }
    ));
}

#[test]
fn timeout_terlampaui_mengembalikan_error() {
    let budget = ScanBudget::with_limits(1024, Duration::from_nanos(1));
    let err = budget.check_time().unwrap_err();
    assert!(matches!(
        err,
        VergeError::QueryResourceLimit {
            limit_type: "time",
            ..
        }
    ));
}

#[test]
fn unlimited_tidak_pernah_error() {
    let mut budget = ScanBudget::unlimited();
    assert!(budget.track_memory(1_000_000).is_ok());
    assert!(budget.check_time().is_ok());
}
