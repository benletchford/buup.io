import { Transformer } from '../types';

const csvToJson: Transformer = {
    id: 'csvtojson',
    title: 'CSV to JSON',
    description: 'Convert CSV data to JSON format',
    inverse: 'jsontocsv',
    transform: (input: string): string => {
        try {
            if (!input.trim()) return '';
            
            // Split the input into lines
            const lines = input.split(/\r?\n/).filter(line => line.trim());
            if (lines.length === 0) return '[]';
            
            // Parse the header row
            const headers = parseCSVLine(lines[0]);
            
            // Parse the data rows
            const result = [];
            for (let i = 1; i < lines.length; i++) {
                const values = parseCSVLine(lines[i]);
                if (values.length === 0) continue;
                
                // Create an object for each row
                const obj: Record<string, string> = {};
                for (let j = 0; j < headers.length; j++) {
                    // Use the header as the key, or a default if the header is empty
                    const key = headers[j] || `column${j + 1}`;
                    obj[key] = j < values.length ? values[j] : '';
                }
                result.push(obj);
            }
            
            return JSON.stringify(result, null, 2);
        } catch {
            return 'Error converting CSV to JSON';
        }
    }
};

// Helper function to parse a CSV line, handling quoted values
function parseCSVLine(line: string): string[] {
    const result: string[] = [];
    let current = '';
    let inQuotes = false;
    
    for (let i = 0; i < line.length; i++) {
        const char = line[i];
        
        if (char === '"') {
            // Handle escaped quotes (two double quotes in a row)
            if (inQuotes && i + 1 < line.length && line[i + 1] === '"') {
                current += '"';
                i++; // Skip the next quote
            } else {
                // Toggle quote mode
                inQuotes = !inQuotes;
            }
        } else if (char === ',' && !inQuotes) {
            // End of field
            result.push(current);
            current = '';
        } else {
            current += char;
        }
    }
    
    // Add the last field
    result.push(current);
    
    return result;
}

const jsonToCsv: Transformer = {
    id: 'jsontocsv',
    title: 'JSON to CSV',
    description: 'Convert JSON data to CSV format',
    inverse: 'csvtojson',
    transform: (input: string): string => {
        try {
            if (!input.trim()) return '';
            
            // Parse the JSON input
            const data = JSON.parse(input);
            
            // Handle non-array JSON
            if (!Array.isArray(data)) {
                return 'Input must be a JSON array';
            }
            
            if (data.length === 0) return '';
            
            // Extract all possible headers from all objects
            const headers = new Set<string>();
            data.forEach(item => {
                if (typeof item === 'object' && item !== null) {
                    Object.keys(item).forEach(key => headers.add(key));
                }
            });
            
            const headerArray = Array.from(headers);
            
            // Create the CSV header row
            let csv = headerArray.map(escapeCSVValue).join(',') + '\n';
            
            // Create the data rows
            data.forEach(item => {
                if (typeof item !== 'object' || item === null) {
                    // Skip non-object items
                    return;
                }
                
                const row = headerArray.map(header => {
                    const value = item[header];
                    // Convert null to empty string
                    return escapeCSVValue(value !== undefined && value !== null ? String(value) : '');
                });
                
                csv += row.join(',') + '\n';
            });
            
            return csv;
        } catch {
            return 'Error converting JSON to CSV';
        }
    }
};

// Helper function to escape CSV values
function escapeCSVValue(value: string): string {
    // If the value contains commas, newlines, or quotes, wrap it in quotes and escape any quotes
    if (value.includes(',') || value.includes('\n') || value.includes('"')) {
        return `"${value.replace(/"/g, '""')}"`;
    }
    return value;
}

export default {
    csvtojson: csvToJson,
    jsontocsv: jsonToCsv
} as const;
