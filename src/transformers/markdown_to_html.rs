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
        let mut in_list = false;
        let lines = input.lines().peekable();

        for line in lines {
            // Handle code blocks
            if line.trim().starts_with("```") {
                in_code_block = !in_code_block;
                if in_code_block {
                    html.push_str("<pre><code>");
                } else {
                    html.push_str("</code></pre>\n");
                }
                continue;
            }

            if in_code_block {
                html.push_str(&line.replace("<", "&lt;").replace(">", "&gt;"));
                html.push('\n');
                continue;
            }

            // Handle headers
            let level = line.chars().take_while(|&c| c == '#').count();
            if level > 0 && level <= 6 {
                let content = line[level..].trim();
                html.push_str(&format!("<h{}>{}</h{}>\n", level, content, level));
                continue;
            }

            // Handle lists
            if line.trim().starts_with("- ") || line.trim().starts_with("* ") {
                if !in_list {
                    html.push_str("<ul>\n");
                    in_list = true;
                }
                let content = line.trim()[2..].trim();
                html.push_str(&format!("<li>{}</li>\n", content));
                continue;
            } else if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }

            // Handle bold and italic
            let mut processed_line = String::new();
            let mut chars = line.chars().peekable();
            let mut bold = false;
            let mut italic = false;
            while let Some(c) = chars.next() {
                if c == '*' {
                    if let Some('*') = chars.peek() {
                        chars.next();
                        if bold {
                            processed_line.push_str("</strong>");
                        } else {
                            processed_line.push_str("<strong>");
                        }
                        bold = !bold;
                        continue;
                    } else {
                        if italic {
                            processed_line.push_str("</em>");
                        } else {
                            processed_line.push_str("<em>");
                        }
                        italic = !italic;
                        continue;
                    }
                }
                processed_line.push(c);
            }

            // Handle links
            let mut result = String::new();
            let mut chars = processed_line.chars().peekable();
            while let Some(c) = chars.next() {
                if c == '[' {
                    let mut link_text = String::new();
                    while let Some(&next) = chars.peek() {
                        if next == ']' {
                            chars.next();
                            if let Some(&'(') = chars.peek() {
                                chars.next();
                                let mut url = String::new();
                                while let Some(&next) = chars.peek() {
                                    if next == ')' {
                                        chars.next();
                                        result.push_str(&format!(
                                            "<a href=\"{}\">{}</a>",
                                            url, link_text
                                        ));
                                        break;
                                    }
                                    url.push(chars.next().unwrap());
                                }
                            } else {
                                result.push('[');
                                result.push_str(&link_text);
                                result.push(']');
                            }
                            break;
                        }
                        link_text.push(chars.next().unwrap());
                    }
                } else {
                    result.push(c);
                }
            }

            // Handle paragraphs
            if !result.trim().is_empty() {
                if !result.starts_with("<h")
                    && !result.starts_with("<ul")
                    && !result.starts_with("<li")
                {
                    html.push_str("<p>");
                    html.push_str(&result);
                    html.push_str("</p>\n");
                } else {
                    html.push_str(&result);
                }
            }
        }

        // Close any open tags
        if in_list {
            html.push_str("</ul>\n");
        }
        if in_code_block {
            html.push_str("</code></pre>\n");
        }

        Ok(html)
    }

    fn default_test_input(&self) -> &'static str {
        "# Hello World\n\nThis is a **bold** and *italic* text.\n\n- List item 1\n- List item 2\n\n[Link text](https://example.com)\n\n```\ncode block\n```"
    }
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
}
