//! Claude Code session widget — Card 3 in the Context Panel stack.
//!
//! Looks for active Claude Code project sessions in `~/.claude/projects/<encoded-cwd>/`.
//! `cwd` is encoded by replacing `/` with `-` (the Claude Code convention).

use std::path::{Path, PathBuf};
use std::time::SystemTime;

use warpui::{
    elements::{
        ConstrainedBox, Container, CrossAxisAlignment, Element, Flex, ParentElement, Text,
    },
    AppContext, Entity, ModelHandle, SingletonEntity, View, ViewContext,
};

use crate::{
    appearance::Appearance,
    pane_group::{WorkingDirectoriesEvent, WorkingDirectoriesModel},
    ui_components::icons::Icon,
};

#[derive(Clone, Debug, Default)]
struct SessionsSummary {
    count: usize,
    last_modified: Option<SystemTime>,
    /// Approximate turn count of the most recently modified session.
    /// `None` when the file couldn't be read or there is no recent
    /// session.
    last_turn_count: Option<usize>,
}

pub struct ClaudeCodeWidget {
    cwd: Option<PathBuf>,
    summary: SessionsSummary,
}

impl ClaudeCodeWidget {
    pub fn new(
        working_directories_model: ModelHandle<WorkingDirectoriesModel>,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        ctx.subscribe_to_model(&working_directories_model, Self::handle_event);
        // warp-lite v0.3.2: Pre-populate cwd + scan sessions on first render.
        let cwd = working_directories_model
            .as_ref(ctx)
            .any_focused_repo()
            .cloned();
        let summary = cwd.as_deref().map(scan_sessions_for_cwd).unwrap_or_default();
        Self { cwd, summary }
    }

    fn handle_event(
        &mut self,
        _model: ModelHandle<WorkingDirectoriesModel>,
        event: &WorkingDirectoriesEvent,
        ctx: &mut ViewContext<Self>,
    ) {
        if let WorkingDirectoriesEvent::FocusedRepoChanged { focused_repo, .. } = event {
            if focused_repo == &self.cwd {
                return;
            }
            self.cwd = focused_repo.clone();
            self.summary = self
                .cwd
                .as_deref()
                .map(scan_sessions_for_cwd)
                .unwrap_or_default();
            ctx.notify();
        }
    }
}

impl Entity for ClaudeCodeWidget {
    type Event = ();
}

impl View for ClaudeCodeWidget {
    fn ui_name() -> &'static str {
        "ContextPanelClaudeCode"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);
        let theme = appearance.theme();
        let background = theme.background();
        let text_color = theme.main_text_color(background);
        let sub_text_color = theme.sub_text_color(text_color);
        let font_family = appearance.ui_font_family();
        let font_size = appearance.ui_font_size();

        let icon = ConstrainedBox::new(
            Icon::Tool.to_warpui_icon(text_color.into()).finish(),
        )
        .with_width(14.)
        .with_height(14.)
        .finish();

        let header = Flex::row()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_child(Container::new(icon).with_margin_right(6.).finish())
            .with_child(
                Text::new("Claude Code", font_family, font_size)
                    .with_color(text_color.into())
                    .finish(),
            )
            .finish();

        let lines: Vec<String> = if self.cwd.is_none() {
            vec!["(no active terminal)".to_string()]
        } else if self.summary.count == 0 {
            vec!["No active sessions".to_string()]
        } else {
            let suffix = relative_time(self.summary.last_modified);
            let mut out = vec![format!("{} session(s){}", self.summary.count, suffix)];
            if let Some(turns) = self.summary.last_turn_count {
                if turns > 0 {
                    out.push(format!(
                        "Latest: ~{} turn{}",
                        turns,
                        if turns == 1 { "" } else { "s" }
                    ));
                }
            }
            out
        };

        let mut body = Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Start);
        for (i, line) in lines.iter().enumerate() {
            body = body.with_child(
                Container::new(
                    Text::new(line.clone(), font_family, font_size)
                        .with_color(sub_text_color.into())
                        .finish(),
                )
                .with_margin_top(if i == 0 { 4. } else { 2. })
                .finish(),
            );
        }

        Container::new(
            Flex::column()
                .with_child(header)
                .with_child(body.finish())
                .finish(),
        )
        .with_horizontal_padding(12.)
        .with_vertical_padding(10.)
        .finish()
    }
}

fn relative_time(stamp: Option<SystemTime>) -> String {
    let Some(stamp) = stamp else { return String::new() };
    let now = SystemTime::now();
    let elapsed = now
        .duration_since(stamp)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let pretty = if elapsed < 60 {
        format!("{}s ago", elapsed)
    } else if elapsed < 3600 {
        format!("{}m ago", elapsed / 60)
    } else if elapsed < 86_400 {
        format!("{}h ago", elapsed / 3600)
    } else {
        format!("{}d ago", elapsed / 86_400)
    };
    format!(" • last {}", pretty)
}

/// Scans `~/.claude/projects/<encoded-cwd>/` and returns the count and most
/// recent modification timestamp of any JSONL session files.
fn scan_sessions_for_cwd(cwd: &Path) -> SessionsSummary {
    let Some(home) = home_dir() else {
        return SessionsSummary::default();
    };
    let encoded = encode_path(cwd);
    let dir = home.join(".claude").join("projects").join(&encoded);
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return SessionsSummary::default();
    };

    let mut count = 0usize;
    let mut newest: Option<SystemTime> = None;
    let mut newest_path: Option<PathBuf> = None;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("jsonl") {
            continue;
        }
        count += 1;
        if let Ok(meta) = entry.metadata() {
            if let Ok(modified) = meta.modified() {
                let bumped = match newest {
                    Some(prev) if prev > modified => false,
                    _ => true,
                };
                if bumped {
                    newest = Some(modified);
                    newest_path = Some(path.clone());
                }
            }
        }
    }

    let last_turn_count = newest_path
        .as_deref()
        .and_then(approximate_turn_count);

    SessionsSummary {
        count,
        last_modified: newest,
        last_turn_count,
    }
}

/// Cheap, allocation-free approximation: count newlines in the JSONL
/// file. Bounded by `MAX_BYTES` so a runaway transcript can't stall
/// the UI thread on the synchronous read.
fn approximate_turn_count(path: &Path) -> Option<usize> {
    const MAX_BYTES: u64 = 4 * 1024 * 1024;
    let meta = std::fs::metadata(path).ok()?;
    if !meta.is_file() {
        return None;
    }
    if meta.len() > MAX_BYTES {
        // Don't try to count for huge transcripts; the widget shows
        // session count regardless, so this is an information-only
        // skip rather than a failure.
        return None;
    }
    let bytes = std::fs::read(path).ok()?;
    Some(bytes.iter().filter(|b| **b == b'\n').count())
}

fn encode_path(path: &Path) -> String {
    // Claude Code uses '-' as the path separator after stripping the leading '/'.
    let display = path.to_string_lossy();
    let trimmed = display.trim_start_matches('/');
    let mut encoded = String::with_capacity(trimmed.len() + 1);
    encoded.push('-');
    for ch in trimmed.chars() {
        encoded.push(if ch == '/' { '-' } else { ch });
    }
    encoded
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}
