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

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token: String,
}

#[cfg(test)]
mod tests {
    use super::analyze;
    use super::Token;

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
