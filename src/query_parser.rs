use nom::branch::{alt, permutation};
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::{alphanumeric1, space1};
use nom::combinator::{all_consuming, map_res, opt};
use nom::error::VerboseError;
use nom::multi::{separated_list0, separated_list1};
use nom::IResult;

use crate::query::{boolean::BooleanQuery, phrase::PhraseQuery, term::TermQuery};
use crate::query::{Occur, Query};

pub struct QueryParser {}

impl QueryParser {
    pub fn parse(query: &str) -> Result<Box<dyn Query>, String> {
        let (_, res) = Self::query(query).map_err(|e| format!("{:?}", e))?;
        Ok(res)
    }

    pub fn query(query: &str) -> IResult<&str, Box<dyn Query>> {
        map_res(
            alt((
                Self::simple_term_query,
                Self::simple_phrase_query,
                Self::boolean_query,
            )),
            |q| Ok::<Box<dyn Query>, VerboseError<&str>>(q),
        )(query)
    }

    pub fn simple_term_query(query: &str) -> IResult<&str, Box<dyn Query>> {
        map_res(all_consuming(Self::term_query), |q| {
            Ok::<Box<dyn Query>, VerboseError<&str>>(q)
        })(query)
    }

    pub fn simple_phrase_query(query: &str) -> IResult<&str, Box<dyn Query>> {
        map_res(all_consuming(Self::phrase_query), |q| {
            Ok::<Box<dyn Query>, VerboseError<&str>>(q)
        })(query)
    }

    pub fn boolean_query(query: &str) -> IResult<&str, Box<dyn Query>> {
        map_res(separated_list1(space1, Self::_boolean_term), |subqueries| {
            Ok::<Box<dyn Query>, VerboseError<&str>>(Box::new(BooleanQuery::new(subqueries)))
        })(query)
    }

    pub fn _boolean_term(query: &str) -> IResult<&str, (Occur, Box<dyn Query>)> {
        map_res(
            permutation((opt(alt((char('+'), char('-')))), Self::subquery)),
            |(occur, query)| match occur {
                Some(op) => match op {
                    '+' => {
                        return Ok::<(Occur, Box<dyn Query>), VerboseError<&str>>((
                            Occur::Must,
                            query,
                        ))
                    }
                    '-' => {
                        return Ok::<(Occur, Box<dyn Query>), VerboseError<&str>>((
                            Occur::MustNot,
                            query,
                        ))
                    }
                    _ => return Err(VerboseError { errors: vec![] }),
                },
                None => {
                    return Ok::<(Occur, Box<dyn Query>), VerboseError<&str>>((
                        Occur::Should,
                        query,
                    ))
                }
            },
        )(query)
    }

    pub fn subquery(query: &str) -> IResult<&str, Box<dyn Query>> {
        map_res(alt((Self::term_query, Self::phrase_query)), |q| {
            Ok::<Box<dyn Query>, VerboseError<&str>>(q)
        })(query)
    }

    pub fn term_query(query: &str) -> IResult<&str, Box<dyn Query>> {
        map_res(alphanumeric1, |q: &str| {
            Ok::<Box<dyn Query>, VerboseError<&str>>(Box::new(TermQuery::new(q.to_string())))
        })(query)
    }

    pub fn phrase_query(query: &str) -> IResult<&str, Box<dyn Query>> {
        map_res(
            permutation((tag("\""), Self::_terms, tag("\""))),
            |(_, ts, _)| Ok::<Box<dyn Query>, VerboseError<&str>>(Box::new(PhraseQuery::new(ts))),
        )(query)
    }

    pub fn _terms(query: &str) -> IResult<&str, Vec<String>> {
        map_res(separated_list0(space1, alphanumeric1), |ts| {
            Ok::<Vec<String>, VerboseError<&str>>(ts.into_iter().map(String::from).collect())
        })(query)
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidQuery,
}

#[cfg(test)]
mod tests {
    use crate::query::Occur;
    use crate::query::{boolean::BooleanQuery, phrase::PhraseQuery, term::TermQuery};

    use super::QueryParser;

    #[test]
    fn test_term_query() {
        assert_eq!(
            format!("{:?}", QueryParser::term_query(&"abc").unwrap().1),
            format!("{:?}", TermQuery::new("abc".to_string()))
        );
    }

