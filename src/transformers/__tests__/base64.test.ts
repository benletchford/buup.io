import { describe, test, expect } from 'vitest';
import transformers from '../base64';

describe('base64 transformers', () => {
  describe('base64 encode', () => {
    const { transform } = transformers.base64encode;

    test('encodes text to base64', () => {
      expect(transform('Hello, world!')).toBe('SGVsbG8sIHdvcmxkIQ==');
    });

    test('handles empty input', () => {
      expect(transform('')).toBe('');
    });
  });

  describe('base64 decode', () => {
    const { transform } = transformers.base64decode;

    test('decodes base64 to text', () => {
      expect(transform('SGVsbG8sIHdvcmxkIQ==')).toBe('Hello, world!');
    });

    test('handles invalid input', () => {
      expect(transform('invalid-base64!')).toBe('Invalid Base64 input');
      expect(transform('')).toBe('');
    });
  });

  describe('transformer relationships', () => {
    test('transformers are inverses of each other', () => {
      const plainText = 'Hello, world!';
      const base64Text = 'SGVsbG8sIHdvcmxkIQ==';

      const encode = transformers.base64encode.transform;
      const decode = transformers.base64decode.transform;

      // text -> base64 -> text should return original
      expect(decode(encode(plainText))).toBe(plainText);

      // base64 -> text -> base64 should return original
      expect(encode(decode(base64Text))).toBe(base64Text);
    });

    test('inverse properties are correctly set', () => {
      expect(transformers.base64encode.inverse).toBe('base64decode');
      expect(transformers.base64decode.inverse).toBe('base64encode');
    });
  });
});
