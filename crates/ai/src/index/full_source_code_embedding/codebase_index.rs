// warp-lite: ai inert stub (v0.5 phase 2)
//
// The original `codebase_index.rs` (~2378 LOC) drove embedding-based codebase
// indexing — Merkle tree construction, fragment chunking, server sync, and
// retrieval. The lightweight fork keeps every symbol the rest of the workspace
// imports, but every method is a no-op that returns a default / error value.

use std::{
    collections::HashSet,
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use chrono::{DateTime, Utc};
use warpui::Entity;

use super::Error;
use crate::{
    index::locations::CodeContextLocation,
    workspace::{WorkspaceMetadata, WorkspaceMetadataEvent},
};

#[derive(Debug, Copy, Clone)]
pub enum SyncProgress {
    /// We're in the process of discovering how many nodes we need to sync.
    Discovering { total_nodes: usize },

    /// We're syncing the nodes to the server.
    Syncing {
        completed_nodes: usize,
        total_nodes: usize,
    },
}

#[derive(Default)]
pub(crate) struct CodebaseIndexTimeStampMetadata {
    #[allow(dead_code)]
    last_edited: Option<DateTime<Utc>>,
    #[allow(dead_code)]
    last_snapshot: Option<DateTime<Utc>>,
    #[allow(dead_code)]
    earliest_unsynced_change: Option<DateTime<Utc>>,
}

impl CodebaseIndexTimeStampMetadata {
    pub fn from_metadata(_metadata: WorkspaceMetadata) -> Self {
        Self::default()
    }
}

/// Inert stub. The lightweight fork has no embedding-based codebase index, so
/// this struct exists only so that `CodebaseIndexManager` and downstream APIs
/// keep their original shape. None of the fields are populated meaningfully.
pub struct CodebaseIndex {
    _private: (),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RetrievalID(usize);

impl RetrievalID {
    #[allow(dead_code)]
    fn new() -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        RetrievalID(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub enum CodebaseIndexEvent {
    RetrievalRequestCompleted {
        retrieval_id: RetrievalID,
        fragments: Arc<HashSet<CodeContextLocation>>,
        out_of_sync_delay: Option<Duration>,
    },
    RetrievalRequestFailed {
        retrieval_id: RetrievalID,
        error: Error,
    },
    SyncStateUpdated,
    IndexMetadataUpdated {
        root_path: PathBuf,
        event: WorkspaceMetadataEvent,
    },
}

#[derive(Debug)]
pub struct ReadFragmentResult {
    pub content: String,
}

impl Entity for CodebaseIndex {
    type Event = CodebaseIndexEvent;
}
