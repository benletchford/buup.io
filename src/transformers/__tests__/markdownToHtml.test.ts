import { describe, test, expect } from 'vitest';
import transformers from '../markdownToHtml';

describe('Markdown to HTML transformer', () => {
    const { transform } = transformers.markdowntohtml;

    describe('basic markdown elements', () => {
        test('converts headers', () => {
            expect(transform('# Heading 1')).toBe('<h1>Heading 1</h1>');
            expect(transform('## Heading 2')).toBe('<h2>Heading 2</h2>');
            expect(transform('### Heading 3')).toBe('<h3>Heading 3</h3>');
            expect(transform('#### Heading 4')).toBe('<h4>Heading 4</h4>');
            expect(transform('##### Heading 5')).toBe('<h5>Heading 5</h5>');
            expect(transform('###### Heading 6')).toBe('<h6>Heading 6</h6>');
        });

        test('converts paragraphs', () => {
            expect(transform('This is a paragraph')).toBe('<p>This is a paragraph</p>');
            
            const multiParagraph = `First paragraph

Second paragraph`;
            expect(transform(multiParagraph)).toBe('<p>First paragraph</p>\n\n<p>Second paragraph</p>');
        });

        test('converts emphasis', () => {
            expect(transform('*italic*')).toBe('<p><em>italic</em></p>');
            expect(transform('_italic_')).toBe('<p><em>italic</em></p>');
            expect(transform('**bold**')).toBe('<p><strong>bold</strong></p>');
            expect(transform('__bold__')).toBe('<p><strong>bold</strong></p>');
        });

        test('converts links', () => {
            expect(transform('[Link text](https://example.com)')).toBe('<p><a href="https://example.com">Link text</a></p>');
        });

        test('converts images', () => {
            expect(transform('![Alt text](image.jpg)')).toBe('<p><img alt="Alt text" src="image.jpg"></p>');
        });

        test('converts code blocks', () => {
            const codeBlock = '```\nfunction test() {\n  return true;\n}\n```';
            expect(transform(codeBlock)).toBe('<pre><code>function test() {\n  return true;\n}</code></pre>');
        });

        test('converts inline code', () => {
            expect(transform('Use the `code` function')).toBe('<p>Use the <code>code</code> function</p>');
        });

        test('converts horizontal rules', () => {
            expect(transform('---')).toBe('<hr>');
        });
    });

    describe('lists', () => {
        test('converts unordered lists', () => {
            const list = `- Item 1
- Item 2
- Item 3`;
            expect(transform(list)).toBe('<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>');
        });

        test('converts ordered lists', () => {
            const list = `1. Item 1
2. Item 2
3. Item 3`;
            expect(transform(list)).toBe('<ol><li>Item 1</li><li>Item 2</li><li>Item 3</li></ol>');
        });
    });

    describe('complex markdown', () => {
        test('converts a complex markdown document', () => {
            const markdown = `# Sample Document

This is a paragraph with **bold** and *italic* text.

## Subsection

Here's a [link](https://example.com) and some \`inline code\`.

- List item 1
- List item 2
- List item 3

1. Ordered item 1
2. Ordered item 2
3. Ordered item 3

\`\`\`
function example() {
  return "Hello World";
}
\`\`\`

---

### Another subsection

![Sample image](image.jpg)`;

            const html = transform(markdown);
            
            // Check for key elements in the converted HTML
            expect(html).toContain('<h1>Sample Document</h1>');
            expect(html).toContain('<p>This is a paragraph with <strong>bold</strong> and <em>italic</em> text.</p>');
            expect(html).toContain('<h2>Subsection</h2>');
            expect(html).toContain('<a href="https://example.com">link</a>');
            expect(html).toContain('<code>inline code</code>');
            expect(html).toContain('<ul><li>List item 1</li><li>List item 2</li><li>List item 3</li></ul>');
            expect(html).toContain('<ol><li>Ordered item 1</li><li>Ordered item 2</li><li>Ordered item 3</li></ol>');
            expect(html).toContain('<pre><code>function example() {');
            expect(html).toContain('<hr>');
            expect(html).toContain('<h3>Another subsection</h3>');
            expect(html).toContain('<img alt="Sample image" src="image.jpg">');
        });
    });

    describe('edge cases', () => {
        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('');
            expect(transform('\n\n')).toBe('');
        });

        test('handles plain text without markdown', () => {
            expect(transform('Just plain text')).toBe('<p>Just plain text</p>');
        });
    });
});
