// warp-lite: ai inert stub (v0.5 phase 2)
//
// The original `manager.rs` (~1000 LOC) drove the lifecycle of every codebase
// index in the app — building, snapshot persistence, file-watcher driven
// incremental sync, retrieval requests, and per-repo state tracking. The
// lightweight fork ships none of that, but the rest of the workspace still
// imports `CodebaseIndexManager` and friends from this module. To keep those
// call sites compiling, we keep the *shape* of every public type and method
// and replace every body with a minimal no-op that returns a default,
// `Ok(())`, or `Err(...)` as appropriate.

use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::Arc,
};

use repo_metadata::BuildTreeError;
use thiserror::Error;
use warpui::{AppContext, Entity, ModelContext, ModelHandle, SingletonEntity};

use super::{
    codebase_index::{RetrievalID, SyncProgress},
    fragment_metadata::FragmentMetadata,
    store_client::StoreClient,
    CodebaseIndex, Error as CodebaseIndexError, NodeHash,
};

use crate::{
    index::locations::CodeContextLocation,
    workspace::{WorkspaceMetadata, WorkspaceMetadataEvent},
};

/// User-facing indexing completion status.
pub enum CodebaseIndexFinishedStatus {
    Completed,
    Failed(CodebaseIndexingError),
}

#[derive(Error, Debug)]
pub enum RetrieveFileError {
    #[error("Codebase index still indexing")]
    IndexSyncing,
    #[error("Codebase index failed: {0:#}")]
    IndexFailed(CodebaseIndexingError),
    #[error("Codebase index not found")]
    IndexNotFound,
}

pub enum CodebaseIndexManagerEvent {
    RetrievalRequestCompleted {
        retrieval_id: RetrievalID,
        fragments: Arc<HashSet<CodeContextLocation>>,
        out_of_sync_delay: Option<std::time::Duration>,
    },
    RetrievalRequestFailed {
        retrieval_id: RetrievalID,
        error_message: String,
    },
    SyncStateUpdated,
    IndexMetadataUpdated {
        root_path: PathBuf,
        event: WorkspaceMetadataEvent,
    },
    RemoveExpiredIndexMetadata {
        expired_metadata: Arc<Vec<PathBuf>>,
    },
    NewIndexCreated,
}

/// User-facing indexing errors.
#[derive(Error, Debug)]
pub enum CodebaseIndexingError {
    #[error("Build tree error")]
    BuildTreeError,
    #[error("Repo size exceeded max file limit")]
    ExceededMaxFileLimit,
    #[error("Maximum directory depth exceeded")]
    MaxDepthExceeded,
    #[error("Failed to generate embeddings for some hashes:\n{0:#?}")]
    FailedToGenerateEmbeddings(Vec<FragmentMetadata>),
    #[error("Failed to sync intermediate nodes:\n{0:#?}")]
    FailedToSyncIntermediateNodes(Vec<NodeHash>),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<&CodebaseIndexError> for CodebaseIndexingError {
    fn from(value: &CodebaseIndexError) -> Self {
        match value {
            CodebaseIndexError::BuildTreeError(build_tree_error) => match build_tree_error {
                BuildTreeError::ExceededMaxFileLimit => Self::ExceededMaxFileLimit,
                BuildTreeError::MaxDepthExceeded => Self::MaxDepthExceeded,
                _ => Self::BuildTreeError,
            },
            CodebaseIndexError::FailedToGenerateEmbeddings(failed_fragments) => {
                Self::FailedToGenerateEmbeddings(failed_fragments.clone())
            }
            CodebaseIndexError::FailedToSyncIntermediateNodes(failed_hashes) => {
                Self::FailedToSyncIntermediateNodes(failed_hashes.clone())
            }
            _ => Self::Other(anyhow::anyhow!(value.to_string())),
        }
    }
}

/// User-facing codebase index status.
pub struct CodebaseIndexStatus {
    pub(super) has_pending: bool,
    pub(super) has_synced_version: bool,
    pub(super) last_sync_successful: Option<CodebaseIndexFinishedStatus>,
    pub(super) sync_progress: Option<SyncProgress>,
}

impl CodebaseIndexStatus {
    pub fn has_pending(&self) -> bool {
        self.has_pending
    }

    pub fn has_synced_version(&self) -> bool {
        self.has_synced_version
    }

    pub fn last_sync_successful(&self) -> Option<bool> {
        self.last_sync_successful
            .as_ref()
            .map(|res| matches!(res, CodebaseIndexFinishedStatus::Completed))
    }

    pub fn last_sync_result(&self) -> Option<&CodebaseIndexFinishedStatus> {
        self.last_sync_successful.as_ref()
    }

