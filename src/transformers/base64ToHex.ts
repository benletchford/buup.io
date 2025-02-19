import { Transformer } from '../types';

const base64ToHex: Transformer = {
    id: 'base64tohex',
    title: 'Base64 to Hex',
    description: 'Convert Base64 encoded text to hexadecimal',
    inverse: 'hextobase64',
    transform: (input: string): string => {
        try {
            // Decode base64 to binary string
            const binary = atob(input);
            // Convert each character to hex
            const hex = Array.from(binary)
                .map(char => char.charCodeAt(0).toString(16).padStart(2, '0'))
                .join('');
            return hex.toUpperCase();
        } catch {
            return 'Invalid Base64 input';
        }
    }
};

const hexToBase64: Transformer = {
    id: 'hextobase64',
    title: 'Hex to Base64',
    description: 'Convert hexadecimal to Base64 encoded text',
    inverse: 'base64tohex',
    transform: (input: string): string => {
        try {
            // Handle empty input
            if (!input) return '';
            
            // Remove any spaces and ensure even length
            const cleanHex = input.replace(/\s/g, '');
            if (cleanHex.length % 2 !== 0 || !/^[0-9A-Fa-f]+$/.test(cleanHex)) {
                throw new Error('Invalid hex input');
            }
            
            // Convert hex pairs to characters
            const binary = cleanHex.match(/.{2}/g)!
                .map(hex => String.fromCharCode(parseInt(hex, 16)))
                .join('');
            
            return btoa(binary);
        } catch {
            return 'Invalid hex input';
        }
    }
};

export default {
    base64tohex: base64ToHex,
    hextobase64: hexToBase64
} as const;
