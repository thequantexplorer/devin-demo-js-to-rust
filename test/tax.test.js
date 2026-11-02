import { describe, it, expect } from 'vitest';
import { applyTax, money } from '../src/index.js';

describe('applyTax', () => {
  it('splits net, tax and gross', () => {
    expect(applyTax(money(100, 'USD'), 8.25)).toEqual({
      net: { amount: 10000, currency: 'USD' },
      tax: { amount: 825, currency: 'USD' },
      gross: { amount: 10825, currency: 'USD' },
    });
  });

  it('rounds tax half-even', () => {
    // 10.10 * 5% = 0.505 -> 0.50
    expect(applyTax(money(10.1, 'USD'), 5).tax.amount).toBe(50);
  });

  it('rejects negative rates', () => {
    expect(() => applyTax(money(1, 'USD'), -1)).toThrow(RangeError);
  });
});