    pub fn sync_progress(&self) -> Option<&SyncProgress> {
        self.sync_progress.as_ref()
    }
}

pub enum BuildSource<'a> {
    FromPath(&'a Path),
    FromPersistedMetadata(WorkspaceMetadata),
}

/// Manager for the codebase index states across the app.
///
/// In the lightweight fork this struct holds nothing meaningful — it only
/// exists so that `app/` can keep referring to `CodebaseIndexManager` from
/// dozens of call sites without dragging the full embedding pipeline back in.
pub struct CodebaseIndexManager {
    #[allow(dead_code)]
    codebase_indices: HashMap<PathBuf, ModelHandle<CodebaseIndex>>,
    #[allow(dead_code)]
    store_client: Arc<dyn StoreClient>,
    #[allow(dead_code)]
    max_indices: Option<usize>,
    #[allow(dead_code)]
    max_files_repo_limit: usize,
    #[allow(dead_code)]
    embedding_generation_batch_size: usize,
}

impl CodebaseIndexManager {
    pub fn new(
        _persisted_index_metadata: Vec<WorkspaceMetadata>,
        max_index_count: Option<usize>,
        max_files_repo_limit: usize,
        embedding_generation_batch_size: usize,
        store_client: Arc<dyn StoreClient>,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self {
            codebase_indices: HashMap::new(),
            store_client,
            max_indices: max_index_count,
            max_files_repo_limit,
            embedding_generation_batch_size,
        }
    }

    #[cfg(feature = "test-util")]
    pub fn new_for_test(store_client: Arc<dyn StoreClient>, _ctx: &mut ModelContext<Self>) -> Self {
        Self {
            codebase_indices: HashMap::new(),
            store_client,
            max_indices: None,
            max_files_repo_limit: 0,
            embedding_generation_batch_size: 100,
        }
    }

    #[cfg(feature = "local_fs")]
    pub fn clean_up_deleted_indices(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn drop_index(&mut self, _root_path: PathBuf, _ctx: &mut ModelContext<Self>) {}

    pub fn handle_active_session_changed(&mut self, _active_directory: &Path) {}

    pub fn update_max_limits(
        &mut self,
        new_max_indices: Option<usize>,
        new_max_files_per_repo: usize,
        new_embedding_generation_batch_size: usize,
        _ctx: &mut ModelContext<Self>,
    ) {
        self.max_indices = new_max_indices;
        self.max_files_repo_limit = new_max_files_per_repo;
        self.embedding_generation_batch_size = new_embedding_generation_batch_size;
    }

    pub fn can_create_new_indices(&self) -> bool {
        // Inert stub: never create new indices in the lightweight fork.
        false
    }

    pub fn handle_session_bootstrapped(&mut self, _working_directory: &Path) {}

    pub fn get_codebase_index_statuses<'a>(
        &'a self,
        _app: &'a AppContext,
    ) -> impl Iterator<Item = (&'a PathBuf, CodebaseIndexStatus)> {
        std::iter::empty()
    }

    pub fn get_codebase_index_status_for_path<'a>(
        &'a self,
        _root_path: &Path,
        _app: &'a AppContext,
    ) -> Option<CodebaseIndexStatus> {
        None
    }

    pub fn get_codebase_paths(&self) -> impl Iterator<Item = &PathBuf> {
        self.codebase_indices.keys()
    }

    pub fn num_active_indices(&self) -> usize {
        0
    }

    pub fn index_directory(&mut self, _directory: PathBuf, _ctx: &mut ModelContext<Self>) {}

    pub fn build_and_sync_codebase_index(
        &mut self,
        _build_source: BuildSource,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn reset_codebase_indexing(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn root_path_for_codebase(&self, _path: &Path) -> Option<PathBuf> {
        None
    }

    pub fn try_manual_resync_codebase(&self, _repo_path: &Path, _ctx: &mut ModelContext<Self>) {}

    pub fn retrieve_relevant_files(
        &self,
        _query: String,
        _repo_path: &Path,
        _ctx: &mut ModelContext<Self>,
    ) -> Result<RetrievalID, RetrieveFileError> {
        Err(RetrieveFileError::IndexNotFound)
    }

    pub fn abort_retrieval_request(
        &self,
        _repo_path: &Path,
        _retrieval_id: RetrievalID,
        _ctx: &mut ModelContext<Self>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    #[cfg(feature = "local_fs")]
    pub fn write_snapshot(&mut self, _working_directory: &Path, _ctx: &mut ModelContext<Self>) {}

    pub fn trigger_incremental_sync_for_path(
        &mut self,
        _directory_path: &Path,
        _ctx: &mut ModelContext<Self>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

impl Entity for CodebaseIndexManager {
    type Event = CodebaseIndexManagerEvent;
}

impl SingletonEntity for CodebaseIndexManager {}
