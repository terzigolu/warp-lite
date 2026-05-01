//! Context Panel view — host for the widget stack.
//!
//! v0.3 MVP: a static column of cards rendered as nested ChildViews. Resizing
//! and collapse-state are deferred; the panel inherits the same right-side
//! slot used by the resource center / AI panel.

use warpui::{
    elements::{
        Container, CrossAxisAlignment, Element, Flex, MainAxisSize, ParentElement, Text,
    },
    presenter::ChildView,
    AppContext, Entity, ModelHandle, SingletonEntity, View, ViewContext, ViewHandle,
};

use crate::{appearance::Appearance, pane_group::WorkingDirectoriesModel};

use super::widgets::{
    claude_code::ClaudeCodeWidget, foreground_process::ForegroundProcessWidget, git::GitWidget,
    working_directory::WorkingDirectoryWidget,
};

pub enum ContextPanelEvent {
    ClosePanel,
}

pub struct ContextPanelView {
    working_directory: ViewHandle<WorkingDirectoryWidget>,
    git: ViewHandle<GitWidget>,
    claude_code: ViewHandle<ClaudeCodeWidget>,
    foreground_process: ViewHandle<ForegroundProcessWidget>,
}

impl ContextPanelView {
    pub fn new(
        working_directories_model: ModelHandle<WorkingDirectoriesModel>,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        let wd_model = working_directories_model.clone();
        let working_directory = ctx.add_view(move |child_ctx| {
            WorkingDirectoryWidget::new(wd_model, child_ctx)
        });
        let git_model = working_directories_model.clone();
        let git = ctx.add_view(move |child_ctx| GitWidget::new(git_model, child_ctx));
        let claude_model = working_directories_model.clone();
        let claude_code =
            ctx.add_view(move |child_ctx| ClaudeCodeWidget::new(claude_model, child_ctx));
        let fg_model = working_directories_model.clone();
        let foreground_process = ctx.add_view(move |child_ctx| {
            ForegroundProcessWidget::new(fg_model, child_ctx)
        });
        Self {
            working_directory,
            git,
            claude_code,
            foreground_process,
        }
    }
}

impl Entity for ContextPanelView {
    type Event = ContextPanelEvent;
}

impl View for ContextPanelView {
    fn ui_name() -> &'static str {
        "ContextPanel"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);
        let theme = appearance.theme();
        let background = theme.background();
        let text_color = theme.main_text_color(background);
        let font_family = appearance.ui_font_family();

        let header = Container::new(
            Text::new("Context", font_family, 13.)
                .with_color(text_color.into())
                .finish(),
        )
        .with_horizontal_padding(12.)
        .with_vertical_padding(10.)
        .finish();

        let widget_stack = Flex::column()
            .with_main_axis_size(MainAxisSize::Min)
            .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
            .with_child(ChildView::new(&self.working_directory).finish())
            .with_child(ChildView::new(&self.git).finish())
            .with_child(ChildView::new(&self.claude_code).finish())
            .with_child(ChildView::new(&self.foreground_process).finish())
            .finish();

        Container::new(
            Flex::column()
                .with_main_axis_size(MainAxisSize::Max)
                .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
                .with_child(header)
                .with_child(widget_stack)
                .finish(),
        )
        .with_background(background)
        .finish()
    }
}
