// warp-lite: ai inert stub (v0.5 phase 2)
//
// The original `snapshot.rs` (~230 LOC) handled snapshot persistence for the
// codebase index — splitting valid/invalid metadata, cleaning up stale files,
// reading snapshots, and migrating between state directories. The lightweight
// fork ships no codebase index, so all helpers are inert no-ops. The shape
// (function names, signatures, return types) is preserved so `manager.rs` can
// keep referring to them via `super::snapshot::*`.

use std::{
    collections::HashSet,
    hash::{DefaultHasher, Hash, Hasher},
    path::{Path, PathBuf},
};

use crate::workspace::WorkspaceMetadata;

/// Number of days after which an index snapshot would have been considered
/// expired in upstream Warp. Kept as a constant so any consumer that imports
/// it still type-checks; not actually consulted.
pub(super) const REPO_SNAPSHOT_SHELF_LIFE_DAYS: u64 = 30;

#[allow(dead_code)]
const REPO_SNAPSHOT_SUBDIR_NAME: &str = "codebase_index_snapshots";

/// Inert: returns `(invalid, valid)` where `invalid` is empty and `valid` is
/// the original input. The lightweight fork has no codebase index lifecycle.
pub(super) fn split_snapshot_metadata_by_validity(
    persisted_codebase_indices: Vec<WorkspaceMetadata>,
) -> (Vec<WorkspaceMetadata>, Vec<WorkspaceMetadata>) {
    (Vec::new(), persisted_codebase_indices)
}

/// Inert: no snapshot files to clean up.
pub(super) fn clean_up_snapshot_files(
    _snapshot_file_dir: &Path,
    _persisted_codebase_indices: &[WorkspaceMetadata],
) {
    let _ = HashSet::<PathBuf>::new();
}

/// Inert: snapshots are never present in the lightweight fork.
pub(super) fn has_snapshot(_repo_path: &Path) -> bool {
    false
}

/// Inert: no snapshot directory is created.
pub(super) fn snapshot_dir() -> Option<PathBuf> {
    None
}

/// Build a deterministic snapshot path purely so callers that derive a path
/// (e.g. for logging) still get a sensible value.
pub(super) fn snapshot_path(snapshot_dir: &Path, repo_path: &Path) -> PathBuf {
    let mut hasher = DefaultHasher::new();
    repo_path.hash(&mut hasher);
    snapshot_dir.join(format!("snapshot_{}", hasher.finish()))
}

#[cfg(feature = "local_fs")]
pub(super) fn migrate_snapshots_to_secure_dir_if_needed() -> anyhow::Result<()> {
    Ok(())
}
