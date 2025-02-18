import { describe, test, expect } from 'vitest';
import transformers from '../datetime';

describe('datetime transformers', () => {
  describe('UTC to Sydney', () => {
    const { transform } = transformers['utc-to-sydney'];

    test('converts ISO UTC time to Sydney time', () => {
      expect(transform('2024-02-18T12:00:00Z')).toBe('2024-02-18T23:00:00+11:00');
      expect(transform('2024-02-18T12:00:00')).toBe('2024-02-18T23:00:00+11:00');
    });

    test('handles natural language input', () => {
      // Note: These tests assume running in UTC timezone
      expect(transform('Feb 18 2024 12:00')).toBe('2024-02-18T23:00:00+11:00');
      expect(transform('2024-02-18 12:00')).toBe('2024-02-18T23:00:00+11:00');
    });

    test('handles invalid input', () => {
      expect(transform('')).toBe('');
      expect(transform('invalid date')).toBe('');
    });
  });

  describe('Sydney to UTC', () => {
    const { transform } = transformers['sydney-to-utc'];

    test('converts Sydney time to UTC', () => {
      expect(transform('2024-02-18T23:00:00+11:00')).toBe('2024-02-18T12:00:00Z');
      expect(transform('2024-02-18T23:00:00')).toBe('2024-02-18T12:00:00Z');
    });

    test('handles natural language input', () => {
      expect(transform('Feb 18 2024 23:00')).toBe('2024-02-18T12:00:00Z');
      expect(transform('2024-02-18 23:00')).toBe('2024-02-18T12:00:00Z');
    });

    test('handles invalid input', () => {
      expect(transform('')).toBe('');
      expect(transform('invalid date')).toBe('');
    });
  });

  describe('transformer relationships', () => {
    test('transformers are inverses of each other', () => {
      const utcTime = '2024-02-18T12:00:00Z';
      const sydneyTime = '2024-02-18T23:00:00+11:00';

      const toSydney = transformers['utc-to-sydney'].transform;
      const toUTC = transformers['sydney-to-utc'].transform;

      // UTC -> Sydney -> UTC should return original
      expect(toUTC(toSydney(utcTime))).toBe(utcTime);

      // Sydney -> UTC -> Sydney should return original
      expect(toSydney(toUTC(sydneyTime))).toBe(sydneyTime);
    });

    test('inverse properties are correctly set', () => {
      expect(transformers['utc-to-sydney'].inverse).toBe('sydney-to-utc');
      expect(transformers['sydney-to-utc'].inverse).toBe('utc-to-sydney');
    });
  });
});
