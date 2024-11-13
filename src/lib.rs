/// Parses URLs into their components using Pest-based grammar.
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

/// Main parser struct for URL parsing, based on the grammar defined in `grammar.pest`.
///
/// # Grammar Rules
///

/// - **url**: Parses a complete URL structure including scheme, domain, path, query and fragment.
///
/// url = { scheme ~ "://" ~ domain ~ path? ~ query? ~ fragment? }

/// - **scheme**: Defines the URL scheme, which consists of alphanumeric characters (for example, `http` or `https`).
///
/// scheme = @{ ASCII_ALPHANUMERIC+ }

/// - **domain**: Parses the domain of the URL, excluding `/` and `?` characters to avoid conflicts with other components.
///
/// domain = @{ (!("/" | "?") ~ ANY)+ }

/// - **path**: Matches the path component, beginning with `/` and excluding `?` and `#` to separate it from query and fragment components.
///
/// path = { "/" ~ (!("?" | "#") ~ ANY)* }

/// - **query**: Parses the query string of a URL, starting with `?` and avoiding the fragment identifier `#`.
///
/// query = { "?" ~ (!"#" ~ ANY)* }

/// - **fragment**: Matches the fragment component of the URL, starting with `#` and containing any characters after it.
///
/// fragment = { "#" ~ ANY* }

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct URLParser;

/// Contains the parsed components of a URL.
#[derive(Debug)]
pub struct ParsedURL {
    /// Scheme component of the URL (for example, `http`, `https`).
    pub scheme: String,
    /// Domain part of the URL (for example, `example.com`).
    pub domain: String,
    /// Optional path component of the URL (for example, `/products/electronics`).
    pub path: Option<String>,
    /// Optional query component, starting with `?` (for example, `?query=1`).
    pub query: Option<String>,
    /// Optional fragment component, starting with `#` (for example, `#reviews`).
    pub fragment: Option<String>,
}

/// Describes possible errors that can occur during URL parsing.
#[derive(Error, Debug)]
pub enum ParseError {
    /// General error indicating parsing failure.
    #[error("Failed to parse URL")]
    ParsingFailed,
    /// Error indicating that an unexpected rule was encountered during parsing.
    #[error("Unexpected rule encountered: {0}")]
    UnexpectedRule(String),
}

impl ParsedURL {
    /// Parses a URL string into its components based on the defined grammar.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice representing the URL to be parsed.
    ///
    /// # Returns
    ///
    /// Returns a `ParsedURL` struct on success or a `ParseError` on failure.
    ///
    /// # Grammar Rules Breakdown
    ///
    /// - **url**: The top-level rule, defining the full URL structure:
    ///     - Composed of `scheme`, `domain`, optional `path`, optional `query` and optional `fragment`.
    /// - **scheme**: Matches a sequence of alphanumeric characters, typically representing the protocol (for example, `http`).
    /// - **domain**: Defines the domain of the URL, excluding characters `/` and `?`.
    /// - **path**: Begins with a `/` and excludes `?` and `#` to avoid query and fragment parsing conflicts.
    /// - **query**: Prefixed with `?`, and continues until a `#` (fragment symbol) if present.
    /// - **fragment**: Starts with `#` and matches any remaining characters.
    pub fn parse(url: &str) -> Result<Self, ParseError> {
        let mut pairs = URLParser::parse(Rule::url, url).map_err(|_| ParseError::ParsingFailed)?;

        let url_pair = pairs.next().ok_or(ParseError::ParsingFailed)?;
        let mut scheme = String::new();
        let mut domain = String::new();
        let mut path = None;
        let mut query = None;
        let mut fragment = None;

        for pair in url_pair.into_inner() {
            match pair.as_rule() {
                Rule::scheme => scheme = pair.as_str().to_string(),
                Rule::domain => domain = pair.as_str().to_string(),
                Rule::path => path = Some(pair.as_str().to_string()),
                Rule::query => query = Some(pair.as_str().to_string()),
                Rule::fragment => fragment = Some(pair.as_str().to_string()),
                rule => return Err(ParseError::UnexpectedRule(format!("{:?}", rule))),
            }
        }

        Ok(Self {
            scheme,
            domain,
            path,
            query,
            fragment,
        })
    }
}
