use crate::{Transform, TransformError, TransformerCategory};

/// Markdown to HTML transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarkdownToHtml;

impl Transform for MarkdownToHtml {
    fn name(&self) -> &'static str {
        "Markdown to HTML"
    }

    fn id(&self) -> &'static str {
        "markdowntohtml"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Formatter
    }

    fn description(&self) -> &'static str {
        "Converts Markdown text to HTML format"
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let mut html = String::new();
        let mut in_code_block = false;
        let mut code_language = String::new();
        let mut in_list = false;
        let mut in_ordered_list = false;
        let mut in_blockquote = false;
        let lines = input.lines();

        for line in lines {
            // Handle code blocks
            if line.trim().starts_with("```") {
                if in_code_block {
                    html.push_str("</code></pre>\n");
                    in_code_block = false;
                    code_language.clear();
                } else {
                    in_code_block = true;
                    code_language.clear();
                    // Extract language if specified
                    let language_start = line.trim_start().chars().skip(3).collect::<String>();
                    if !language_start.is_empty() {
                        code_language = language_start.trim().to_string();
                        if !code_language.is_empty() {
                            html.push_str(&format!(
                                "<pre><code class=\"language-{}\">",
                                code_language
                            ));
                        } else {
                            html.push_str("<pre><code>");
                        }
                    } else {
                        html.push_str("<pre><code>");
                    }
                }
                continue;
            }

            if in_code_block {
                html.push_str(&line.replace('<', "&lt;").replace('>', "&gt;"));
                html.push('\n');
                continue;
            }

            // Handle horizontal rules
            if line.trim() == "---" || line.trim() == "***" || line.trim() == "___" {
                html.push_str("<hr>\n");
                continue;
            }

            // Handle blockquotes
            if line.trim().starts_with('>') {
                if !in_blockquote {
                    html.push_str("<blockquote>\n");
                    in_blockquote = true;
                }
                let content = line.trim()[1..].trim_start();
                let processed_content = process_inline_markdown(content);
                html.push_str(&format!("<p>{}</p>\n", processed_content));
                continue;
            } else if in_blockquote && line.trim().is_empty() {
                html.push_str("</blockquote>\n");
                in_blockquote = false;
                continue;
            }

            // Handle headers
            let level = line.chars().take_while(|&c| c == '#').count();
            if level > 0 && level <= 6 && line.chars().nth(level) == Some(' ') {
                let content = line[level..].trim();
                let processed_content = process_inline_markdown(content);
                html.push_str(&format!("<h{}>{}</h{}>\n", level, processed_content, level));
                continue;
            }

            // Handle ordered lists
            if let Some(content) = line.trim().strip_prefix("1. ") {
                if !in_ordered_list {
                    if in_list {
                        html.push_str("</ul>\n");
                        in_list = false;
                    }
                    html.push_str("<ol>\n");
                    in_ordered_list = true;
                }
                let processed_content = process_inline_markdown(content);
                html.push_str(&format!("<li>{}</li>\n", processed_content));
                continue;
            } else if in_ordered_list && line.trim().len() >= 3 {
                // Check for any number followed by a dot and space (e.g., "2. ", "10. ")
                let parts: Vec<&str> = line.trim().splitn(2, ". ").collect();
                if parts.len() == 2 && parts[0].parse::<usize>().is_ok() {
                    let processed_content = process_inline_markdown(parts[1]);
                    html.push_str(&format!("<li>{}</li>\n", processed_content));
                    continue;
                } else if in_ordered_list {
                    html.push_str("</ol>\n");
                    in_ordered_list = false;
                }
            } else if in_ordered_list && line.trim().is_empty() {
                html.push_str("</ol>\n");
                in_ordered_list = false;
            }

            // Handle unordered lists
            if line.trim().starts_with("- ") || line.trim().starts_with("* ") {
                if !in_list {
                    if in_ordered_list {
                        html.push_str("</ol>\n");
                        in_ordered_list = false;
                    }
                    html.push_str("<ul>\n");
                    in_list = true;
                }
                let marker_len = 2; // Both "- " and "* " are 2 chars long
                let content = line.trim()[marker_len..].trim();
                let processed_content = process_inline_markdown(content);
                html.push_str(&format!("<li>{}</li>\n", processed_content));
                continue;
            } else if in_list && line.trim().is_empty() {
                html.push_str("</ul>\n");
                in_list = false;
                continue;
            }

            // Handle paragraphs
            if !line.trim().is_empty() {
                let processed_line = process_inline_markdown(line);

                // Skip adding paragraph tags around certain elements that are already block-level
                if !processed_line.starts_with("<h")
                    && !processed_line.starts_with("<ul")
                    && !processed_line.starts_with("<ol")
                    && !processed_line.starts_with("<li")
                    && !processed_line.starts_with("<blockquote")
                {
                    html.push_str("<p>");
                    html.push_str(&processed_line);
                    html.push_str("</p>\n");
                } else {
                    html.push_str(&processed_line);
                    html.push('\n');
                }
            } else if !in_list && !in_ordered_list && !in_blockquote && !line.trim().is_empty() {
                html.push('\n');
            }
        }

        // Close any open tags
        if in_list {
            html.push_str("</ul>\n");
        }
        if in_ordered_list {
            html.push_str("</ol>\n");
        }
        if in_blockquote {
            html.push_str("</blockquote>\n");
        }
        if in_code_block {
            html.push_str("</code></pre>\n");
        }

        Ok(html)
    }

    fn default_test_input(&self) -> &'static str {
        "# Hello World\n\nThis is a **bold** and *italic* text with ~~strikethrough~~ and `inline code`.\n\n- List item 1\n- List item 2\n\n1. Ordered item 1\n2. Ordered item 2\n\n> This is a blockquote\n\n[Link text](https://example.com)\n\n---\n\n```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```"
    }
}

