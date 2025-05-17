use crate::{Transform, TransformError, TransformerCategory};

/// SQL Minifier transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SqlMinifier;

impl Transform for SqlMinifier {
    fn name(&self) -> &'static str {
        "SQL Minifier"
    }

    fn id(&self) -> &'static str {
        "sqlminifier"
    }

    fn description(&self) -> &'static str {
        "Minifies SQL queries by removing unnecessary whitespace and formatting"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Formatter
    }

    fn default_test_input(&self) -> &'static str {
        r#"SELECT id, username, email
FROM users
WHERE status = 'active'
  AND created_at > '2023-01-01'
ORDER BY created_at DESC
LIMIT 10"#
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        // Skip empty input
        if input.trim().is_empty() {
            return Ok(String::new());
        }

        minify_sql(input)
    }
}

/// Minify SQL by removing all unnecessary whitespace while preserving semantics
fn minify_sql(input: &str) -> Result<String, TransformError> {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut in_string = false;
    let mut string_delimiter = '"';
    let mut in_comment = false;
    let mut in_multiline_comment = false;
    let mut last_char = '\0';
    let mut last_token_is_keyword = false;
    let mut current_word = String::new();

    while let Some(c) = chars.next() {
        // Handle string literals (preserve everything inside them)
        if (c == '\'' || c == '"') && !in_comment && !in_multiline_comment {
            if !in_string {
                // Starting a string
                in_string = true;
                string_delimiter = c;
                result.push(c);
            } else if c == string_delimiter {
                // Check for escaped quotes
                if chars.peek() == Some(&c) {
                    // This is an escaped quote within the string
                    result.push(c);
                    chars.next(); // Consume the second quote
                    result.push(c);
                } else {
                    // End of string
                    in_string = false;
                    result.push(c);
                }
            } else {
                // Just a quote character inside a string delimited by a different quote
                result.push(c);
            }
            last_char = c;
            continue;
        }

        // Inside a string - add all characters as-is
        if in_string {
            result.push(c);
            last_char = c;
            continue;
        }

        // Handle single-line comments
        if c == '-' && chars.peek() == Some(&'-') && !in_multiline_comment {
            in_comment = true;
            chars.next(); // consume the second dash

            // Skip the entire comment
            for next_c in chars.by_ref() {
                if next_c == '\n' {
                    in_comment = false;
                    break;
                }
            }
            continue;
        }

        // Skip characters in comment
        if in_comment {
            if c == '\n' {
                in_comment = false;
            }
            continue;
        }

        // Handle multi-line comments
        if c == '/' && chars.peek() == Some(&'*') && !in_comment {
            in_multiline_comment = true;
            chars.next(); // consume the *

            // Skip the entire comment
            let mut asterisk_seen = false;
            for next_c in chars.by_ref() {
                if asterisk_seen && next_c == '/' {
                    in_multiline_comment = false;
                    break;
                }
                asterisk_seen = next_c == '*';
            }
            continue;
        }

        // Skip characters in multi-line comment
        if in_multiline_comment {
            continue;
        }

        // Handle whitespace
        if c.is_whitespace() {
            // Just skip whitespace
            continue;
        }

        // Handle keywords and identifiers
        if c.is_alphabetic() || c == '_' {
            current_word.clear();
            current_word.push(c);

            // Collect the entire word
            while let Some(&next_c) = chars.peek() {
                if next_c.is_alphanumeric() || next_c == '_' {
                    current_word.push(next_c);
                    chars.next();
                } else {
                    break;
                }
            }

            // Check if it's a keyword
            let upper_word = current_word.to_uppercase();
            let is_keyword = is_sql_keyword(&upper_word);

            // Add space before keyword/identifier if needed
            let need_space = (is_keyword || last_token_is_keyword)
                && !result.is_empty()
                && !is_separator(last_char);
            if need_space {
                result.push(' ');
            }

            // Add the word to the result
            if is_keyword {
                result.push_str(&upper_word);
                last_token_is_keyword = true;
            } else {
                result.push_str(&current_word);
                last_token_is_keyword = false;
            }

            last_char = current_word.chars().last().unwrap_or('_');
            continue;
        }

        // Handle separators (punctuation, operators)
        if is_separator(c) {
            // Special case for commas - no space before, but we reset the last token
            if c == ',' {
                result.push(c);
                last_token_is_keyword = false;
            }
            // Special case for operators - no space before but ensure space after
            else if "=<>!+*/".contains(c) {
                // For compound operators like >=, <=, != etc.
                result.push(c);
                if chars.peek() == Some(&'=') {
                    result.push('=');
                    chars.next();
                }
                last_token_is_keyword = false;
            }
            // Other separators
            else {
                result.push(c);
                last_token_is_keyword = false;
            }

            last_char = c;
            continue;
        }

        // Numbers and other characters
        result.push(c);
        last_token_is_keyword = false;
        last_char = c;
    }

    Ok(result)
}

