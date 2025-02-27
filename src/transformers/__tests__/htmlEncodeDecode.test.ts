import { describe, test, expect } from 'vitest';
import transformers from '../htmlEncodeDecode';

describe('HTML encode/decode transformers', () => {
    describe('HTML encode', () => {
        const { transform } = transformers.htmlencode;

        test('encodes HTML special characters', () => {
            expect(transform('<div>Hello & World</div>')).toBe('&lt;div&gt;Hello &amp; World&lt;/div&gt;');
        });

        test('encodes quotes', () => {
            expect(transform('Text with "double" and \'single\' quotes')).toBe('Text with &quot;double&quot; and &#39;single&#39; quotes');
        });

        test('encodes multiple special characters', () => {
            const input = '<a href="https://example.com?param=value&another=value">Link</a>';
            const expected = '&lt;a href=&quot;https://example.com?param=value&amp;another=value&quot;&gt;Link&lt;/a&gt;';
            expect(transform(input)).toBe(expected);
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('  ');
        });

        test('handles text without special characters', () => {
            expect(transform('Hello World')).toBe('Hello World');
        });
    });

    describe('HTML decode', () => {
        const { transform } = transformers.htmldecode;

        test('decodes HTML special characters', () => {
            expect(transform('&lt;div&gt;Hello &amp; World&lt;/div&gt;')).toBe('<div>Hello & World</div>');
        });

        test('decodes quotes', () => {
            expect(transform('Text with &quot;double&quot; and &#39;single&#39; quotes')).toBe('Text with "double" and \'single\' quotes');
        });

        test('decodes multiple special characters', () => {
            const input = '&lt;a href=&quot;https://example.com?param=value&amp;another=value&quot;&gt;Link&lt;/a&gt;';
            const expected = '<a href="https://example.com?param=value&another=value">Link</a>';
            expect(transform(input)).toBe(expected);
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('  ');
        });

        test('handles text without entities', () => {
            expect(transform('Hello World')).toBe('Hello World');
        });

        test('handles partial entities', () => {
            expect(transform('This is &amp and &lt')).toBe('This is &amp and &lt');
        });
    });

    describe('transformer relationships', () => {
        test('transformers are inverses of each other', () => {
            const plainText = '<div>Hello & World</div> Text with "double" and \'single\' quotes';
            const encodedText = '&lt;div&gt;Hello &amp; World&lt;/div&gt; Text with &quot;double&quot; and &#39;single&#39; quotes';

            const encode = transformers.htmlencode.transform;
            const decode = transformers.htmldecode.transform;

            // plain -> encoded -> plain should return original
            expect(decode(encode(plainText))).toBe(plainText);

            // encoded -> plain -> encoded should return original
            expect(encode(decode(encodedText))).toBe(encodedText);
        });

        test('inverse property is correctly set', () => {
            expect(transformers.htmlencode.inverse).toBe('htmldecode');
            expect(transformers.htmldecode.inverse).toBe('htmlencode');
        });
    });
});
