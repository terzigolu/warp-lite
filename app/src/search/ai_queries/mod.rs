// warp-lite stub: AI queries removed.
#[cfg(any())]
mod _disabled {
    pub mod fuzzy_match;
}

#[allow(dead_code)]
pub mod fuzzy_match {
    #[derive(Default, Clone, Debug)] pub struct FuzzyMatch;
    #[derive(Default, Clone, Debug)]
    pub struct QueryTextMatchResultStub {
        pub matched_indices: Vec<usize>,
    }
    #[derive(Default, Clone, Debug)]
    pub struct FuzzyMatchAIQueryResults {
        pub query_text_match_result: QueryTextMatchResultStub,
    }
    impl FuzzyMatchAIQueryResults {
        pub fn try_match<A, B>(_: A, _: B) -> Option<Self> { None }
        pub fn score(&self) -> ordered_float::OrderedFloat<f64> { ordered_float::OrderedFloat(0.0) }
    }
}
