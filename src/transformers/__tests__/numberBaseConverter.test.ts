import { describe, test, expect } from 'vitest';
import transformers from '../numberBaseConverter';

describe('Number base converter transformers', () => {
    describe('decimal to binary', () => {
        const { transform } = transformers.decimaltobinary;

        test('converts decimal to binary', () => {
            expect(transform('10')).toBe('1010');
            expect(transform('42')).toBe('101010');
            expect(transform('255')).toBe('11111111');
            expect(transform('0')).toBe('0');
            expect(transform('1')).toBe('1');
        });

        test('handles negative numbers', () => {
            // JavaScript's toString(2) for negative numbers returns the binary representation
            // of the absolute value with a minus sign prefix
            expect(transform('-10')).toBe('-1010');
            expect(transform('-42')).toBe('-101010');
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('');
            expect(transform(' 42 ')).toBe('101010');
        });

        test('handles invalid input', () => {
            expect(transform('not a number')).toBe('Invalid decimal number');
            expect(transform('42.5')).toBe('101010'); // Truncates to integer
            expect(transform('0xFF')).toBe('11111111'); // Parses as decimal 255
        });
    });

    describe('binary to decimal', () => {
        const { transform } = transformers.binarytodecimal;

        test('converts binary to decimal', () => {
            expect(transform('1010')).toBe('10');
            expect(transform('101010')).toBe('42');
            expect(transform('11111111')).toBe('255');
            expect(transform('0')).toBe('0');
            expect(transform('1')).toBe('1');
        });

        test('handles binary with spaces', () => {
            expect(transform('1010 1010')).toBe('170');
            expect(transform('1111 0000')).toBe('240');
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('');
            expect(transform(' 1010 ')).toBe('10');
        });

        test('handles invalid input', () => {
            expect(transform('not a binary')).toBe('Invalid binary number');
            expect(transform('12345')).toBe('Invalid binary number'); // Contains non-binary digits
            expect(transform('0b1010')).toBe('10'); // Handles 0b prefix
        });
    });

    describe('decimal to hexadecimal', () => {
        const { transform } = transformers.decimaltohex;

        test('converts decimal to hexadecimal', () => {
            expect(transform('10')).toBe('A');
            expect(transform('42')).toBe('2A');
            expect(transform('255')).toBe('FF');
            expect(transform('0')).toBe('0');
            expect(transform('16')).toBe('10');
        });

        test('handles negative numbers', () => {
            expect(transform('-10')).toBe('-A');
            expect(transform('-42')).toBe('-2A');
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('');
            expect(transform(' 42 ')).toBe('2A');
        });

        test('handles invalid input', () => {
            expect(transform('not a number')).toBe('Invalid decimal number');
            expect(transform('42.5')).toBe('2A'); // Truncates to integer
        });
    });

    describe('hexadecimal to decimal', () => {
        const { transform } = transformers.hextodecimal;

        test('converts hexadecimal to decimal', () => {
            expect(transform('A')).toBe('10');
            expect(transform('2A')).toBe('42');
            expect(transform('FF')).toBe('255');
            expect(transform('0')).toBe('0');
            expect(transform('10')).toBe('16');
        });

        test('handles hexadecimal with 0x prefix', () => {
            expect(transform('0xA')).toBe('10');
            expect(transform('0x2A')).toBe('42');
            expect(transform('0xFF')).toBe('255');
        });

        test('handles lowercase hexadecimal', () => {
            expect(transform('a')).toBe('10');
            expect(transform('2a')).toBe('42');
            expect(transform('ff')).toBe('255');
        });

        test('handles hexadecimal with spaces', () => {
            expect(transform('FF 00')).toBe('65280');
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('');
            expect(transform(' FF ')).toBe('255');
        });

        test('handles invalid input', () => {
            expect(transform('not a hex')).toBe('Invalid hexadecimal number');
            expect(transform('0xGG')).toBe('Invalid hexadecimal number'); // Contains non-hex digits
        });
    });

    describe('decimal to octal', () => {
        const { transform } = transformers.decimaltooctal;

        test('converts decimal to octal', () => {
            expect(transform('8')).toBe('10');
            expect(transform('42')).toBe('52');
            expect(transform('255')).toBe('377');
            expect(transform('0')).toBe('0');
            expect(transform('64')).toBe('100');
        });

        test('handles negative numbers', () => {
            expect(transform('-8')).toBe('-10');
            expect(transform('-42')).toBe('-52');
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('');
            expect(transform(' 42 ')).toBe('52');
        });

        test('handles invalid input', () => {
            expect(transform('not a number')).toBe('Invalid decimal number');
            expect(transform('42.5')).toBe('52'); // Truncates to integer
        });
    });

    describe('octal to decimal', () => {
        const { transform } = transformers.octaltodecimal;

        test('converts octal to decimal', () => {
            expect(transform('10')).toBe('8');
            expect(transform('52')).toBe('42');
            expect(transform('377')).toBe('255');
            expect(transform('0')).toBe('0');
            expect(transform('100')).toBe('64');
        });

        test('handles octal with spaces', () => {
            expect(transform('10 10')).toBe('520');
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('');
            expect(transform(' 52 ')).toBe('42');
        });

        test('handles invalid input', () => {
            expect(transform('not an octal')).toBe('Invalid octal number');
            expect(transform('89')).toBe('Invalid octal number'); // Contains non-octal digits
        });
    });

    describe('binary to hexadecimal', () => {
        const { transform } = transformers.binarytohex;

        test('converts binary to hexadecimal', () => {
            expect(transform('1010')).toBe('A');
            expect(transform('101010')).toBe('2A');
            expect(transform('11111111')).toBe('FF');
            expect(transform('0')).toBe('0');
            expect(transform('10000')).toBe('10');
        });

        test('handles binary with spaces', () => {
            expect(transform('1010 1010')).toBe('AA');
            expect(transform('1111 0000')).toBe('F0');
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('');
            expect(transform(' 1010 ')).toBe('A');
        });

        test('handles invalid input', () => {
            expect(transform('not a binary')).toBe('Invalid binary number');
            expect(transform('12345')).toBe('Invalid binary number'); // Contains non-binary digits
        });
    });

    describe('hexadecimal to binary', () => {
        const { transform } = transformers.hextobinary;

        test('converts hexadecimal to binary', () => {
            expect(transform('A')).toBe('1010');
            expect(transform('2A')).toBe('101010');
            expect(transform('FF')).toBe('11111111');
            expect(transform('0')).toBe('0');
            expect(transform('10')).toBe('10000');
        });

        test('handles hexadecimal with 0x prefix', () => {
            expect(transform('0xA')).toBe('1010');
            expect(transform('0x2A')).toBe('101010');
            expect(transform('0xFF')).toBe('11111111');
        });

        test('handles lowercase hexadecimal', () => {
            expect(transform('a')).toBe('1010');
            expect(transform('2a')).toBe('101010');
            expect(transform('ff')).toBe('11111111');
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('');
            expect(transform(' FF ')).toBe('11111111');
        });

        test('handles invalid input', () => {
            expect(transform('not a hex')).toBe('Invalid hexadecimal number');
            expect(transform('0xGG')).toBe('Invalid hexadecimal number'); // Contains non-hex digits
        });
    });

    describe('transformer relationships', () => {
        test('decimal-binary transformers are inverses of each other', () => {
            const decimalValue = '42';
            const binaryValue = '101010';
            
            const toBinary = transformers.decimaltobinary.transform;
            const toDecimal = transformers.binarytodecimal.transform;
            
            expect(toDecimal(toBinary(decimalValue))).toBe(decimalValue);
            expect(toBinary(toDecimal(binaryValue))).toBe(binaryValue);
        });

        test('decimal-hex transformers are inverses of each other', () => {
            const decimalValue = '255';
            const hexValue = 'FF';
            
            const toHex = transformers.decimaltohex.transform;
            const toDecimal = transformers.hextodecimal.transform;
            
            expect(toDecimal(toHex(decimalValue))).toBe(decimalValue);
            expect(toHex(toDecimal(hexValue))).toBe(hexValue);
        });

        test('decimal-octal transformers are inverses of each other', () => {
            const decimalValue = '64';
            const octalValue = '100';
            
            const toOctal = transformers.decimaltooctal.transform;
            const toDecimal = transformers.octaltodecimal.transform;
            
            expect(toDecimal(toOctal(decimalValue))).toBe(decimalValue);
            expect(toOctal(toDecimal(octalValue))).toBe(octalValue);
        });

        test('binary-hex transformers are inverses of each other', () => {
            const binaryValue = '11111111';
            const hexValue = 'FF';
            
            const toHex = transformers.binarytohex.transform;
            const toBinary = transformers.hextobinary.transform;
            
            expect(toBinary(toHex(binaryValue))).toBe(binaryValue);
            expect(toHex(toBinary(hexValue))).toBe(hexValue);
        });

        test('inverse properties are correctly set', () => {
            expect(transformers.decimaltobinary.inverse).toBe('binarytodecimal');
            expect(transformers.binarytodecimal.inverse).toBe('decimaltobinary');
            expect(transformers.decimaltohex.inverse).toBe('hextodecimal');
            expect(transformers.hextodecimal.inverse).toBe('decimaltohex');
            expect(transformers.decimaltooctal.inverse).toBe('octaltodecimal');
            expect(transformers.octaltodecimal.inverse).toBe('decimaltooctal');
        });
    });
});
