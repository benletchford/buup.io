import { describe, test, expect } from 'vitest';
import transformers from '../jwtDecode';

describe('JWT decode transformer', () => {
    const { transform } = transformers.jwtdecode;

    test('decodes a valid JWT token', () => {
        // This is a sample JWT token with the following payload:
        // Header: { "alg": "HS256", "typ": "JWT" }
        // Payload: { "sub": "1234567890", "name": "John Doe", "iat": 1516239022 }
        const jwt = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
        
        const result = transform(jwt);
        const parsed = JSON.parse(result);
        
        // Check header
        expect(parsed.header).toEqual({
            alg: 'HS256',
            typ: 'JWT'
        });
        
        // Check payload
        expect(parsed.payload).toEqual({
            sub: '1234567890',
            name: 'John Doe',
            iat: 1516239022
        });
        
        // Check signature (just verify it's present)
        expect(parsed.signature).toBe('SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c');
    });

    test('decodes a JWT token with complex payload', () => {
        // This is a sample JWT token with a more complex payload
        // Header: { "alg": "HS256", "typ": "JWT" }
        // Payload: { 
        //   "sub": "1234567890", 
        //   "name": "John Doe", 
        //   "admin": true, 
        //   "roles": ["user", "editor"], 
        //   "exp": 1516239122 
        // }
        const jwt = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsInJvbGVzIjpbInVzZXIiLCJlZGl0b3IiXSwiZXhwIjoxNTE2MjM5MTIyfQ.9vILLL9dIRfgFlHu0S-98uO5YRN9jQ4rKtWLDETKmT4';
        
        const result = transform(jwt);
        const parsed = JSON.parse(result);
        
        // Check payload with complex structure
        expect(parsed.payload).toEqual({
            sub: '1234567890',
            name: 'John Doe',
            admin: true,
            roles: ['user', 'editor'],
            exp: 1516239122
        });
    });

    test('handles invalid JWT format', () => {
        // Missing parts
        expect(transform('header.payload')).toBe('Invalid JWT token format. Expected format: header.payload.signature');
        
        // Empty string
        expect(transform('')).toBe('');
        
        // Whitespace
        expect(transform('  ')).toBe('');
    });

    test('handles invalid JWT content', () => {
        // Valid format but invalid content
        const invalidJwt = 'invalid.invalid.invalid';
        
        // This should throw an error when trying to decode or parse the parts
        const result = transform(invalidJwt);
        expect(result).toContain('Error decoding JWT:');
    });

    test('handles JWT with special characters', () => {
        // This JWT contains a payload with special characters in the values
        // Payload: { "name": "John & Jane", "message": "<script>alert('hello')</script>" }
        const jwt = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJuYW1lIjoiSm9obiAmIEphbmUiLCJtZXNzYWdlIjoiPHNjcmlwdD5hbGVydCgnaGVsbG8nKTwvc2NyaXB0PiJ9.8nYFMX2JewUXIQMHVvpPrDGTzQ9t9Br1NlrYR7z3ZVo';
        
        const result = transform(jwt);
        const parsed = JSON.parse(result);
        
        expect(parsed.payload).toEqual({
            name: 'John & Jane',
            message: "<script>alert('hello')</script>"
        });
    });
});
