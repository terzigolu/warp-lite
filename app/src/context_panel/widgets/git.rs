//! Git widget — Card 2 in the Context Panel stack.
//!
//! Shows the current branch, ahead/behind counts vs upstream, working-tree
//! modified/untracked counts, and the last commit subject for the focused
//! repository. Re-fetches asynchronously on `FocusedRepoChanged` events.

use std::path::PathBuf;

use warpui::{
    elements::{
        ConstrainedBox, Container, CrossAxisAlignment, Element, Flex, ParentElement, Text,
    },
    AppContext, Entity, ModelHandle, SingletonEntity, View, ViewContext,
};

use crate::{
    appearance::Appearance,
    context_panel::git_state::GitState,
    pane_group::{WorkingDirectoriesEvent, WorkingDirectoriesModel},
    ui_components::icons::Icon,
};

pub struct GitWidget {
    repo: Option<PathBuf>,
    state: Option<GitState>,
    loading: bool,
}

impl GitWidget {
    pub fn new(
        working_directories_model: ModelHandle<WorkingDirectoriesModel>,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        ctx.subscribe_to_model(&working_directories_model, Self::handle_event);
        // warp-lite v0.3.2: Pre-populate from current model state and kick off
        // the initial fetch so the widget shows real data the first time the
        // panel opens (subscriptions only fire on change).
        let initial_repo = working_directories_model
            .as_ref(ctx)
            .any_focused_repo()
            .cloned();
        let mut me = Self {
            repo: initial_repo.clone(),
            state: None,
            loading: false,
        };
        if let Some(repo) = initial_repo {
            me.kick_off_fetch(repo, ctx);
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
            if focused_repo == &self.repo {
                return;
            }
            self.repo = focused_repo.clone();
            self.state = None;
            self.loading = false;
            if let Some(repo) = self.repo.clone() {
                self.kick_off_fetch(repo, ctx);
            }
            ctx.notify();
        }
    }

    fn kick_off_fetch(&mut self, repo: PathBuf, ctx: &mut ViewContext<Self>) {
        self.loading = true;
        ctx.spawn(
            async move { GitState::fetch(&repo).await },
            |me, state, ctx| {
                if me.repo.as_deref() == Some(state.repo_path.as_path()) {
                    me.state = Some(state);
                    me.loading = false;
                    ctx.notify();
                }
            },
        );
    }
}

impl Entity for GitWidget {
    type Event = ();
}

impl View for GitWidget {
    fn ui_name() -> &'static str {
        "ContextPanelGit"
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
            Icon::GitBranch.to_warpui_icon(text_color.into()).finish(),
        )
        .with_width(14.)
        .with_height(14.)
        .finish();

        let header = Flex::row()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_child(Container::new(icon).with_margin_right(6.).finish())
            .with_child(
                Text::new("Git", font_family, font_size)
                    .with_color(text_color.into())
                    .finish(),
            )
            .finish();

        let lines: Vec<String> = if self.repo.is_none() {
            vec!["(not a git repository)".to_string()]
        } else if self.loading && self.state.is_none() {
            vec!["Loading…".to_string()]
        } else if let Some(state) = &self.state {
            let mut out = Vec::new();
            if let Some(branch) = &state.branch {
                out.push(format!("{} • {}↑ {}↓", branch, state.ahead, state.behind));
            }
            if let Some(remote) = state.pretty_remote() {
                out.push(remote);
            }
            if state.modified > 0 || state.untracked > 0 {
                out.push(format!(
                    "{} modified, {} untracked",
                    state.modified, state.untracked
                ));
            } else {
                out.push("Working tree clean".to_string());
            }
            if state.stashes > 0 {
                out.push(format!("{} stash entr{}", state.stashes,
                    if state.stashes == 1 { "y" } else { "ies" }));
            }
            if state.ahead > 0 {
                out.push(format!("git push to send {} commit{}", state.ahead,
                    if state.ahead == 1 { "" } else { "s" }));
            }
            if let Some(commit) = &state.last_commit {
                out.push(format!("Last: {}", commit));
            }
            out
        } else {
            vec!["(no git data)".to_string()]
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
