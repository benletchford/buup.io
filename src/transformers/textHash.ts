import { Transformer } from '../types';

// Helper function to convert ArrayBuffer to hex string
function bufferToHex(buffer: ArrayBuffer): string {
    return Array.from(new Uint8Array(buffer))
        .map(b => b.toString(16).padStart(2, '0'))
        .join('');
}

// MD5 implementation
function md5(input: string): string {
    if (input === '') return '';

    // Convert string to UTF-8 encoded bytes
    const message = new TextEncoder().encode(input);
    
    // MD5 constants
    const K = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
        0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
        0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
        0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
        0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
        0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
        0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
        0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
        0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
        0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
        0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
        0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
        0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
        0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391
    ];
    
    // Per-round shift amounts
    const S = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
        5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20,
        4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
        6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21
    ];
    
    // Initial hash values
    let a0 = 0x67452301;
    let b0 = 0xefcdab89;
    let c0 = 0x98badcfe;
    let d0 = 0x10325476;
    
    // Preprocessing: padding the message
    const paddedLength = (((message.length + 8) >>> 6) + 1) << 6;
    const padded = new Uint8Array(paddedLength);
    
    // Copy original message
    padded.set(message);
    
    // Append 1 bit
    padded[message.length] = 0x80;
    
    // Append original length in bits as 64-bit little-endian integer
    const bitLength = message.length * 8;
    const dataView = new DataView(padded.buffer);
    dataView.setUint32(paddedLength - 8, bitLength, true);
    
    // Process the message in 16-word blocks (512 bits)
    for (let i = 0; i < paddedLength; i += 64) {
        // Break chunk into sixteen 32-bit words
        const M = new Uint32Array(16);
        for (let j = 0; j < 16; j++) {
            M[j] = dataView.getUint32(i + j * 4, true);
        }
        
        // Initialize hash value for this chunk
        let A = a0;
        let B = b0;
        let C = c0;
        let D = d0;
        
        // Main loop
        for (let j = 0; j < 64; j++) {
            let F, g;
            
            if (j < 16) {
                F = (B & C) | ((~B) & D);
                g = j;
            } else if (j < 32) {
                F = (D & B) | ((~D) & C);
                g = (5 * j + 1) % 16;
            } else if (j < 48) {
                F = B ^ C ^ D;
                g = (3 * j + 5) % 16;
            } else {
                F = C ^ (B | (~D));
                g = (7 * j) % 16;
            }
            
            F = (F + A + K[j] + M[g]) >>> 0;
            A = D;
            D = C;
            C = B;
            B = (B + ((F << S[j]) | (F >>> (32 - S[j])))) >>> 0;
        }
        
        // Add this chunk's hash to result
        a0 = (a0 + A) >>> 0;
        b0 = (b0 + B) >>> 0;
        c0 = (c0 + C) >>> 0;
        d0 = (d0 + D) >>> 0;
    }
    
    // Convert to little-endian bytes
    const result = new Uint8Array(16);
    const resultView = new DataView(result.buffer);
    resultView.setUint32(0, a0, true);
    resultView.setUint32(4, b0, true);
    resultView.setUint32(8, c0, true);
    resultView.setUint32(12, d0, true);
    
    // Convert to hex string
    return bufferToHex(result.buffer);
}

