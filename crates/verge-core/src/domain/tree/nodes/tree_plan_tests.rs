//! File: `tree_plan_tests.rs`
//!
//! Deskripsi: Test partisi baris menjadi node tree.
//! Layer: domain/tree/nodes
//! Tanggung jawab: Membuktikan daun berbagi blok dan tidak ada baris hilang.
//!
//! Author: Miruameli · Created: 2026-10-03 · Modified: 2026-10-03
//! Version: 0.1.0 · License: Apache-2.0
//!
//! Dependencies: `tree_node.rs`, `tree_node_codec.rs`, `tree_builder.rs`
//! Related issues: #18 (Milestone 3)
//! Related ADR: ADR-0006 (Prolly tree untuk tabel)

use crate::domain::ident::value_objects::digest::Digest;
use crate::domain::tree::nodes::tree_node::LEAF_SIZE_LIMIT;
use crate::domain::tree::nodes::tree_node_codec::encode;
use crate::domain::tree::table_codec::TableRows;
use crate::domain::tree::tree_builder::build_plan;
use crate::domain::tree::value_objects::row_key::RowKey;
use crate::domain::tree::value_objects::table_row::TableRow;

/// Membuat baris dengan kunci dan nilai tertentu.
fn row(key: &str, value: &str) -> TableRow {
    TableRow::new(
        RowKey::new(key.as_bytes().to_vec()),
        value.as_bytes().to_vec(),
    )
    .expect("kunci tidak kosong")
}

/// Membungkus baris menjadi tabel uji dengan header tetap.
fn table_of(rows: &[TableRow]) -> TableRows {
    TableRows::from_parts(b"id,name".to_vec(), rows.to_vec())
}

/// Menghasilkan baris sebanyak `count` dengan nilai sepanjang `value_len`.
fn many_rows(count: usize, value_len: usize) -> Vec<TableRow> {
    let filler = "x".repeat(value_len);
    (0..count)
        .map(|index| row(&format!("{index:06}"), &format!(",{filler}")))
        .collect()
}

#[test]
fn baris_melebihi_batas_membentuk_lebih_dari_satu_daun() {
    let rows = many_rows(400, LEAF_SIZE_LIMIT / 4);
    let plan = build_plan(&table_of(&rows)).expect("plan berhasil");

    assert!(
        plan.leaf_ids.len() > 1,
        "baris besar harus dipisah antar daun"
    );
    assert_eq!(
        plan.nodes.len(),
        plan.leaf_ids.len() + 2,
        "satu header dan satu akar di atas daun"
    );
}

#[test]
fn baris_kecil_mujur_berada_dalam_satu_daun() {
    let rows = many_rows(3, 8);
    let plan = build_plan(&table_of(&rows)).expect("plan berhasil");

    // KONTEKS: node header ikut menjadi anak akar, sehingga tree dengan satu
    // daun tetap punya satu node internal yang menunjuk header dan daun itu.
    assert_eq!(plan.leaf_ids.len(), 1);
    assert_eq!(plan.nodes.len(), 3, "header, satu daun, dan satu akar");
    assert_ne!(
        plan.root, plan.leaf_ids[0],
        "akar bukan daun karena memuat header"
    );
}

#[test]
fn partisi_yang_sama_menghasilkan_identifier_yang_sama() {
    let rows = many_rows(50, 200);
    let first = build_plan(&table_of(&rows)).expect("plan pertama");
    let second = build_plan(&table_of(&rows)).expect("plan kedua");

    assert_eq!(first.root, second.root);
    assert_eq!(first.leaf_ids, second.leaf_ids);
}

#[test]
fn semua_baris_tercakup_pada_daun_yang_dihasilkan() {
    let rows = many_rows(120, 100);
    let plan = build_plan(&table_of(&rows)).expect("plan berhasil");

    let mut collected = 0_usize;
    for node in plan.nodes.iter().filter(|node| node.is_leaf()) {
        collected += node.row_count();
    }
    assert_eq!(
        collected,
        rows.len(),
        "tidak ada baris yang hilang saat partisi"
    );
}

#[test]
fn identifier_daun_sesuai_dengan_isinya() {
    let rows = many_rows(120, 100);
    let plan = build_plan(&table_of(&rows)).expect("plan berhasil");

    for node in plan.nodes.iter().filter(|node| node.is_leaf()) {
        assert!(
            plan.leaf_ids.contains(&Digest::of(&encode(node))),
            "setiap daun harus punya identifier yang dihitung dari isinya"
        );
    }
}

#[test]
fn tabel_tanpa_baris_ditolak_saat_pembangunan_tree() {
    assert!(
        build_plan(&TableRows::default()).is_err(),
        "tabel kosong tidak punya akar tree"
    );
}

#[test]
fn daun_yang_tidak_berubah_berbagi_blok_antar_dua_tabel() {
    let base = many_rows(200, 100);
    let first = build_plan(&table_of(&base)).expect("plan pertama");
    let mut extended = base.clone();
    extended.push(row("999999", ",baru"));
    let second = build_plan(&table_of(&extended)).expect("plan kedua");

    let shared = first
        .leaf_ids
        .iter()
        .filter(|id| second.leaf_ids.contains(id))
        .count();
    assert!(
        shared > 0,
        "daun yang tidak berubah harus dipakai ulang, bukan ditulis ulang"
    );
}
