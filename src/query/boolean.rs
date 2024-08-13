use super::{Occur, Query};
use crate::{index::Index, searcher::SearchResult};

#[derive(Debug)]
#[allow(dead_code)]
pub struct BooleanQuery {
    quries: Vec<(Occur, Box<dyn Query>)>,
}

impl BooleanQuery {
    pub fn new(queries: Vec<(Occur, Box<dyn Query>)>) -> Self {
        Self { quries: queries }
    }
}

impl Query for BooleanQuery {
    fn execute(&self, index: &Index) -> Vec<SearchResult> {
        // TODO
        let _ = index;
        Vec::new()
    }
}
