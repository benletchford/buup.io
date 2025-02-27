import { Transformer } from '../types';

const decimalToBinary: Transformer = {
    id: 'decimaltobinary',
    title: 'Decimal to Binary',
    description: 'Convert decimal number to binary',
    inverse: 'binarytodecimal',
    transform: (input: string): string => {
        try {
            if (!input.trim()) return '';
            
            // Handle hex input with 0x prefix
            if (/^0x[0-9A-Fa-f]+$/.test(input.trim())) {
                const num = parseInt(input.trim().substring(2), 16);
                return num.toString(2);
            }
            
            // Parse the input as a decimal number
            const num = parseInt(input.trim(), 10);
            
            if (isNaN(num)) {
                return 'Invalid decimal number';
            }
            
            // Convert to binary
            return num.toString(2);
        } catch {
            return 'Error converting decimal to binary';
        }
    }
};

const binaryToDecimal: Transformer = {
    id: 'binarytodecimal',
    title: 'Binary to Decimal',
    description: 'Convert binary number to decimal',
    inverse: 'decimaltobinary',
    transform: (input: string): string => {
        try {
            if (!input.trim()) return '';
            
            // Handle 0b prefix
            let cleanInput = input.trim();
            if (cleanInput.startsWith('0b')) {
                cleanInput = cleanInput.substring(2);
            }
            
            // Remove spaces
            cleanInput = cleanInput.replace(/\s+/g, '');
            
            // Check if the input contains only 0s and 1s
            if (!/^[01]+$/.test(cleanInput)) {
                return 'Invalid binary number';
            }
            
            // Parse the input as a binary number
            const num = parseInt(cleanInput, 2);
            
            // Convert to decimal
            return num.toString(10);
        } catch {
            return 'Error converting binary to decimal';
        }
    }
};

const decimalToHex: Transformer = {
    id: 'decimaltohex',
    title: 'Decimal to Hex',
    description: 'Convert decimal number to hexadecimal',
    inverse: 'hextodecimal',
    transform: (input: string): string => {
        try {
            if (!input.trim()) return '';
            
            // Parse the input as a decimal number
            const num = parseInt(input.trim(), 10);
            
            if (isNaN(num)) {
                return 'Invalid decimal number';
            }
            
            // Convert to hexadecimal
            return num.toString(16).toUpperCase();
        } catch {
            return 'Error converting decimal to hexadecimal';
        }
    }
};

const hexToDecimal: Transformer = {
    id: 'hextodecimal',
    title: 'Hex to Decimal',
    description: 'Convert hexadecimal number to decimal',
    inverse: 'decimaltohex',
    transform: (input: string): string => {
        try {
            if (!input.trim()) return '';
            
            // Remove any spaces, 0x prefix, or other separators
            let cleanInput = input.trim();
            if (cleanInput.startsWith('0x') || cleanInput.startsWith('0X')) {
                cleanInput = cleanInput.substring(2);
            }
            
            // Remove spaces
            cleanInput = cleanInput.replace(/\s+/g, '');
            
            // Check if the input contains only hex characters
            if (!/^[0-9A-Fa-f]+$/.test(cleanInput)) {
                return 'Invalid hexadecimal number';
            }
            
            // Parse the input as a hexadecimal number
            const num = parseInt(cleanInput, 16);
            
            // Convert to decimal
            return num.toString(10);
        } catch {
            return 'Error converting hexadecimal to decimal';
        }
    }
};

const decimalToOctal: Transformer = {
    id: 'decimaltooctal',
    title: 'Decimal to Octal',
    description: 'Convert decimal number to octal',
    inverse: 'octaltodecimal',
    transform: (input: string): string => {
        try {
            if (!input.trim()) return '';
            
            // Parse the input as a decimal number
            const num = parseInt(input.trim(), 10);
            
            if (isNaN(num)) {
                return 'Invalid decimal number';
            }
            
            // Convert to octal
            return num.toString(8);
        } catch {
            return 'Error converting decimal to octal';
        }
    }
};

const octalToDecimal: Transformer = {
    id: 'octaltodecimal',
    title: 'Octal to Decimal',
    description: 'Convert octal number to decimal',
    inverse: 'decimaltooctal',
    transform: (input: string): string => {
        try {
            if (!input.trim()) return '';
            
            // Remove any spaces or other separators
            const cleanInput = input.trim().replace(/\s+/g, '');
            
            // Check if the input contains only octal digits
            if (!/^[0-7]+$/.test(cleanInput)) {
                return 'Invalid octal number';
            }
            
            // Parse the input as an octal number
            const num = parseInt(cleanInput, 8);
            
            // Convert to decimal
            return num.toString(10);
        } catch {
            return 'Error converting octal to decimal';
        }
    }
};

const binaryToHex: Transformer = {
    id: 'binarytohex',
    title: 'Binary to Hex',
    description: 'Convert binary number to hexadecimal',
    transform: (input: string): string => {
        try {
            if (!input.trim()) return '';
            
            // Handle 0b prefix
            let cleanInput = input.trim();
            if (cleanInput.startsWith('0b')) {
                cleanInput = cleanInput.substring(2);
            }
            
            // Remove spaces
            cleanInput = cleanInput.replace(/\s+/g, '');
            
            // Check if the input contains only 0s and 1s
            if (!/^[01]+$/.test(cleanInput)) {
                return 'Invalid binary number';
            }
            
            // Parse the input as a binary number
            const num = parseInt(cleanInput, 2);
            
            // Convert to hexadecimal
            return num.toString(16).toUpperCase();
        } catch {
            return 'Error converting binary to hexadecimal';
        }
    }
};

const hexToBinary: Transformer = {
    id: 'hextobinary',
    title: 'Hex to Binary',
    description: 'Convert hexadecimal number to binary',
    transform: (input: string): string => {
        try {
            if (!input.trim()) return '';
            
            // Remove any spaces, 0x prefix, or other separators
            let cleanInput = input.trim();
            if (cleanInput.startsWith('0x') || cleanInput.startsWith('0X')) {
                cleanInput = cleanInput.substring(2);
            }
            
            // Remove spaces
            cleanInput = cleanInput.replace(/\s+/g, '');
            
            // Check if the input contains only hex characters
            if (!/^[0-9A-Fa-f]+$/.test(cleanInput)) {
                return 'Invalid hexadecimal number';
            }
            
            // Parse the input as a hexadecimal number
            const num = parseInt(cleanInput, 16);
            
            // Convert to binary
            return num.toString(2);
        } catch {
            return 'Error converting hexadecimal to binary';
        }
    }
};

export default {
    decimaltobinary: decimalToBinary,
    binarytodecimal: binaryToDecimal,
    decimaltohex: decimalToHex,
    hextodecimal: hexToDecimal,
    decimaltooctal: decimalToOctal,
    octaltodecimal: octalToDecimal,
    binarytohex: binaryToHex,
    hextobinary: hexToBinary
} as const;
