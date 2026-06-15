// SPDX-License-Identifier: Apache-2.0

#![cfg(feature = "qdrant")]

use std::{collections::BTreeMap, env};

use chrono::Utc;
use mimisbrunnr::{
    preflight_qdrant, MemoryBackend, MemoryQuery, MemoryTier, QdrantEndpoints, QdrantVectorStore,
    QdrantVectorStoreConfig, RrfOptions, StoreMemory, VectorMemoryBackend, VectorMemoryConfig,
    PINNED_FASTEMBED_DIMENSIONS, PINNED_FASTEMBED_MODEL,
};

#[test]
fn qdrant_vector_backend_pins_fastembed_model_and_dimensions() {
    let config = QdrantVectorStoreConfig::new("http://127.0.0.1:6334");

    assert_eq!(config.url, "http://127.0.0.1:6334");
    assert_eq!(config.rest_url, None);
    assert_eq!(PINNED_FASTEMBED_MODEL, "intfloat/multilingual-e5-small");
    assert_eq!(PINNED_FASTEMBED_DIMENSIONS, 384);
}

#[test]
fn qdrant_endpoint_normalization_accepts_single_rest_url() {
    let endpoints = QdrantVectorStoreConfig::new("http://127.0.0.1:6333")
        .endpoints()
        .expect("REST URL should derive gRPC sibling");
    assert_eq!(
        endpoints,
        QdrantEndpoints {
            grpc_url: "http://127.0.0.1:6334".to_string(),
            rest_url: "http://127.0.0.1:6333".to_string(),
        }
    );
}

#[test]
fn qdrant_endpoint_normalization_names_custom_port_error() {
    let error = QdrantVectorStoreConfig::new("http://127.0.0.1:7444")
        .endpoints()
        .expect_err("custom single port should be actionable");
    assert!(error
        .to_string()
        .contains("pass --qdrant-rest-url explicitly"));
}

#[tokio::test]
async fn qdrant_preflight_reports_actionable_custom_port_error_without_network() {
    let error = preflight_qdrant(QdrantVectorStoreConfig::new("http://127.0.0.1:7444"))
        .await
        .expect_err("custom single port should fail before probing network");
    assert!(error
        .to_string()
        .contains("pass --qdrant-rest-url explicitly"));
}

#[tokio::test]
#[ignore = "requires a local Qdrant instance and QDRANT_URL"]
async fn live_qdrant_vector_backend_satisfies_memory_contract() {
    let Ok(url) = env::var("QDRANT_URL") else {
        eprintln!("QDRANT_URL is not set; skipping live Qdrant test");
        return;
    };
    let mut config = QdrantVectorStoreConfig::new(url);
    config.api_key = env::var("QDRANT_API_KEY").ok();
    let store = QdrantVectorStore::connect(config).expect("Qdrant store should connect");
    let backend = VectorMemoryBackend::new(
        store,
        VectorMemoryConfig::new(format!("brunnr_test_{}", Utc::now().timestamp_millis())),
    )
    .expect("backend should construct");

    let stored = backend
        .store(StoreMemory {
            content: "Qdrant stores durable multilingual context".to_string(),
            tags: vec!["qdrant".to_string()],
            metadata: BTreeMap::new(),
            tier: MemoryTier::L1Atom,
            node_id: Some("node:qdrant".to_string()),
            created_at: None,
            scope: None,
            agent_id: None,
            session_id: None,
            task_id: None,
            user_id: None,
        })
        .await
        .expect("store should succeed");
    backend
        .store(StoreMemory {
            content: "hybrid vector keyword retrieval".to_string(),
            tags: Vec::new(),
            metadata: BTreeMap::new(),
            tier: MemoryTier::L1Atom,
            node_id: Some("node:qdrant-rrf".to_string()),
            created_at: None,
            scope: None,
            agent_id: None,
            session_id: None,
            task_id: None,
            user_id: None,
        })
        .await
        .expect("second store should succeed");

    let found = backend
        .find(MemoryQuery::new("durable context").with_limit(5))
        .await
        .expect("find should succeed");
    let drill_down = backend
        .get_node("node:qdrant")
        .await
        .expect("get_node should succeed");
    let hybrid = backend
        .hybrid_rrf(
            MemoryQuery::new("hybrid").with_limit(5),
            MemoryQuery::new("retrieval").with_limit(5),
            RrfOptions {
                limit: 5,
                ..RrfOptions::default()
            },
        )
        .await
        .expect("hybrid should succeed");

    assert!(
        found.iter().any(|hit| hit.record.node_id == "node:qdrant"),
        "find should return Qdrant record, got {found:?}"
    );
    assert_eq!(drill_down, Some(stored));
    assert!(
        hybrid
            .iter()
            .any(|hit| hit.record.node_id == "node:qdrant-rrf"),
        "hybrid should return Qdrant RRF record, got {hybrid:?}"
    );
}

#[tokio::test]
#[ignore = "requires a local Qdrant instance and QDRANT_URL"]
async fn live_qdrant_collections_isolate_two_projects() {
    let Ok(url) = env::var("QDRANT_URL") else {
        eprintln!("QDRANT_URL is not set; skipping live Qdrant isolation test");
        return;
    };
    let collection_suffix = Utc::now().timestamp_millis();
    let mut config_a = QdrantVectorStoreConfig::new(url.clone());
    config_a.api_key = env::var("QDRANT_API_KEY").ok();
    let mut config_b = QdrantVectorStoreConfig::new(url);
    config_b.api_key = env::var("QDRANT_API_KEY").ok();
    let project_a = VectorMemoryBackend::new(
        QdrantVectorStore::connect(config_a).expect("project A Qdrant should connect"),
        VectorMemoryConfig::new(format!("brunnr_project_a_{collection_suffix}")),
    )
    .expect("project A backend should construct");
    let project_b = VectorMemoryBackend::new(
        QdrantVectorStore::connect(config_b).expect("project B Qdrant should connect"),
        VectorMemoryConfig::new(format!("brunnr_project_b_{collection_suffix}")),
    )
    .expect("project B backend should construct");

    project_a
        .store(StoreMemory {
            content: "same phrase isolated to qdrant project alpha".to_string(),
            tags: Vec::new(),
            metadata: BTreeMap::new(),
            tier: MemoryTier::L1Atom,
            node_id: Some("node:qdrant-project-a".to_string()),
            created_at: None,
            scope: None,
            agent_id: None,
            session_id: None,
            task_id: None,
            user_id: Some("user-a".to_string()),
        })
        .await
        .expect("project A store should succeed");
    project_b
        .store(StoreMemory {
            content: "same phrase isolated to qdrant project beta".to_string(),
            tags: Vec::new(),
            metadata: BTreeMap::new(),
            tier: MemoryTier::L1Atom,
            node_id: Some("node:qdrant-project-b".to_string()),
            created_at: None,
            scope: None,
            agent_id: None,
            session_id: None,
            task_id: None,
            user_id: Some("user-b".to_string()),
        })
        .await
        .expect("project B store should succeed");

    let hits_a = project_a
        .find(MemoryQuery::new("same phrase isolated").with_limit(10))
        .await
        .expect("project A find should succeed");
    let hits_b = project_b
        .find(MemoryQuery::new("same phrase isolated").with_limit(10))
        .await
        .expect("project B find should succeed");

    assert_eq!(hits_a.len(), 1);
    assert_eq!(hits_a[0].record.node_id, "node:qdrant-project-a");
    assert_eq!(hits_b.len(), 1);
    assert_eq!(hits_b[0].record.node_id, "node:qdrant-project-b");
}
