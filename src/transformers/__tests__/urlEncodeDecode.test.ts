import { describe, test, expect } from 'vitest';
import transformers from '../urlEncodeDecode';

describe('URL encode/decode transformers', () => {
    describe('URL encode', () => {
        const { transform } = transformers.urlencode;

        test('encodes URL special characters', () => {
            expect(transform('Hello World!')).toBe('Hello%20World%21');
        });

        test('encodes URL reserved characters', () => {
            expect(transform('key=value&param=test')).toBe('key%3Dvalue%26param%3Dtest');
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('  ');
        });

        test('handles special characters', () => {
            expect(transform('こんにちは')).toBe('%E3%81%93%E3%82%93%E3%81%AB%E3%81%A1%E3%81%AF');
        });
    });

    describe('URL decode', () => {
        const { transform } = transformers.urldecode;

        test('decodes URL special characters', () => {
            expect(transform('Hello%20World%21')).toBe('Hello World!');
        });

        test('decodes URL reserved characters', () => {
            expect(transform('key%3Dvalue%26param%3Dtest')).toBe('key=value&param=test');
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('  ');
        });

        test('handles special characters', () => {
            expect(transform('%E3%81%93%E3%82%93%E3%81%AB%E3%81%A1%E3%81%AF')).toBe('こんにちは');
        });

        test('handles invalid input', () => {
            expect(transform('%invalid')).toBe('Invalid URL-encoded input');
        });
    });

    describe('transformer relationships', () => {
        test('transformers are inverses of each other', () => {
            const plainText = 'Hello World! key=value&param=test こんにちは';
            const encodedText = 'Hello%20World%21%20key%3Dvalue%26param%3Dtest%20%E3%81%93%E3%82%93%E3%81%AB%E3%81%A1%E3%81%AF';

            const encode = transformers.urlencode.transform;
            const decode = transformers.urldecode.transform;

            // plain -> encoded -> plain should return original
            expect(decode(encode(plainText))).toBe(plainText);

            // encoded -> plain -> encoded should return original
            expect(encode(decode(encodedText))).toBe(encodedText);
        });

        test('inverse property is correctly set', () => {
            expect(transformers.urlencode.inverse).toBe('urldecode');
            expect(transformers.urldecode.inverse).toBe('urlencode');
        });
    });
});
