use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Token {
    term: String,
}

pub fn analyze(text: String) -> Vec<Token> {
    if text.len() == 0 {
        return vec![];
    }
    text.split_whitespace()
        .map(|t| Token {
            term: t.to_string(),
        })
        .collect::<Vec<Token>>()
}

#[derive(Debug)]
pub struct Index {
    inverted_index: RefCell<HashMap<String, Vec<u64>>>,
    max_doc_id: u64,
}

impl Index {
    pub fn new() -> Index {
        Index {
            inverted_index: RefCell::new(HashMap::new()),
            max_doc_id: 0u64,
        }
    }

    pub fn add(&mut self, text: String) {
        let doc_id = self.max_doc_id + 1;
        let tokens = analyze(text);
        for token in tokens {
            let map = self.inverted_index.get_mut();
            if let Some(postings_list) = map.get_mut(&token.term) {
                postings_list.push(doc_id);
            } else {
                let posting_list = vec![doc_id];
                map.insert(token.term, posting_list);
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
        let mut index = Index::new();
        assert_eq!(index.max_doc_id, 0);
        index.add(String::from("lorem ipsum"));
        assert_eq!(index.max_doc_id, 1);
        let posting_list_of_test = index
            .inverted_index
            .get_mut()
            .get(&"lorem".to_string())
            .unwrap();
        assert_eq!(posting_list_of_test.len(), 1);
        let posting_list_of_test = index
            .inverted_index
            .get_mut()
            .get(&"ipsum".to_string())
            .unwrap();
        assert_eq!(posting_list_of_test.len(), 1);
    }

    #[test]
    fn test_analyze() {
        assert_eq!(analyze("".to_string()), vec![]);
        assert_eq!(analyze(" ".to_string()), vec![]);
        assert_eq!(analyze("   ".to_string()), vec![]);
        assert_eq!(
            analyze("aaa bbb cc d".to_string()),
            vec![
                Token {
                    term: String::from("aaa")
                },
                Token {
                    term: String::from("bbb")
                },
                Token {
                    term: String::from("cc")
                },
                Token {
                    term: String::from("d")
                },
            ]
        );
    }
}
