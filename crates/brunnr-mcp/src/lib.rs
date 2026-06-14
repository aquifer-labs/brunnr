// SPDX-License-Identifier: Apache-2.0

use std::{path::PathBuf, sync::Arc};

#[cfg(feature = "qdrant")]
use std::env;

use brunnr_core::{MemoryBackendKind, MemoryConfig};
use mimisbrunnr::{
    FilesBackend, MemoryBackend, MemoryQuery, MemoryScope, MemoryTier, MuninnAnchorStore,
    SessionAnchor, SqliteVecVectorStore, SqliteVecVectorStoreConfig, StoreMemory,
    VectorMemoryBackend, VectorMemoryConfig,
};
use rmcp::{
    handler::server::{
        router::tool::ToolRouter,
        wrapper::{Json, Parameters},
    },
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
    transport::stdio,
    ErrorData, ServerHandler, ServiceExt,
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "qdrant")]
use mimisbrunnr::{QdrantVectorStore, QdrantVectorStoreConfig};

const TOOL_INSTRUCTIONS: &str =
    "ALWAYS search the project memory before non-trivial work; store durable, reusable learnings.";

#[derive(Clone)]
pub struct MemoryServer {
    backend: Arc<dyn MemoryBackend>,
    anchor_store: Option<MuninnAnchorStore>,
    router_enabled: bool,
    tool_router: ToolRouter<Self>,
}

