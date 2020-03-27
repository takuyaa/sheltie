use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::analyzer::analyze;

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

    pub fn get_postings_list(&self, term: &String) -> Option<&PostingsList> {
        self.inverted_index.get(term)
    }
}

#[cfg(test)]
mod tests {
    use super::Index;

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
}
