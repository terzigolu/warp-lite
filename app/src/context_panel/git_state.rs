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
    pub stashes: u32,
    pub last_commit: Option<String>,
    pub remote_url: Option<String>,
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

        if let Ok(stash) =
            crate::util::git::run_git_command(repo_path, &["stash", "list"]).await
        {
            state.stashes = stash.lines().filter(|l| !l.trim().is_empty()).count() as u32;
        }

        if let Ok(url) =
            crate::util::git::run_git_command(repo_path, &["config", "--get", "remote.origin.url"])
                .await
        {
            let trimmed = url.trim();
            if !trimmed.is_empty() {
                state.remote_url = Some(trimmed.to_string());
            }
        }

        state
    }

    /// Best-effort prettifier for a remote URL: keeps `owner/repo` for
    /// GitHub-style remotes (https or ssh), otherwise returns the raw
    /// URL.
    pub fn pretty_remote(&self) -> Option<String> {
        let raw = self.remote_url.as_deref()?;
        // git@host:owner/repo(.git)?
        if let Some((_host, path)) = raw.split_once(':') {
            if raw.starts_with("git@") {
                return Some(strip_dot_git(path).to_string());
            }
        }
        // https://host/owner/repo(.git)?
        if let Some(rest) = raw.strip_prefix("https://").or_else(|| raw.strip_prefix("http://")) {
            if let Some(idx) = rest.find('/') {
                return Some(strip_dot_git(&rest[idx + 1..]).to_string());
            }
        }
        Some(raw.to_string())
    }
}

fn strip_dot_git(s: &str) -> &str {
    s.strip_suffix(".git").unwrap_or(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pretty_remote_handles_github_https() {
        let s = GitState {
            remote_url: Some("https://github.com/foo/bar.git".to_string()),
            ..Default::default()
        };
        assert_eq!(s.pretty_remote().as_deref(), Some("foo/bar"));
    }

    #[test]
    fn pretty_remote_handles_github_ssh() {
        let s = GitState {
            remote_url: Some("git@github.com:foo/bar.git".to_string()),
            ..Default::default()
        };
        assert_eq!(s.pretty_remote().as_deref(), Some("foo/bar"));
    }

    #[test]
    fn pretty_remote_handles_unknown() {
        let s = GitState {
            remote_url: Some("file:///tmp/repo".to_string()),
            ..Default::default()
        };
        assert_eq!(
            s.pretty_remote().as_deref(),
            Some("file:///tmp/repo")
        );
    }
}
