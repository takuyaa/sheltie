use std::cell::RefCell;
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
}

#[derive(Debug)]
pub struct Index {
    inverted_index: RefCell<HashMap<String, PostingsList>>,
    max_doc_id: usize,
}

impl Index {
    pub fn new() -> Index {
        Index {
            inverted_index: RefCell::new(HashMap::new()),
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
        let inverted_index = self.inverted_index.get_mut();
        for (token, freq) in &freq_map {
            if let Some(postings_list) = inverted_index.get_mut(token) {
                postings_list.add(doc_id, *freq);
            } else {
                let mut posting_list = PostingsList::new();
                posting_list.add(doc_id, *freq);
                inverted_index.insert(token.clone(), posting_list);
            }
        }
        self.max_doc_id = doc_id;
    }
}

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
            index
        };
        assert_eq!(index.max_doc_id, 1);
        let inverted_index = index.inverted_index.borrow();

        let posting_list_one = inverted_index.get(&"one".to_string()).unwrap();
        assert_eq!(posting_list_one.len(), 1);

        let posting_list_of_two = inverted_index.get(&"two".to_string()).unwrap();
        assert_eq!(posting_list_of_two.len(), 1);
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
