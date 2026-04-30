// warp-lite stub: AI context menu removed. Public surface preserved as no-ops.

#[cfg(any())]
mod _disabled_subtree {
    mod blocks;
    mod code;
    mod commands;
    mod conversations;
    mod diffset;
    mod files;
    pub mod mixer;
    mod notebooks;
    mod rules;
    pub mod search;
    #[cfg(not(target_family = "wasm"))]
    mod skills;
    mod styles;
    pub mod view;
    mod workflows;
}

/// Safely truncate a string at the given byte index, ensuring we don't split UTF-8 characters
pub fn safe_truncate(s: &mut String, new_len: usize) {
    if new_len >= s.len() {
        return;
    }
    let safe_len = floor_char_boundary(s, new_len);
    s.truncate(safe_len);
}

/// Find the largest valid character boundary at or before the given byte index
pub fn floor_char_boundary(original_string: &str, idx: usize) -> usize {
    if idx >= original_string.len() {
        original_string.len()
    } else {
        let mut curr = idx;
        while curr > 0 && !original_string.is_char_boundary(curr) {
            curr -= 1;
        }
        curr
    }
}

// Stubs of public types previously exported via search/view/etc. that other crate
// modules still reference. These are intentionally inert.
#[allow(dead_code)]
pub mod view {
    use warpui::{AppContext, Element, Entity, View};
    #[derive(Default)]
    pub struct AIContextMenu;
    impl AIContextMenu {
        pub fn new(_ctx: &mut warpui::ViewContext<Self>) -> Self { Self }
        pub fn new3<A, B, C>(_: A, _: B, _: C) -> Self { Self }
        pub fn should_render<A>(&self, _: A) -> bool { false }
        pub fn get_categories_for_mode<A, B, C, D, E>(_: A, _: B, _: C, _: D, _: E) -> Vec<AIContextMenuCategory> { Vec::new() }
        pub fn set_is_in_ambient_agent<A, B>(&mut self, _: A, _: &mut B) {}
        pub fn set_is_shared_session_viewer<A, B>(&mut self, _: A, _: &mut B) {}
        pub fn set_is_cli_agent_input<A, B>(&mut self, _: A, _: &mut B) {}
        pub fn set_input_mode<A, B>(&mut self, _: A, _: &mut B) {}
        pub fn update_search_query<A, B>(&mut self, _: A, _: &mut B) {}
        pub fn close<A>(&mut self, _: A) {}
        pub fn reset_menu_state<A>(&mut self, _: A) {}
        pub fn select_current_item<A>(&mut self, _: A) {}
        pub fn handle_action<A, B>(&mut self, _: &A, _: B) {}
    }
    impl Entity for AIContextMenu { type Event = AIContextMenuEvent; }
    impl View for AIContextMenu {
        fn ui_name() -> &'static str { "AIContextMenu_stub" }
        fn render(&self, _: &AppContext) -> Box<dyn Element> {
            Box::new(warpui::elements::Empty::new())
        }
    }
    impl warpui::TypedActionView for AIContextMenu {
        type Action = AIContextMenuAction;
    }
    #[derive(Default, Clone, Copy, Debug)]
    pub enum AIContextMenuCategory { #[default] Default }
    #[derive(Clone, Debug)]
    pub enum AIContextMenuEvent {
        Close { item_count: Option<usize>, query_length: usize },
        ResultAccepted { action: super::mixer::AIContextMenuSearchableAction, item_count: Option<usize>, query_length: usize },
        CategorySelected { category: AIContextMenuCategory },
    }
    #[derive(Clone, Debug, Default)]
    pub enum AIContextMenuAction {
        #[default] Default,
        Prev,
        Next,
    }
}
#[allow(dead_code)]
pub mod search {
    #[derive(Default, Clone, Debug)]
    pub struct AIContextMenuSearch;
    pub fn is_valid_search_query<A, B, C>(_a: A, _b: B, _c: C) -> bool { false }
}
#[allow(dead_code)]
pub mod mixer {
    #[derive(Default, Clone, Debug)]
    pub struct AIContextMenuMixer;
    #[derive(Clone, Debug)]
    pub enum AIContextMenuSearchableAction {
        InsertText { text: String },
        InsertFilePath { file_path: String },
        InsertDriveObject { object_type: String, object_uid: String },
        InsertPlan { ai_document_uid: String },
        InsertConversation { conversation_id: String },
        InsertDiffSet { diff_mode: crate::code_review::diff_state::DiffMode },
        InsertSkill { name: String },
    }
}
