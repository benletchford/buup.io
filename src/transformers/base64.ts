import { Transformer } from '../types';

const base64Encode: Transformer = {
    id: 'base64encode',
    title: 'Base64 Encode',
    description: 'Encode text to Base64 format',
    inverse: 'base64decode',
    transform: (input: string): string => {
        try {
            return btoa(input);
        } catch (_: unknown) {
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
            return atob(input);
        } catch (_: unknown) {
            return 'Invalid Base64 input';
        }
    }
};

export default {
    base64encode: base64Encode,
    base64decode: base64Decode
} as const;
