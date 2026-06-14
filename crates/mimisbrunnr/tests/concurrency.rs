// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use brunnr_test_support::TempDir;
use mimisbrunnr::{
    Distance, FilesBackend, MemoryBackend, MemoryQuery, MemoryResult, MemoryScope, MemoryTier,
    SqliteVecVectorStore, StoreMemory, TextEmbedder, VectorMemoryBackend, VectorMemoryConfig,
};

#[tokio::test]
async fn files_backend_isolates_concurrent_task_scopes() {
    let tempdir = TempDir::new("files-concurrency");
    let backend = Arc::new(FilesBackend::new(tempdir.path()));
    assert_concurrent_scope_isolation(backend).await;
}

#[tokio::test]
async fn sqlite_vec_backend_isolates_concurrent_task_scopes() {
    let store = SqliteVecVectorStore::in_memory().expect("sqlite-vec should open");
    let backend = VectorMemoryBackend::with_embedder(
        store,
        VectorMemoryConfig {
            collection: "concurrency".to_string(),
            dimensions: TEST_DIMENSIONS,
            distance: Distance::Cosine,
        },
        Arc::new(TestEmbedder),
    )
    .expect("backend should construct");
    assert_concurrent_scope_isolation(Arc::new(backend)).await;
}

async fn assert_concurrent_scope_isolation(backend: Arc<dyn MemoryBackend>) {
    let mut handles = Vec::new();
    for index in 0..8 {
        let backend = Arc::clone(&backend);
        handles.push(tokio::spawn(async move {
            backend
                .store(StoreMemory {
                    content: "concurrent scoped memory".to_string(),
                    tags: Vec::new(),
                    metadata: Default::default(),
                    tier: MemoryTier::L1Atom,
                    node_id: Some(format!("node:scope-{index}")),
                    created_at: None,
                    scope: Some(MemoryScope::Task),
                    agent_id: None,
                    session_id: None,
                    task_id: Some(format!("task-{index}")),
                    user_id: None,
                })
                .await
        }));
    }
    for handle in handles {
        handle
            .await
            .expect("store task should join")
            .expect("store should succeed");
    }

    for index in 0..8 {
        let mut query = MemoryQuery::new("scoped").with_limit(10);
        query.scope = Some(MemoryScope::Task);
        query.task_id = Some(format!("task-{index}"));
        let hits = backend.find(query).await.expect("find should succeed");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].record.node_id, format!("node:scope-{index}"));
    }

    let mut handles = Vec::new();
    for _ in 0..8 {
        let backend = Arc::clone(&backend);
        handles.push(tokio::spawn(async move {
            backend
                .store(StoreMemory {
                    content: "idempotent concurrent duplicate".to_string(),
                    tags: Vec::new(),
                    metadata: Default::default(),
                    tier: MemoryTier::L1Atom,
                    node_id: Some("node:duplicate".to_string()),
                    created_at: None,
                    scope: Some(MemoryScope::Task),
                    agent_id: None,
                    session_id: None,
                    task_id: Some("task-duplicate".to_string()),
                    user_id: None,
                })
                .await
        }));
    }
    for handle in handles {
        handle
            .await
            .expect("duplicate store task should join")
            .expect("duplicate store should succeed");
    }

    let mut query = MemoryQuery::new("duplicate").with_limit(10);
    query.scope = Some(MemoryScope::Task);
    query.task_id = Some("task-duplicate".to_string());
    let hits = backend.find(query).await.expect("find should succeed");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].record.node_id, "node:duplicate");
}

const TEST_DIMENSIONS: usize = 8;

struct TestEmbedder;

impl TextEmbedder for TestEmbedder {
    fn embed_query(&self, text: &str) -> MemoryResult<Vec<f32>> {
        Ok(test_embedding(text))
    }

    fn embed_passage(&self, text: &str) -> MemoryResult<Vec<f32>> {
        Ok(test_embedding(text))
    }
}

fn test_embedding(text: &str) -> Vec<f32> {
    let mut vector = vec![0.0; TEST_DIMENSIONS];
    for token in text.split_whitespace() {
        let index = token.bytes().fold(0usize, |hash, byte| {
            hash.wrapping_mul(31).wrapping_add(byte as usize)
        }) % TEST_DIMENSIONS;
        vector[index] += 1.0;
    }
    let magnitude = vector.iter().map(|value| value * value).sum::<f32>().sqrt();
    if magnitude > 0.0 {
        for value in &mut vector {
            *value /= magnitude;
        }
    }
    vector
}
