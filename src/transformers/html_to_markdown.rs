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
        let mut code_language = String::new();
        let mut in_blockquote = false;
        let mut list_stack = Vec::new(); // Track nested lists and their types (ul, ol)
        let lines = input.lines();

        for mut line in lines {
            // Only trim if not in code block
            if !in_code_block {
                line = line.trim();
            }

            // Handle code blocks
            if let Some(after_tag) = line.strip_prefix("<pre><code") {
                in_code_block = true;
                code_block_content.clear();
                code_language.clear();

                // Check for language class using a more robust approach
                if let Some(class_attr) = after_tag.find("class=") {
                    if let Some(language_part) = after_tag[class_attr..].find("language-") {
                        let language_start = class_attr + language_part + 9; // 9 is the length of "language-"

                        // Find where the language specification ends (at the next quote)
                        if let Some(quote_end) = after_tag[language_start..].find('"') {
                            code_language =
                                after_tag[language_start..language_start + quote_end].to_string();
                        }
                    }
                }

                // Find where the content starts (after closing >)
                if let Some(content_start) = after_tag.find('>') {
                    let content = &after_tag[content_start + 1..];
                    if !content.is_empty() {
                        code_block_content.push_str(content);
                        code_block_content.push('\n');
                    }
                }
                continue;
            } else if line == "</code></pre>" {
                in_code_block = false;
                markdown.push_str("```");
                if !code_language.is_empty() {
                    markdown.push_str(&code_language);
                }
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

            // Handle blockquotes
            if line.starts_with("<blockquote>") {
                in_blockquote = true;

                // Handle one-line blockquote like <blockquote>text</blockquote>
                if line.ends_with("</blockquote>") {
                    let content =
                        line["<blockquote>".len()..line.len() - "</blockquote>".len()].trim();
                    let processed_content = replace_html_tags_with_markdown(content);
                    markdown.push_str(&format!("> {}\n", processed_content));
                    in_blockquote = false;
                }
                continue;
            } else if line.starts_with("<p>") && in_blockquote {
                if let Some(content) = line["<p>".len()..].trim().strip_suffix("</p>") {
                    let processed_content = replace_html_tags_with_markdown(content);
                    markdown.push_str(&format!("> {}\n", processed_content));
                }
                continue;
            } else if line == "</blockquote>" {
                in_blockquote = false;
                markdown.push('\n');
                continue;
            }

            if in_blockquote && !line.starts_with("<") && !line.ends_with(">") {
                let processed_line = replace_html_tags_with_markdown(line);
                markdown.push_str(&format!("> {}\n", processed_line));
                continue;
            }

            // Handle horizontal rule
            if line == "<hr>" || line == "<hr/>" || line == "<hr />" {
                markdown.push_str("---\n\n");
                continue;
            }

            // Handle headers
            let mut is_header = false;
            for i in 1..=6 {
                let tag = format!("<h{}>", i);
                let closing_tag = format!("</h{}>", i);
                if line.starts_with(&tag) && line.ends_with(&closing_tag) {
                    let content = line[tag.len()..line.len() - closing_tag.len()].trim();
                    // Process any HTML tags inside the header
                    let processed_content = replace_html_tags_with_markdown(content);
                    markdown.push_str(&format!("{} {}\n\n", "#".repeat(i), processed_content));
                    is_header = true;
                    break;
                }
            }
            if is_header {
                continue;
            }

            // Handle lists
            if line.starts_with("<ul>") {
                list_stack.push(("ul", 0));
                continue;
            } else if line.starts_with("<ol>") {
                list_stack.push(("ol", 0));
                continue;
            } else if line == "</ul>" || line == "</ol>" {
                if !list_stack.is_empty() {
                    list_stack.pop();
                }
                if list_stack.is_empty() {
                    markdown.push('\n');
                }
                continue;
            }

            if line.starts_with("<li>") && line.ends_with("</li>") {
                let content = line[4..line.len() - 5].trim();
                let processed_line = replace_html_tags_with_markdown(content);

                // Get current list indentation level and type
                let indent = list_stack.len().saturating_sub(1) * 2;
                let list_marker = if !list_stack.is_empty() && list_stack.last().unwrap().0 == "ol"
                {
                    // For ordered lists, increment counter
                    let last_idx = list_stack.len() - 1;
                    let (list_type, count) = list_stack[last_idx];
                    let new_count = count + 1;
                    list_stack[last_idx] = (list_type, new_count);
                    format!("{}. ", new_count)
                } else {
                    "- ".to_string()
                };

                markdown.push_str(&format!(
                    "{}{}{}\n",
                    " ".repeat(indent),
                    list_marker,
                    processed_line
                ));
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
            } else if !(processed_line.is_empty()
                || (processed_line.starts_with("<") && processed_line.ends_with(">")))
            {
                markdown.push_str(&format!("{}\n", processed_line));
            }
        }

        Ok(markdown.trim().to_string())
    }

    fn default_test_input(&self) -> &'static str {
        "<h1>Hello World</h1>\n<p>This is a <strong>bold</strong> and <em>italic</em> text.</p>\n<ul>\n<li>List item 1</li>\n<li>List item 2</li>\n</ul>\n<ol>\n<li>Ordered item 1</li>\n<li>Ordered item 2</li>\n</ol>\n<p><a href=\"https://example.com\">Link text</a></p>\n<blockquote><p>A blockquote</p></blockquote>\n<hr>\n<pre><code class=\"language-rust\">fn main() {\n    println!(\"Hello, world!\");\n}\n</code></pre>"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_to_markdown() {
        let transformer = HtmlToMarkdown;
        let input = "<h1>Title</h1>\n<p>This is <strong>bold</strong> and <em>italic</em>.</p>\n<ul>\n<li>Item 1</li>\n<li>Item 2</li>\n</ul>\n<p><a href=\"https://example.com\">Link</a></p>";
        let expected = "# Title\n\nThis is **bold** and *italic*.\n\n- Item 1\n- Item 2\n\n[Link](https://example.com)";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_code_block() {
        let transformer = HtmlToMarkdown;
        let input = "<pre><code>code here\n</code></pre>";
        let expected = "```\ncode here\n```";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_code_block_with_language() {
        let transformer = HtmlToMarkdown;
        let input = "<pre><code class=\"language-rust\">fn main() {\n    println!(\"Hello!\");\n}\n</code></pre>";
        let expected = "```rust\nfn main() {\n    println!(\"Hello!\");\n}\n```";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_ordered_list() {
        let transformer = HtmlToMarkdown;
        let input = "<ol>\n<li>First item</li>\n<li>Second item</li>\n</ol>";
        let expected = "1. First item\n2. Second item";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_blockquote() {
        let transformer = HtmlToMarkdown;
        let input = "<blockquote>This is a quote</blockquote>";
        let expected = "> This is a quote";
        assert_eq!(transformer.transform(input).unwrap(), expected);
    }

    #[test]
    fn test_horizontal_rule() {
        let transformer = HtmlToMarkdown;
        let input = "<p>Before</p>\n<hr>\n<p>After</p>";
        let expected = "Before\n\n---\n\nAfter";
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
    // Strikethrough
    while let Some(start) = s.find("<s>") {
        if let Some(end) = s[start..].find("</s>") {
            let content_start = start + 3;
            let content_end = start + end;
            let content = &s[content_start..content_end];
            let replacement = format!("~~{}~~", content);
            s.replace_range(start..content_end + 4, &replacement);
        } else {
            break;
        }
    }
    while let Some(start) = s.find("<del>") {
        if let Some(end) = s[start..].find("</del>") {
            let content_start = start + 5;
            let content_end = start + end;
            let content = &s[content_start..content_end];
            let replacement = format!("~~{}~~", content);
            s.replace_range(start..content_end + 6, &replacement);
        } else {
            break;
        }
    }
    // Inline code
    while let Some(start) = s.find("<code>") {
        if let Some(end) = s[start..].find("</code>") {
            let content_start = start + 6;
            let content_end = start + end;
            let content = &s[content_start..content_end];
            let replacement = format!("`{}`", content);
            s.replace_range(start..content_end + 7, &replacement);
        } else {
            break;
        }
    }
    s
}
