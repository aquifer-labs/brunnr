// SPDX-License-Identifier: Apache-2.0

use std::sync::{Arc, Mutex};

use chrono::Utc;
use fastembed::{EmbeddingModel, TextEmbedding, TextInitOptions};
use futures_util::{future::BoxFuture, FutureExt};
use serde::{Deserialize, Serialize};

use crate::{
    identity::stable_memory_id, reciprocal_rank_fusion, CollectionCompat, Distance, Filter,
    FilterCondition, FilterValue, MemoryBackend, MemoryError, MemoryId, MemoryQuery, MemoryRecord,
    MemoryResult, MemoryScope, MemoryTier, PayloadIndex, RrfOptions, SearchHit, SearchSource,
    SessionLaneLock, StoreMemory, VectorCollection, VectorPoint, VectorSearch, VectorSearchHit,
    VectorSearchSource, VectorStore, COMPAT_POINT_ID,
};

pub const PINNED_FASTEMBED_MODEL: &str = "intfloat/multilingual-e5-small";
pub const PINNED_FASTEMBED_DIMENSIONS: usize = 384;

pub trait TextEmbedder: Send + Sync {
    fn embed_query(&self, text: &str) -> MemoryResult<Vec<f32>>;

    fn embed_passage(&self, text: &str) -> MemoryResult<Vec<f32>>;
}

pub struct FastembedTextEmbedder {
    inner: Mutex<TextEmbedding>,
}

impl FastembedTextEmbedder {
    pub fn new() -> MemoryResult<Self> {
        let inner = TextEmbedding::try_new(
            TextInitOptions::new(EmbeddingModel::MultilingualE5Small)
                .with_show_download_progress(false),
        )
        .map_err(|error| MemoryError::BackendUnavailable(error.to_string()))?;
        Ok(Self {
            inner: Mutex::new(inner),
        })
    }

    fn embed_prefixed(&self, prefix: &str, text: &str) -> MemoryResult<Vec<f32>> {
        let input = format!("{prefix}: {text}");
        let mut embedder = self
            .inner
            .lock()
            .map_err(|error| MemoryError::BackendUnavailable(error.to_string()))?;
        let mut embeddings = embedder
            .embed([input], None)
            .map_err(|error| MemoryError::BackendUnavailable(error.to_string()))?;
        embeddings.pop().ok_or_else(|| {
            MemoryError::BackendUnavailable("fastembed returned no embeddings".to_string())
        })
    }
}

impl TextEmbedder for FastembedTextEmbedder {
    fn embed_query(&self, text: &str) -> MemoryResult<Vec<f32>> {
        self.embed_prefixed("query", text)
    }

