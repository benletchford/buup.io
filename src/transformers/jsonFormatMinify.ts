import { Transformer } from '../types';

const jsonFormat: Transformer = {
    id: 'jsonformat',
    title: 'JSON Format',
    description: 'Format JSON for readability',
    inverse: 'jsonminify',
    transform: (input: string): string => {
        try {
            // Handle empty input
            if (!input.trim()) return '';
            
            // Parse the JSON string to an object
            const parsed = JSON.parse(input);
            // Stringify with indentation for formatting
            return JSON.stringify(parsed, null, 2);
        } catch {
            return 'Invalid JSON input';
        }
    }
};

const jsonMinify: Transformer = {
    id: 'jsonminify',
    title: 'JSON Minify',
    description: 'Minify JSON by removing whitespace',
    inverse: 'jsonformat',
    transform: (input: string): string => {
        try {
            // Handle empty input
            if (!input.trim()) return '';
            
            // Parse the JSON string to an object
            const parsed = JSON.parse(input);
            // Stringify without indentation for minification
            return JSON.stringify(parsed);
        } catch {
            return 'Invalid JSON input';
        }
    }
};

export default {
    jsonformat: jsonFormat,
    jsonminify: jsonMinify
} as const;
