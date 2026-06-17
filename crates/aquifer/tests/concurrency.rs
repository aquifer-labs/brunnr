// SPDX-License-Identifier: Apache-2.0

use std::{sync::Arc, time::Duration};

use aquifer::{
    FilesBackend, MemoryBackend, MemoryQuery, MemoryResult, MemoryScope, MemoryTier,
    SessionLaneLock, SqliteVecVectorStore, StoreMemory, TextEmbedder, VectorMemoryBackend,
    VectorMemoryConfig,
};
use artesian_test_support::TempDir;

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
            ..VectorMemoryConfig::new("concurrency")
        },
        Arc::new(TestEmbedder),
    )
    .expect("backend should construct");
    assert_concurrent_scope_isolation(Arc::new(backend)).await;
}

#[tokio::test]
async fn session_lane_lock_serializes_and_times_out() {
    let tempdir = TempDir::new("lane-lock-timeout");
    let lock = SessionLaneLock::new(tempdir.path()).with_timeout(Duration::from_millis(50));
    let guard = lock
        .acquire("shared-collection", Some("session-a"))
        .await
        .expect("first lane acquire should succeed");

    let blocked = lock.acquire("shared-collection", Some("session-a")).await;
    assert!(blocked.is_err());
    assert!(blocked
        .expect_err("lane should time out")
        .to_string()
        .contains("timed out acquiring session lane lock"));

    guard.release().expect("release should succeed");
    lock.acquire("shared-collection", Some("session-a"))
        .await
        .expect("lane should reacquire after release");
}

#[tokio::test]
async fn sqlite_vec_multi_writer_integrity_and_tenant_isolation() {
    let store = SqliteVecVectorStore::in_memory().expect("sqlite-vec should open");
    let backend = Arc::new(
        VectorMemoryBackend::with_embedder(
            store,
            VectorMemoryConfig {
                collection: "shared-project".to_string(),
                dimensions: TEST_DIMENSIONS,
                ..VectorMemoryConfig::new("shared-project")
            },
            Arc::new(TestEmbedder),
        )
        .expect("backend should construct"),
    );

    let writer_count = 24;
    let mut handles = Vec::new();
    for index in 0..writer_count {
        let backend = Arc::clone(&backend);
        handles.push(tokio::spawn(async move {
            backend
                .store(StoreMemory {
                    content: format!("contention memory tenant word {index}"),
                    tags: Vec::new(),
                    metadata: Default::default(),
                    tier: MemoryTier::L1Atom,
                    node_id: Some(format!("node:tenant-{index}")),
                    created_at: None,
                    scope: Some(MemoryScope::Session),
                    agent_id: Some(format!("agent-{}", index % 3)),
                    session_id: Some(format!("session-{}", index % 4)),
                    task_id: Some(format!("task-{index}")),
                    user_id: Some(format!("user-{}", index % 2)),
                })
                .await
        }));
    }
    for handle in handles {
        handle
            .await
            .expect("writer should join")
            .expect("writer should store");
    }

    let mut query = MemoryQuery::new("contention memory tenant").with_limit(writer_count);
    query.scope = Some(MemoryScope::Session);
    query.user_id = Some("user-0".to_string());
    let user_zero = backend.find(query).await.expect("find should succeed");
    assert_eq!(user_zero.len(), writer_count / 2);
    assert!(user_zero
        .iter()
        .all(|hit| hit.record.user_id.as_deref() == Some("user-0")));

    let mut query = MemoryQuery::new("contention memory tenant").with_limit(writer_count);
    query.scope = Some(MemoryScope::Session);
    query.user_id = Some("user-1".to_string());
    let user_one = backend.find(query).await.expect("find should succeed");
    assert_eq!(user_one.len(), writer_count / 2);
    assert!(user_one
        .iter()
        .all(|hit| hit.record.user_id.as_deref() == Some("user-1")));
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
