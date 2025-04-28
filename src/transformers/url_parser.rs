use crate::{Transform, TransformError, TransformerCategory};

/// URL Parser transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UrlParser;

// List of known non-hierarchical schemes (add more as needed)
const NON_HIERARCHICAL_SCHEMES: &[&str] = &["mailto", "urn", "tel", "sms", "news", "isbn"];

impl Transform for UrlParser {
    fn name(&self) -> &'static str {
        "URL Parser"
    }

    fn id(&self) -> &'static str {
        "urlparser"
    }

    fn description(&self) -> &'static str {
        "Parses a URL into its components (scheme, authority, path, query, fragment)"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Other
    }

    // Basic URL Parser (doesn't handle all edge cases, e.g., complex userinfo, IPv6 hosts)
    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let input = input.trim();
        if input.is_empty() {
            return Err(TransformError::InvalidArgument("Input URL is empty".into()));
        }

        let mut remainder = input;

        // 1. Scheme
        // Determine scheme, whether it's hierarchical, and the remainder of the string
        let (scheme, is_hierarchical, remainder_after_scheme) = if let Some(pos) =
            remainder.find("://")
        {
            let scheme_part = &remainder[..pos];
            // Validate scheme characters before ://
            if scheme_part.is_empty()
                || !scheme_part.starts_with(|c: char| c.is_ascii_alphabetic())
                || !scheme_part
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.')
            {
                return Err(TransformError::InvalidArgument(
                    format!("Invalid characters in scheme before '://': {}", scheme_part).into(),
                ));
            }
            (Some(scheme_part), true, &remainder[pos + 3..]) // Standard hierarchical scheme
        } else if let Some(pos) = remainder.find(':') {
            let potential_scheme = &remainder[..pos];
            // Check if the part before ':' looks structurally like a scheme
            if !potential_scheme.is_empty()
                && potential_scheme.starts_with(|c: char| c.is_ascii_alphabetic())
                && potential_scheme
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.')
            {
                // It might be a scheme. Check if it's a known non-hierarchical one.
                let lower_scheme = potential_scheme.to_ascii_lowercase();
                if NON_HIERARCHICAL_SCHEMES.contains(&lower_scheme.as_str()) {
                    // It's a known non-hierarchical scheme (e.g., mailto:)
                    (Some(potential_scheme), false, &remainder[pos + 1..])
                } else {
                    // Looks like a scheme syntactically but not known non-hierarchical,
                    // and no '://' was present. Assume it's not a scheme (e.g., host:port, drive letter).
                    (None, true, remainder) // Treat as having no scheme
                }
            } else {
                // The part before ':' doesn't look like a scheme (e.g., contains invalid chars)
                (None, true, remainder) // Treat as having no scheme
            }
        } else {
            // No ':' found at all
            (None, true, remainder) // Treat as having no scheme
        };

        // Update the remainder based on whether a scheme was parsed
        remainder = remainder_after_scheme;

        // 2. Fragment
        let fragment = if let Some(pos) = remainder.find('#') {
            let frag = &remainder[pos + 1..];
            remainder = &remainder[..pos];
            Some(frag)
        } else {
            None
        };

        // 3. Query
        let query = if let Some(pos) = remainder.find('?') {
            let q = &remainder[pos + 1..];
            remainder = &remainder[..pos];
            Some(q)
        } else {
            None
        };

        // 4. Authority and Path
        let (authority, path_str) = if !is_hierarchical {
            // For non-hierarchical schemes, the rest is the path (SSP)
            (None, remainder)
        } else if remainder.starts_with("//") {
            // Handle authority explicitly starting with //
            remainder = &remainder[2..];
            if let Some(pos) = remainder.find('/') {
                (Some(&remainder[..pos]), &remainder[pos..])
            } else {
                (Some(remainder), "")
            }
        } else if remainder.starts_with('/') {
            // Path starts immediately (e.g., /foo/bar?q=1 or file:///foo/bar)
            (None, remainder)
        } else if let Some(pos) = remainder.find('/') {
            // Authority present before path (e.g., host:port/path)
            (Some(&remainder[..pos]), &remainder[pos..])
        } else {
            // Only authority or path-rootless
            if scheme.is_some() {
                // If scheme present, assume remainder is authority if non-empty
                (Some(remainder), "")
            } else {
                // No scheme - check for host:port format or path
                let is_likely_host_port = remainder.contains(':')
                    && remainder.chars().filter(|&c| c == ':').count() == 1
                    && remainder
                        .split(':')
                        .nth(1)
                        .unwrap_or("")
                        .chars()
                        .all(|c| c.is_ascii_digit())
                    && !remainder.contains('/')
                    && !remainder.contains('?')
                    && !remainder.contains('#');

                if is_likely_host_port {
                    // Treat as authority (host:port) if it matches the pattern
                    (Some(remainder), "")
                } else {
                    // Otherwise treat as path
                    (None, remainder)
                }
            }
        };

        // Further parse authority into userinfo, host, port (basic)
        let mut userinfo = None;
        let mut host = None;
        let mut port = None;

        if let Some(auth_str) = authority {
            let mut auth_rem = auth_str;
            if let Some(pos) = auth_rem.rfind('@') {
                userinfo = Some(&auth_rem[..pos]);
                auth_rem = &auth_rem[pos + 1..];
            }

            // Very basic host/port split (doesn't handle IPv6 brackets)
            if let Some(pos) = auth_rem.rfind(':') {
                // Check if colon is part of IPv6 address (crude check)
                if !auth_rem[..pos].contains(':') {
                    // Likely not IPv6
                    host = Some(&auth_rem[..pos]); // Assign host
                    let port_str = &auth_rem[pos + 1..];
                    if port_str.chars().all(|c| c.is_ascii_digit()) {
                        port = Some(port_str); // Assign port if valid
                    } // If port is invalid, host remains as parsed above, port remains None
                } else {
                    // Assume IPv6 or complex host, treat whole as host
                    host = Some(auth_rem);
                    // port remains None
                }
            } else {
                // No colon found, the whole remaining string is the host
                host = Some(auth_rem);
                // port remains None
            }
        }

        let mut result = String::new();
        result.push_str(&format!("Scheme: {}\n", scheme.unwrap_or("-")));
        result.push_str(&format!("UserInfo: {}\n", userinfo.unwrap_or("-")));
        result.push_str(&format!("Host: {}\n", host.unwrap_or("-")));
        result.push_str(&format!("Port: {}\n", port.unwrap_or("-")));
        result.push_str(&format!(
            "Path: {}\n",
            if path_str.is_empty() { "-" } else { path_str }
        ));
        result.push_str(&format!("Query: {}\n", query.unwrap_or("-")));
        result.push_str(&format!("Fragment: {}", fragment.unwrap_or("-")));

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_component(output: &str, label: &str) -> String {
        output
            .lines()
            .find(|line| line.starts_with(label))
            .map(|line| {
                line.split_once(':')
                    .map(|(_, v)| v.trim().to_string())
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }

    #[test]
    fn test_url_parser_full() {
        let transformer = UrlParser;
        let url =
            "https://user:pass@example.com:8080/path/to/resource?key=value&key2=value2#fragment";
        let result = transformer.transform(url).unwrap();
        assert_eq!(get_component(&result, "Scheme"), "https");
        assert_eq!(get_component(&result, "UserInfo"), "user:pass");
        assert_eq!(get_component(&result, "Host"), "example.com");
        assert_eq!(get_component(&result, "Port"), "8080");
        assert_eq!(get_component(&result, "Path"), "/path/to/resource");
        assert_eq!(get_component(&result, "Query"), "key=value&key2=value2");
        assert_eq!(get_component(&result, "Fragment"), "fragment");
    }

    #[test]
    fn test_url_parser_simple_http() {
        let transformer = UrlParser;
        let url = "http://example.com/home";
        let result = transformer.transform(url).unwrap();
        assert_eq!(get_component(&result, "Scheme"), "http");
        assert_eq!(get_component(&result, "UserInfo"), "-");
        assert_eq!(get_component(&result, "Host"), "example.com");
        assert_eq!(get_component(&result, "Port"), "-");
        assert_eq!(get_component(&result, "Path"), "/home");
        assert_eq!(get_component(&result, "Query"), "-");
        assert_eq!(get_component(&result, "Fragment"), "-");
    }

    #[test]
    fn test_url_parser_ftp() {
        let transformer = UrlParser;
        let url = "ftp://user@ftp.example.org/";
        let result = transformer.transform(url).unwrap();
        assert_eq!(get_component(&result, "Scheme"), "ftp");
        assert_eq!(get_component(&result, "UserInfo"), "user");
        assert_eq!(get_component(&result, "Host"), "ftp.example.org");
        assert_eq!(get_component(&result, "Port"), "-");
        assert_eq!(get_component(&result, "Path"), "/");
        assert_eq!(get_component(&result, "Query"), "-");
        assert_eq!(get_component(&result, "Fragment"), "-");
    }

    #[test]
    fn test_url_parser_mailto() {
        let transformer = UrlParser;
        let url = "mailto:user@example.com";
        let result = transformer.transform(url).unwrap();
        assert_eq!(get_component(&result, "Scheme"), "mailto");
        assert_eq!(get_component(&result, "UserInfo"), "-"); // Corrected
        assert_eq!(get_component(&result, "Host"), "-"); // Corrected
        assert_eq!(get_component(&result, "Port"), "-");
        assert_eq!(get_component(&result, "Path"), "user@example.com"); // Corrected
        assert_eq!(get_component(&result, "Query"), "-");
        assert_eq!(get_component(&result, "Fragment"), "-");
    }

    #[test]
    fn test_url_parser_urn() {
        let transformer = UrlParser;
        let url = "urn:isbn:0451450523";
        let result = transformer.transform(url).unwrap();
        assert_eq!(get_component(&result, "Scheme"), "urn");
        assert_eq!(get_component(&result, "UserInfo"), "-"); // Corrected
        assert_eq!(get_component(&result, "Host"), "-"); // Corrected
        assert_eq!(get_component(&result, "Port"), "-");
        assert_eq!(get_component(&result, "Path"), "isbn:0451450523"); // Corrected
        assert_eq!(get_component(&result, "Query"), "-");
        assert_eq!(get_component(&result, "Fragment"), "-");
    }

    #[test]
    fn test_url_parser_path_only() {
        let transformer = UrlParser;
        let url = "/path/only?query#frag";
        let result = transformer.transform(url).unwrap();
        assert_eq!(get_component(&result, "Scheme"), "-");
        assert_eq!(get_component(&result, "UserInfo"), "-");
        assert_eq!(get_component(&result, "Host"), "-");
        assert_eq!(get_component(&result, "Port"), "-");
        assert_eq!(get_component(&result, "Path"), "/path/only");
        assert_eq!(get_component(&result, "Query"), "query");
        assert_eq!(get_component(&result, "Fragment"), "frag");
    }

    #[test]
    fn test_url_parser_host_port_only() {
        let transformer = UrlParser;
        let url = "example.com:8080"; // No scheme
        let result = transformer.transform(url).unwrap();
        assert_eq!(get_component(&result, "Scheme"), "-");
        assert_eq!(get_component(&result, "UserInfo"), "-");
        assert_eq!(get_component(&result, "Host"), "example.com");
        assert_eq!(get_component(&result, "Port"), "8080");
        assert_eq!(get_component(&result, "Path"), "-");
        assert_eq!(get_component(&result, "Query"), "-");
        assert_eq!(get_component(&result, "Fragment"), "-");
    }

    #[test]
    fn test_url_parser_empty() {
        let transformer = UrlParser;
        assert!(matches!(
            transformer.transform(""),
            Err(TransformError::InvalidArgument(_))
        ));
    }

    #[test]
    fn test_url_parser_invalid_scheme() {
        let transformer = UrlParser;
        assert!(matches!(
            transformer.transform("1http://example.com"),
            Err(TransformError::InvalidArgument(_))
        ));
        assert!(matches!(
            transformer.transform("://example.com"),
            Err(TransformError::InvalidArgument(_))
        ));
    }
}
