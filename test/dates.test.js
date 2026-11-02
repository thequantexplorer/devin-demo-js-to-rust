import { describe, it, expect } from 'vitest';
import { addMonths, dueDate } from '../src/index.js';

describe('addMonths', () => {
  it('clamps to the end of shorter months', () => {
    expect(addMonths('2026-01-31', 1)).toBe('2026-02-28');
    expect(addMonths('2028-01-31', 1)).toBe('2028-02-29');
    expect(addMonths('2026-05-31', 1)).toBe('2026-06-30');
  });

  it('crosses year ends in both directions', () => {
    expect(addMonths('2026-11-15', 3)).toBe('2027-02-15');
    expect(addMonths('2026-03-31', -1)).toBe('2026-02-28');
    expect(addMonths('2026-01-15', -1)).toBe('2025-12-15');
  });

  it('only accepts real YYYY-MM-DD dates', () => {
    expect(() => addMonths('2026-02-30', 1)).toThrow('Not a real calendar date');
    expect(() => addMonths('02/28/2026', 1)).toThrow(RangeError);
    expect(() => addMonths('2026-01-15', 1.5)).toThrow(RangeError);
  });
});

describe('dueDate', () => {
  it('adds a number of days and rolls off weekends', () => {
    expect(dueDate('2026-01-15', 30)).toBe('2026-02-16'); // Feb 14 is a Saturday
    expect(dueDate('2026-01-15', 30, { rollToBusinessDay: false })).toBe('2026-02-14');
  });

  it('reads net terms', () => {
    expect(dueDate('2026-03-02', 'net30')).toBe('2026-04-01');
    expect(dueDate('2026-03-02', 'NET 45')).toBe('2026-04-16');
  });

  it('is due the same day for due_on_receipt', () => {
    expect(dueDate('2026-03-10', 'due_on_receipt')).toBe('2026-03-10');
  });

  it('skips holidays when rolling', () => {
    expect(dueDate('2026-11-26', 0, { holidays: ['2026-11-26'] })).toBe('2026-11-27');
  });

  it('rejects unknown or negative terms', () => {
    expect(() => dueDate('2026-01-15', 'end of month')).toThrow('Unknown payment terms');
    expect(() => dueDate('2026-01-15', -5)).toThrow(RangeError);
  });
});
