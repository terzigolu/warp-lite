//! Async helper for sampling a "foreground process" near the focused
//! working directory.
//!
//! The Context Panel does not have direct access to a tab's controlling
//! TTY (that would require plumbing through `terminal_manager`), so this
//! module takes a coarser, dependency-free approach:
//!
//! 1. Use `lsof -F pn -d cwd` to enumerate every running process whose
//!    current working directory equals the focused repository path
//!    (or any descendant when the user is deeper in the tree).
//! 2. Feed those PIDs into `ps -o pid,pcpu,etime,command -p ...` to get
//!    the elapsed time, CPU%, and command line.
//! 3. Pick the *youngest* non-shell process — that is the most likely
//!    candidate for "what the user just kicked off in this terminal".
//!
//! This isn't tab-precise, but it gives a useful real-time indicator
//! (e.g. `cargo build`, `pnpm dev`, `python script.py`) without needing
//! a full pty bridge. When nothing matches, we fall back to "(idle)".

use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{anyhow, Result};
use command::r#async::Command;
use command::Stdio;

#[derive(Clone, Debug)]
pub struct ForegroundProcess {
    pub pid: u32,
    pub name: String,
    pub command_line: String,
    pub cpu_percent: f32,
    pub elapsed: Duration,
}

impl ForegroundProcess {
    pub fn short_command(&self) -> String {
        // Trim very long command lines for display.
        const MAX: usize = 60;
        if self.command_line.len() <= MAX {
            self.command_line.clone()
        } else {
            let mut s: String = self.command_line.chars().take(MAX - 1).collect();
            s.push('…');
            s
        }
    }

    pub fn elapsed_pretty(&self) -> String {
        let secs = self.elapsed.as_secs();
        if secs < 60 {
            format!("{}s", secs)
        } else if secs < 3600 {
            format!("{}m{}s", secs / 60, secs % 60)
        } else {
            format!("{}h{}m", secs / 3600, (secs % 3600) / 60)
        }
    }
}

/// Returns the most likely foreground process running under `cwd`, or
/// `Ok(None)` when the directory is idle. Errors propagate rather than
/// silently returning `None` so callers can distinguish "no process"
/// from "lookup failed".
#[cfg(all(unix, feature = "local_fs"))]
pub async fn fetch(cwd: &Path) -> Result<Option<ForegroundProcess>> {
    let pids = pids_with_cwd(cwd).await?;
    if pids.is_empty() {
        return Ok(None);
    }
    let processes = ps_for_pids(&pids).await?;
    Ok(pick_best(processes))
}

#[cfg(not(all(unix, feature = "local_fs")))]
pub async fn fetch(_cwd: &Path) -> Result<Option<ForegroundProcess>> {
    Ok(None)
}

