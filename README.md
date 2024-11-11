### URL Parser

URL Parser is a Rust parser developed to parse URLs into structured components such as scheme, domain, path, query and fragment.

### Parsing Process

URL Parser processes a URL string and extracts the following components:

1. **Scheme**: The protocol used (for example, "http", "https").
2. **Domain**: The domain name or IP address of the URL (for example, "example.com").
3. **Path**: The path that specifies the resource location on the server (for example, "/products/electronics").
4. **Query**: The query string following a '?', used for passing parameters (for example, "?query=1").
5. **Fragment**: The section identifier following a '#', referring to a part of the resource (for example, "#reviews").

The parsed components are essential for applications needing to analyze, validate or manipulate URLs.

### Grammar

## WHITESPACE

```
WHITESPACE = _{ " " | "\t" | "\n" }
```

**Purpose**: Defines space characters.
**Explanation**: This rule finds a single space character (" "), tab ("\t") or newline character ("\n"). The underscore indicates that this rule is silent (it does not create any output in the parse tree). This rule is not used directly in URL parsing, but can be useful for ignoring spaces when needed.

## url

```
url = { scheme ~ "://" ~ domain ~ path? ~ query? ~ fragment? }
```

**Purpose**: Defines the overall structure of the URL.
**Explanation**: The url rule is the basic rule for parsing a full URL. It expects:

1. scheme followed by ://
2. domain
3. optional path
4. optional query
5. optional fragment

'?' after some components (path?, query?, fragment?) means that these components are optional.

## scheme

```
scheme = @{ ASCII_ALPHANUMERIC+ }
```

**Purpose**: Identifies the URL scheme or protocol (for example, "http", "https", "ftp").
**Explanation**: A scheme rule matches one or more alphanumeric characters (letters and numbers). The '@' symbol marks this rule as an atomic rule, which means that it is processed as a single unit to improve performance and ensure correct parsing.

## domain

```
domain = @{ (!("/" | "?") ~ ANY)+ }
```

**Purpose**: Defines the domain part of the URL (for example, "example.com", "localhost", "192.168.0.1").
**Explanation**: The domain rule finds one or more characters that are not the '/' or '?' character. The ANY keyword matches any character other than a newline. This allows the domain to include subdomains, IP addresses or hostnames, as long as they do not contain '/' or '?'.

## path

```
path = { "/" ~ (!("?" | "#") ~ ANY)* }
```

**Purpose**: Defines a component of a URL path that indicates the location of a resource on the server (for example, "/products/electronics").
**Explanation**: The path rule begins with '/' and then allows zero or more characters that are not the '?' or '#' character. This excludes query parameters and fragment identifiers from the path part.

## query

```
query = { "?" ~ (!"#" ~ ANY)* }
```

**Purpose**: Defines a query string in the URL that is typically used to pass parameters (for example, "?query=1").
**Explanation**: The query rule starts with '?' and allows zero or more characters that are not '#'. This ensures that the query string does not include a fragment character (#) that belongs to a URL fragment component.

## fragment

```
fragment = { "#" ~ ANY* }
```

**Purpose**: Defines a section identifier that points to a specific section within the resource (for example, "#reviews").
**Explanation**: The fragment rule starts with '#' and allows any number of characters after it. This rule allows the fragment to contain any characters after the '#'.
