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

const formatDateTime = (date: Date, timezone: string, formatStr: string): string => {
  try {
    const zonedDate = toZonedTime(date, timezone);
    return format(zonedDate, formatStr);
  } catch {
    return '';
  }
};

const createTransform = (from: TimezoneConfig, to: TimezoneConfig) => (input: string): string => {
  try {
    const parsedDate = parseDateTime(input, from);
    if (!parsedDate) {
      return '';
    }

    // For UTC to Sydney conversion
    if (from.name === 'utc' && to.name === 'sydney') {
      // Convert the UTC date to Sydney timezone and format with timezone offset
      return formatDateTime(parsedDate, to.tzDatabase, "yyyy-MM-dd'T'HH:mm:ssXXX");
    }
    
    // For Sydney to UTC conversion
    if (from.name === 'sydney' && to.name === 'utc') {
      // Convert to UTC and format with Z suffix
      return formatDateTime(parsedDate, 'UTC', "yyyy-MM-dd'T'HH:mm:ss'Z'");
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
