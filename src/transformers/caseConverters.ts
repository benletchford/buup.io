import { Transformer } from '../types';

function splitWords(input: string): string[] {
    // Handle empty input
    if (!input || !input.trim()) {
        return [];
    }
    
    // First, check if the input contains separators
    const hasSeparators = /[-_.\s]/.test(input);
    
    if (hasSeparators) {
        // Normalize the input by replacing all separators with spaces
        // This handles multiple types of separators in the same string
        const normalized = input
            .replace(/_/g, ' ')
            .replace(/-/g, ' ')
            .replace(/\./g, ' ');
        
        // Then split by whitespace and filter out empty strings
        return normalized.split(/\s+/).filter(Boolean);
    } else {
        // For camelCase or PascalCase, split by capital letters
        // This regex captures:
        // 1. A capital letter followed by lowercase letters
        // 2. Multiple capital letters together
        // 3. Numbers as separate words
        const matches = input.match(/[A-Z]?[a-z]+|[A-Z]+(?=[A-Z][a-z]|$)|[0-9]+/g);
        
        // If we couldn't extract any words, return the input as a single word
        return matches || [input];
    }
}

const toCamelCase: Transformer = {
    id: 'tocamelcase',
    title: 'To camelCase',
    description: 'Convert text to camelCase',
    transform: (input: string): string => {
        try {
            if (!input.trim()) return '';
            const words = splitWords(input);
            if (words.length === 0) return '';
            return words[0].toLowerCase() + words.slice(1).map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase()).join('');
        } catch {
            return 'Error converting to camelCase';
        }
    }
};

const toPascalCase: Transformer = {
    id: 'topascalcase',
    title: 'To PascalCase',
    description: 'Convert text to PascalCase',
    transform: (input: string): string => {
        try {
            if (!input.trim()) return '';
            const words = splitWords(input);
            if (words.length === 0) return '';
            return words.map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase()).join('');
        } catch {
            return 'Error converting to PascalCase';
        }
    }
};

const toSnakeCase: Transformer = {
    id: 'tosnakecase',
    title: 'To snake_case',
    description: 'Convert text to snake_case',
    transform: (input: string): string => {
        try {
            if (!input.trim()) return '';
            const words = splitWords(input);
            if (words.length === 0) return '';
            return words.map(word => word.toLowerCase()).join('_');
        } catch {
            return 'Error converting to snake_case';
        }
    }
};

const toKebabCase: Transformer = {
    id: 'tokebabcase',
    title: 'To kebab-case',
    description: 'Convert text to kebab-case',
    transform: (input: string): string => {
        try {
            if (!input.trim()) return '';
            const words = splitWords(input);
            if (words.length === 0) return '';
            return words.map(word => word.toLowerCase()).join('-');
        } catch {
            return 'Error converting to kebab-case';
        }
    }
};

const toConstantCase: Transformer = {
    id: 'toconstantcase',
    title: 'To CONSTANT_CASE',
    description: 'Convert text to CONSTANT_CASE',
    transform: (input: string): string => {
        try {
            if (!input.trim()) return '';
            const words = splitWords(input);
            if (words.length === 0) return '';
            return words.map(word => word.toUpperCase()).join('_');
        } catch {
            return 'Error converting to CONSTANT_CASE';
        }
    }
};

export default {
    tocamelcase: toCamelCase,
    topascalcase: toPascalCase,
    tosnakecase: toSnakeCase,
    tokebabcase: toKebabCase,
    toconstantcase: toConstantCase
} as const;
