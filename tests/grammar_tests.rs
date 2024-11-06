use anyhow::Result;
use pest::Parser;
use url_parser::{URLParser, Rule};

#[test]
fn test_scheme_positive() -> Result<()> {
    let valid_schemes = vec!["http", "https", "ftp"];
    for scheme in valid_schemes {
        let parse_result = URLParser::parse(Rule::scheme, scheme);
        assert!(parse_result.is_ok(), "Expected scheme '{}' to be valid", scheme);
    }
    Ok(())
}

#[test]
fn test_scheme_negative() -> Result<()> {
    let invalid_schemes = vec!["http:", "123#", "-ftp"];
    for scheme in invalid_schemes {
        let parse_result = URLParser::parse(Rule::scheme, scheme);
        assert!(parse_result.is_err(), "Expected scheme '{}' to be invalid", scheme);
    }
    Ok(())
}

#[test]
fn test_domain_positive() -> Result<()> {
    let valid_domains = vec!["example.com", "localhost", "192.168.0.1"];
    for domain in valid_domains {
        let parse_result = URLParser::parse(Rule::domain, domain);
        assert!(parse_result.is_ok(), "Expected domain '{}' to be valid", domain);
    }
    Ok(())
}

#[test]
fn test_domain_negative() -> Result<()> {
    let invalid_domains = vec!["example.com/", "localhost?", "192.168.0.1?query"];
    for domain in invalid_domains {
        let parse_result = URLParser::parse(Rule::domain, domain);
        assert!(parse_result.is_err(), "Expected domain '{}' to be invalid", domain);
    }
    Ok(())
}

#[test]
fn test_url_positive() -> Result<()> {
    let valid_urls = vec!["http://example.com", "https://localhost", "ftp://192.168.0.1"];
    for url in valid_urls {
        let parse_result = URLParser::parse(Rule::url, url);
        assert!(parse_result.is_ok(), "Expected url '{}' to be valid", url);
    }
    Ok(())
}

#[test]
fn test_url_negative() -> Result<()> {
    let invalid_urls = vec!["http:/example.com", "https:localhost", "ftp://192.168.0.1/extra"];
    for url in invalid_urls {
        let parse_result = URLParser::parse(Rule::url, url);
        assert!(parse_result.is_err(), "Expected url '{}' to be invalid", url);
    }
    Ok(())
}