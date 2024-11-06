use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct URLParser;

#[derive(Debug)]
pub struct ParsedURL {
    pub scheme: String,
    pub domain: String,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Failed to parse URL")]
    ParsingFailed,
    #[error("Unexpected rule encountered: {0}")]
    UnexpectedRule(String),
}

impl ParsedURL {
    pub fn parse(url: &str) -> Result<Self, ParseError> {
        let mut pairs = URLParser::parse(Rule::url, url).map_err(|_| ParseError::ParsingFailed)?;
        
        let url_pair = pairs.next().ok_or(ParseError::ParsingFailed)?;
        let mut scheme = String::new();
        let mut domain = String::new();

        for pair in url_pair.into_inner() {
            match pair.as_rule() {
                Rule::scheme => scheme = pair.as_str().to_string(),
                Rule::domain => domain = pair.as_str().to_string(),
                rule => return Err(ParseError::UnexpectedRule(format!("{:?}", rule))),
            }
        }

        Ok(Self { scheme, domain })
    }
}