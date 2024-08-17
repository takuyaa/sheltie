use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::analyzer::analyze;
use crate::index::{Index, PostingsList};

pub struct Searcher<'a> {
    index: &'a Index,
}

impl<'a> Searcher<'a> {
    pub fn new(index: &'a Index) -> Self {
        Self { index: &index }
    }

    // Search inverted index by document-at-a-time manner using binary heaps
    pub fn search(&self, text: &String, k: usize) -> Vec<SearchResult> {
        let results = {
            let tokens = &analyze(text);
            let mut terms = {
                let mut terms = BinaryHeap::with_capacity(tokens.len());
                // Set the cursors of all postings lists. The cursors points will be
                // sorted by a binary heap (min-heap).
                for token in tokens {
                    if let Some(postings_list) = self.index.get_postings_list(&token.token) {
                        let cursor = Cursor::new(postings_list);
                        if let Some(cursor) = cursor {
                            terms.push(Reverse(cursor));
                        }
                    }
                }
                terms
            };
            let mut results = BinaryHeap::with_capacity(k);
            while let Some(Reverse(mut cursor_min)) = terms.pop() {
                // doc_id is a document ID which is now processing.
                if let Some(doc_id) = cursor_min.next_doc {
                    // process first document.
                    let mut score = 1.0f64; // cumulative score of the document, fixed score for now.
                    if cursor_min.next() {
                        terms.push(Reverse(cursor_min));
                    }
                    while let Some(Reverse(cursor)) = terms.peek() {
                        if cursor.next_doc != Some(doc_id) {
                            break;
                        }
                        if let Some(Reverse(mut cursor)) = terms.pop() {
                            if let Some(_) = cursor.next_doc {
                                score += 1.0f64; // fixed score for now.
                                if cursor.next() {
                                    terms.push(Reverse(cursor));
                                }
                            }
                        }
                    }
                    results.push(ScoredDoc {
                        doc_id: doc_id,
                        score: score,
                    });
                }
            }
            results
        };
        results
            .iter()
            .take(k)
            .map(|r| SearchResult {
                doc_id: *r.doc_id,
                score: r.score,
            })
            .collect()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SearchResult {
    pub doc_id: usize,
    pub score: f64,
}

pub struct ScoredDoc<'a> {
    pub doc_id: &'a usize,
    pub score: f64,
}

impl Ord for ScoredDoc<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score
            .partial_cmp(&other.score)
            .unwrap_or(Ordering::Less) // should fail?
    }
}

impl PartialOrd for ScoredDoc<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl PartialEq for ScoredDoc<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for ScoredDoc<'_> {}

#[derive(Debug)]
struct Cursor<'a> {
    postings_list: &'a PostingsList,
    position: usize, // index position for the postings list
    next_doc: Option<&'a usize>,
}

impl<'a> Cursor<'a> {
    pub fn new(postings_list: &'a PostingsList) -> Option<Self> {
        let head = 0;
        postings_list.get_doc_id(head).map(|doc_id| Cursor {
            postings_list: postings_list,
            position: 0,
            next_doc: Some(doc_id),
        })
    }

    pub fn next(&mut self) -> bool {
        let next_doc = self.postings_list.get_doc_id(self.position + 1);
        if let Some(next_doc) = next_doc {
            self.position += 1;
            self.next_doc = Some(next_doc);
            true
        } else {
            false
        }
    }
}

impl Ord for Cursor<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.next_doc.cmp(&other.next_doc)
    }
}

impl PartialOrd for Cursor<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.next_doc.cmp(&other.next_doc))
    }
}

impl PartialEq for Cursor<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.next_doc == other.next_doc
    }
}

impl Eq for Cursor<'_> {}

#[cfg(test)]
mod tests {
    use super::Searcher;
    use crate::index::Index;

    #[test]
    fn test_search() {
        let index = {
            let mut index = Index::new();
            index.add(&String::from("two one two"));
            index.add(&String::from("one two three two three three"));
            index
        };

        let searcher = Searcher { index: &index };

        let results = searcher.search(&"one".to_string(), 10);
        assert_eq!(results.len(), 2);
        let results = searcher.search(&"two".to_string(), 10);
        assert_eq!(results.len(), 2);
        let results = searcher.search(&"one two".to_string(), 10);
        assert_eq!(results.len(), 2);
        let results = searcher.search(&"three".to_string(), 10);
        assert_eq!(results.len(), 1);
    }
}
