import { Transformer } from '../types';

const urlEncode: Transformer = {
    id: 'urlencode',
    title: 'URL Encode',
    description: 'Encode text to URL-safe format',
    inverse: 'urldecode',
    transform: (input: string): string => {
        try {
            if (!input.trim()) return input;
            
            // Use encodeURIComponent which encodes all characters except: A-Z a-z 0-9 - _ . ! ~ * ' ( )
            // We need to manually encode ! and ' to match the test expectations
            return encodeURIComponent(input)
                .replace(/!/g, '%21')
                .replace(/'/g, '%27')
                .replace(/\(/g, '%28')
                .replace(/\)/g, '%29')
                .replace(/\*/g, '%2A')
                .replace(/~/g, '%7E');
        } catch {
            return 'Invalid input for URL encoding';
        }
    }
};

const urlDecode: Transformer = {
    id: 'urldecode',
    title: 'URL Decode',
    description: 'Decode URL-encoded text to plain text',
    inverse: 'urlencode',
    transform: (input: string): string => {
        try {
            if (input === '') return '';
            
            return decodeURIComponent(input);
        } catch {
            return 'Invalid URL-encoded input';
        }
    }
};

export default {
    urlencode: urlEncode,
    urldecode: urlDecode
} as const;
