//! Foreground Process widget — Card 4 in the Context Panel stack.
//!
//! Subscribes to `WorkingDirectoriesModel::FocusedRepoChanged` to know
//! which directory to inspect, then samples
//! `context_panel::foreground_process::fetch` once per second to keep
//! the elapsed-time / CPU%% display fresh while a long-running command
//! is happening.
//!
//! See `app/src/context_panel/foreground_process.rs` for the polling
//! strategy. This widget is purposely lazy: when the panel is closed
//! the timer keeps ticking but the cost is a single `lsof` + `ps` pair
//! every two seconds, which is negligible.

use std::path::PathBuf;
use std::time::Duration;

use warpui::r#async::Timer;
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

/// How often to re-poll `lsof`/`ps` while the widget has a focused
/// directory. Every poll spawns two short-lived subprocesses, so
/// 2 seconds is the sweet spot between "responsive" and "free".
const POLL_INTERVAL: Duration = Duration::from_secs(2);

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
        // warp-lite v0.3.2: Pre-populate cwd + start polling on first render.
        let cwd = working_directories_model
            .as_ref(ctx)
            .any_focused_repo()
            .cloned();
        let mut me = Self {
            cwd,
            process: None,
            sampled: false,
            poll_generation: 0,
        };
        if me.cwd.is_some() {
            me.spawn_poll(ctx);
        }
        me
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
                self.spawn_poll(ctx);
            }
            ctx.notify();
        }
    }

    fn spawn_poll(&mut self, ctx: &mut ViewContext<Self>) {
        let Some(cwd) = self.cwd.clone() else { return };
        let generation = self.poll_generation;
        ctx.spawn(
            async move {
                let process = fg::fetch(&cwd).await.ok().flatten();
                Timer::after(POLL_INTERVAL).await;
                (generation, process)
            },
            |me, (generation, process), ctx| {
                if me.poll_generation != generation {
                    // Stale — directory changed under us.
                    return;
                }
                me.process = process;
                me.sampled = true;
                ctx.notify();
                me.spawn_poll(ctx);
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
