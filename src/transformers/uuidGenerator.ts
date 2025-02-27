import { Transformer } from '../types';

const uuidGenerator: Transformer = {
    id: 'uuidgenerator',
    title: 'UUID Generator',
    description: 'Generate random UUIDs',
    transform: (input: string): string => {
        try {
            // Generate a random UUID v4
            // This implementation follows the RFC4122 version 4 UUID format
            
            // If input is provided, use it as a seed for deterministic UUID generation
            // Otherwise, generate a completely random UUID
            let uuid = '';
            
            if (input.trim()) {
                // Simple deterministic UUID generation based on input string
                // Note: This is not cryptographically secure, just for convenience
                const hash = simpleHash(input);
                uuid = deterministicUUID(hash);
            } else {
                // Generate a completely random UUID
                uuid = randomUUID();
            }
            
            return uuid;
        } catch {
            return 'Error generating UUID';
        }
    }
};

// Generate a random UUID v4
function randomUUID(): string {
    return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (c) => {
        const r = Math.random() * 16 | 0;
        const v = c === 'x' ? r : (r & 0x3 | 0x8);
        return v.toString(16);
    });
}

// Generate a deterministic UUID based on a hash value
function deterministicUUID(hash: number): string {
    // Use the hash to seed a simple PRNG
    let seed = hash;
    const next = () => {
        seed = (seed * 9301 + 49297) % 233280;
        return seed / 233280;
    };
    
    return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (c) => {
        const r = Math.floor(next() * 16);
        const v = c === 'x' ? r : (r & 0x3 | 0x8);
        return v.toString(16);
    });
}

// Simple string hashing function
function simpleHash(str: string): number {
    let hash = 0;
    for (let i = 0; i < str.length; i++) {
        const char = str.charCodeAt(i);
        hash = ((hash << 5) - hash) + char;
        hash = hash & hash; // Convert to 32bit integer
    }
    return Math.abs(hash);
}

// UUID to Timestamp extractor (for v1 UUIDs)
const uuidToTimestamp: Transformer = {
    id: 'uuidtotimestamp',
    title: 'UUID to Timestamp',
    description: 'Extract timestamp from UUID v1',
    transform: (input: string): string => {
        try {
            if (!input.trim()) return '';
            
            // Remove hyphens and validate length
            const cleanUuid = input.replace(/-/g, '');
            if (cleanUuid.length !== 32) {
                return 'Invalid UUID format';
            }
            
            // Check if it's a v1 UUID
            const version = parseInt(cleanUuid.charAt(12), 16);
            if (version !== 1) {
                return 'Not a v1 UUID. Only v1 UUIDs contain timestamp information.';
            }
            
            // Extract time_low, time_mid, and time_hi_and_version fields
            const timeLow = cleanUuid.substring(0, 8);
            const timeMid = cleanUuid.substring(8, 12);
            const timeHiAndVersion = cleanUuid.substring(12, 16);
            
            // Remove version bits from timeHiAndVersion
            const timeHi = (parseInt(timeHiAndVersion, 16) & 0x0FFF).toString(16).padStart(4, '0');
            
            // Reconstruct the timestamp (100-nanosecond intervals since UUID epoch)
            const timestamp = parseInt(timeHi + timeMid + timeLow, 16);
            
            // Convert to milliseconds and adjust for UUID epoch (October 15, 1582)
            const uuidEpoch = Date.UTC(1582, 9, 15, 0, 0, 0, 0);
            const milliseconds = Math.floor(timestamp / 10000) + uuidEpoch;
            
            const date = new Date(milliseconds);
            
            return `UUID Timestamp: ${date.toISOString()}`;
        } catch {
            return 'Error extracting timestamp from UUID';
        }
    }
};

export default {
    uuidgenerator: uuidGenerator,
    uuidtotimestamp: uuidToTimestamp
} as const;