    fn embed_passage(&self, text: &str) -> MemoryResult<Vec<f32>> {
        self.embed_prefixed("passage", text)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VectorMemoryConfig {
    pub collection: String,
    pub embedding_model: String,
    pub dimensions: usize,
    pub distance: Distance,
}

impl VectorMemoryConfig {
    pub fn new(collection: impl Into<String>) -> Self {
        Self {
            collection: collection.into(),
            embedding_model: PINNED_FASTEMBED_MODEL.to_string(),
            dimensions: PINNED_FASTEMBED_DIMENSIONS,
            distance: Distance::Cosine,
        }
    }
}

pub struct VectorMemoryBackend<V: VectorStore> {
    store: V,
    config: VectorMemoryConfig,
    embedder: Arc<dyn TextEmbedder>,
}

impl<V: VectorStore> VectorMemoryBackend<V> {
    pub fn new(store: V, config: VectorMemoryConfig) -> MemoryResult<Self> {
        Self::with_embedder(store, config, Arc::new(FastembedTextEmbedder::new()?))
    }

    pub fn with_embedder(
        store: V,
        config: VectorMemoryConfig,
        embedder: Arc<dyn TextEmbedder>,
    ) -> MemoryResult<Self> {
        Ok(Self {
            store,
            config,
            embedder,
        })
    }

    pub fn vector_store(&self) -> &V {
        &self.store
    }

    pub fn config(&self) -> &VectorMemoryConfig {
        &self.config
    }

    async fn ensure_ready(&self) -> MemoryResult<()> {
        self.store
            .ensure_collection(VectorCollection {
                name: self.config.collection.clone(),
                dimensions: self.config.dimensions,
                distance: self.config.distance,
            })
            .await?;
        for field in [
            "node_id",
            "scope",
            "agent_id",
            "session_id",
            "task_id",
            "user_id",
        ] {
            self.store
                .ensure_payload_index(
                    &self.config.collection,
                    PayloadIndex {
                        field: field.to_string(),
                    },
                )
                .await?;
        }
        self.ensure_compat_metadata().await?;
        Ok(())
    }

    async fn ensure_compat_metadata(&self) -> MemoryResult<()> {
        let expected = CollectionCompat::from_config(&self.config);
        if let Some(point) = self
            .store
            .get(&self.config.collection, COMPAT_POINT_ID)
            .await?
        {
            let payload: CompatPayload = serde_json::from_value(point.payload)?;
            payload.compat.validate_compatible(&expected)?;
            return Ok(());
        }

        self.store
            .upsert(
                &self.config.collection,
                vec![VectorPoint {
                    id: COMPAT_POINT_ID.to_string(),
                    vector: vec![0.0; self.config.dimensions],
                    payload: serde_json::to_value(CompatPayload {
                        kind: compat_payload_kind(),
                        id: compat_point_id(),
                        node_id: compat_point_id(),
                        compat: expected,
                    })?,
                }],
            )
            .await
    }

    async fn vector_hits(&self, query: MemoryQuery) -> MemoryResult<Vec<SearchHit>> {
        self.ensure_ready().await?;
        let vector = self.embedder.embed_query(&query.text)?;
        let hits = self
            .store
            .search(
                &self.config.collection,
                VectorSearch {
                    vector: Some(vector),
                    text: None,
                    filter: filter_from_query(&query),
                    limit: query.limit,
                    source: VectorSearchSource::Vector,
                },
            )
            .await?;
        vector_hits_to_memory_hits(hits, SearchSource::Vector)
    }

    async fn keyword_hits(&self, query: MemoryQuery) -> MemoryResult<Vec<SearchHit>> {
        self.ensure_ready().await?;
        let hits = self
            .store
            .search(
                &self.config.collection,
                VectorSearch {
                    vector: None,
                    text: Some(query.text.clone()),
                    filter: filter_from_query(&query),
                    limit: query.limit,
                    source: VectorSearchSource::Keyword,
                },
            )
            .await?;
        vector_hits_to_memory_hits(hits, SearchSource::Keyword)
    }
}

impl<V: VectorStore> MemoryBackend for VectorMemoryBackend<V> {
    fn find(&self, query: MemoryQuery) -> BoxFuture<'_, MemoryResult<Vec<SearchHit>>> {
        async move {
            let options = RrfOptions {
                limit: query.limit,
                ..RrfOptions::default()
            };
            self.hybrid_rrf(query.clone(), query, options).await
        }
        .boxed()
    }

    fn store(&self, memory: StoreMemory) -> BoxFuture<'_, MemoryResult<MemoryRecord>> {
        async move {
            let _lane_guard = SessionLaneLock::default_rooted()
                .acquire(&self.config.collection, memory.session_id.as_deref())
                .await?;
            self.ensure_ready().await?;
            let id = stable_memory_id(&memory);
            if let Some(existing) = self.store.get(&self.config.collection, id.as_str()).await? {
                return point_to_record(existing);
            }

            let node_id = memory.node_id.unwrap_or_else(|| format!("node:{id}"));
            let record = MemoryRecord {
                id,
                node_id,
                content: memory.content,
                tags: memory.tags,
                metadata: memory.metadata,
                tier: memory.tier,
                created_at: memory.created_at.unwrap_or_else(Utc::now),
                scope: memory.scope,
                agent_id: memory.agent_id,
                session_id: memory.session_id,
                task_id: memory.task_id,
                user_id: memory.user_id,
            };
            let vector = self.embedder.embed_passage(&record.content)?;
            self.store
                .upsert(
                    &self.config.collection,
                    vec![VectorPoint {
                        id: record.id.to_string(),
                        vector,
                        payload: serde_json::to_value(MemoryPayload::from(&record))?,
                    }],
                )
                .await?;
            Ok(record)
        }
        .boxed()
    }

