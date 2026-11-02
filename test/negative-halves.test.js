// Regression tests: refunds must round like the mirror of the charge
// (README "Rules"). roundHalfEven(-2.5) used to return -3 instead of -2.
// rust/tests/negative_halves.rs runs the same inputs through the Rust crate.
import { describe, it, expect } from 'vitest';
import { money, multiply, roundHalfEven, applyTax } from '../src/index.js';

describe('negative exact halves', () => {
  it('rounds negative halves to the even neighbour', () => {
    expect(roundHalfEven(-2.5)).toBe(-2);
    expect(roundHalfEven(-0.5)).toBe(0);
    expect(roundHalfEven(-4.5)).toBe(-4);
  });

  it('builds half-cent refunds like the mirror of the charge', () => {
    expect(money(-0.125, 'USD').amount).toBe(-12);
    expect(money(-0.005, 'USD').amount).toBe(0);
  });

  it('multiplies a refund on a half like its mirror', () => {
    expect(multiply(money(-0.05, 'USD'), 0.5)).toEqual({ amount: -2, currency: 'USD' });
  });

  it('taxes a refund with the exact negative of the charge tax', () => {
    // 10.10 * 5% = 0.505 -> 0.50, so -10.10 * 5% must be -0.50.
    expect(applyTax(money(-10.1, 'USD'), 5)).toEqual({
      net: { amount: -1010, currency: 'USD' },
      tax: { amount: -50, currency: 'USD' },
      gross: { amount: -1060, currency: 'USD' },
    });
  });
});
