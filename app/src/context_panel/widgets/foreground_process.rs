//! Foreground Process widget — Card 4 in the Context Panel stack.
//!
//! warp-lite v0.3.4: was a 2-second `lsof`+`ps` poll loop spawned at
//! widget construction. The loop ran for the lifetime of the
//! workspace (panel-open state was never consulted) which contributed
//! to a perceived input-latency stutter. Now: fetch once per
//! `FocusedRepoChanged` event only — no recurring timer.

use std::path::PathBuf;

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
    context_panel::foreground_process::{self as fg, ForegroundProcess},
    pane_group::{WorkingDirectoriesEvent, WorkingDirectoriesModel},
    ui_components::icons::Icon,
};

pub struct ForegroundProcessWidget {
    cwd: Option<PathBuf>,
    process: Option<ForegroundProcess>,
    /// Set after the first fetch returns, so we can tell the difference
    /// between "haven't sampled yet" and "sampled, found nothing".
    sampled: bool,
    /// Monotonically increasing token bumped on every cwd change. The
    /// in-flight async task carries its own copy and bails if it doesn't
    /// match — that way stale results from a previous directory can't
    /// overwrite a fresh state.
    poll_generation: u64,
}

impl ForegroundProcessWidget {
    pub fn new(
        working_directories_model: ModelHandle<WorkingDirectoriesModel>,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        ctx.subscribe_to_model(&working_directories_model, Self::handle_event);
        let detected = DetectedRepositories::handle(ctx);
        ctx.subscribe_to_model(&detected, Self::handle_repo_event);
        let cwd = working_directories_model
            .as_ref(ctx)
            .any_focused_repo()
            .cloned()
            .or_else(|| {
                DetectedRepositories::as_ref(ctx)
                    .detected_root_paths()
                    .next()
            });
        let mut me = Self {
            cwd,
            process: None,
            sampled: false,
            poll_generation: 0,
        };
        if me.cwd.is_some() {
            me.spawn_fetch(ctx);
        }
        me
    }

    fn handle_repo_event(
        &mut self,
        _model: ModelHandle<DetectedRepositories>,
        event: &DetectedRepositoriesEvent,
        ctx: &mut ViewContext<Self>,
    ) {
        let DetectedRepositoriesEvent::DetectedGitRepo { repository, source } = event;
        if !matches!(source, RepoDetectionSource::TerminalNavigation) {
            return;
        }
        let new_cwd = repository.as_ref(ctx).root_dir().to_local_path();
        if new_cwd == self.cwd {
            return;
        }
        self.cwd = new_cwd;
        self.process = None;
        self.sampled = false;
        self.poll_generation = self.poll_generation.wrapping_add(1);
        if self.cwd.is_some() {
            self.spawn_fetch(ctx);
        }
        ctx.notify();
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
            self.process = None;
            self.sampled = false;
            self.poll_generation = self.poll_generation.wrapping_add(1);
            if self.cwd.is_some() {
                self.spawn_fetch(ctx);
            }
            ctx.notify();
        }
    }

    /// One-shot fetch (no timer). Re-runs only on the next
    /// `FocusedRepoChanged` event so the widget can never become a
    /// background loop.
    fn spawn_fetch(&mut self, ctx: &mut ViewContext<Self>) {
        let Some(cwd) = self.cwd.clone() else { return };
        let generation = self.poll_generation;
        ctx.spawn(
            async move {
                let process = fg::fetch(&cwd).await.ok().flatten();
                (generation, process)
            },
            |me, (generation, process), ctx| {
                if me.poll_generation != generation {
                    return;
                }
                me.process = process;
                me.sampled = true;
                ctx.notify();
            },
        );
    }
}

impl Entity for ForegroundProcessWidget {
    type Event = ();
}

impl View for ForegroundProcessWidget {
    fn ui_name() -> &'static str {
        "ContextPanelForegroundProcess"
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
            Icon::Terminal.to_warpui_icon(text_color.into()).finish(),
        )
        .with_width(14.)
        .with_height(14.)
        .finish();

        let header = Flex::row()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_child(Container::new(icon).with_margin_right(6.).finish())
            .with_child(
                Text::new("Running Process", font_family, font_size)
                    .with_color(text_color.into())
                    .finish(),
            )
            .finish();

        let lines: Vec<String> = if self.cwd.is_none() {
            vec!["(no active terminal)".to_string()]
        } else if let Some(p) = &self.process {
            vec![
                p.short_command(),
                format!("{} • {:.0}% CPU • PID {}", p.elapsed_pretty(), p.cpu_percent, p.pid),
            ]
        } else if self.sampled {
            vec!["No running process".to_string()]
        } else {
            vec!["Sampling…".to_string()]
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