// SHA-1 implementation
function sha1(input: string): string {
    if (input === '') return '';

    // Convert string to UTF-8 encoded bytes
    const message = new TextEncoder().encode(input);
    
    // SHA-1 constants
    const K = [
        0x5a827999, // 0 <= t <= 19
        0x6ed9eba1, // 20 <= t <= 39
        0x8f1bbcdc, // 40 <= t <= 59
        0xca62c1d6  // 60 <= t <= 79
    ];
    
    // Initial hash values
    let h0 = 0x67452301;
    let h1 = 0xEFCDAB89;
    let h2 = 0x98BADCFE;
    let h3 = 0x10325476;
    let h4 = 0xC3D2E1F0;
    
    // Pre-processing: padding the message
    const paddedLength = (((message.length + 8) >>> 6) + 1) << 6;
    const padded = new Uint8Array(paddedLength);
    
    // Copy original message
    padded.set(message);
    
    // Append 1 bit
    padded[message.length] = 0x80;
    
    // Append original length in bits as 64-bit big-endian integer
    const bitLength = message.length * 8;
    const dataView = new DataView(padded.buffer);
    dataView.setBigUint64(paddedLength - 8, BigInt(bitLength), false);
    
    // Process the message in 16-word blocks (512 bits)
    for (let i = 0; i < paddedLength; i += 64) {
        // Break chunk into sixteen 32-bit words
        const words = new Uint32Array(80);
        for (let j = 0; j < 16; j++) {
            words[j] = dataView.getUint32(i + j * 4, false);
        }
        
        // Extend the sixteen 32-bit words into eighty 32-bit words
        for (let j = 16; j < 80; j++) {
            words[j] = ((words[j-3] ^ words[j-8] ^ words[j-14] ^ words[j-16]) << 1) | 
                       ((words[j-3] ^ words[j-8] ^ words[j-14] ^ words[j-16]) >>> 31);
        }
        
        // Initialize hash value for this chunk
        let a = h0;
        let b = h1;
        let c = h2;
        let d = h3;
        let e = h4;
        
        // Main loop
        for (let j = 0; j < 80; j++) {
            let f, k;
            
            if (j < 20) {
                f = (b & c) | ((~b) & d);
                k = K[0];
            } else if (j < 40) {
                f = b ^ c ^ d;
                k = K[1];
            } else if (j < 60) {
                f = (b & c) | (b & d) | (c & d);
                k = K[2];
            } else {
                f = b ^ c ^ d;
                k = K[3];
            }
            
            const temp = ((a << 5) | (a >>> 27)) + f + e + k + words[j];
            e = d;
            d = c;
            c = ((b << 30) | (b >>> 2));
            b = a;
            a = temp >>> 0;
        }
        
        // Add this chunk's hash to result
        h0 = (h0 + a) >>> 0;
        h1 = (h1 + b) >>> 0;
        h2 = (h2 + c) >>> 0;
        h3 = (h3 + d) >>> 0;
        h4 = (h4 + e) >>> 0;
    }
    
    // Produce the final hash value as a 160-bit number
    const result = new Uint8Array(20);
    const resultView = new DataView(result.buffer);
    resultView.setUint32(0, h0, false);
    resultView.setUint32(4, h1, false);
    resultView.setUint32(8, h2, false);
    resultView.setUint32(12, h3, false);
    resultView.setUint32(16, h4, false);
    
    // Convert to hex string
    return bufferToHex(result.buffer);
}

