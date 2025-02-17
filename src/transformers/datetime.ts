import { TransformerModule } from '../types';
import { format, toZonedTime } from 'date-fns-tz';
import { parse, isValid } from 'date-fns';

// Common timezone library
const TIMEZONES = {
    'UTC': 'UTC',
    'Sydney': 'Australia/Sydney',
    'London': 'Europe/London',
    'NewYork': 'America/New_York',
    'LosAngeles': 'America/Los_Angeles',
    'Tokyo': 'Asia/Tokyo',
    'Paris': 'Europe/Paris',
    'Singapore': 'Asia/Singapore',
    'Dubai': 'Asia/Dubai',
    'Auckland': 'Pacific/Auckland'
} as const;

const DATE_FORMAT = "yyyy-MM-dd'T'HH:mm:ss.SSSxxx";

const parseDateTime = (input: string, sourceTimezone: string): Date | null => {
    // Try ISO format first
    let date = new Date(input);
    if (isValid(date)) {
        return date;
    }

    // Try common formats
    const formats = [
        'yyyy-MM-dd HH:mm:ss',
        'dd/MM/yyyy HH:mm:ss',
        'MM/dd/yyyy HH:mm:ss',
        'yyyy-MM-dd',
        'dd/MM/yyyy',
        'MM/dd/yyyy'
    ];

    for (const fmt of formats) {
        try {
            date = parse(input, fmt, new Date());
            if (isValid(date)) {
                // For non-ISO formats, assume the time is in the source timezone
                // and set hours to 00:00:00 for date-only formats
                if (!fmt.includes('HH:mm:ss')) {
                    const zonedDate = toZonedTime(date, sourceTimezone);
                    const midnight = new Date(zonedDate);
                    midnight.setUTCHours(0, 0, 0, 0);
                    return midnight;
                }
                return date;
            }
        } catch {
            continue;
        }
    }

    return null;
};

// Generate transformers for each timezone pair with UTC
const generateTransformers = (): TransformerModule => {
    const transformers: TransformerModule = {};

    Object.entries(TIMEZONES).forEach(([name, zone]) => {
        if (zone === 'UTC') return; // Skip UTC-to-UTC transformer

        // Create timezone to UTC transformer
        const toUtcId = `${name.toLowerCase()}ToUtc`;
        transformers[toUtcId] = {
            id: toUtcId,
            title: `${name} to UTC`,
            description: `Convert ${name} time to UTC`,
            inverse: `utcTo${name}`,
            transform: (input: string): string => {
                try {
                    const date = parseDateTime(input, zone);
                    if (!date) {
                        return 'Invalid date format. Try: YYYY-MM-DD HH:mm:ss or DD/MM/YYYY HH:mm:ss';
                    }

                    // Convert to UTC
                    const zonedDate = toZonedTime(date, zone);
                    const utcDate = toZonedTime(zonedDate, 'UTC');
                    return format(utcDate, DATE_FORMAT, { timeZone: 'UTC' });
                } catch {
                    return 'Error converting time. Please check your input format.';
                }
            }
        };

        // Create UTC to timezone transformer
        const fromUtcId = `utcTo${name}`;
        transformers[fromUtcId] = {
            id: fromUtcId,
            title: `UTC to ${name}`,
            description: `Convert UTC time to ${name} time`,
            inverse: toUtcId,
            transform: (input: string): string => {
                try {
                    const date = parseDateTime(input, 'UTC');
                    if (!date) {
                        return 'Invalid date format. Try: YYYY-MM-DD HH:mm:ss or DD/MM/YYYY HH:mm:ss';
                    }

                    // Convert to target timezone
                    const utcDate = toZonedTime(date, 'UTC');
                    const zonedDate = toZonedTime(utcDate, zone);
                    return format(zonedDate, DATE_FORMAT, { timeZone: zone });
                } catch {
                    return 'Error converting time. Please check your input format.';
                }
            }
        };
    });

    return transformers;
};

export default generateTransformers();
