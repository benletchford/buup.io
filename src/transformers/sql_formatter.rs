use crate::{Transform, TransformError, TransformerCategory};

/// SQL Formatter transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SqlFormatter;

impl Transform for SqlFormatter {
    fn name(&self) -> &'static str {
        "SQL Formatter"
    }

    fn id(&self) -> &'static str {
        "sqlformatter"
    }

    fn description(&self) -> &'static str {
        "Formats SQL queries with proper indentation and spacing"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Formatter
    }

    fn default_test_input(&self) -> &'static str {
        "SELECT id, username, email FROM users WHERE status = 'active' AND created_at > '2023-01-01' ORDER BY created_at DESC LIMIT 10"
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        // Skip empty input
        if input.trim().is_empty() {
            return Ok(String::new());
        }

        format_sql(input)
    }
}

enum SqlTokenType {
    Keyword,
    Identifier,
    String,
    Number,
    Operator,
    Punctuation,
    Whitespace,
    Parenthesis,
}

// Keywords that should be on their own line
const NEWLINE_KEYWORDS: [&str; 16] = [
    "FROM",
    "WHERE",
    "LEFT JOIN",
    "RIGHT JOIN",
    "INNER JOIN",
    "OUTER JOIN",
    "FULL JOIN",
    "CROSS JOIN",
    "JOIN",
    "GROUP BY",
    "HAVING",
    "ORDER BY",
    "LIMIT",
    "UNION",
    "UNION ALL",
    "INTERSECT",
];

// Keywords that start a new logical section
const MAJOR_KEYWORDS: [&str; 7] = [
    "SELECT", "INSERT", "UPDATE", "DELETE", "CREATE", "ALTER", "DROP",
];

