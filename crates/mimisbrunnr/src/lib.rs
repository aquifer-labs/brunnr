// SPDX-License-Identifier: Apache-2.0

//! Mímisbrunnr memory API and local backends.

mod anchor;
mod backend;
mod backfill;
mod compat;
mod files;
mod identity;
mod lane_lock;
#[cfg(feature = "qdrant")]
mod qdrant;
mod retrieval;
mod rrf;
#[cfg(feature = "sqlite-vec")]
mod sqlite_vec;
mod types;
mod upgrade;
#[cfg(feature = "vector")]
mod vector;
#[cfg(feature = "vector")]
mod vector_memory;
mod working;

pub use anchor::{recover_after_compaction, MuninnAnchorStore, RecoveryContext, SessionAnchor};
pub use backend::MemoryBackend;
pub use backfill::{
    backfill_directory, collect_memory_paths, parse_memory_path, BackfillFailure, BackfillStats,
};
pub use compat::{CollectionCompat, COMPAT_POINT_ID, OKF_VERSION};
pub use files::FilesBackend;
pub use identity::stable_memory_id;
pub use lane_lock::{SessionLaneGuard, SessionLaneLock};
#[cfg(feature = "qdrant")]
pub use qdrant::{
    preflight_qdrant, QdrantBackend, QdrantEndpoints, QdrantPreflightReport, QdrantVectorStore,
    QdrantVectorStoreConfig,
};
#[cfg(feature = "vector")]
pub use retrieval::FastembedReranker;
pub use retrieval::{LocalLexicalReranker, Reranker};
pub use rrf::reciprocal_rank_fusion;
#[cfg(feature = "sqlite-vec")]
pub use sqlite_vec::{SqliteVecBackend, SqliteVecVectorStore, SqliteVecVectorStoreConfig};
pub use types::{
    MemoryError, MemoryId, MemoryQuery, MemoryRecord, MemoryResult, MemoryScope, MemoryTier,
    RrfOptions, SearchHit, SearchSource, StoreMemory,
};
pub use upgrade::{
    default_migration_collection, export_okf_bundle, migrate_okf_bundle, migration_manifest_path,
    verify_okf_bundle, MigrationPlan, MigrationReport, OkfExportReport, OkfVerifyReport,
    SnapshotReport, VectorCollectionAdmin,
};
#[cfg(feature = "vector")]
pub use vector::{
    Distance, Filter, FilterCondition, FilterValue, PayloadIndex, RangeFilter, VectorCollection,
    VectorPoint, VectorSearch, VectorSearchHit, VectorSearchSource, VectorStore,
    VectorStoreCapabilities,
};
#[cfg(feature = "vector")]
pub use vector_memory::{
    FastembedTextEmbedder, TextEmbedder, VectorMemoryBackend, VectorMemoryConfig,
    PINNED_FASTEMBED_DIMENSIONS, PINNED_FASTEMBED_MODEL,
};
pub use working::{
    InMemoryWorkingMemory, WorkingMemory, WorkingMemoryMode, WorkingMemoryView, WorkingTurn,
};
