// warp-lite stub: notebook embedding (cloud-coupled) removed.
#[cfg(any())]
mod _disabled {
    mod embedded_fuzzy_match;
    mod notebooks;
    pub mod searcher;
    pub mod view;
    mod workflows;
}

#[allow(dead_code)]
pub mod searcher {
    #[derive(Default, Clone, Debug)] pub struct EmbeddingSearcher;
    #[derive(Clone, Debug)]
    pub enum EmbeddingSearchItemAction {
        AcceptWorkflow(crate::server::ids::SyncId),
        AcceptNotebook(crate::server::ids::SyncId),
    }
}
#[allow(dead_code)]
pub mod view {
    use warpui::{AppContext, Element, Entity, View};
    #[derive(Default, Clone, Debug)] pub struct EmbeddingView;
    #[derive(Clone, Debug)]
    pub enum EmbeddingSearchEvent {
        Close,
        ItemSelected { payload: std::sync::Arc<super::searcher::EmbeddingSearchItemAction> },
    }
    #[derive(Default)]
    pub struct EmbeddingSearchMenu;
    impl EmbeddingSearchMenu {
        pub fn new(_ctx: &mut warpui::ViewContext<Self>) -> Self { Self }
        pub fn reset_state<A>(&mut self, _: A) {}
        pub fn set_embedding_space<A, B>(&mut self, _: A, _: B) {}
    }
    impl warpui::TypedActionView for EmbeddingSearchMenu {
        type Action = StubAction;
    }
    #[derive(Clone, Debug, Default)] pub struct StubAction;
    impl Entity for EmbeddingSearchMenu { type Event = EmbeddingSearchEvent; }
    impl View for EmbeddingSearchMenu {
        fn ui_name() -> &'static str { "EmbeddingSearchMenu_stub" }
        fn render(&self, _: &AppContext) -> Box<dyn Element> {
            Box::new(warpui::elements::Empty::new())
        }
    }
}
