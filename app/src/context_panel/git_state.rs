//! Async helpers for fetching the focused repository's git status.
//!
//! Uses the existing `app::util::git::run_git_command` async wrapper, which
//! spawns `git` subprocesses with `kill_on_drop(true)`.

use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default)]
pub struct GitState {
    pub repo_path: PathBuf,
    pub branch: Option<String>,
    pub ahead: u32,
    pub behind: u32,
    pub modified: u32,
    pub untracked: u32,
    pub last_commit: Option<String>,
}

impl GitState {
    pub fn empty(repo_path: PathBuf) -> Self {
        Self {
            repo_path,
            ..Default::default()
        }
    }

    /// Run all 4 git probes; gracefully degrade when individual probes fail.
    pub async fn fetch(repo_path: &Path) -> Self {
        let mut state = Self::empty(repo_path.to_path_buf());

        if let Ok(branch) =
            crate::util::git::run_git_command(repo_path, &["symbolic-ref", "--short", "HEAD"]).await
        {
            let trimmed = branch.trim();
            if !trimmed.is_empty() {
                state.branch = Some(trimmed.to_string());
            }
        }

        if let Ok(ahead) = crate::util::git::run_git_command(
            repo_path,
            &["rev-list", "--count", "@{u}..HEAD"],
        )
        .await
        {
            state.ahead = ahead.trim().parse().unwrap_or(0);
        }
        if let Ok(behind) = crate::util::git::run_git_command(
            repo_path,
            &["rev-list", "--count", "HEAD..@{u}"],
        )
        .await
        {
            state.behind = behind.trim().parse().unwrap_or(0);
        }

        if let Ok(porcelain) =
            crate::util::git::run_git_command(repo_path, &["status", "--porcelain"]).await
        {
            for line in porcelain.lines() {
                if line.starts_with("??") {
                    state.untracked += 1;
                } else if !line.trim().is_empty() {
                    state.modified += 1;
                }
            }
        }

        if let Ok(log) =
            crate::util::git::run_git_command(repo_path, &["log", "-1", "--oneline"]).await
        {
            let trimmed = log.trim();
            if !trimmed.is_empty() {
                state.last_commit = Some(trimmed.to_string());
            }
        }

        state
    }
}
