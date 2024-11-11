use anyhow::Result;
use pest::Parser;
use url_parser::{Rule, URLParser};

#[test]
fn test_scheme_positive() -> Result<()> {
    let valid_schemes = vec!["http", "https", "ftp"];
    for scheme in valid_schemes {
        let parse_result = URLParser::parse(Rule::scheme, scheme);
        assert!(
            parse_result.is_ok(),
            "Expected scheme '{}' to be valid",
            scheme
        );
    }
    Ok(())
}

#[test]
fn test_scheme_negative() -> Result<()> {
    let invalid_schemes = vec!["http:", "123#", "-ftp"];
    for scheme in invalid_schemes {
        let parse_result = URLParser::parse(Rule::scheme, scheme);
        assert!(
            parse_result.is_err(),
            "Expected scheme '{}' to be invalid",
            scheme
        );
    }
    Ok(())
}

#[test]
fn test_domain_positive() -> Result<()> {
    let valid_domains = vec!["example.com", "localhost", "192.168.0.1"];
    for domain in valid_domains {
        let parse_result = URLParser::parse(Rule::domain, domain);
        assert!(
            parse_result.is_ok(),
            "Expected domain '{}' to be valid",
            domain
        );
    }
    Ok(())
}

#[test]
fn test_domain_negative() -> Result<()> {
    let invalid_domains = vec!["example.com/", "localhost?", "192.168.0.1?query"];
    for domain in invalid_domains {
        let parse_result = URLParser::parse(Rule::domain, domain);
        assert!(
            parse_result.is_err(),
            "Expected domain '{}' to be invalid",
            domain
        );
    }
    Ok(())
}

#[test]
fn test_path_positive() -> Result<()> {
    let valid_paths = vec!["/home", "/path/to/resource", "/file.html"];
    for path in valid_paths {
        let parse_result = URLParser::parse(Rule::path, path);
        assert!(parse_result.is_ok(), "Expected path '{}' to be valid", path);
    }
    Ok(())
}

#[test]
fn test_path_negative() -> Result<()> {
    let invalid_paths = vec!["home", "path/to/resource", "/file.html?query"];
    for path in invalid_paths {
        let parse_result = URLParser::parse(Rule::path, path);
        assert!(
            parse_result.is_err(),
            "Expected path '{}' to be invalid",
            path
        );
    }
    Ok(())
}

#[test]
fn test_query_positive() -> Result<()> {
    let valid_queries = vec!["?name=value", "?q=rust+language", "?page=1&sort=asc"];
    for query in valid_queries {
        let parse_result = URLParser::parse(Rule::query, query);
        assert!(
            parse_result.is_ok(),
            "Expected query '{}' to be valid",
            query
        );
    }
    Ok(())
}

#[test]
fn test_query_negative() -> Result<()> {
    let invalid_queries = vec!["name=value", "?page#section", "?=value"];
    for query in invalid_queries {
        let parse_result = URLParser::parse(Rule::query, query);
        assert!(
            parse_result.is_err(),
            "Expected query '{}' to be invalid",
            query
        );
    }
    Ok(())
}

#[test]
fn test_fragment_positive() -> Result<()> {
    let valid_fragments = vec!["#section1", "#top", "#details"];
    for fragment in valid_fragments {
        let parse_result = URLParser::parse(Rule::fragment, fragment);
        assert!(
            parse_result.is_ok(),
            "Expected fragment '{}' to be valid",
            fragment
        );
    }
    Ok(())
}

#[test]
fn test_fragment_negative() -> Result<()> {
    let invalid_fragments = vec!["section1", "#", "#?query"];
    for fragment in invalid_fragments {
        let parse_result = URLParser::parse(Rule::fragment, fragment);
        assert!(
            parse_result.is_err(),
            "Expected fragment '{}' to be invalid",
            fragment
        );
    }
    Ok(())
}

#[test]
fn test_url_positive() -> Result<()> {
    let valid_urls = vec![
        "http://example.com",
        "https://localhost",
        "ftp://192.168.0.1",
    ];
    for url in valid_urls {
        let parse_result = URLParser::parse(Rule::url, url);
        assert!(parse_result.is_ok(), "Expected url '{}' to be valid", url);
    }
    Ok(())
}

#[test]
fn test_url_negative() -> Result<()> {
    let invalid_urls = vec![
        "http:/example.com",
        "https:localhost",
        "ftp://192.168.0.1/extra",
    ];
    for url in invalid_urls {
        let parse_result = URLParser::parse(Rule::url, url);
        assert!(
            parse_result.is_err(),
            "Expected url '{}' to be invalid",
            url
        );
    }
    Ok(())
}

#[test]
fn test_full_url_positive() -> Result<()> {
    let valid_urls = vec![
        "https://example.com/path/to/resource",
        "ftp://192.168.0.1/file.html",
        "http://localhost?name=value",
        "https://example.com/path#section1",
        "http://example.com/path/to/resource?name=value#section1",
    ];
    for url in valid_urls {
        let parse_result = URLParser::parse(Rule::url, url);
        assert!(parse_result.is_ok(), "Expected URL '{}' to be valid", url);
    }
    Ok(())
}

#[test]
fn test_full_url_negative() -> Result<()> {
    let invalid_urls = vec![
        "https//example.com/path/to/resource",
        "ftp:/192.168.0.1/file.html",
        "http://localhost name=value",
        "https://example.com/path#?section1",
        "://example.com/path/to/resource",
    ];
    for url in invalid_urls {
        let parse_result = URLParser::parse(Rule::url, url);
        assert!(
            parse_result.is_err(),
            "Expected URL '{}' to be invalid",
            url
        );
    }
    Ok(())
}
