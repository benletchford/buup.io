use crate::{Transform, TransformError, TransformerCategory};

/// HTML to Markdown transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HtmlToMarkdown;

impl Transform for HtmlToMarkdown {
    fn name(&self) -> &'static str {
        "HTML to Markdown"
    }

    fn id(&self) -> &'static str {
        "htmltomarkdown"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Formatter
    }

    fn description(&self) -> &'static str {
        "Converts HTML to Markdown format"
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        let mut markdown = String::new();
        let mut in_code_block = false;
        let mut code_block_content = String::new();
        let lines = input.lines().peekable();

        for mut line in lines {
            // Only trim if not in code block
            if !in_code_block {
                line = line.trim();
            }

            // Handle code blocks
            if let Some(after_tag) = line.strip_prefix("<pre><code>") {
                in_code_block = true;
                code_block_content.clear();
                // If there is code after the tag, capture it
                if !after_tag.is_empty() {
                    code_block_content.push_str(after_tag);
                    code_block_content.push('\n');
                }
                continue;
            } else if line == "</code></pre>" {
                in_code_block = false;
                markdown.push_str("```");
                markdown.push('\n');
                if !code_block_content.is_empty() {
                    markdown.push_str(&code_block_content);
                }
                markdown.push_str("```\n\n");
                continue;
            }

            if in_code_block {
                code_block_content.push_str(line);
                code_block_content.push('\n');
                continue;
            }

            // Handle headers
            let mut is_header = false;
            for i in 1..=6 {
                let tag = format!("<h{}>", i);
                if line.starts_with(&tag) {
                    let content = line[tag.len()..line.len() - tag.len() - 1].trim();
                    markdown.push_str(&format!("{} {}\n\n", "#".repeat(i), content));
                    is_header = true;
                    break;
                }
            }
            if is_header {
                continue;
            }

            // Handle lists
            if line.starts_with("<ul>") {
                continue;
            } else if line == "</ul>" {
                markdown.push('\n');
                continue;
            }

            if line.starts_with("<li>") {
                let content = line[4..line.len() - 5].trim();
                let mut processed_line = content.to_string();
                // Tag replacements for list items
                processed_line = replace_html_tags_with_markdown(&processed_line);
                markdown.push_str(&format!("  {}\n", processed_line));
                continue;
            }

            // Handle paragraphs and other lines
            let mut processed_line = line.to_string();
            processed_line = replace_html_tags_with_markdown(&processed_line);
            if line.starts_with("<p>") && line.ends_with("</p>") {
                let content = &processed_line[3..processed_line.len() - 4].trim();
                if !content.is_empty() {
                    markdown.push_str(&format!("{}\n\n", content));
                }
            } else if !line.is_empty() && !line.starts_with("<") && !line.ends_with(">") {
                markdown.push_str(&format!("{}\n\n", processed_line));
            } else if !processed_line.is_empty() {
                markdown.push_str(&format!("{}\n", processed_line));
            }
        }

        Ok(markdown.trim().to_string())
    }

    fn default_test_input(&self) -> &'static str {
        "<h1>Hello World</h1>\n<p>This is a <strong>bold</strong> and <em>italic</em> text.</p>\n<ul>\n<li>List item 1</li>\n<li>List item 2</li>\n</ul>\n<p><a href=\"https://example.com\">Link text</a></p>\n<pre><code>code block\n</code></pre>"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_to_markdown() {
        let transformer = HtmlToMarkdown;
        let input = "<h1>Title</h1>\n<p>This is <strong>bold</strong> and <em>italic</em>.</p>\n<ul>\n<li>Item 1</li>\n<li>Item 2</li>\n</ul>\n<p><a href=\"https://example.com\">Link</a></p>";
        let expected = "# Title\n\nThis is **bold** and *italic*.\n\n  Item 1\n  Item 2\n\n[Link](https://example.com)";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_code_block() {
        let transformer = HtmlToMarkdown;
        let input = "<pre><code>code here\n</code></pre>";
        let expected = "```\ncode here\n```";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }
}

// Helper function for tag replacements
fn replace_html_tags_with_markdown(input: &str) -> String {
    let mut s = input.to_string();
    // Links
    while let Some(start) = s.find("<a href=\"") {
        if let Some(href_end) = s[start + 9..].find('"') {
            let href_start = start + 9;
            let href_end = href_start + href_end;
            let url = &s[href_start..href_end];
            if let Some(text_start) = s[href_end..].find('>') {
                let text_start = href_end + text_start + 1;
                if let Some(text_end) = s[text_start..].find("</a>") {
                    let text_end = text_start + text_end;
                    let text = &s[text_start..text_end];
                    let replacement = format!("[{}]({})", text, url);
                    s.replace_range(start..text_end + 4, &replacement);
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
    // Bold
    while let Some(start) = s.find("<strong>") {
        if let Some(end) = s[start..].find("</strong>") {
            let content_start = start + 8;
            let content_end = start + end;
            let content = &s[content_start..content_end];
            let replacement = format!("**{}**", content);
            s.replace_range(start..content_end + 9, &replacement);
        } else {
            break;
        }
    }
    while let Some(start) = s.find("<b>") {
        if let Some(end) = s[start..].find("</b>") {
            let content_start = start + 3;
            let content_end = start + end;
            let content = &s[content_start..content_end];
            let replacement = format!("**{}**", content);
            s.replace_range(start..content_end + 4, &replacement);
        } else {
            break;
        }
    }
    // Italic
    while let Some(start) = s.find("<em>") {
        if let Some(end) = s[start..].find("</em>") {
            let content_start = start + 4;
            let content_end = start + end;
            let content = &s[content_start..content_end];
            let replacement = format!("*{}*", content);
            s.replace_range(start..content_end + 5, &replacement);
        } else {
            break;
        }
    }
    while let Some(start) = s.find("<i>") {
        if let Some(end) = s[start..].find("</i>") {
            let content_start = start + 3;
            let content_end = start + end;
            let content = &s[content_start..content_end];
            let replacement = format!("*{}*", content);
            s.replace_range(start..content_end + 4, &replacement);
        } else {
            break;
        }
    }
    s
}