// SHA-256 implementation
function sha256(input: string): string {
    if (input === '') return '';

    // Convert string to UTF-8 encoded bytes
    const message = new TextEncoder().encode(input);
    
    // SHA-256 constants (first 32 bits of the fractional parts of the cube roots of the first 64 primes)
    const K = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
    ];
    
    // Initial hash values (first 32 bits of the fractional parts of the square roots of the first 8 primes)
    let h0 = 0x6a09e667;
    let h1 = 0xbb67ae85;
    let h2 = 0x3c6ef372;
    let h3 = 0xa54ff53a;
    let h4 = 0x510e527f;
    let h5 = 0x9b05688c;
    let h6 = 0x1f83d9ab;
    let h7 = 0x5be0cd19;
    
    // Pre-processing: padding the message
    const paddedLength = (((message.length + 8) >>> 6) + 1) << 6;
    const padded = new Uint8Array(paddedLength);
    
    // Copy original message
    padded.set(message);
    
    // Append 1 bit
    padded[message.length] = 0x80;
    
    // Append original length in bits as 64-bit big-endian integer
    const bitLength = message.length * 8;
    const dataView = new DataView(padded.buffer);
    dataView.setBigUint64(paddedLength - 8, BigInt(bitLength), false);
    
    // Process the message in 16-word blocks (512 bits)
    for (let i = 0; i < paddedLength; i += 64) {
        // Break chunk into sixteen 32-bit words
        const w = new Uint32Array(64);
        for (let j = 0; j < 16; j++) {
            w[j] = dataView.getUint32(i + j * 4, false);
        }
        
        // Extend the sixteen 32-bit words into sixty-four 32-bit words
        for (let j = 16; j < 64; j++) {
            const s0 = ((w[j-15] >>> 7) | (w[j-15] << 25)) ^ 
                       ((w[j-15] >>> 18) | (w[j-15] << 14)) ^ 
                       (w[j-15] >>> 3);
            const s1 = ((w[j-2] >>> 17) | (w[j-2] << 15)) ^ 
                       ((w[j-2] >>> 19) | (w[j-2] << 13)) ^ 
                       (w[j-2] >>> 10);
            w[j] = (w[j-16] + s0 + w[j-7] + s1) >>> 0;
        }
        
        // Initialize hash value for this chunk
        let a = h0;
        let b = h1;
        let c = h2;
        let d = h3;
        let e = h4;
        let f = h5;
        let g = h6;
        let h = h7;
        
        // Main loop
        for (let j = 0; j < 64; j++) {
            const S1 = ((e >>> 6) | (e << 26)) ^ 
                       ((e >>> 11) | (e << 21)) ^ 
                       ((e >>> 25) | (e << 7));
            const ch = (e & f) ^ ((~e) & g);
            const temp1 = (h + S1 + ch + K[j] + w[j]) >>> 0;
            const S0 = ((a >>> 2) | (a << 30)) ^ 
                       ((a >>> 13) | (a << 19)) ^ 
                       ((a >>> 22) | (a << 10));
            const maj = (a & b) ^ (a & c) ^ (b & c);
            const temp2 = (S0 + maj) >>> 0;
            
            h = g;
            g = f;
            f = e;
            e = (d + temp1) >>> 0;
            d = c;
            c = b;
            b = a;
            a = (temp1 + temp2) >>> 0;
        }
        
        // Add this chunk's hash to result
        h0 = (h0 + a) >>> 0;
        h1 = (h1 + b) >>> 0;
        h2 = (h2 + c) >>> 0;
        h3 = (h3 + d) >>> 0;
        h4 = (h4 + e) >>> 0;
        h5 = (h5 + f) >>> 0;
        h6 = (h6 + g) >>> 0;
        h7 = (h7 + h) >>> 0;
    }
    
    // Produce the final hash value as a 256-bit number
    const result = new Uint8Array(32);
    const resultView = new DataView(result.buffer);
    resultView.setUint32(0, h0, false);
    resultView.setUint32(4, h1, false);
    resultView.setUint32(8, h2, false);
    resultView.setUint32(12, h3, false);
    resultView.setUint32(16, h4, false);
    resultView.setUint32(20, h5, false);
    resultView.setUint32(24, h6, false);
    resultView.setUint32(28, h7, false);
    
    // Convert to hex string
    return bufferToHex(result.buffer);
}

// Transformer for MD5 hash
const md5Hash: Transformer = {
    id: 'md5hash',
    title: 'MD5 Hash',
    description: 'Generate MD5 hash from text input',
    transform: (input: string): string => {
        try {
            return md5(input);
        } catch (error) {
            console.error('Error generating MD5 hash:', error);
            return 'Error generating MD5 hash';
        }
    }
};

// Transformer for SHA-1 hash
const sha1Hash: Transformer = {
    id: 'sha1hash',
    title: 'SHA-1 Hash',
    description: 'Generate SHA-1 hash from text input',
    transform: (input: string): string => {
        try {
            return sha1(input);
        } catch (error) {
            console.error('Error generating SHA-1 hash:', error);
            return 'Error generating SHA-1 hash';
        }
    }
};

// Transformer for SHA-256 hash
const sha256Hash: Transformer = {
    id: 'sha256hash',
    title: 'SHA-256 Hash',
    description: 'Generate SHA-256 hash from text input',
    transform: (input: string): string => {
        try {
            return sha256(input);
        } catch (error) {
            console.error('Error generating SHA-256 hash:', error);
            return 'Error generating SHA-256 hash';
        }
    }
};

export default {
    md5hash: md5Hash,
    sha1hash: sha1Hash,
    sha256hash: sha256Hash
} as const;
