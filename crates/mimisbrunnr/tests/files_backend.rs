// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::path::PathBuf;

use brunnr_test_support::TempDir;
use mimisbrunnr::{FilesBackend, MemoryBackend, MemoryQuery, MemoryTier, SearchHit, StoreMemory};
use tokio::fs;

#[tokio::test]
async fn files_backend_stores_okf_markdown_and_finds_it() {
    let tempdir = TempDir::new("files-store");
    let backend = FilesBackend::new(tempdir.path());

    let stored = backend
        .store(StoreMemory {
            content: "Files backend keeps memory readable".to_string(),
            tags: vec!["files".to_string()],
            metadata: BTreeMap::new(),
            tier: MemoryTier::L1Atom,
            node_id: Some("node:files".to_string()),
            created_at: None,
            scope: None,
            agent_id: None,
            session_id: None,
            task_id: None,
            user_id: None,
        })
        .await
        .expect("store should succeed");

    let date_tag = stored.created_at.format("%Y-%m-%d").to_string();
    let memory_dir = tempdir.join(["memory", &date_tag].iter().collect::<std::path::PathBuf>());
    let path = std::fs::read_dir(memory_dir)
        .expect("memory date dir should exist")
        .map(|entry| entry.expect("record entry should be readable").path())
        .find(|path| {
            path.file_stem()
                .and_then(|stem| stem.to_str())
                .is_some_and(|stem| stem == stored.id.as_str())
        })
        .expect("record file should exist");
    let rendered = fs::read_to_string(path)
        .await
        .expect("record should be readable");
    let hits = backend
        .find(MemoryQuery::new("readable"))
        .await
        .expect("find should succeed");

    assert!(rendered.contains("type: memory"));
    assert!(rendered.contains("Files backend keeps memory readable"));
    assert_eq!(hits, vec![SearchHit::keyword(stored, 1.0)]);
}

#[tokio::test]
async fn files_backend_drills_down_by_node_id() {
    let tempdir = TempDir::new("files-node");
    let backend = FilesBackend::new(tempdir.path());
    let stored = backend
        .store(StoreMemory {
            content: "Ground truth evidence".to_string(),
            tags: Vec::new(),
            metadata: BTreeMap::new(),
            tier: MemoryTier::L0Raw,
            node_id: Some("node:evidence".to_string()),
            created_at: None,
            scope: None,
            agent_id: None,
            session_id: None,
            task_id: None,
            user_id: None,
        })
        .await
        .expect("store should succeed");

    assert_eq!(
        backend
            .get_node("node:evidence")
            .await
            .expect("get_node should succeed"),
        Some(stored)
    );
}

#[tokio::test]
async fn files_backend_reads_okf_bundle_fixture() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("examples")
        .join("okf-bundle");
    let backend = FilesBackend::new(fixture);

    let hits = backend
        .find(MemoryQuery::new("reciprocal rank fusion").with_limit(5))
        .await
        .expect("OKF bundle should be searchable");
    let record = backend
        .get_node("node:rrf")
        .await
        .expect("OKF node drill-down should succeed");

    assert!(
        hits.iter().any(|hit| hit.record.node_id == "node:rrf"),
        "RRF OKF record should be found, got {hits:?}"
    );
    assert_eq!(
        record.expect("node:rrf should exist").metadata["okf_type"],
        "reference"
    );
}
