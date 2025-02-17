import { describe, it, expect } from 'vitest';
import base64Transformers from '../base64';

describe('base64 transformers', () => {
  describe('base64encode', () => {
    const { base64encode } = base64Transformers;

    it('should encode plain text to base64', () => {
      expect(base64encode.transform('Hello, World!')).toBe('SGVsbG8sIFdvcmxkIQ==');
    });

    it('should encode empty string', () => {
      expect(base64encode.transform('')).toBe('');
    });

    it('should encode special characters', () => {
      expect(base64encode.transform('🚀 Special chars: @#$%')).toBe('8J+agCBTcGVjaWFsIGNoYXJzOiBAIyQl');
    });

    it('should have correct metadata', () => {
      expect(base64encode.id).toBe('base64encode');
      expect(base64encode.title).toBe('Base64 Encode');
      expect(base64encode.description).toBe('Encode text to Base64 format');
      expect(base64encode.inverse).toBe('base64decode');
    });
  });

  describe('base64decode', () => {
    const { base64decode } = base64Transformers;

    it('should decode base64 to plain text', () => {
      expect(base64decode.transform('SGVsbG8sIFdvcmxkIQ==')).toBe('Hello, World!');
    });

    it('should decode empty string', () => {
      expect(base64decode.transform('')).toBe('');
    });

    it('should handle invalid base64 input', () => {
      expect(base64decode.transform('Invalid Base64!')).toBe('Invalid Base64 input');
    });

    it('should decode special characters', () => {
      expect(base64decode.transform('8J+agCBTcGVjaWFsIGNoYXJzOiBAIyQl')).toBe('🚀 Special chars: @#$%');
    });

    it('should have correct metadata', () => {
      expect(base64decode.id).toBe('base64decode');
      expect(base64decode.title).toBe('Base64 Decode');
      expect(base64decode.description).toBe('Decode Base64 text to plain text');
      expect(base64decode.inverse).toBe('base64encode');
    });
  });

  describe('bidirectional conversion', () => {
    const { base64encode, base64decode } = base64Transformers;

    it('should correctly encode and decode back to original', () => {
      const original = 'Test string with special chars: 🚀 @#$%';
      const encoded = base64encode.transform(original);
      const decoded = base64decode.transform(encoded);
      expect(decoded).toBe(original);
    });
  });
});