// Helper function to process inline Markdown elements
fn process_inline_markdown(input: &str) -> String {
    let mut result = input.to_string();

    // Process inline code (backticks)
    while let Some(start) = result.find('`') {
        if let Some(end) = result[start + 1..].find('`') {
            let code_content = &result[start + 1..start + 1 + end];
            let code_html = format!("<code>{}</code>", code_content);
            result.replace_range(start..=start + 1 + end, &code_html);
        } else {
            break;
        }
    }

    // Process bold (double asterisks)
    while let Some(start) = result.find("**") {
        if let Some(end) = result[start + 2..].find("**") {
            let bold_content = &result[start + 2..start + 2 + end];
            let bold_html = format!("<strong>{}</strong>", bold_content);
            result.replace_range(start..=start + 2 + end + 1, &bold_html);
        } else {
            break;
        }
    }

    // Process italic (single asterisk)
    while let Some(start) = result.find('*') {
        if let Some(end) = result[start + 1..].find('*') {
            let italic_content = &result[start + 1..start + 1 + end];
            let italic_html = format!("<em>{}</em>", italic_content);
            result.replace_range(start..=start + 1 + end, &italic_html);
        } else {
            break;
        }
    }

    // Process strikethrough (double tilde)
    while let Some(start) = result.find("~~") {
        if let Some(end) = result[start + 2..].find("~~") {
            let strike_content = &result[start + 2..start + 2 + end];
            let strike_html = format!("<del>{}</del>", strike_content);
            result.replace_range(start..=start + 2 + end + 1, &strike_html);
        } else {
            break;
        }
    }

    // Process links
    while let Some(start) = result.find('[') {
        if let Some(text_end) = result[start..].find(']') {
            let text_end = start + text_end;
            if result.len() > text_end + 1 && result.as_bytes()[text_end + 1] == b'(' {
                if let Some(url_end) = result[text_end + 1..].find(')') {
                    let url_end = text_end + 1 + url_end;
                    let link_text = &result[start + 1..text_end];
                    let url = &result[text_end + 2..url_end];
                    let link_html = format!("<a href=\"{}\">{}</a>", url, link_text);
                    result.replace_range(start..=url_end, &link_html);
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    // Sanitize angle brackets for HTML entities, but preserve HTML tags we've already created
    let mut final_result = String::new();
    let mut i = 0;
    let bytes = result.as_bytes();

    while i < bytes.len() {
        // Check for HTML tag start
        if bytes[i] == b'<' && i + 1 < bytes.len() {
            if is_start_of_html_tag(&bytes[i + 1..]) {
                // This is an HTML tag, add it as is
                final_result.push('<');
                i += 1;

                // Add characters until we reach the end of tag
                while i < bytes.len() && bytes[i] != b'>' {
                    final_result.push(bytes[i] as char);
                    i += 1;
                }

                if i < bytes.len() {
                    final_result.push('>');
                    i += 1;
                }
            } else {
                // Not an HTML tag, escape it
                final_result.push_str("&lt;");
                i += 1;
            }
        } else if bytes[i] == b'>' && (i == 0 || bytes[i - 1] != b'/') {
            // Only escape '>' that are not part of a closing tag
            let preceding_is_tag = i >= 2 && bytes[i - 1] == b'/' && bytes[i - 2] == b'<';
            if !preceding_is_tag {
                final_result.push_str("&gt;");
            } else {
                final_result.push('>');
            }
            i += 1;
        } else {
            final_result.push(bytes[i] as char);
            i += 1;
        }
    }

    final_result
}

// Helper function to determine if we're at the start of an HTML tag
fn is_start_of_html_tag(bytes: &[u8]) -> bool {
    let html_tags = &[
        b"a " as &[u8],
        b"a>" as &[u8],
        b"a href" as &[u8],
        b"/a>" as &[u8],
        b"strong" as &[u8],
        b"/strong" as &[u8],
        b"em" as &[u8],
        b"/em" as &[u8],
        b"del" as &[u8],
        b"/del" as &[u8],
        b"code" as &[u8],
        b"/code" as &[u8],
        b"p>" as &[u8],
        b"/p>" as &[u8],
    ];

    for &tag in html_tags {
        if bytes.len() >= tag.len() && bytes[..tag.len()] == *tag {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_to_html() {
        let transformer = MarkdownToHtml;
        let input = "# Title\n\nThis is **bold** and *italic*.\n\n- Item 1\n- Item 2\n\n[Link](https://example.com)";
        let expected = "<h1>Title</h1>\n<p>This is <strong>bold</strong> and <em>italic</em>.</p>\n<ul>\n<li>Item 1</li>\n<li>Item 2</li>\n</ul>\n<p><a href=\"https://example.com\">Link</a></p>\n";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_code_block() {
        let transformer = MarkdownToHtml;
        let input = "```\ncode here\n```";
        let expected = "<pre><code>code here\n</code></pre>\n";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_code_block_with_language() {
        let transformer = MarkdownToHtml;
        let input = "```rust\nfn main() {\n    println!(\"Hello!\");\n}\n```";
        let expected = "<pre><code class=\"language-rust\">fn main() {\n    println!(\"Hello!\");\n}\n</code></pre>\n";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_ordered_list() {
        let transformer = MarkdownToHtml;
        let input = "1. First item\n2. Second item";
        let expected = "<ol>\n<li>First item</li>\n<li>Second item</li>\n</ol>\n";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_blockquote() {
        let transformer = MarkdownToHtml;
        let input = "> This is a quote";
        let expected = "<blockquote>\n<p>This is a quote</p>\n</blockquote>\n";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_horizontal_rule() {
        let transformer = MarkdownToHtml;
        let input = "Before\n\n---\n\nAfter";
        let expected = "<p>Before</p>\n<hr>\n<p>After</p>\n";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_strikethrough() {
        let transformer = MarkdownToHtml;
        let input = "This is ~~strikethrough~~ text";
        let expected = "<p>This is <del>strikethrough</del> text</p>\n";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_inline_code() {
        let transformer = MarkdownToHtml;
        let input = "This is `inline code` text";
        let expected = "<p>This is <code>inline code</code> text</p>\n";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }
}
