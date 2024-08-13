pub mod boolean;
pub mod phrase;
pub mod term;

use crate::{index::Index, searcher::SearchResult};

pub trait Query: std::fmt::Debug {
    fn execute(&self, index: &Index) -> Vec<SearchResult>;
}

#[derive(Debug)]
pub enum Occur {
    Should,
    Must,
    MustNot,
}
