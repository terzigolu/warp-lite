//! Working Directory widget — Card 1 in the Context Panel stack.
//!
//! For v0.3 MVP this is a self-contained presentational widget that renders a
//! placeholder until wired up to `WorkingDirectoriesModel` in a later pass.

use warpui::{
    elements::{
        ConstrainedBox, Container, CrossAxisAlignment, Element, Flex, ParentElement, Text,
    },
    AppContext, Entity, SingletonEntity, View,
};

use crate::{appearance::Appearance, ui_components::icons::Icon};

/// Card-style widget that shows the focused tab's working directory.
pub struct WorkingDirectoryWidget;

impl WorkingDirectoryWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WorkingDirectoryWidget {
    fn default() -> Self {
        Self::new()
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
            Icon::Folder
                .to_warpui_icon(text_color.into())
                .finish(),
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

        let body = Container::new(
            Text::new("(no active terminal)", font_family, font_size)
                .with_color(sub_text_color.into())
                .finish(),
        )
        .with_margin_top(4.)
        .finish();

        Container::new(
            Flex::column()
                .with_child(header)
                .with_child(body)
                .finish(),
        )
        .with_horizontal_padding(12.)
        .with_vertical_padding(10.)
        .finish()
    }
}
