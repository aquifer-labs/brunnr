// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use brunnr_mcp::{AnchorSetRequest, FindRequest, MemoryServer, StoreRequest, ToolsFindRequest};
use brunnr_test_support::TempDir;
use mimisbrunnr::{
    Distance, MemoryResult, SqliteVecVectorStore, TextEmbedder, VectorMemoryBackend,
    VectorMemoryConfig,
};
use rmcp::handler::server::wrapper::Parameters;

#[tokio::test]
async fn memory_tools_store_and_find_with_files_backend() {
    let tempdir = TempDir::new("mcp");
    let server = MemoryServer::new(tempdir.path());

    let stored = server
        .memory_store(Parameters(StoreRequest {
            content: "MCP memory tool round trip".to_string(),
            tags: Some(vec!["mcp".to_string()]),
            node_id: Some("node:mcp".to_string()),
            scope: None,
            agent_id: None,
            session_id: None,
            task_id: None,
            user_id: None,
        }))
        .await
        .expect("store should succeed")
        .0;

    let found = server
        .memory_find(Parameters(FindRequest {
            query: "round".to_string(),
            limit: Some(5),
            node_id: Some("node:mcp".to_string()),
            scope: None,
            agent_id: None,
            session_id: None,
            task_id: None,
            user_id: None,
        }))
        .await
        .expect("find should succeed")
        .0;

    assert_eq!(stored.node_id, "node:mcp");
    assert_eq!(found.hits.len(), 1);
    assert_eq!(found.hits[0].node_id, "node:mcp");
    assert_eq!(found.hits[0].content, "MCP memory tool round trip");
}

#[tokio::test]
async fn memory_anchor_tools_round_trip_with_files_backend() {
    let tempdir = TempDir::new("mcp-anchor");
    let server = MemoryServer::new(tempdir.path());

    server
        .memory_anchor_set(Parameters(AnchorSetRequest {
            current_task: "implement anchor tools".to_string(),
            plan_pointer: Some("docs/self-repair.md#muninn".to_string()),
            last_decisions: Some(vec!["append-only log".to_string()]),
            next_step: "verify MCP round trip".to_string(),
        }))
        .await
        .expect("anchor set should succeed");
    let response = server
        .memory_anchor_get()
        .await
        .expect("anchor get should succeed")
        .0;

    let anchor = response.anchor.expect("anchor should exist");
    assert_eq!(anchor.current_task, "implement anchor tools");
    assert_eq!(anchor.next_step, "verify MCP round trip");
    assert_eq!(anchor.last_decisions, vec!["append-only log"]);
}

#[tokio::test]
async fn tools_find_is_opt_in_and_reports_token_delta() {
    let tempdir = TempDir::new("mcp-tools-find");
    let disabled = MemoryServer::new(tempdir.path());
    assert!(
        disabled
            .tools_find(Parameters(ToolsFindRequest {
                task: "resume from anchor and search memory".to_string(),
                limit: Some(2),
            }))
            .await
            .is_err(),
        "router should be disabled by default"
    );

    let enabled = MemoryServer::new(tempdir.path()).with_router_enabled(true);
    let response = enabled
        .tools_find(Parameters(ToolsFindRequest {
            task: "resume from anchor and search memory".to_string(),
            limit: Some(2),
        }))
        .await
        .expect("tools.find should run when enabled")
        .0;

    assert!(!response.tools.is_empty());
    assert!(response.prompt_tokens_delta > 0);
    assert!(response
        .tools
        .iter()
        .any(|tool| tool.name == "memory.anchor.get" || tool.name == "memory.find"));
}

#[tokio::test]
async fn memory_tools_store_and_find_with_sqlite_vec_backend() {
    let store = SqliteVecVectorStore::in_memory().expect("sqlite-vec should open");
    let backend = VectorMemoryBackend::with_embedder(
        store,
        VectorMemoryConfig {
            collection: "mcp_sqlite".to_string(),
            dimensions: TEST_DIMENSIONS,
            distance: Distance::Cosine,
        },
        Arc::new(TestEmbedder),
    )
    .expect("backend should construct");
    let server = MemoryServer::with_backend(Arc::new(backend));

    server
        .memory_store(Parameters(StoreRequest {
            content: "MCP sqlite vector memory round trip".to_string(),
            tags: Some(vec!["mcp".to_string()]),
            node_id: Some("node:mcp-sqlite".to_string()),
            scope: None,
            agent_id: None,
            session_id: None,
            task_id: None,
            user_id: None,
        }))
        .await
        .expect("store should succeed");

    let found = server
        .memory_find(Parameters(FindRequest {
            query: "vector".to_string(),
            limit: Some(5),
            node_id: Some("node:mcp-sqlite".to_string()),
            scope: None,
            agent_id: None,
            session_id: None,
            task_id: None,
            user_id: None,
        }))
        .await
        .expect("find should succeed")
        .0;

    assert_eq!(found.hits.len(), 1);
    assert_eq!(found.hits[0].node_id, "node:mcp-sqlite");
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
