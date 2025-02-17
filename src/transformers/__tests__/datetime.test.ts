import { describe, it, expect } from 'vitest';
import datetimeTransformers from '../datetime';

describe('datetime transformers', () => {
  describe('Sydney timezone conversions', () => {
    const { sydneyToUtc, utcToSydney } = datetimeTransformers;

    it('should convert Sydney time to UTC', () => {
      // Test ISO format
      const result1 = sydneyToUtc.transform('2024-02-17T22:00:00.000+11:00');
      expect(result1).toMatch(/2024-02-17T11:00:00\.000[+Z]/);

      // Test common format
      const result2 = sydneyToUtc.transform('2024-02-17 22:00:00');
      expect(result2).toMatch(/2024-02-17T11:00:00\.000[+Z]/);
    });

    it('should convert UTC to Sydney time', () => {
      // Test ISO format
      const result3 = utcToSydney.transform('2024-02-17T11:00:00.000+00:00');
      expect(result3).toMatch(/2024-02-17T22:00:00\.000[+]1[01]:00/);

      // Test common format
      const result4 = utcToSydney.transform('2024-02-17 11:00:00');
      expect(result4).toMatch(/2024-02-17T22:00:00\.000[+]1[01]:00/);
    });

    it('should have correct metadata', () => {
      expect(sydneyToUtc.id).toBe('sydneyToUtc');
      expect(sydneyToUtc.title).toBe('Sydney to UTC');
      expect(sydneyToUtc.description).toBe('Convert Sydney time to UTC');
      expect(sydneyToUtc.inverse).toBe('utcToSydney');

      expect(utcToSydney.id).toBe('utcToSydney');
      expect(utcToSydney.title).toBe('UTC to Sydney');
      expect(utcToSydney.description).toBe('Convert UTC time to Sydney time');
      expect(utcToSydney.inverse).toBe('sydneyToUtc');
    });
  });

  describe('date format handling', () => {
    const { londonToUtc } = datetimeTransformers;

    it('should handle different date formats', () => {
      // ISO format
      const result5 = londonToUtc.transform('2024-02-17T10:00:00.000+00:00');
      expect(result5).toMatch(/2024-02-17T10:00:00\.000[+Z]/);

      // yyyy-MM-dd HH:mm:ss
      const result6 = londonToUtc.transform('2024-02-17 10:00:00');
      expect(result6).toMatch(/2024-02-17T10:00:00\.000[+Z]/);

      // dd/MM/yyyy HH:mm:ss
      const result7 = londonToUtc.transform('17/02/2024 10:00:00');
      expect(result7).toMatch(/2024-02-17T10:00:00\.000[+Z]/);

      // MM/dd/yyyy HH:mm:ss
      const result8 = londonToUtc.transform('02/17/2024 10:00:00');
      expect(result8).toMatch(/2024-02-17T10:00:00\.000[+Z]/);
    });

    it('should handle date-only formats', () => {
      // yyyy-MM-dd
      const result9 = londonToUtc.transform('2024-02-17');
      expect(result9).toMatch(/2024-02-17T00:00:00\.000[+Z]/);

      // dd/MM/yyyy
      const result10 = londonToUtc.transform('17/02/2024');
      expect(result10).toMatch(/2024-02-17T00:00:00\.000[+Z]/);

      // MM/dd/yyyy
      const result11 = londonToUtc.transform('02/17/2024');
      expect(result11).toMatch(/2024-02-17T00:00:00\.000[+Z]/);
    });
  });

  describe('error handling', () => {
    const { tokyoToUtc, utcToTokyo } = datetimeTransformers;

    it('should handle invalid date formats', () => {
      expect(tokyoToUtc.transform('invalid date'))
        .toBe('Invalid date format. Try: YYYY-MM-DD HH:mm:ss or DD/MM/YYYY HH:mm:ss');
      
      expect(utcToTokyo.transform('not a date'))
        .toBe('Invalid date format. Try: YYYY-MM-DD HH:mm:ss or DD/MM/YYYY HH:mm:ss');
    });

    it('should handle empty input', () => {
      expect(tokyoToUtc.transform(''))
        .toBe('Invalid date format. Try: YYYY-MM-DD HH:mm:ss or DD/MM/YYYY HH:mm:ss');
    });
  });

  describe('bidirectional conversion', () => {
    const { singaporeToUtc, utcToSingapore } = datetimeTransformers;

    it('should correctly convert time back and forth', () => {
      const originalTime = '2024-02-17T15:00:00.000+08:00'; // Singapore time
      const utcTime = singaporeToUtc.transform(originalTime);
      const backToSingapore = utcToSingapore.transform(utcTime);
      
      // Compare the timestamps by parsing them to account for equivalent but differently formatted times
      const originalDate = new Date(originalTime);
      const finalDate = new Date(backToSingapore);
      expect(finalDate.getTime()).toBe(originalDate.getTime());
    });
  });

  describe('all timezone pairs', () => {
    const timezones = [
      'sydney', 'london', 'newyork', 'losangeles', 'tokyo',
      'paris', 'singapore', 'dubai', 'auckland'
    ];

    it('should have all timezone pairs', () => {
      timezones.forEach(timezone => {
        const toUtc = `${timezone}ToUtc`;
        const fromUtc = `utcTo${timezone}`;

        expect(datetimeTransformers).toHaveProperty(toUtc);
        expect(datetimeTransformers).toHaveProperty(fromUtc);
        
        expect(datetimeTransformers[toUtc].inverse).toBe(fromUtc);
        expect(datetimeTransformers[fromUtc].inverse).toBe(toUtc);
      });
    });
  });
});
