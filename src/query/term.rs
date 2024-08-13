use super::Query;
use crate::{index::Index, searcher::SearchResult};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct TermQuery {
    term: String,
}

impl TermQuery {
    pub fn new(term: String) -> Self {
        Self { term: term }
    }
}

impl Query for TermQuery {
    fn execute(&self, index: &Index) -> Vec<SearchResult> {
        // TODO
        let _ = index;
        Vec::new()
    }
}
