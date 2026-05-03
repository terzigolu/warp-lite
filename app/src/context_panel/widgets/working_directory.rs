//! Working Directory widget — Card 1 in the Context Panel stack.
//!
//! Subscribes to `WorkingDirectoriesModel::FocusedRepoChanged` to display the
//! focused tab's working directory and a coarse project-type hint.

use std::path::{Path, PathBuf};

use repo_metadata::repositories::{
    DetectedRepositories, DetectedRepositoriesEvent, RepoDetectionSource,
};
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

/// Card-style widget that shows the focused tab's working directory.
pub struct WorkingDirectoryWidget {
    working_directories_model: ModelHandle<WorkingDirectoriesModel>,
    cwd: Option<PathBuf>,
    /// warp-lite v0.3.5: cached display path (`~/foo/bar`) and project-type
    /// hint. The hint requires up to 11 filesystem stats; recomputing those
    /// on every `render()` (which fires on every workspace `notify()`,
    /// including each keystroke) was the dominant input-latency contributor.
    /// Recomputed only when `cwd` changes.
    display: String,
    project_type: Option<&'static str>,
}

impl WorkingDirectoryWidget {
    pub fn new(
        working_directories_model: ModelHandle<WorkingDirectoriesModel>,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        ctx.subscribe_to_model(&working_directories_model, Self::handle_event);
        // warp-lite v0.4: subscribe to repo discovery as a fallback signal.
        // `WorkingDirectoriesModel::focused_repo` only populates after a
        // pane-group refresh with a focused terminal id; in many real
        // sessions the user opens the panel before that fires. Repo
        // detection is the more reliable "the user is in repo X" signal.
        let detected = DetectedRepositories::handle(ctx);
        ctx.subscribe_to_model(&detected, Self::handle_repo_event);
        let cwd = working_directories_model
            .as_ref(ctx)
            .any_focused_repo()
            .cloned()
            // warp-lite v0.4: bootstrap fallback. Subscriptions only deliver
            // future events, so if the panel mounts before the first
            // FocusedRepoChanged or DetectedGitRepo event fires, fall back
            // to whatever the singleton already discovered at startup.
            .or_else(|| {
                DetectedRepositories::as_ref(ctx)
                    .detected_root_paths()
                    .next()
            });
        let (display, project_type) = Self::derive_cached(cwd.as_deref());
        Self {
            working_directories_model,
            cwd,
            display,
            project_type,
        }
    }

    fn handle_event(
        &mut self,
        model: ModelHandle<WorkingDirectoriesModel>,
        event: &WorkingDirectoriesEvent,
        ctx: &mut ViewContext<Self>,
    ) {
        if let WorkingDirectoriesEvent::FocusedRepoChanged { focused_repo, .. } = event {
            self.cwd = focused_repo
                .clone()
                .or_else(|| model.as_ref(ctx).any_focused_repo().cloned());
            let _ = self.working_directories_model.id();
            let (display, project_type) = Self::derive_cached(self.cwd.as_deref());
            self.display = display;
            self.project_type = project_type;
            ctx.notify();
        }
    }

    fn handle_repo_event(
        &mut self,
        _model: ModelHandle<DetectedRepositories>,
        event: &DetectedRepositoriesEvent,
        ctx: &mut ViewContext<Self>,
    ) {
        let DetectedRepositoriesEvent::DetectedGitRepo { repository, source } = event;
        // Only surface the repo that the user just navigated to. Other
        // detection sources (code review, project rules indexing) can fire
        // for repos the user is *not* currently in and would mislead the
        // widget.
        if !matches!(source, RepoDetectionSource::TerminalNavigation) {
            return;
        }
        let new_cwd = repository.as_ref(ctx).root_dir().to_local_path();
        if new_cwd != self.cwd {
            self.cwd = new_cwd;
            let (display, project_type) = Self::derive_cached(self.cwd.as_deref());
            self.display = display;
            self.project_type = project_type;
            ctx.notify();
        }
    }

    fn derive_cached(cwd: Option<&Path>) -> (String, Option<&'static str>) {
        match cwd {
            Some(p) => (Self::display_path(p), Self::project_type_hint(p)),
            None => ("(no active terminal)".to_string(), None),
        }
    }

    fn project_type_hint(path: &Path) -> Option<&'static str> {
        // Order matters: prefer the most specific manifest first so a
        // mixed repo (e.g. Rust with a frontend in a subdir, or Node
        // with a Cargo.toml for native bindings) gets the dominant
        // language label.
        if path.join("Cargo.toml").exists() {
            Some("Rust project")
        } else if path.join("go.mod").exists() {
            Some("Go project")
        } else if path.join("pyproject.toml").exists()
            || path.join("setup.py").exists()
            || path.join("requirements.txt").exists()
        {
            Some("Python project")
        } else if path.join("Gemfile").exists() {
            Some("Ruby project")
        } else if path.join("pubspec.yaml").exists() {
            Some("Dart project")
        } else if path.join("mix.exs").exists() {
            Some("Elixir project")
        } else if path.join("Package.swift").exists() {
            Some("Swift project")
        } else if path.join("pom.xml").exists()
            || path.join("build.gradle").exists()
            || path.join("build.gradle.kts").exists()
        {
            Some("JVM project")
        } else if path.join("package.json").exists() {
            Some("Node project")
        } else if path.join("Dockerfile").exists() {
            Some("Docker project")
        } else if path.join(".git").exists() {
            Some("Git repository")
        } else {
            None
        }
    }

    /// Renders the path with the user's home directory abbreviated to
    /// `~` so deep nested paths stay readable in a narrow side panel.
    fn display_path(path: &Path) -> String {
        if let Some(home) = std::env::var_os("HOME") {
            let home_path = PathBuf::from(home);
            if let Ok(suffix) = path.strip_prefix(&home_path) {
                if suffix.as_os_str().is_empty() {
                    return "~".to_string();
                }
                return format!("~/{}", suffix.display());
            }
        }
        path.display().to_string()
    }
}

impl Entity for WorkingDirectoryWidget {
    type Event = ();
}

impl View for WorkingDirectoryWidget {
    fn ui_name() -> &'static str {
        "ContextPanelWorkingDirectory"
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
            Icon::Folder.to_warpui_icon(text_color.into()).finish(),
        )
        .with_width(14.)
        .with_height(14.)
        .finish();

        let header = Flex::row()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_child(Container::new(icon).with_margin_right(6.).finish())
            .with_child(
                Text::new("Working Directory", font_family, font_size)
                    .with_color(text_color.into())
                    .finish(),
            )
            .finish();

        let path_text = self.display.clone();
        let project_hint = self.project_type.map(|s| s.to_string());

        let mut body = Flex::column()
            .with_cross_axis_alignment(CrossAxisAlignment::Start)
            .with_child(
                Container::new(
                    Text::new(path_text, font_family, font_size)
                        .with_color(sub_text_color.into())
                        .finish(),
                )
                .with_margin_top(4.)
                .finish(),
            );

        if let Some(hint) = project_hint {
            body = body.with_child(
                Container::new(
                    Text::new(hint, font_family, font_size)
                        .with_color(sub_text_color.into())
                        .finish(),
                )
                .with_margin_top(2.)
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
