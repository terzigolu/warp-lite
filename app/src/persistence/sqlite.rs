// warp-lite: AI/cloud-coupled SQLite persistence stripped down to a no-op stub.
// This file used to contain ~3700 lines of cloud_object/AgentConversation/AI table
// persistence. It is replaced with a minimal stub that satisfies the public
// surface used by `super::mod` and a few external callers.

#![allow(dead_code, unused_imports, unused_variables)]

use std::path::PathBuf;
use std::sync::mpsc::SyncSender;

use anyhow::Result;
use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use warpui::AppContext;

use super::{ModelEvent, PersistedData, WriterHandles};

const WARP_SQLITE_FILE_NAME: &str = "warp.sqlite";

/// Returns the on-disk location of the warp-lite sqlite database.
pub fn database_file_path() -> PathBuf {
    warp_core::paths::state_dir().join(WARP_SQLITE_FILE_NAME)
}

/// Initialize the persistence subsystem (no-op in warp-lite).
pub fn initialize(_ctx: &mut AppContext) -> (Option<PersistedData>, Option<WriterHandles>) {
    (None, None)
}

/// Open a read-only connection to the sqlite file (best effort; warp-lite no
/// longer relies on this for any feature, but we keep it for compatibility).
pub fn establish_ro_connection(database_url: &str) -> Result<SqliteConnection> {
    let full = format!("file:{database_url}?mode=ro");
    Ok(SqliteConnection::establish(&full)?)
}

/// Stub used by logout flows; warp-lite has no cloud-coupled state to wipe.
pub(super) fn remove(_sender: SyncSender<ModelEvent>) {}

/// Stub used by logout flows; warp-lite has no cloud-coupled state to restore.
pub(super) fn reconstruct(_sender: SyncSender<ModelEvent>) {}

/// Stub for the historical `init_db` helper.
pub(super) fn init_db() -> Result<SqliteConnection> {
    Ok(SqliteConnection::establish(":memory:")?)
}