    #[test]
    fn test_phrase_query() {
        assert_eq!(
            format!("{:?}", QueryParser::phrase_query(&"\"abc\"").unwrap().1),
            format!("{:?}", PhraseQuery::new(vec!["abc".to_string()]))
        );
        assert_eq!(
            format!("{:?}", QueryParser::phrase_query(&"\"abc def\"").unwrap().1),
            format!(
                "{:?}",
                PhraseQuery::new(vec!["abc".to_string(), "def".to_string()])
            )
        );
    }

    #[test]
    fn test_boolean_term() {
        // TermQuery
        assert_eq!(
            format!("{:?}", QueryParser::_boolean_term(&"abc").unwrap().1),
            format!("{:?}", (Occur::Should, TermQuery::new("abc".to_string())))
        );
        assert_eq!(
            format!("{:?}", QueryParser::_boolean_term(&"+abc").unwrap().1),
            format!("{:?}", (Occur::Must, TermQuery::new("abc".to_string())))
        );
        assert_eq!(
            format!("{:?}", QueryParser::_boolean_term(&"-abc").unwrap().1),
            format!("{:?}", (Occur::MustNot, TermQuery::new("abc".to_string())))
        );

        // PhraseQuery
        assert_eq!(
            format!(
                "{:?}",
                QueryParser::_boolean_term(&"\"abc def\"").unwrap().1
            ),
            format!(
                "{:?}",
                (
                    Occur::Should,
                    PhraseQuery::new(vec!["abc".to_string(), "def".to_string()])
                )
            )
        );
        assert_eq!(
            format!("{:?}", QueryParser::_boolean_term(&"+\"abc\"").unwrap().1),
            format!(
                "{:?}",
                (Occur::Must, PhraseQuery::new(vec!["abc".to_string()]))
            )
        );
        assert_eq!(
            format!("{:?}", QueryParser::_boolean_term(&"-\"abc\"").unwrap().1),
            format!(
                "{:?}",
                (Occur::MustNot, PhraseQuery::new(vec!["abc".to_string()]))
            )
        );
    }

    #[test]
    fn test_boolean_query() {
        assert_eq!(
            format!("{:?}", QueryParser::boolean_query(&"abc def").unwrap().1),
            format!(
                "{:?}",
                BooleanQuery::new(vec![
                    (Occur::Should, Box::new(TermQuery::new("abc".to_string()))),
                    (Occur::Should, Box::new(TermQuery::new("def".to_string()))),
                ])
            )
        );

        assert_eq!(
            format!("{:?}", QueryParser::boolean_query(&"+abc +def").unwrap().1),
            format!(
                "{:?}",
                BooleanQuery::new(vec![
                    (Occur::Must, Box::new(TermQuery::new("abc".to_string()))),
                    (Occur::Must, Box::new(TermQuery::new("def".to_string()))),
                ])
            )
        );

        assert_eq!(
            format!(
                "{:?}",
                QueryParser::boolean_query(&"+abc def -g +\"hi\"")
                    .unwrap()
                    .1
            ),
            format!(
                "{:?}",
                BooleanQuery::new(vec![
                    (Occur::Must, Box::new(TermQuery::new("abc".to_string()))),
                    (Occur::Should, Box::new(TermQuery::new("def".to_string()))),
                    (Occur::MustNot, Box::new(TermQuery::new("g".to_string()))),
                    (
                        Occur::Must,
                        Box::new(PhraseQuery::new(vec!["hi".to_string()]))
                    ),
                ])
            )
        );
    }

    #[test]
    fn test_parse() {
        // TermQuery
        assert_eq!(
            format!("{:?}", QueryParser::parse(&"abc").unwrap()),
            format!("{:?}", TermQuery::new("abc".to_string()))
        );
        // PhraseQuery
        assert_eq!(
            format!("{:?}", QueryParser::parse(&"\"abc\"").unwrap()),
            format!("{:?}", PhraseQuery::new(vec!["abc".to_string()]))
        );
        // BooleanQuery
        assert_eq!(
            format!("{:?}", QueryParser::parse(&"abc def").unwrap()),
            format!(
                "{:?}",
                BooleanQuery::new(vec![
                    (Occur::Should, Box::new(TermQuery::new("abc".to_string()))),
                    (Occur::Should, Box::new(TermQuery::new("def".to_string())))
                ])
            )
        );
    }
}
