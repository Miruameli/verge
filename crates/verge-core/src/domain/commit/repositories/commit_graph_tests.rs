//! File: `commit_graph_tests.rs`
//!
//! Deskripsi: Test graph commit: insert, branch, tag, ancestry, dan log.
//! Layer: domain/commit/repositories
//! Tanggung jawab: Membuktikan invariant graph dan perilaku pointer.
//!
//! Author: Miruameli
//! Created: 2026-10-03
//! Modified: 2026-10-03
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `commit_graph.rs`, `commit_history.rs`
//!
//! Related issues:
//!   - #1 (Milestone 1)
//!
//! Related ADR:
//!   - ADR-0002 (Storage immutable content-addressed)

use super::commit_graph::CommitGraph;
use crate::domain::commit::entities::commit::Commit;
use crate::domain::commit::value_objects::commit_id::CommitId;
use crate::domain::commit::value_objects::commit_ref::Ref;
use crate::domain::ident::value_objects::digest::Digest;
use crate::shared::exceptions::verge_error::VergeError;

/// Membuat commit dengan tree tetap agar test fokus pada graph.
fn commit(message: &str, parents: Vec<CommitId>) -> Commit {
    Commit::new(parents, Digest::of(b"tree"), "author", message, 0)
}

/// Graph lurus dengan satu root dan satu anak.
fn linear_graph() -> (CommitGraph, CommitId, CommitId) {
    let mut graph = CommitGraph::new();
    let root = commit("root", vec![]);
    let root_id = root.id();
    graph.insert(root).expect("root masuk");
    let child = commit("child", vec![root_id]);
    let child_id = child.id();
    graph.insert(child).expect("child masuk");
    (graph, root_id, child_id)
}

#[test]
fn commit_dengan_parent_tak_dikenal_ditolak() {
    let mut graph = CommitGraph::new();
    let orphan = commit("orphan", vec![Digest::of(b"tidak-ada")]);
    assert!(matches!(
        graph.insert(orphan),
        Err(VergeError::MissingParent(_))
    ));
    assert!(
        graph.is_empty(),
        "commit yang ditolak tidak boleh terindeks"
    );
}

#[test]
fn ancestry_mengikuti_parent() {
    let (graph, root_id, child_id) = linear_graph();
    assert!(graph.is_ancestor(root_id, child_id));
    assert!(!graph.is_ancestor(child_id, root_id));
    assert!(!graph.is_ancestor(root_id, root_id), "ancestry itu ketat");
    assert!(graph.is_ancestor_or_self(root_id, root_id));
}

#[test]
fn ancestry_berhenti_pada_diamond() {
    let mut graph = CommitGraph::new();
    let root = commit("root", vec![]);
    let root_id = root.id();
    graph.insert(root).expect("root masuk");
    let left = commit("left", vec![root_id]);
    let right = commit("right", vec![root_id]);
    let merge = commit("merge", vec![left.id(), right.id()]);
    let (left_id, right_id, merge_id) = (left.id(), right.id(), merge.id());
    graph.insert(left).expect("left masuk");
    graph.insert(right).expect("right masuk");
    graph.insert(merge).expect("merge masuk");
    assert!(graph.is_ancestor(left_id, merge_id));
    assert!(graph.is_ancestor(right_id, merge_id));
    assert!(!graph.is_ancestor(merge_id, root_id));
}

#[test]
fn tag_tidak_bisa_dibuat_dua_kali() {
    let (mut graph, root_id, child_id) = linear_graph();
    graph.set_tag("v1", root_id).expect("tag dibuat");
    assert!(matches!(
        graph.set_tag("v1", child_id),
        Err(VergeError::TagAlreadyExists(_))
    ));
    assert_eq!(graph.resolve(&Ref::Tag("v1".to_owned())), Some(root_id));
}

#[test]
fn branch_berpindah_tanpa_menyalin_data() {
    let (mut graph, root_id, child_id) = linear_graph();
    graph.set_branch("main", root_id).expect("branch dibuat");
    graph
        .set_branch("main", child_id)
        .expect("branch berpindah");
    assert_eq!(
        graph.resolve(&Ref::Branch("main".to_owned())),
        Some(child_id)
    );
    assert_eq!(graph.branch_names().collect::<Vec<_>>(), vec!["main"]);
    assert_eq!(graph.len(), 2, "perpindahan branch tidak menambah commit");
}

#[test]
fn branch_ke_commit_tak_dikenal_ditolak() {
    let (mut graph, _, _) = linear_graph();
    assert!(matches!(
        graph.set_branch("main", Digest::of(b"tidak-ada")),
        Err(VergeError::CommitNotFound(_))
    ));
}

#[test]
fn first_parent_log_terbaru_dulu_dan_terbatas() {
    let mut graph = CommitGraph::new();
    let root = commit("c0", vec![]);
    let mut tip = root.id();
    graph.insert(root).expect("c0 masuk");
    for index in 1..4 {
        let next = commit(&format!("c{index}"), vec![tip]);
        tip = next.id();
        graph.insert(next).expect("child masuk");
    }
    let log = graph.first_parent_log(tip, 2);
    assert_eq!(log.len(), 2);
    assert_eq!(log[0].summary(), "c3");
    assert_eq!(log[1].summary(), "c2");
    assert_eq!(graph.len(), 4);
}