    fn hybrid_rrf(
        &self,
        keyword_query: MemoryQuery,
        vector_query: MemoryQuery,
        options: RrfOptions,
    ) -> BoxFuture<'_, MemoryResult<Vec<SearchHit>>> {
        async move {
            self.ensure_ready().await?;
            if self.store.capabilities().supports_server_side_hybrid {
                let vector = self.embedder.embed_query(&vector_query.text)?;
                let hits = self
                    .store
                    .search(
                        &self.config.collection,
                        VectorSearch {
                            vector: Some(vector),
                            text: Some(keyword_query.text),
                            filter: filter_from_query(&vector_query),
                            limit: options.limit,
                            source: VectorSearchSource::Hybrid,
                        },
                    )
                    .await?;
                return vector_hits_to_memory_hits(hits, SearchSource::Hybrid);
            }

            let keyword_hits = self.keyword_hits(keyword_query).await?;
            let vector_hits = self.vector_hits(vector_query).await?;
            Ok(reciprocal_rank_fusion(
                &[keyword_hits, vector_hits],
                options,
            ))
        }
        .boxed()
    }

    fn get_node(&self, node_id: &str) -> BoxFuture<'_, MemoryResult<Option<MemoryRecord>>> {
        let node_id = node_id.to_string();
        async move {
            self.ensure_ready().await?;
            if let Some(point) = self.store.get(&self.config.collection, &node_id).await? {
                return point_to_record(point).map(Some);
            }
            let mut hits = self
                .store
                .search(
                    &self.config.collection,
                    VectorSearch {
                        vector: None,
                        text: None,
                        filter: Filter::node_id(node_id),
                        limit: 1,
                        source: VectorSearchSource::Keyword,
                    },
                )
                .await?;
            hits.pop().map(|hit| point_to_record(hit.point)).transpose()
        }
        .boxed()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MemoryPayload {
    id: MemoryId,
    node_id: String,
    content: String,
    tags: Vec<String>,
    metadata: std::collections::BTreeMap<String, String>,
    tier: MemoryTier,
    created_at: chrono::DateTime<Utc>,
    #[serde(default)]
    scope: Option<MemoryScope>,
    #[serde(default)]
    agent_id: Option<String>,
    #[serde(default)]
    session_id: Option<String>,
    #[serde(default)]
    task_id: Option<String>,
    #[serde(default)]
    user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CompatPayload {
    #[serde(default = "compat_payload_kind")]
    kind: String,
    #[serde(default = "compat_point_id")]
    id: String,
    #[serde(default = "compat_point_id")]
    node_id: String,
    #[serde(flatten)]
    compat: CollectionCompat,
}

fn compat_payload_kind() -> String {
    "brunnr.compat".to_string()
}

fn compat_point_id() -> String {
    COMPAT_POINT_ID.to_string()
}

impl From<&MemoryRecord> for MemoryPayload {
    fn from(record: &MemoryRecord) -> Self {
        Self {
            id: record.id.clone(),
            node_id: record.node_id.clone(),
            content: record.content.clone(),
            tags: record.tags.clone(),
            metadata: record.metadata.clone(),
            tier: record.tier,
            created_at: record.created_at,
            scope: record.scope,
            agent_id: record.agent_id.clone(),
            session_id: record.session_id.clone(),
            task_id: record.task_id.clone(),
            user_id: record.user_id.clone(),
        }
    }
}

impl From<MemoryPayload> for MemoryRecord {
    fn from(payload: MemoryPayload) -> Self {
        Self {
            id: payload.id,
            node_id: payload.node_id,
            content: payload.content,
            tags: payload.tags,
            metadata: payload.metadata,
            tier: payload.tier,
            created_at: payload.created_at,
            scope: payload.scope,
            agent_id: payload.agent_id,
            session_id: payload.session_id,
            task_id: payload.task_id,
            user_id: payload.user_id,
        }
    }
}

fn filter_from_query(query: &MemoryQuery) -> Filter {
    let mut filter = query
        .node_id
        .as_ref()
        .map_or_else(Filter::default, Filter::node_id);
    filter.must_not.push(FilterCondition::Eq {
        field: "node_id".to_string(),
        value: FilterValue::String(COMPAT_POINT_ID.to_string()),
    });
    if let Some(scope) = query.scope {
        filter.must_eq("scope", scope.as_str());
    }
    if let Some(agent_id) = &query.agent_id {
        filter.must_eq("agent_id", agent_id);
    }
    if let Some(session_id) = &query.session_id {
        filter.must_eq("session_id", session_id);
    }
    if let Some(task_id) = &query.task_id {
        filter.must_eq("task_id", task_id);
    }
    if let Some(user_id) = &query.user_id {
        filter.must_eq("user_id", user_id);
    }
    filter
}

fn vector_hits_to_memory_hits(
    hits: Vec<VectorSearchHit>,
    source: SearchSource,
) -> MemoryResult<Vec<SearchHit>> {
    hits.into_iter()
        .map(|hit| {
            Ok(SearchHit {
                record: point_to_record(hit.point)?,
                score: hit.score,
                source,
            })
        })
        .collect()
}

fn point_to_record(point: VectorPoint) -> MemoryResult<MemoryRecord> {
    let payload: MemoryPayload = serde_json::from_value(point.payload)?;
    Ok(MemoryRecord::from(payload))
}
