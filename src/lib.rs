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

#[cfg(test)]
mod tests {
    use super::analyze;
    use super::Token;

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
