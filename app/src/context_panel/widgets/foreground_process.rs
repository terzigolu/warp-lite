//! Foreground Process widget — Card 4 in the Context Panel stack.
//!
//! v0.3 introduces a placeholder. Wiring the active tab's TTY into a
//! `ps -t <tty>` async poll requires a pty bridge that is deferred to a
//! later release; today the card just announces that the data is on its
//! way without crowding the panel.

use warpui::{
    elements::{
        ConstrainedBox, Container, CrossAxisAlignment, Element, Flex, ParentElement, Text,
    },
    AppContext, Entity, SingletonEntity, View,
};

use crate::{appearance::Appearance, ui_components::icons::Icon};

pub struct ForegroundProcessWidget;

impl ForegroundProcessWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ForegroundProcessWidget {
    fn default() -> Self {
        Self::new()
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

        let body = Container::new(
            Text::new(
                "(coming in a follow-up)",
                font_family,
                font_size,
            )
            .with_color(sub_text_color.into())
            .finish(),
        )
        .with_margin_top(4.)
        .finish();

        Container::new(Flex::column().with_child(header).with_child(body).finish())
            .with_horizontal_padding(12.)
            .with_vertical_padding(10.)
            .finish()
    }
}
