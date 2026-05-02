// warp-lite: ai inert stub (v0.5 phase 2)
//
// The original `sync_client.rs` (~600 LOC) drove the actual server-sync
// pipeline for the codebase index — `GenerateEmbeddingsTask`,
// `UpdateIntermediateNodesTask`, `SyncMerkleTreeTask`, batched flushes, retry
// queues, and so on. None of that runs in the lightweight fork. We keep just
// the public types that other modules / external consumers re-export.

use std::ops::AddAssign;

use anyhow::anyhow;
use warp_core::sync_queue::{IsTransientError, SyncQueueTaskTrait};

use super::{store_client::IntermediateNode, Error, NodeHash};

#[derive(Debug, Clone, Default)]
pub struct FlushFragmentResult {
    pub fragment_count: usize,
    pub total_fragment_size_bytes: usize,
}

impl AddAssign for FlushFragmentResult {
    fn add_assign(&mut self, rhs: Self) {
        self.fragment_count += rhs.fragment_count;
        self.total_fragment_size_bytes += rhs.total_fragment_size_bytes;
    }
}

#[allow(dead_code)]
pub struct GenerateEmbeddingsTask {
    _private: (),
}

#[allow(dead_code)]
pub struct UpdateIntermediateNodesTask {
    nodes: Vec<IntermediateNode>,
}

#[allow(dead_code)]
pub struct SyncMerkleTreeTask {
    nodes: Vec<NodeHash>,
}

pub enum SyncTask {
    GenerateEmbeddings(GenerateEmbeddingsTask),
    UpdateIntermediateNodes(UpdateIntermediateNodesTask),
    SyncMerkleTree(SyncMerkleTreeTask),
}

pub enum SyncQueueResult {
    /// Sync completed successfully.
    Success,
    /// Sync was aborted before completion.
    Aborted,
    /// Sync failed with an error string.
    Failed(String),
}

impl IsTransientError for Error {
    fn is_transient(&self) -> bool {
        false
    }
}

impl SyncQueueTaskTrait for SyncTask {
    type Error = Error;
    type Result = ();
    type Fut = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Result, Self::Error>> + Send>,
    >;

    fn run(&mut self) -> Self::Fut {
        Box::pin(async {
            // warp-lite stub: sync queue is wired up but never executes anything.
            Err(Error::Other(anyhow!("warp-lite: ai inert stub")))
        })
    }
}
