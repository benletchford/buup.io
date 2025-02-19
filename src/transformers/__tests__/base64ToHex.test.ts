import { describe, test, expect } from 'vitest';
import transformers from '../base64ToHex';

describe('base64/hex transformers', () => {
    describe('base64 to hex', () => {
        const { transform } = transformers.base64tohex;

        test('converts base64 to hex', () => {
            expect(transform('SGVsbG8=')).toBe('48656C6C6F');
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles invalid input', () => {
            expect(transform('invalid-base64!')).toBe('Invalid Base64 input');
        });
    });

    describe('hex to base64', () => {
        const { transform } = transformers.hextobase64;

        test('converts hex to base64', () => {
            expect(transform('48656C6C6F')).toBe('SGVsbG8=');
        });

        test('handles lowercase hex', () => {
            expect(transform('48656c6c6f')).toBe('SGVsbG8=');
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles invalid input', () => {
            expect(transform('invalid')).toBe('Invalid hex input');
            expect(transform('123')).toBe('Invalid hex input'); // Odd length
            expect(transform('XY')).toBe('Invalid hex input'); // Non-hex characters
        });
    });

    describe('transformer relationships', () => {
        test('transformers are inverses of each other', () => {
            const base64Text = 'SGVsbG8=';
            const hexText = '48656C6C6F';

            const toHex = transformers.base64tohex.transform;
            const toBase64 = transformers.hextobase64.transform;

            // base64 -> hex -> base64 should return original
            expect(toBase64(toHex(base64Text))).toBe(base64Text);

            // hex -> base64 -> hex should return original
            expect(toHex(toBase64(hexText))).toBe(hexText);
        });

        test('inverse property is correctly set', () => {
            expect(transformers.hextobase64.inverse).toBe('base64tohex');
        });
    });
});