// Format SQL query with proper indentation and spacing
fn format_sql(input: &str) -> Result<String, TransformError> {
    let mut result = String::with_capacity(input.len() * 2);
    let mut input_chars = input.chars().peekable();
    let mut indent_level: usize = 0;
    let mut at_beginning_of_line = true;
    let mut previous_token_type = SqlTokenType::Whitespace;
    let mut buffer = String::new();
    let mut in_string = false;
    let mut string_quote_char = '"';
    let mut in_comment = false;
    let mut in_multiline_comment = false;
    let mut pending_whitespace = false;

    while let Some(c) = input_chars.next() {
        // Handle strings (quoted literals)
        if (c == '\'' || c == '"') && !in_comment && !in_multiline_comment {
            if !in_string {
                // Starting a string
                in_string = true;
                string_quote_char = c;

                // Add a space before string if needed
                if !matches!(
                    previous_token_type,
                    SqlTokenType::Whitespace | SqlTokenType::Operator | SqlTokenType::Parenthesis
                ) {
                    result.push(' ');
                }

                result.push(c);
            } else if c == string_quote_char {
                // Check for escaped quotes
                if input_chars.peek() == Some(&c) {
                    // This is an escaped quote within the string
                    result.push(c);
                    input_chars.next(); // Consume the second quote
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
            previous_token_type = SqlTokenType::String;
            continue;
        }

        // Inside a string - add all characters as-is
        if in_string {
            result.push(c);
            continue;
        }

        // Handle single-line comments
        if c == '-' && input_chars.peek() == Some(&'-') && !in_multiline_comment {
            in_comment = true;
            if !at_beginning_of_line {
                result.push(' ');
            }
            result.push(c);
            continue;
        }

        if in_comment {
            result.push(c);
            if c == '\n' {
                in_comment = false;
                at_beginning_of_line = true;

                // Apply indentation at beginning of line
                result.push_str(&"    ".repeat(indent_level));
            }
            continue;
        }

        // Handle multi-line comments
        if c == '/' && input_chars.peek() == Some(&'*') && !in_comment {
            in_multiline_comment = true;
            if !at_beginning_of_line {
                result.push(' ');
            }
            result.push(c);
            continue;
        }

        if in_multiline_comment {
            result.push(c);
            if c == '*' && input_chars.peek() == Some(&'/') {
                input_chars.next(); // Consume the '/'
                result.push('/');
                in_multiline_comment = false;
            }
            continue;
        }

        // Handle whitespace
        if c.is_whitespace() {
            if at_beginning_of_line && c != '\n' {
                // Skip leading whitespace
                continue;
            }

            if c == '\n' {
                // Handle newlines
                if !at_beginning_of_line {
                    result.push('\n');
                    at_beginning_of_line = true;

                    // Apply indentation at beginning of new line
                    result.push_str(&"    ".repeat(indent_level));
                }
            } else if !at_beginning_of_line {
                // Collapse multiple spaces into one
                pending_whitespace = true;
            }

            previous_token_type = SqlTokenType::Whitespace;
            continue;
        }

        // Handle parentheses
        if c == '(' {
            if pending_whitespace && !at_beginning_of_line {
                result.push(' ');
            }
            pending_whitespace = false;

            result.push(c);
            indent_level += 1;

            // Add newline after opening parenthesis
            result.push('\n');

            // Apply indentation for the next line
            result.push_str(&"    ".repeat(indent_level));

            // We are now at the beginning of a line
            at_beginning_of_line = true;

            previous_token_type = SqlTokenType::Parenthesis;
            continue;
        }

        if c == ')' {
            pending_whitespace = false;

            // Add newline before closing parenthesis if not at the beginning of a line
            if !at_beginning_of_line {
                result.push('\n');
            }

            indent_level = indent_level.saturating_sub(1);

            // Apply indentation for the closing parenthesis
            if at_beginning_of_line {
                // Remove previous indentation and apply the updated one
                result.truncate(result.rfind('\n').map(|pos| pos + 1).unwrap_or(0));
            }

            result.push_str(&"    ".repeat(indent_level));
            result.push(c);

            previous_token_type = SqlTokenType::Parenthesis;
            at_beginning_of_line = false;
            continue;
        }

        // Handle punctuation and operators
        if c == ',' {
            result.push(c);

            // For SELECT statements, add newline after comma
            result.push('\n');
            at_beginning_of_line = true;

            // Apply indentation for the next line
            result.push_str(&"    ".repeat(indent_level));

            previous_token_type = SqlTokenType::Punctuation;
            continue;
        }

        if "+-*/=%<>!|&".contains(c) {
            if pending_whitespace {
                result.push(' ');
            }
            pending_whitespace = false;

            result.push(c);

            // Add space after operator (but not before checking for multi-char operators)
            if !matches!(input_chars.peek(), Some(&'=') | Some(&'>') | Some(&'<')) {
                result.push(' ');
            }

            previous_token_type = SqlTokenType::Operator;
            at_beginning_of_line = false;
            continue;
        }

        // Handle keywords and identifiers
        if c.is_alphabetic() || c == '_' || c == '@' || c == '#' || c == '$' {
            buffer.clear();
            buffer.push(c);

            // Collect the entire identifier or keyword
            while let Some(&next_c) = input_chars.peek() {
                if next_c.is_alphanumeric()
                    || next_c == '_'
                    || next_c == '@'
                    || next_c == '#'
                    || next_c == '$'
                {
                    buffer.push(next_c);
                    input_chars.next();
                } else {
                    break;
                }
            }

            // Check if it's a keyword
            let upper_buffer = buffer.to_uppercase();
            let is_keyword = is_sql_keyword(&upper_buffer);

            // Handle keyword formatting
            if is_keyword {
                // Determine if we need a newline before this keyword
                let needs_newline = NEWLINE_KEYWORDS.contains(&upper_buffer.as_str())
                    || (MAJOR_KEYWORDS.contains(&upper_buffer.as_str()) && !at_beginning_of_line);

                if needs_newline && !at_beginning_of_line {
                    result.push('\n');

                    // Apply indentation for this line
                    result.push_str(&"    ".repeat(indent_level));
                } else if pending_whitespace && !at_beginning_of_line {
                    result.push(' ');
                }

                pending_whitespace = false;

                // Add the keyword in uppercase
                result.push_str(&upper_buffer);

                // Make sure there's a space after keywords
                result.push(' ');

                previous_token_type = SqlTokenType::Keyword;
            } else {
                // It's an identifier
                if pending_whitespace && !at_beginning_of_line {
                    result.push(' ');
                }
                pending_whitespace = false;

                // Add the identifier as-is
                result.push_str(&buffer);

                previous_token_type = SqlTokenType::Identifier;
            }

            at_beginning_of_line = false;
            continue;
        }

        // Handle numbers
        if c.is_numeric() || (c == '.' && input_chars.peek().is_some_and(|p| p.is_numeric())) {
            if pending_whitespace && !at_beginning_of_line {
                result.push(' ');
            }
            pending_whitespace = false;

            result.push(c);

            // Collect the rest of the number
            while let Some(&next_c) = input_chars.peek() {
                if next_c.is_numeric() || next_c == '.' {
                    result.push(next_c);
                    input_chars.next();
                } else {
                    break;
                }
            }

            previous_token_type = SqlTokenType::Number;
            at_beginning_of_line = false;
            continue;
        }

        // Handle any other characters
        if pending_whitespace && !at_beginning_of_line {
            result.push(' ');
        }
        pending_whitespace = false;

        result.push(c);
        at_beginning_of_line = false;

        // Most likely punctuation
        previous_token_type = SqlTokenType::Punctuation;
    }

    Ok(result)
}

// Check if a word is a SQL keyword
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
    fn test_sql_formatter_empty() {
        let transformer = SqlFormatter;
        assert_eq!(transformer.transform("").unwrap(), "");
        assert_eq!(transformer.transform("  ").unwrap(), "");
    }

    #[test]
    fn test_sql_formatter_simple_select() {
        let transformer = SqlFormatter;
        let input = "SELECT id, name, email FROM users WHERE active = true ORDER BY name";

        // Test against the exact output format
        let expected =
            "SELECT  id,\nname,\nemail\nFROM  users\nWHERE  active =  true ORDER  BY  name";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_sql_formatter_joins() {
        let transformer = SqlFormatter;
        let input = "SELECT u.id, u.name, o.order_date FROM users u JOIN orders o ON u.id = o.user_id WHERE o.total > 100";

        // Test against the exact output format
        let expected = "SELECT  u.id,\nu.name,\no.order_date\nFROM  users u\nJOIN  orders o ON  u.id =  o.user_id\nWHERE  o.total >  100";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_sql_formatter_nested_queries() {
        let transformer = SqlFormatter;
        let input = "SELECT * FROM (SELECT id, COUNT(*) as count FROM orders GROUP BY id) AS subquery WHERE count > 5";

        // Test against the exact output format
        let expected = "SELECT  * \nFROM  (\n    SELECT  id,\n    COUNT(\n        * \n    ) AS  count\n    FROM  orders GROUP  BY  id\n) AS  subquery\nWHERE  count >  5";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_sql_formatter_string_literals() {
        let transformer = SqlFormatter;
        let input = "SELECT * FROM users WHERE name = 'John''s' AND department = \"Sales\"";

        // Test against the exact output format
        let expected =
            "SELECT  * \nFROM  users\nWHERE  name = 'John''s' AND  department = \"Sales\"";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }
}
