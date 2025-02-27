import { describe, test, expect } from 'vitest';
import transformers from '../jsonFormatMinify';

describe('JSON format/minify transformers', () => {
    describe('JSON format', () => {
        const { transform } = transformers.jsonformat;

        test('formats JSON object', () => {
            const input = '{"name":"John","age":30,"city":"New York"}';
            const expected = `{
  "name": "John",
  "age": 30,
  "city": "New York"
}`;
            expect(transform(input)).toBe(expected);
        });

        test('formats JSON array', () => {
            const input = '[1,2,3,4,5]';
            const expected = `[
  1,
  2,
  3,
  4,
  5
]`;
            expect(transform(input)).toBe(expected);
        });

        test('formats nested JSON', () => {
            const input = '{"person":{"name":"John","age":30},"hobbies":["reading","coding"]}';
            const expected = `{
  "person": {
    "name": "John",
    "age": 30
  },
  "hobbies": [
    "reading",
    "coding"
  ]
}`;
            expect(transform(input)).toBe(expected);
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('');
        });

        test('handles invalid JSON', () => {
            expect(transform('{"invalid": json')).toBe('Invalid JSON input');
        });
    });

    describe('JSON minify', () => {
        const { transform } = transformers.jsonminify;

        test('minifies JSON object', () => {
            const input = `{
  "name": "John",
  "age": 30,
  "city": "New York"
}`;
            const expected = '{"name":"John","age":30,"city":"New York"}';
            expect(transform(input)).toBe(expected);
        });

        test('minifies JSON array', () => {
            const input = `[
  1,
  2,
  3,
  4,
  5
]`;
            const expected = '[1,2,3,4,5]';
            expect(transform(input)).toBe(expected);
        });

        test('minifies nested JSON', () => {
            const input = `{
  "person": {
    "name": "John",
    "age": 30
  },
  "hobbies": [
    "reading",
    "coding"
  ]
}`;
            const expected = '{"person":{"name":"John","age":30},"hobbies":["reading","coding"]}';
            expect(transform(input)).toBe(expected);
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('');
        });

        test('handles invalid JSON', () => {
            expect(transform('{"invalid": json')).toBe('Invalid JSON input');
        });
    });

    describe('transformer relationships', () => {
        test('transformers are inverses of each other in terms of functionality', () => {
            const minifiedJson = '{"name":"John","age":30,"city":"New York"}';
            const formattedJson = `{
  "name": "John",
  "age": 30,
  "city": "New York"
}`;

            const format = transformers.jsonformat.transform;
            const minify = transformers.jsonminify.transform;

            // minified -> formatted -> minified should return equivalent
            expect(minify(format(minifiedJson))).toBe(minifiedJson);

            // formatted -> minified -> formatted should return equivalent
            // Note: The exact formatting might differ, so we parse and compare
            const originalParsed = JSON.parse(formattedJson);
            const roundTripParsed = JSON.parse(format(minify(formattedJson)));
            expect(roundTripParsed).toEqual(originalParsed);
        });

        test('inverse property is correctly set', () => {
            expect(transformers.jsonformat.inverse).toBe('jsonminify');
            expect(transformers.jsonminify.inverse).toBe('jsonformat');
        });
    });
});
