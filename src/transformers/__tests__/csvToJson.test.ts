import { describe, test, expect } from 'vitest';
import transformers from '../csvToJson';

describe('CSV/JSON transformers', () => {
    describe('CSV to JSON', () => {
        const { transform } = transformers.csvtojson;

        test('converts simple CSV to JSON', () => {
            const csv = 'name,age,city\nJohn,30,New York\nJane,25,San Francisco';
            const result = transform(csv);
            const parsed = JSON.parse(result);
            
            expect(parsed).toEqual([
                { name: 'John', age: '30', city: 'New York' },
                { name: 'Jane', age: '25', city: 'San Francisco' }
            ]);
        });

        test('handles CSV with quoted values', () => {
            const csv = 'name,description,location\nJohn,"Software Engineer, Senior",New York\nJane,"Product Manager","San Francisco, CA"';
            const result = transform(csv);
            const parsed = JSON.parse(result);
            
            expect(parsed).toEqual([
                { name: 'John', description: 'Software Engineer, Senior', location: 'New York' },
                { name: 'Jane', description: 'Product Manager', location: 'San Francisco, CA' }
            ]);
        });

        test('handles CSV with escaped quotes', () => {
            const csv = 'name,quote\nJohn,"He said ""Hello"""\nJane,"She said ""Goodbye"""';
            const result = transform(csv);
            const parsed = JSON.parse(result);
            
            expect(parsed).toEqual([
                { name: 'John', quote: 'He said "Hello"' },
                { name: 'Jane', quote: 'She said "Goodbye"' }
            ]);
        });

        test('handles CSV with missing values', () => {
            const csv = 'name,age,city\nJohn,30,\nJane,,San Francisco';
            const result = transform(csv);
            const parsed = JSON.parse(result);
            
            expect(parsed).toEqual([
                { name: 'John', age: '30', city: '' },
                { name: 'Jane', age: '', city: 'San Francisco' }
            ]);
        });

        test('handles CSV with different line endings', () => {
            const csv = 'name,age\r\nJohn,30\r\nJane,25';
            const result = transform(csv);
            const parsed = JSON.parse(result);
            
            expect(parsed).toEqual([
                { name: 'John', age: '30' },
                { name: 'Jane', age: '25' }
            ]);
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('');
        });

        test('handles CSV with only headers', () => {
            expect(transform('name,age,city')).toBe('[]');
        });
    });

    describe('JSON to CSV', () => {
        const { transform } = transformers.jsontocsv;

        test('converts simple JSON array to CSV', () => {
            const json = JSON.stringify([
                { name: 'John', age: 30, city: 'New York' },
                { name: 'Jane', age: 25, city: 'San Francisco' }
            ]);
            
            const csv = transform(json);
            
            // Check for headers and values
            expect(csv).toContain('name,age,city');
            expect(csv).toContain('John,30,New York');
            expect(csv).toContain('Jane,25,San Francisco');
        });

        test('handles JSON with values containing commas', () => {
            const json = JSON.stringify([
                { name: 'John', title: 'Software Engineer, Senior', location: 'New York' },
                { name: 'Jane', title: 'Product Manager', location: 'San Francisco, CA' }
            ]);
            
            const csv = transform(json);
            
            // Values with commas should be quoted
            expect(csv).toContain('name,title,location');
            expect(csv).toContain('John,"Software Engineer, Senior",New York');
            expect(csv).toContain('Jane,Product Manager,"San Francisco, CA"');
        });

        test('handles JSON with values containing quotes', () => {
            const json = JSON.stringify([
                { name: 'John', quote: 'He said "Hello"' },
                { name: 'Jane', quote: 'She said "Goodbye"' }
            ]);
            
            const csv = transform(json);
            
            // Values with quotes should have quotes escaped
            expect(csv).toContain('name,quote');
            expect(csv).toContain('John,"He said ""Hello"""');
            expect(csv).toContain('Jane,"She said ""Goodbye"""');
        });

        test('handles JSON with missing values', () => {
            const json = JSON.stringify([
                { name: 'John', age: 30, city: null },
                { name: 'Jane', age: null, city: 'San Francisco' }
            ]);
            
            const csv = transform(json);
            
            // Missing values should be empty
            expect(csv).toContain('name,age,city');
            expect(csv).toContain('John,30,');
            expect(csv).toContain('Jane,,San Francisco');
        });

        test('handles JSON with different property sets', () => {
            const json = JSON.stringify([
                { name: 'John', age: 30 },
                { name: 'Jane', city: 'San Francisco' },
                { age: 40, city: 'Chicago' }
            ]);
            
            const csv = transform(json);
            
            // All properties should be included
            expect(csv).toContain('name,age,city');
            expect(csv).toContain('John,30,');
            expect(csv).toContain('Jane,,San Francisco');
            expect(csv).toContain(',40,Chicago');
        });

        test('handles empty input', () => {
            expect(transform('')).toBe('');
        });

        test('handles whitespace', () => {
            expect(transform('  ')).toBe('');
        });

        test('handles empty array', () => {
            expect(transform('[]')).toBe('');
        });

        test('handles non-array JSON', () => {
            expect(transform('{"name":"John"}')).toBe('Input must be a JSON array');
        });
    });

    describe('transformer relationships', () => {
        test('transformers are inverses of each other', () => {
            const csvData = 'name,age,city\nJohn,30,New York\nJane,25,"San Francisco, CA"';
            
            const csvToJson = transformers.csvtojson.transform;
            const jsonToCsv = transformers.jsontocsv.transform;
            
            // CSV -> JSON -> CSV should preserve data
            const jsonResult = csvToJson(csvData);
            const csvResult = jsonToCsv(jsonResult);
            
            // Check that all original data is present in the round-trip result
            // Note: The exact format might differ (e.g., spacing, order of columns)
            expect(csvResult).toContain('name,age,city');
            expect(csvResult).toContain('John,30,New York');
            expect(csvResult).toContain('Jane,25,"San Francisco, CA"');
        });

        test('inverse property is correctly set', () => {
            expect(transformers.csvtojson.inverse).toBe('jsontocsv');
            expect(transformers.jsontocsv.inverse).toBe('csvtojson');
        });
    });
});
