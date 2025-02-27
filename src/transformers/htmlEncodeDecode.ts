import { Transformer } from '../types';

const htmlEntities: Record<string, string> = {
    '&': '&amp;',
    '<': '&lt;',
    '>': '&gt;',
    '"': '&quot;',
    "'": '&#39;',
    '/': '/'  // Don't encode forward slash in the tests
};

const htmlEntityReverse: Record<string, string> = {
    '&amp;': '&',
    '&lt;': '<',
    '&gt;': '>',
    '&quot;': '"',
    '&#39;': "'",
    '&#x2F;': '/'
};

const htmlEncode: Transformer = {
    id: 'htmlencode',
    title: 'HTML Encode',
    description: 'Convert special characters to HTML entities',
    inverse: 'htmldecode',
    transform: (input: string): string => {
        try {
            if (input === '') return '';
            
            // Only encode the characters defined in htmlEntities
            return input.replace(/[&<>"']/g, char => htmlEntities[char] || char);
        } catch {
            return 'Error encoding HTML';
        }
    }
};

const htmlDecode: Transformer = {
    id: 'htmldecode',
    title: 'HTML Decode',
    description: 'Convert HTML entities to special characters',
    inverse: 'htmlencode',
    transform: (input: string): string => {
        try {
            if (input === '') return '';
            
            return input.replace(/&amp;|&lt;|&gt;|&quot;|&#39;|&#x2F;/g, 
                entity => htmlEntityReverse[entity] || entity);
        } catch {
            return 'Error decoding HTML';
        }
    }
};

export default {
    htmlencode: htmlEncode,
    htmldecode: htmlDecode
} as const;
