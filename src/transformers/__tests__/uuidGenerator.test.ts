import { describe, test, expect } from 'vitest';
import transformers from '../uuidGenerator';

describe('UUID transformers', () => {
    describe('UUID generator', () => {
        const { transform } = transformers.uuidgenerator;

        test('generates a valid UUID v4', () => {
            const uuid = transform('');
            
            // UUID v4 format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
            // where x is any hexadecimal digit and y is one of 8, 9, A, or B
            const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
            
            expect(uuid).toMatch(uuidRegex);
        });

        test('generates different UUIDs on multiple calls with empty input', () => {
            const uuid1 = transform('');
            const uuid2 = transform('');
            
            expect(uuid1).not.toBe(uuid2);
        });

        test('generates deterministic UUIDs when input is provided', () => {
            const seed = 'test-seed';
            
            const uuid1 = transform(seed);
            const uuid2 = transform(seed);
            
            // Same input should produce same UUID
            expect(uuid1).toBe(uuid2);
            
            // Different input should produce different UUID
            const uuid3 = transform('different-seed');
            expect(uuid1).not.toBe(uuid3);
        });

        test('handles whitespace input', () => {
            const uuid = transform('  ');
            
            // Should be treated as empty input and generate a random UUID
            const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
            expect(uuid).toMatch(uuidRegex);
        });
    });

    describe('UUID to timestamp', () => {
        const { transform } = transformers.uuidtotimestamp;

        test('extracts timestamp from UUID v1', () => {
            // This is a sample UUID v1 generated on 2023-01-01T00:00:00Z
            const uuidV1 = '5d816450-8978-11ed-a1eb-0242ac120002';
            
            const result = transform(uuidV1);
            
            // Should contain a timestamp
            expect(result).toContain('UUID Timestamp:');
            expect(result).toContain('2023-01-01');
        });

        test('handles UUID v4 (which has no timestamp)', () => {
            // This is a sample UUID v4
            const uuidV4 = 'f47ac10b-58cc-4372-a567-0e02b2c3d479';
            
            const result = transform(uuidV4);
            
            expect(result).toBe('Not a v1 UUID. Only v1 UUIDs contain timestamp information.');
        });

        test('handles invalid UUID format', () => {
            expect(transform('not-a-uuid')).toBe('Invalid UUID format');
            expect(transform('12345')).toBe('Invalid UUID format');
            expect(transform('')).toBe('');
        });
    });

    describe('transformer relationships', () => {
        test('inverse property is correctly set', () => {
            // UUID generator doesn't have an inverse
            expect(transformers.uuidgenerator.inverse).toBeUndefined();
            
            // UUID to timestamp doesn't have an inverse
            expect(transformers.uuidtotimestamp.inverse).toBeUndefined();
        });
    });
});
