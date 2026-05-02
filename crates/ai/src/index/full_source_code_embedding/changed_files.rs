// warp-lite: ai inert stub (v0.5 phase 2)
//
// Original: tracked added/updated/deleted file paths so the codebase index
// could apply incremental updates. The lightweight fork does no incremental
// indexing, so this is just a marker type kept for type-system compatibility.

use std::collections::HashSet;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub(super) struct ChangedFiles {
    pub(super) deletions: HashSet<PathBuf>,
    pub(super) upsertions: HashSet<PathBuf>,
}
