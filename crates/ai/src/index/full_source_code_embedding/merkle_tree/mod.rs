// warp-lite: ai inert stub (v0.5 phase 2)
//
// Original: a Merkle tree over the codebase used to drive incremental
// embedding sync (`hash`, `node`, `tree`, `serialized_tree`). The lightweight
// fork keeps only the hash types — `ContentHash` and `NodeHash` are part of
// the public surface (`crate::index::full_source_code_embedding::*`) and are
// referenced by `app/src/server/server_api/ai.rs`.

mod hash;

pub(super) use hash::MerkleHash;
pub use hash::{ContentHash, NodeHash};