impl MemoryServer {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        Self::with_backend_and_anchor(
            Arc::new(FilesBackend::new(&root)),
            Some(MuninnAnchorStore::new(root)),
        )
    }

    pub fn with_backend(backend: Arc<dyn MemoryBackend>) -> Self {
        Self::with_backend_and_anchor(backend, None)
    }

    pub fn with_backend_and_anchor(
        backend: Arc<dyn MemoryBackend>,
        anchor_store: Option<MuninnAnchorStore>,
    ) -> Self {
        Self {
            backend,
            anchor_store,
            router_enabled: false,
            tool_router: Self::tool_router(),
        }
    }

    pub fn with_router_enabled(mut self, enabled: bool) -> Self {
        self.router_enabled = enabled;
        self
    }

    pub fn from_config(config: &MemoryConfig) -> anyhow::Result<Self> {
        Ok(Self::with_backend_and_anchor(
            open_memory_backend(config)?,
            Some(MuninnAnchorStore::new(&config.root)),
        ))
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FindRequest {
    pub query: String,
    pub limit: Option<usize>,
    pub node_id: Option<String>,
    pub scope: Option<ScopeRequest>,
    pub agent_id: Option<String>,
    pub session_id: Option<String>,
    pub task_id: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub struct FindResponse {
    pub hits: Vec<FindHit>,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub struct FindHit {
    pub id: String,
    pub node_id: String,
    pub content: String,
    pub score: f32,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct StoreRequest {
    pub content: String,
    pub tags: Option<Vec<String>>,
    pub node_id: Option<String>,
    pub scope: Option<ScopeRequest>,
    pub agent_id: Option<String>,
    pub session_id: Option<String>,
    pub task_id: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ScopeRequest {
    Shared,
    Agent,
    Session,
    Task,
}

impl From<ScopeRequest> for MemoryScope {
    fn from(value: ScopeRequest) -> Self {
        match value {
            ScopeRequest::Shared => Self::Shared,
            ScopeRequest::Agent => Self::Agent,
            ScopeRequest::Session => Self::Session,
            ScopeRequest::Task => Self::Task,
        }
    }
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub struct StoreResponse {
    pub id: String,
    pub node_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AnchorSetRequest {
    pub current_task: String,
    pub plan_pointer: Option<String>,
    pub last_decisions: Option<Vec<String>>,
    pub next_step: String,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub struct AnchorGetResponse {
    pub anchor: Option<AnchorPayload>,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub struct AnchorPayload {
    pub current_task: String,
    pub plan_pointer: Option<String>,
    pub last_decisions: Vec<String>,
    pub next_step: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ToolsFindRequest {
    pub task: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub struct ToolsFindResponse {
    pub tools: Vec<ToolMatch>,
    pub prompt_tokens_before: usize,
    pub prompt_tokens_after: usize,
    pub prompt_tokens_delta: isize,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub struct ToolMatch {
    pub name: String,
    pub description: String,
    pub score: f32,
}

#[tool_router]
impl MemoryServer {
    #[tool(
        name = "memory.find",
        description = "Find durable project memories by query. ALWAYS search the project memory before non-trivial work."
    )]
    pub async fn memory_find(
        &self,
        Parameters(request): Parameters<FindRequest>,
    ) -> Result<Json<FindResponse>, ErrorData> {
        let mut query = MemoryQuery::new(request.query);
        query.limit = request.limit.unwrap_or(10);
        query.node_id = request.node_id;
        query.scope = request.scope.map(Into::into);
        query.agent_id = request.agent_id;
        query.session_id = request.session_id;
        query.task_id = request.task_id;
        query.user_id = request.user_id;
        let hits = self
            .backend
            .find(query)
            .await
            .map_err(|error| ErrorData::internal_error(error.to_string(), None))?
            .into_iter()
            .map(|hit| FindHit {
                id: hit.record.id.to_string(),
                node_id: hit.record.node_id,
                content: hit.record.content,
                score: hit.score,
                tags: hit.record.tags,
            })
            .collect();
        Ok(Json(FindResponse { hits }))
    }

    #[tool(
        name = "memory.store",
        description = "Store durable, reusable learnings in project memory."
    )]
    pub async fn memory_store(
        &self,
        Parameters(request): Parameters<StoreRequest>,
    ) -> Result<Json<StoreResponse>, ErrorData> {
        let record = self
            .backend
            .store(StoreMemory {
                content: request.content,
                tags: request.tags.unwrap_or_default(),
                metadata: Default::default(),
                tier: MemoryTier::L1Atom,
                node_id: request.node_id,
                created_at: None,
                scope: request.scope.map(Into::into),
                agent_id: request.agent_id,
                session_id: request.session_id,
                task_id: request.task_id,
                user_id: request.user_id,
            })
            .await
            .map_err(|error| ErrorData::internal_error(error.to_string(), None))?;
        Ok(Json(StoreResponse {
            id: record.id.to_string(),
            node_id: record.node_id,
        }))
    }

    #[tool(
        name = "memory.anchor.get",
        description = "Read the current Muninn session anchor from OKF log.md before resuming work."
    )]
    pub async fn memory_anchor_get(&self) -> Result<Json<AnchorGetResponse>, ErrorData> {
        let store = self.anchor_store.as_ref().ok_or_else(|| {
            ErrorData::internal_error("Muninn anchor store is not configured".to_string(), None)
        })?;
        let anchor = store
            .get()
            .await
            .map_err(|error| ErrorData::internal_error(error.to_string(), None))?
            .map(AnchorPayload::from);
        Ok(Json(AnchorGetResponse { anchor }))
    }

    #[tool(
        name = "memory.anchor.set",
        description = "Write the current task, plan pointer, decisions, and next step to OKF log.md."
    )]
    pub async fn memory_anchor_set(
        &self,
        Parameters(request): Parameters<AnchorSetRequest>,
    ) -> Result<Json<AnchorGetResponse>, ErrorData> {
        let store = self.anchor_store.as_ref().ok_or_else(|| {
            ErrorData::internal_error("Muninn anchor store is not configured".to_string(), None)
        })?;
        let mut anchor = SessionAnchor::new(request.current_task, request.next_step);
        anchor.plan_pointer = request.plan_pointer;
        anchor.last_decisions = request.last_decisions.unwrap_or_default();
        let anchor = store
            .set(anchor)
            .await
            .map_err(|error| ErrorData::internal_error(error.to_string(), None))?;
        Ok(Json(AnchorGetResponse {
            anchor: Some(AnchorPayload::from(anchor)),
        }))
    }

    #[tool(
        name = "tools.find",
        description = "Opt-in router: return only MCP tools relevant to a task and estimate prompt-token savings."
    )]
    pub async fn tools_find(
        &self,
        Parameters(request): Parameters<ToolsFindRequest>,
    ) -> Result<Json<ToolsFindResponse>, ErrorData> {
        if !self.router_enabled {
            return Err(ErrorData::internal_error(
                "tools.find router is disabled by config".to_string(),
                None,
            ));
        }
        let limit = request.limit.unwrap_or(3).max(1);
        let mut tools = tool_registry()
            .iter()
            .map(|tool| ToolMatch {
                name: tool.name.to_string(),
                description: tool.description.to_string(),
                score: lexical_score(&request.task, tool.description),
            })
            .filter(|tool| tool.score > 0.0)
            .collect::<Vec<_>>();
        tools.sort_by(|left, right| {
            right
                .score
                .partial_cmp(&left.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        tools.truncate(limit);
        let prompt_tokens_before = estimate_tokens(&request.task)
            + tool_registry()
                .iter()
                .map(|tool| estimate_tokens(tool.description))
                .sum::<usize>();
        let prompt_tokens_after = estimate_tokens(&request.task)
            + tools
                .iter()
                .map(|tool| estimate_tokens(&tool.description))
                .sum::<usize>();
        Ok(Json(ToolsFindResponse {
            tools,
            prompt_tokens_before,
            prompt_tokens_after,
            prompt_tokens_delta: prompt_tokens_before as isize - prompt_tokens_after as isize,
        }))
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for MemoryServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build()).with_instructions(
            format!(
                "Brunnr memory server exposing memory.find and memory.store. {TOOL_INSTRUCTIONS}"
            ),
        )
    }
}

impl From<SessionAnchor> for AnchorPayload {
    fn from(anchor: SessionAnchor) -> Self {
        Self {
            current_task: anchor.current_task,
            plan_pointer: anchor.plan_pointer,
            last_decisions: anchor.last_decisions,
            next_step: anchor.next_step,
            updated_at: anchor.updated_at.to_rfc3339(),
        }
    }
}

pub async fn run_stdio(root: impl Into<PathBuf>) -> anyhow::Result<()> {
    let server = MemoryServer::new(root);
    server.serve(stdio()).await?.waiting().await?;
    Ok(())
}

pub async fn run_stdio_with_config(config: &MemoryConfig) -> anyhow::Result<()> {
    let server = MemoryServer::from_config(config)?;
    server.serve(stdio()).await?.waiting().await?;
    Ok(())
}

pub async fn run_stdio_with_config_and_router(
    config: &MemoryConfig,
    router_enabled: bool,
) -> anyhow::Result<()> {
    let server = MemoryServer::from_config(config)?.with_router_enabled(router_enabled);
    server.serve(stdio()).await?.waiting().await?;
    Ok(())
}

pub fn open_memory_backend(config: &MemoryConfig) -> anyhow::Result<Arc<dyn MemoryBackend>> {
    match config.backend {
        MemoryBackendKind::Files => Ok(Arc::new(FilesBackend::new(&config.root))),
        MemoryBackendKind::SqliteVec => {
            let store = SqliteVecVectorStore::open(SqliteVecVectorStoreConfig::new(sqlite_path(
                &config.root,
            )))?;
            Ok(Arc::new(VectorMemoryBackend::new(
                store,
                VectorMemoryConfig::new(&config.collection),
            )?))
        }
        MemoryBackendKind::Qdrant => open_qdrant_backend(config),
        MemoryBackendKind::TencentDb => anyhow::bail!("TencentDB backend is not available yet"),
    }
}

#[cfg(feature = "qdrant")]
fn open_qdrant_backend(config: &MemoryConfig) -> anyhow::Result<Arc<dyn MemoryBackend>> {
    let url = config
        .qdrant_url
        .clone()
        .or_else(|| env::var("QDRANT_URL").ok())
        .ok_or_else(|| anyhow::anyhow!("Qdrant backend requires qdrant_url or QDRANT_URL"))?;
    let mut vector_config = QdrantVectorStoreConfig::new(url);
    if let Some(env_name) = &config.qdrant_api_key_env {
        vector_config.api_key = env::var(env_name).ok();
    }
    let store = QdrantVectorStore::connect(vector_config)?;
    Ok(Arc::new(VectorMemoryBackend::new(
        store,
        VectorMemoryConfig::new(&config.collection),
    )?))
}

#[cfg(not(feature = "qdrant"))]
fn open_qdrant_backend(_config: &MemoryConfig) -> anyhow::Result<Arc<dyn MemoryBackend>> {
    anyhow::bail!("Qdrant backend requires building brunnr-mcp with the qdrant feature")
}

fn sqlite_path(root: &str) -> PathBuf {
    let path = PathBuf::from(root);
    if path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| matches!(extension, "db" | "sqlite" | "sqlite3"))
    {
        path
    } else {
        path.join("memory.sqlite3")
    }
}

#[derive(Debug, Clone, Copy)]
struct RegisteredTool {
    name: &'static str,
    description: &'static str,
}

fn tool_registry() -> &'static [RegisteredTool] {
    &[
        RegisteredTool {
            name: "memory.find",
            description: "Find durable project memories by query before non-trivial work.",
        },
        RegisteredTool {
            name: "memory.store",
            description: "Store durable reusable learnings in project memory.",
        },
        RegisteredTool {
            name: "memory.anchor.get",
            description: "Read Muninn session anchor from OKF log.md before resuming work.",
        },
        RegisteredTool {
            name: "memory.anchor.set",
            description:
                "Write current task, plan pointer, decisions, and next step to OKF log.md.",
        },
    ]
}

fn lexical_score(task: &str, description: &str) -> f32 {
    let task_terms = terms(task);
    if task_terms.is_empty() {
        return 0.0;
    }
    let description = description.to_ascii_lowercase();
    let matches = task_terms
        .iter()
        .filter(|term| description.contains(term.as_str()))
        .count();
    matches as f32 / task_terms.len() as f32
}

fn terms(input: &str) -> Vec<String> {
    input
        .split_whitespace()
        .map(|term| {
            term.trim_matches(|character: char| !character.is_ascii_alphanumeric())
                .to_ascii_lowercase()
        })
        .filter(|term| term.len() > 2)
        .collect()
}

fn estimate_tokens(text: &str) -> usize {
    text.split_whitespace().count().max(1)
}
