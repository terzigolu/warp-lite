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

use super::widgets::working_directory::WorkingDirectoryWidget;

pub enum ContextPanelEvent {
    ClosePanel,
}

pub struct ContextPanelView {
    working_directory: ViewHandle<WorkingDirectoryWidget>,
}

impl ContextPanelView {
    pub fn new(
        working_directories_model: ModelHandle<WorkingDirectoriesModel>,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        let model_for_widget = working_directories_model.clone();
        let working_directory = ctx.add_view(move |child_ctx| {
            WorkingDirectoryWidget::new(model_for_widget, child_ctx)
        });
        Self { working_directory }
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
