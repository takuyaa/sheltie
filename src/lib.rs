use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    token: String,
}

pub fn analyze(text: &String) -> Vec<Token> {
    if text.len() == 0 {
        return vec![];
    }
    text.split_whitespace()
        .map(|t| Token {
            token: t.to_string(),
        })
        .collect::<Vec<Token>>()
}

#[derive(Debug)]
pub struct PostingsList {
    docs: Vec<usize>,
    freqs: Vec<u32>,
}

impl PostingsList {
    pub fn new() -> PostingsList {
        PostingsList {
            docs: vec![],
            freqs: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.docs.len()
    }

    pub fn add(&mut self, doc_id: usize, freq: u32) {
        self.docs.push(doc_id);
        self.freqs.push(freq);
    }

    pub fn get_doc_id(&self, index: usize) -> Option<&usize> {
        self.docs.get(index)
    }
}

#[derive(Debug)]
pub struct Index {
    inverted_index: HashMap<String, PostingsList>,
    max_doc_id: usize,
}

impl Index {
    pub fn new() -> Self {
        Index {
            inverted_index: HashMap::new(),
            max_doc_id: 0,
        }
    }

    pub fn add(&mut self, text: &String) {
        let tokens = analyze(&text);
        let freq_map = {
            let mut freq_map = HashMap::<String, u32>::new();
            for token in tokens {
                let freq = freq_map.entry(token.token).or_insert(0);
                *freq += 1;
            }
            freq_map
        };

        let doc_id = self.max_doc_id + 1;
        for (token, freq) in &freq_map {
            if let Some(postings_list) = self.inverted_index.get_mut(token) {
                postings_list.add(doc_id, *freq);
            } else {
                let mut posting_list = PostingsList::new();
                posting_list.add(doc_id, *freq);
                self.inverted_index.insert(token.clone(), posting_list);
            }
        }
        self.max_doc_id = doc_id;
    }

    // Search inverted index by document-at-a-time manner using binary heaps
    pub fn search(&self, text: &String, k: usize) -> Vec<Result> {
        let results = {
            let tokens = &analyze(text);
            let mut terms = {
                let mut terms = BinaryHeap::with_capacity(tokens.len());
                // Set the cursors of all postings lists. The cursors points will be
                // sorted by a binary heap (min-heap).
                for token in tokens {
                    if let Some(postings_list) = self.inverted_index.get(&token.token) {
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
            .map(|r| Result {
                doc_id: *r.doc_id,
                score: r.score,
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct Result {
    doc_id: usize,
    score: f64,
}

struct ScoredDoc<'a> {
    doc_id: &'a usize,
    score: f64,
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
    use super::analyze;
    use super::Index;
    use super::Token;

    #[test]
    fn test_index() {
        let index = {
            let mut index = Index::new();
            assert_eq!(index.max_doc_id, 0);
            index.add(&String::from("two one two"));
            assert_eq!(index.max_doc_id, 1);
            index.add(&String::from("one two three two three three"));
            assert_eq!(index.max_doc_id, 2);
            index
        };

        let posting_list_one = index.inverted_index.get(&"one".to_string()).unwrap();
        assert_eq!(posting_list_one.len(), 2);

        let posting_list_of_two = index.inverted_index.get(&"two".to_string()).unwrap();
        assert_eq!(posting_list_of_two.len(), 2);

        let posting_list_of_three = index.inverted_index.get(&"three".to_string()).unwrap();
        assert_eq!(posting_list_of_three.len(), 1);
    }

    #[test]
    fn test_index_search() {
        let index = {
            let mut index = Index::new();
            assert_eq!(index.max_doc_id, 0);
            index.add(&String::from("two one two"));
            assert_eq!(index.max_doc_id, 1);
            index.add(&String::from("one two three two three three"));
            assert_eq!(index.max_doc_id, 2);
            index
        };

        let posting_list_one = index.inverted_index.get(&"one".to_string()).unwrap();
        assert_eq!(posting_list_one.len(), 2);

        let posting_list_of_two = index.inverted_index.get(&"two".to_string()).unwrap();
        assert_eq!(posting_list_of_two.len(), 2);

        let posting_list_of_three = index.inverted_index.get(&"three".to_string()).unwrap();
        assert_eq!(posting_list_of_three.len(), 1);

        let results = index.search(&"one".to_string(), 10);
        assert_eq!(results.len(), 2);
        let results = index.search(&"two".to_string(), 10);
        assert_eq!(results.len(), 2);
        let results = index.search(&"one two".to_string(), 10);
        assert_eq!(results.len(), 2);
        let results = index.search(&"three".to_string(), 10);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_analyze() {
        assert_eq!(analyze(&"".to_string()), vec![]);
        assert_eq!(analyze(&" ".to_string()), vec![]);
        assert_eq!(analyze(&"   ".to_string()), vec![]);
        assert_eq!(
            analyze(&"aaa bbb cc d".to_string()),
            vec![
                Token {
                    token: String::from("aaa")
                },
                Token {
                    token: String::from("bbb")
                },
                Token {
                    token: String::from("cc")
                },
                Token {
                    token: String::from("d")
                },
            ]
        );
    }
}
