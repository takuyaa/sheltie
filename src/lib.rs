#[derive(Debug, PartialEq)]
pub struct Token {
    term: String,
}

pub fn analyze(text: String) -> Vec<Token> {
    let sw = text.split_whitespace();
    sw.map(|t| Token {
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
