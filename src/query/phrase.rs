use super::Query;
use crate::{index::Index, searcher::SearchResult};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct PhraseQuery {
    terms: Vec<String>,
}

impl PhraseQuery {
    pub fn new(terms: Vec<String>) -> Self {
        Self { terms: terms }
    }
}

impl Query for PhraseQuery {
    fn execute(&self, index: &Index) -> Vec<SearchResult> {
        // TODO
        let _ = index;
        Vec::new()
    }
}
