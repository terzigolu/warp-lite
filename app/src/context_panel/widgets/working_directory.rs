//! Working Directory widget — Card 1 in the Context Panel stack.
//!
//! Subscribes to `WorkingDirectoriesModel::FocusedRepoChanged` to display the
//! focused tab's working directory and a coarse project-type hint.

use std::path::{Path, PathBuf};

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
    cwd: Option<PathBuf>,
}

impl WorkingDirectoryWidget {
    pub fn new(
        working_directories_model: ModelHandle<WorkingDirectoriesModel>,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        ctx.subscribe_to_model(&working_directories_model, Self::handle_event);
        Self { cwd: None }
    }

    fn handle_event(
        &mut self,
        _model: ModelHandle<WorkingDirectoriesModel>,
        event: &WorkingDirectoriesEvent,
        ctx: &mut ViewContext<Self>,
    ) {
        if let WorkingDirectoriesEvent::FocusedRepoChanged { focused_repo, .. } = event {
            self.cwd = focused_repo.clone();
            ctx.notify();
        }
    }

    fn project_type_hint(path: &Path) -> Option<&'static str> {
        if path.join("Cargo.toml").exists() {
            Some("Rust project")
        } else if path.join("package.json").exists() {
            Some("Node project")
        } else if path.join("pyproject.toml").exists() || path.join("setup.py").exists() {
            Some("Python project")
        } else if path.join("go.mod").exists() {
            Some("Go project")
        } else if path.join(".git").exists() {
            Some("Git repository")
        } else {
            None
        }
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

        let path_text = match &self.cwd {
            Some(p) => p.display().to_string(),
            None => "(no active terminal)".to_string(),
        };

        let project_hint = self
            .cwd
            .as_deref()
            .and_then(Self::project_type_hint)
            .map(|s| s.to_string());

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