// Check if a character is a separator (punctuation, operator)
fn is_separator(c: char) -> bool {
    "(),;=<>!+-*/".contains(c)
}

// Check if a word is an SQL keyword
fn is_sql_keyword(word: &str) -> bool {
    // Common SQL keywords
    const KEYWORDS: [&str; 59] = [
        "SELECT",
        "FROM",
        "WHERE",
        "INSERT",
        "UPDATE",
        "DELETE",
        "DROP",
        "CREATE",
        "ALTER",
        "TABLE",
        "VIEW",
        "INDEX",
        "TRIGGER",
        "PROCEDURE",
        "FUNCTION",
        "DATABASE",
        "SCHEMA",
        "GRANT",
        "REVOKE",
        "JOIN",
        "INNER",
        "OUTER",
        "LEFT",
        "RIGHT",
        "FULL",
        "CROSS",
        "NATURAL",
        "GROUP",
        "ORDER",
        "BY",
        "HAVING",
        "UNION",
        "ALL",
        "INTERSECT",
        "EXCEPT",
        "INTO",
        "VALUES",
        "SET",
        "AS",
        "ON",
        "AND",
        "OR",
        "NOT",
        "NULL",
        "IS",
        "IN",
        "BETWEEN",
        "LIKE",
        "EXISTS",
        "CASE",
        "WHEN",
        "THEN",
        "ELSE",
        "END",
        "ASC",
        "DESC",
        "LIMIT",
        "OFFSET",
        "WITH",
    ];

    KEYWORDS.contains(&word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_minifier_empty() {
        let transformer = SqlMinifier;
        assert_eq!(transformer.transform("").unwrap(), "");
        assert_eq!(transformer.transform("  ").unwrap(), "");
    }

    #[test]
    fn test_sql_minifier_simple_select() {
        let transformer = SqlMinifier;
        let input = transformer.default_test_input();
        let actual = transformer.transform(input).unwrap();
        assert_eq!(actual, "SELECT id,username,email FROM users WHERE status='active' AND created_at>'2023-01-01' ORDER BY created_at DESC LIMIT10");
    }

    #[test]
    fn test_sql_minifier_complex_query() {
        let transformer = SqlMinifier;
        let input = r#"
        SELECT 
            u.id, 
            u.name, 
            COUNT(o.id) AS order_count
        FROM 
            users u
        LEFT JOIN 
            orders o ON u.id = o.user_id
        WHERE 
            u.status = 'active'
            AND u.created_at > '2023-01-01'
        GROUP BY 
            u.id, 
            u.name
        HAVING 
            COUNT(o.id) > 0
        ORDER BY 
            order_count DESC
        LIMIT 20
        "#;

        let actual = transformer.transform(input).unwrap();
        assert_eq!(actual, "SELECT u.id,u.name,COUNT(o.id)AS order_count FROM usersu LEFT JOIN orderso ON u.id=o.user_id WHERE u.status='active' AND u.created_at>'2023-01-01' GROUP BY u.id,u.name HAVING COUNT(o.id)>0 ORDER BY order_count DESC LIMIT20");
    }

    #[test]
    fn test_sql_minifier_preserves_string_literals() {
        let transformer = SqlMinifier;
        let input = "SELECT * FROM users WHERE name = 'John''s   Data' AND department = \"Sales & Marketing\"";
        let actual = transformer.transform(input).unwrap();
        assert_eq!(
            actual,
            "SELECT*FROM users WHERE name='John''s   Data' AND department=\"Sales & Marketing\""
        );
    }

    #[test]
    fn test_sql_minifier_strips_comments() {
        let transformer = SqlMinifier;
        let input = r#"
        SELECT id, name -- This is the user ID and name
        FROM users 
        /* This is a multi-line comment
         * that spans multiple lines
         */
        WHERE active = 1
        "#;

        let expected = "SELECT id,name FROM users WHERE active=1";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }
}