/// Use `lsof -F pn -d cwd` to find every process whose cwd is, or is
/// inside, `cwd`. `lsof` reports ancestors of the focused dir too, so
/// we filter strictly to descendants.
#[cfg(all(unix, feature = "local_fs"))]
async fn pids_with_cwd(cwd: &Path) -> Result<Vec<u32>> {
    let output = Command::new("/usr/sbin/lsof")
        .args(["-F", "pn", "-d", "cwd"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .output()
        .await
        .map_err(|e| anyhow!("Failed to run lsof: {e}"))?;
    // lsof returns non-zero when some FDs are denied; we still get useful
    // output on stdout, so we ignore the status.
    let stdout = String::from_utf8_lossy(&output.stdout);

    let cwd_str = cwd.to_string_lossy();
    let mut pids = Vec::new();
    let mut current_pid: Option<u32> = None;
    for line in stdout.lines() {
        if let Some(rest) = line.strip_prefix('p') {
            current_pid = rest.parse::<u32>().ok();
        } else if let Some(rest) = line.strip_prefix('n') {
            if let Some(pid) = current_pid {
                if path_is_within(rest, &cwd_str) {
                    pids.push(pid);
                }
            }
            current_pid = None;
        }
    }
    pids.sort_unstable();
    pids.dedup();
    Ok(pids)
}

/// True when `candidate` equals or is a descendant of `parent`. Both
/// strings are compared as raw paths (no canonicalization, no symlink
/// resolution) — a deliberate trade-off to keep the hot path cheap.
fn path_is_within(candidate: &str, parent: &str) -> bool {
    if candidate == parent {
        return true;
    }
    if !candidate.starts_with(parent) {
        return false;
    }
    // candidate must continue with '/' for it to actually be inside.
    candidate.as_bytes().get(parent.len()) == Some(&b'/')
}

#[cfg(all(unix, feature = "local_fs"))]
async fn ps_for_pids(pids: &[u32]) -> Result<Vec<ForegroundProcess>> {
    if pids.is_empty() {
        return Ok(Vec::new());
    }
    let pid_arg = pids
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let output = Command::new("/bin/ps")
        .args(["-o", "pid=,pcpu=,etime=,command=", "-p", &pid_arg])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .output()
        .await
        .map_err(|e| anyhow!("Failed to run ps: {e}"))?;
    if !output.status.success() {
        return Ok(Vec::new());
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut out = Vec::new();
    for line in stdout.lines() {
        if let Some(proc_) = parse_ps_line(line) {
            out.push(proc_);
        }
    }
    Ok(out)
}

fn parse_ps_line(line: &str) -> Option<ForegroundProcess> {
    let trimmed = line.trim_start();
    let mut iter = trimmed.split_whitespace();
    let pid: u32 = iter.next()?.parse().ok()?;
    let pcpu: f32 = iter.next()?.parse().ok()?;
    let etime = iter.next()?;
    let elapsed = parse_etime(etime)?;
    // The remainder is the command line.
    let rest_start = {
        // Find offset of command in original line by skipping the first
        // three whitespace-separated tokens we already consumed.
        let mut offset = 0;
        let bytes = trimmed.as_bytes();
        for _ in 0..3 {
            // skip whitespace
            while offset < bytes.len() && bytes[offset].is_ascii_whitespace() {
                offset += 1;
            }
            // skip token
            while offset < bytes.len() && !bytes[offset].is_ascii_whitespace() {
                offset += 1;
            }
        }
        while offset < bytes.len() && bytes[offset].is_ascii_whitespace() {
            offset += 1;
        }
        offset
    };
    let command_line = trimmed.get(rest_start..)?.to_string();
    let name = command_line
        .split_whitespace()
        .next()
        .and_then(|first| {
            std::path::Path::new(first)
                .file_name()
                .and_then(|s| s.to_str())
                .map(ToString::to_string)
        })
        .unwrap_or_else(|| command_line.clone());
    Some(ForegroundProcess {
        pid,
        name,
        command_line,
        cpu_percent: pcpu,
        elapsed,
    })
}

/// `ps` etime formats: `MM:SS`, `HH:MM:SS`, or `DD-HH:MM:SS`.
fn parse_etime(s: &str) -> Option<Duration> {
    let (days, rest) = match s.split_once('-') {
        Some((d, r)) => (d.parse::<u64>().ok()?, r),
        None => (0u64, s),
    };
    let parts: Vec<&str> = rest.split(':').collect();
    let (h, m, sec) = match parts.as_slice() {
        [h, m, s] => (h.parse::<u64>().ok()?, m.parse::<u64>().ok()?, s.parse::<u64>().ok()?),
        [m, s] => (0u64, m.parse::<u64>().ok()?, s.parse::<u64>().ok()?),
        _ => return None,
    };
    Some(Duration::from_secs(days * 86_400 + h * 3_600 + m * 60 + sec))
}

/// Picks the youngest non-shell process. We deprioritize obvious shell /
/// daemon names so e.g. the user's `zsh` doesn't drown out their
/// `cargo build`.
fn pick_best(mut procs: Vec<ForegroundProcess>) -> Option<ForegroundProcess> {
    procs.retain(|p| !is_uninteresting(&p.name));
    procs.sort_by_key(|p| p.elapsed);
    procs.into_iter().next()
}

fn is_uninteresting(name: &str) -> bool {
    matches!(
        name,
        "zsh"
            | "bash"
            | "fish"
            | "dash"
            | "sh"
            | "tmux"
            | "screen"
            | "ssh-agent"
            | "warp-oss"
            | "warp"
            | "Code Helper"
    ) || name.starts_with("login")
        || name.starts_with("sshd")
        || name.starts_with("warp")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_etime() {
        assert_eq!(parse_etime("12"), None); // missing minutes
        assert_eq!(parse_etime("00:42").unwrap().as_secs(), 42);
        assert_eq!(parse_etime("01:02:03").unwrap().as_secs(), 3723);
        assert_eq!(parse_etime("2-01:02:03").unwrap().as_secs(), 2 * 86_400 + 3723);
    }

    #[test]
    fn parses_ps_line() {
        let line = " 12345  4.2  03:14 cargo build --release";
        let p = parse_ps_line(line).expect("parse");
        assert_eq!(p.pid, 12345);
        assert_eq!(p.name, "cargo");
        assert!((p.cpu_percent - 4.2).abs() < 0.001);
        assert_eq!(p.command_line, "cargo build --release");
        assert_eq!(p.elapsed.as_secs(), 194);
    }

    #[test]
    fn path_within_strict() {
        assert!(path_is_within("/foo/bar", "/foo/bar"));
        assert!(path_is_within("/foo/bar/baz", "/foo/bar"));
        assert!(!path_is_within("/foo/barbaz", "/foo/bar"));
        assert!(!path_is_within("/other", "/foo/bar"));
    }

    #[test]
    fn skips_shell_processes() {
        let procs = vec![
            ForegroundProcess {
                pid: 1,
                name: "zsh".into(),
                command_line: "-zsh".into(),
                cpu_percent: 0.0,
                elapsed: Duration::from_secs(10),
            },
            ForegroundProcess {
                pid: 2,
                name: "cargo".into(),
                command_line: "cargo build".into(),
                cpu_percent: 50.0,
                elapsed: Duration::from_secs(20),
            },
        ];
        let best = pick_best(procs).unwrap();
        assert_eq!(best.pid, 2);
    }
}
