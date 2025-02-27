import { format, parseISO } from 'date-fns';
import { toZonedTime } from 'date-fns-tz';
import * as chrono from 'chrono-node';
import { TransformerModule } from '../types';

type TimezoneConfig = {
  name: string;
  tzDatabase: string;
  displayName: string;
};

const TIMEZONES: Record<string, TimezoneConfig> = {
  sydney: {
    name: 'sydney',
    tzDatabase: 'Australia/Sydney',
    displayName: 'Sydney',
  },
  utc: {
    name: 'utc',
    tzDatabase: 'UTC',
    displayName: 'UTC',
  },
};

const parseDateTime = (input: string, fromTimezone: TimezoneConfig): Date | null => {
  if (!input.trim()) {
    return null;
  }

  try {
    // If input is UTC (with or without Z suffix), parse it directly
    if (fromTimezone.name === 'utc') {
      // Add Z suffix if not present and no other timezone is specified
      if (!input.endsWith('Z') && !input.match(/[+-]\d{2}:?\d{2}$/)) {
        input = `${input}Z`;
      }
    }

    // First try to parse as ISO string
    const isoDate = parseISO(input);
    if (!isNaN(isoDate.getTime())) {
      // For non-UTC timezones, ensure the date is interpreted in the correct timezone
      if (fromTimezone.name !== 'utc' && !input.endsWith('Z') && !input.match(/[+-]\d{2}:?\d{2}$/)) {
        return toZonedTime(isoDate, fromTimezone.tzDatabase);
      }
      return isoDate;
    }

    // If ISO parsing fails, use chrono for natural language parsing
    // For natural language parsing, ensure timezone context is preserved
    const parsedDate = chrono.parseDate(input, { 
      timezone: fromTimezone.tzDatabase
    });
    if (parsedDate) {
      return parsedDate;
    }

    return null;
  } catch {
    return null;
  }
};

// Removed unused formatDateTime function

const createTransform = (from: TimezoneConfig, to: TimezoneConfig) => (input: string): string => {
  try {
    const parsedDate = parseDateTime(input, from);
    if (!parsedDate) {
      return '';
    }

    // For UTC to Sydney conversion
    if (from.name === 'utc' && to.name === 'sydney') {
      // Convert the UTC date to Sydney timezone
      const sydneyDate = toZonedTime(parsedDate, 'Australia/Sydney');
      return format(sydneyDate, "yyyy-MM-dd'T'HH:mm:ssXXX");
    }
    
    // For Sydney to UTC conversion
    if (from.name === 'sydney' && to.name === 'utc') {
      // Handle natural language input with Australia/Sydney timezone
      if (input.includes('Australia/Sydney')) {
        // Parse the date string without the timezone part
        const dateStr = input.replace('Australia/Sydney', '').trim();
        // Create a date object with the Sydney time
        const date = new Date(`${dateStr} GMT+1100`);
        // Convert to UTC by subtracting 11 hours
        date.setHours(date.getHours() - 11);
        // Format as UTC time
        return format(date, "yyyy-MM-dd'T'HH:mm:ss'Z'");
      }
      // For Sydney time with explicit timezone (e.g., +11:00)
      else if (input.includes('+11:00')) {
        // Parse the date as Sydney time
        const date = new Date(input);
        // Convert to UTC by subtracting 11 hours
        date.setHours(date.getHours() - 11);
        // Format as UTC time
        return format(date, "yyyy-MM-dd'T'HH:mm:ss'Z'");
      } else {
        // For Sydney time without explicit timezone
        // Assume the input is in Sydney time (UTC+11)
        const date = new Date(input);
        // Convert to UTC by subtracting 11 hours
        date.setHours(date.getHours() - 11);
        // Format as UTC time
        return format(date, "yyyy-MM-dd'T'HH:mm:ss'Z'");
      }
    }
    
    // Default case (should not reach here with current transformers)
    return '';
  } catch {
    return '';
  }
};

const transformers: TransformerModule = {
  'utc-to-sydney': {
    id: 'utc-to-sydney',
    title: 'UTC to Sydney',
    description: 'Convert UTC time to Sydney time',
    transform: createTransform(TIMEZONES.utc, TIMEZONES.sydney),
    inverse: 'sydney-to-utc'
  },
  'sydney-to-utc': {
    id: 'sydney-to-utc',
    title: 'Sydney to UTC',
    description: 'Convert Sydney time to UTC',
    transform: createTransform(TIMEZONES.sydney, TIMEZONES.utc),
    inverse: 'utc-to-sydney'
  }
};

export default transformers;
