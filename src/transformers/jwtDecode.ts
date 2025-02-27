import { Transformer } from '../types';

const jwtDecode: Transformer = {
    id: 'jwtdecode',
    title: 'JWT Decode',
    description: 'Decode and display JWT token contents',
    transform: (input: string): string => {
        try {
            if (!input.trim()) return '';
            
            // JWT tokens consist of three parts separated by dots
            const parts = input.split('.');
            if (parts.length !== 3) {
                return 'Invalid JWT token format. Expected format: header.payload.signature';
            }
            
            // Decode the header and payload
            const header = decodeJwtPart(parts[0]);
            const payload = decodeJwtPart(parts[1]);
            
            // Format the result
            const result = {
                header: JSON.parse(header),
                payload: JSON.parse(payload),
                signature: parts[2]
            };
            
            return JSON.stringify(result, null, 2);
        } catch (error) {
            return `Error decoding JWT: ${error instanceof Error ? error.message : 'Unknown error'}`;
        }
    }
};

// Helper function to decode a JWT part (header or payload)
function decodeJwtPart(str: string): string {
    // Base64 URL decode
    // Replace characters for standard Base64
    const base64 = str.replace(/-/g, '+').replace(/_/g, '/');
    
    // Add padding if needed
    const padded = base64.padEnd(base64.length + (4 - (base64.length % 4)) % 4, '=');
    
    // Decode
    const decoded = atob(padded);
    
    // Convert to UTF-8 string
    return decoded;
}

export default {
    jwtdecode: jwtDecode
} as const;
