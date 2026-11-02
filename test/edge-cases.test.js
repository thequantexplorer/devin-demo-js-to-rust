// Edge cases from the README "Rules" section that the original tests don't
// cover. rust/tests/edge_cases.rs runs the same inputs through the Rust crate.
import { describe, it, expect } from 'vitest';
import { money, add, multiply, allocate, roundHalfEven, applyTax, addMonths, dueDate } from '../src/index.js';

const amounts = (parts) => parts.map((p) => p.amount);

describe('refunds mirror charges', () => {
  it('builds negative money like the positive mirror', () => {
    expect(money(-19.99, 'USD')).toEqual({ amount: -1999, currency: 'USD' });
    expect(money(-500, 'JPY')).toEqual({ amount: -500, currency: 'JPY' });
  });

  it('adds a refund back to zero', () => {
    expect(add(money(19.99, 'USD'), money(-19.99, 'USD'))).toEqual({ amount: 0, currency: 'USD' });
  });

  it('multiplies a refund like its mirror', () => {
    expect(multiply(money(-19.99, 'USD'), 3)).toEqual({ amount: -5997, currency: 'USD' });
    expect(multiply(money(-0.07, 'USD'), 0.5)).toEqual({ amount: -4, currency: 'USD' });
  });

  it('allocates refunds like their mirror', () => {
    expect(amounts(allocate(money(-10, 'USD'), [70, 30]))).toEqual([-700, -300]);
    expect(amounts(allocate(money(0.02, 'USD'), [1, 1, 1]))).toEqual([1, 1, 0]);
    expect(amounts(allocate(money(-0.02, 'USD'), [1, 1, 1]))).toEqual([-1, -1, 0]);
    expect(amounts(allocate(money(-0.05, 'USD'), [1, 1]))).toEqual([-3, -2]);
  });

  it('taxes a refund with the exact negative of the charge tax', () => {
    expect(applyTax(money(-100, 'USD'), 8.25)).toEqual({
      net: { amount: -10000, currency: 'USD' },
      tax: { amount: -825, currency: 'USD' },
      gross: { amount: -10825, currency: 'USD' },
    });
  });
});

describe('exact halves', () => {
  it('rounds positive halves to even', () => {
    expect(roundHalfEven(4.5)).toBe(4);
    expect(roundHalfEven(5.5)).toBe(6);
    expect(roundHalfEven(-1.5)).toBe(-2);
  });

  it('rounds half-cent money and products to even', () => {
    expect(money(0.125, 'USD').amount).toBe(12);
    expect(money(0.005, 'USD').amount).toBe(0);
    expect(multiply(money(0.03, 'USD'), 0.5).amount).toBe(2);
  });
});

describe('month and year ends', () => {
  it('adds months across month and year ends', () => {
    expect(addMonths('2026-12-31', 1)).toBe('2027-01-31');
    expect(addMonths('2027-01-31', -1)).toBe('2026-12-31');
    expect(addMonths('2026-12-31', -12)).toBe('2025-12-31');
    expect(addMonths('2026-01-01', -1)).toBe('2025-12-01');
    expect(addMonths('2026-03-31', 1)).toBe('2026-04-30');
    expect(addMonths('2026-10-31', -8)).toBe('2026-02-28');
  });

  it('rolls due dates across year ends and holidays', () => {
    expect(dueDate('2026-12-31', 'net1')).toBe('2027-01-01');
    expect(dueDate('2026-12-31', 'net1', { holidays: ['2027-01-01'] })).toBe('2027-01-04');
    expect(dueDate('2026-12-24', 1, { holidays: ['2026-12-25'] })).toBe('2026-12-28');
    expect(dueDate('2026-02-27', 1)).toBe('2026-03-02');
  });
});

describe('leap years', () => {
  it('clamps to Feb 29 only in leap years', () => {
    expect(addMonths('2024-01-31', 1)).toBe('2024-02-29');
    expect(addMonths('2000-01-31', 1)).toBe('2000-02-29');
    expect(addMonths('2100-01-31', 1)).toBe('2100-02-28');
    expect(addMonths('2024-02-29', 12)).toBe('2025-02-28');
    expect(addMonths('2024-02-29', -12)).toBe('2023-02-28');
    expect(addMonths('2024-02-29', 48)).toBe('2028-02-29');
    expect(addMonths('2024-02-29', 0)).toBe('2024-02-29');
    expect(dueDate('2028-02-28', 1)).toBe('2028-02-29');
  });

  it('rejects Feb 29 outside leap years', () => {
    expect(() => addMonths('2025-02-29', 1)).toThrow('Not a real calendar date');
    expect(() => addMonths('2100-02-29', 1)).toThrow('Not a real calendar date');
  });
});

describe('date format', () => {
  it('throws on anything that is not a real YYYY-MM-DD day', () => {
    expect(() => addMonths('2026-04-31', 1)).toThrow('Not a real calendar date');
    expect(() => addMonths('2026-13-01', 1)).toThrow('Not a real calendar date');
    expect(() => addMonths('2026-00-10', 1)).toThrow('Not a real calendar date');
    expect(() => addMonths('2026-1-05', 1)).toThrow('Expected YYYY-MM-DD');
    expect(() => addMonths('2026-01-05T00:00:00Z', 1)).toThrow('Expected YYYY-MM-DD');
    expect(() => dueDate(' 2026-01-05', 30)).toThrow('Expected YYYY-MM-DD');
  });
});
