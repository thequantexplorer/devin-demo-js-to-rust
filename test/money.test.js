import { describe, it, expect } from 'vitest';
import { money, add, multiply, allocate, roundHalfEven } from '../src/index.js';

describe('roundHalfEven', () => {
  it('sends exact halves to the even neighbour', () => {
    expect(roundHalfEven(0.5)).toBe(0);
    expect(roundHalfEven(1.5)).toBe(2);
    expect(roundHalfEven(2.5)).toBe(2);
    expect(roundHalfEven(3.5)).toBe(4);
  });

  it('rounds everything else to the nearest integer', () => {
    expect(roundHalfEven(1.2)).toBe(1);
    expect(roundHalfEven(1.7)).toBe(2);
    expect(roundHalfEven(-1.2)).toBe(-1);
    expect(roundHalfEven(-3.5)).toBe(-4);
  });

  it('rejects NaN and infinity', () => {
    expect(() => roundHalfEven(Number.NaN)).toThrow(RangeError);
    expect(() => roundHalfEven(Infinity)).toThrow('Cannot round Infinity');
  });
});

describe('money', () => {
  it('stores amounts as integer minor units', () => {
    expect(money(19.99, 'USD')).toEqual({ amount: 1999, currency: 'USD' });
    expect(money(500, 'jpy')).toEqual({ amount: 500, currency: 'JPY' });
    expect(money(-10.1, 'EUR')).toEqual({ amount: -1010, currency: 'EUR' });
  });

  it('rejects unsupported currencies', () => {
    expect(() => money(1, 'XYZ')).toThrow(RangeError);
    expect(() => money(1, 'BTC')).toThrow('Unsupported currency: BTC');
  });
});

describe('add', () => {
  it('adds one or more amounts of the same currency', () => {
    const a = money(10, 'USD');
    const b = money(2.5, 'USD');
    expect(add(a, b, b)).toEqual({ amount: 1500, currency: 'USD' });
    expect(add(a, money(-12.5, 'USD'))).toEqual({ amount: -250, currency: 'USD' });
  });

  it('refuses to mix currencies', () => {
    expect(() => add(money(10, 'USD'), money(1, 'EUR'))).toThrow('Currency mismatch: USD vs EUR');
  });
});

describe('multiply', () => {
  it('multiplies with half-even rounding', () => {
    expect(multiply(money(0.05, 'USD'), 0.5)).toEqual({ amount: 2, currency: 'USD' });
    expect(multiply(money(0.07, 'USD'), 0.5)).toEqual({ amount: 4, currency: 'USD' });
    expect(multiply(money(19.99, 'USD'), 3)).toEqual({ amount: 5997, currency: 'USD' });
  });
});

describe('allocate', () => {
  it('splits without losing a cent', () => {
    expect(allocate(money(1, 'USD'), [1, 1, 1]).map((p) => p.amount)).toEqual([34, 33, 33]);
    expect(allocate(money(10, 'USD'), [70, 30]).map((p) => p.amount)).toEqual([700, 300]);
  });

  it('splits a refund like its positive mirror', () => {
    expect(allocate(money(-1, 'USD'), [1, 1, 1]).map((p) => p.amount)).toEqual([-34, -33, -33]);
  });

  it('rejects bad weights', () => {
    expect(() => allocate(money(1, 'USD'), [])).toThrow(RangeError);
    expect(() => allocate(money(1, 'USD'), [0, 0])).toThrow('Weights must add up to more than zero');
    expect(() => allocate(money(1, 'USD'), [2, -1])).toThrow('Weights cannot be negative');
  });
});
