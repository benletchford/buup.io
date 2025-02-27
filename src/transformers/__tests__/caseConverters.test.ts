import { describe, test, expect } from 'vitest';
import transformers from '../caseConverters';

describe('Case converter transformers', () => {
    describe('to camelCase', () => {
        const { transform } = transformers.tocamelcase;

        test('converts space-separated words to camelCase', () => {
            expect(transform('hello world')).toBe('helloWorld');
        });

        test('converts kebab-case to camelCase', () => {
            expect(transform('hello-world')).toBe('helloWorld');
        });

        test('converts snake_case to camelCase', () => {
            expect(transform('hello_world')).toBe('helloWorld');
        });

        test('converts PascalCase to camelCase', () => {
            expect(transform('HelloWorld')).toBe('helloWorld');
        });

        test('handles multiple word separators', () => {
            expect(transform('hello_world-example test')).toBe('helloWorldExampleTest');
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('');
        });
    });

    describe('to PascalCase', () => {
        const { transform } = transformers.topascalcase;

        test('converts space-separated words to PascalCase', () => {
            expect(transform('hello world')).toBe('HelloWorld');
        });

        test('converts kebab-case to PascalCase', () => {
            expect(transform('hello-world')).toBe('HelloWorld');
        });

        test('converts snake_case to PascalCase', () => {
            expect(transform('hello_world')).toBe('HelloWorld');
        });

        test('converts camelCase to PascalCase', () => {
            expect(transform('helloWorld')).toBe('HelloWorld');
        });

        test('handles multiple word separators', () => {
            expect(transform('hello_world-example test')).toBe('HelloWorldExampleTest');
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('');
        });
    });

    describe('to snake_case', () => {
        const { transform } = transformers.tosnakecase;

        test('converts space-separated words to snake_case', () => {
            expect(transform('hello world')).toBe('hello_world');
        });

        test('converts kebab-case to snake_case', () => {
            expect(transform('hello-world')).toBe('hello_world');
        });

        test('converts camelCase to snake_case', () => {
            expect(transform('helloWorld')).toBe('hello_world');
        });

        test('converts PascalCase to snake_case', () => {
            expect(transform('HelloWorld')).toBe('hello_world');
        });

        test('handles multiple word separators', () => {
            expect(transform('hello_world-example test')).toBe('hello_world_example_test');
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('');
        });
    });

    describe('to kebab-case', () => {
        const { transform } = transformers.tokebabcase;

        test('converts space-separated words to kebab-case', () => {
            expect(transform('hello world')).toBe('hello-world');
        });

        test('converts snake_case to kebab-case', () => {
            expect(transform('hello_world')).toBe('hello-world');
        });

        test('converts camelCase to kebab-case', () => {
            expect(transform('helloWorld')).toBe('hello-world');
        });

        test('converts PascalCase to kebab-case', () => {
            expect(transform('HelloWorld')).toBe('hello-world');
        });

        test('handles multiple word separators', () => {
            expect(transform('hello_world-example test')).toBe('hello-world-example-test');
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('');
        });
    });

    describe('to CONSTANT_CASE', () => {
        const { transform } = transformers.toconstantcase;

        test('converts space-separated words to CONSTANT_CASE', () => {
            expect(transform('hello world')).toBe('HELLO_WORLD');
        });

        test('converts kebab-case to CONSTANT_CASE', () => {
            expect(transform('hello-world')).toBe('HELLO_WORLD');
        });

        test('converts snake_case to CONSTANT_CASE', () => {
            expect(transform('hello_world')).toBe('HELLO_WORLD');
        });

        test('converts camelCase to CONSTANT_CASE', () => {
            expect(transform('helloWorld')).toBe('HELLO_WORLD');
        });

        test('converts PascalCase to CONSTANT_CASE', () => {
            expect(transform('HelloWorld')).toBe('HELLO_WORLD');
        });

        test('handles multiple word separators', () => {
            expect(transform('hello_world-example test')).toBe('HELLO_WORLD_EXAMPLE_TEST');
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('');
        });
    });

    describe('complex cases', () => {
        test('handles numbers in input', () => {
            expect(transformers.tocamelcase.transform('hello_world_123')).toBe('helloWorld123');
            expect(transformers.topascalcase.transform('hello_world_123')).toBe('HelloWorld123');
            expect(transformers.tosnakecase.transform('helloWorld123')).toBe('hello_world_123');
            expect(transformers.tokebabcase.transform('helloWorld123')).toBe('hello-world-123');
            expect(transformers.toconstantcase.transform('helloWorld123')).toBe('HELLO_WORLD_123');
        });

        test('handles mixed case input', () => {
            const mixedCase = 'some_MIXED_Case-string withSpaces';
            
            expect(transformers.tocamelcase.transform(mixedCase)).toBe('someMixedCaseStringWithspaces');
            expect(transformers.topascalcase.transform(mixedCase)).toBe('SomeMixedCaseStringWithspaces');
            expect(transformers.tosnakecase.transform(mixedCase)).toBe('some_mixed_case_string_withspaces');
            expect(transformers.tokebabcase.transform(mixedCase)).toBe('some-mixed-case-string-withspaces');
            expect(transformers.toconstantcase.transform(mixedCase)).toBe('SOME_MIXED_CASE_STRING_WITHSPACES');
        });
    });
});
