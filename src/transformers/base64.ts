import { Transformer } from '../types';

const base64Encode: Transformer = {
    id: 'base64encode',
    title: 'Base64 Encode',
    description: 'Encode text to Base64 format',
    inverse: 'base64decode',
    transform: (input: string): string => {
        try {
            // Convert string to UTF-8 bytes, then to base64
            const bytes = new TextEncoder().encode(input);
            const base64 = btoa(String.fromCharCode(...bytes));
            return base64;
        } catch {
            return 'Invalid input for Base64 encoding';
        }
    }
};

const base64Decode: Transformer = {
    id: 'base64decode',
    title: 'Base64 Decode',
    description: 'Decode Base64 text to plain text',
    inverse: 'base64encode',
    transform: (input: string): string => {
        try {
            // Convert base64 to bytes, then to UTF-8 string
            const bytes = Uint8Array.from(atob(input), c => c.charCodeAt(0));
            const text = new TextDecoder().decode(bytes);
            return text;
        } catch {
            return 'Invalid Base64 input';
        }
    }
};

export default {
    base64encode: base64Encode,
    base64decode: base64Decode
} as const;
