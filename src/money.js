import { roundHalfEven } from './rounding.js';

/**
 * @typedef {'USD' | 'EUR' | 'GBP' | 'JPY'} CurrencyCode
 */

/**
 * An amount of money in integer minor units (cents for USD, yen for JPY).
 * Negative amounts are refunds and credit notes.
 * @typedef {object} Money
 * @property {number} amount Integer number of minor units.
 * @property {CurrencyCode} currency
 */

/** Number of minor-unit digits for each supported currency. */
const DECIMALS = { USD: 2, EUR: 2, GBP: 2, JPY: 0 };

/**
 * @param {string} code
 * @returns {CurrencyCode}
 */
function currencyCode(code) {
  const upper = String(code).toUpperCase();
  if (!Object.prototype.hasOwnProperty.call(DECIMALS, upper)) {
    throw new RangeError(`Unsupported currency: ${code}`);
  }
  return /** @type {CurrencyCode} */ (upper);
}

/**
 * @param {Money} a
 * @param {Money} b
 */
function assertSameCurrency(a, b) {
  if (a.currency !== b.currency) {
    throw new TypeError(`Currency mismatch: ${a.currency} vs ${b.currency}`);
  }
}

/**
 * Builds a Money value from a decimal amount, e.g. money(19.99, 'USD').
 * @param {number} major Amount in major units (dollars, euros, yen).
 * @param {string} currency Case-insensitive ISO code.
 * @returns {Money}
 */
export function money(major, currency) {
  const code = currencyCode(currency);
  return { amount: roundHalfEven(major * 10 ** DECIMALS[code]), currency: code };
}

/**
 * Adds one or more Money values of the same currency.
 * @param {Money} first
 * @param {...Money} rest
 * @returns {Money}
 */
export function add(first, ...rest) {
  let amount = first.amount;
  for (const m of rest) {
    assertSameCurrency(first, m);
    amount += m.amount;
  }
  return { amount, currency: first.currency };
}

/**
 * Multiplies by a plain number (quantity, rate) and rounds half-even.
 * @param {Money} value
 * @param {number} factor
 * @returns {Money}
 */
export function multiply(value, factor) {
  return { amount: roundHalfEven(value.amount * factor), currency: value.currency };
}

/**
 * Splits Money into parts by weight without losing a cent. Leftover minor
 * units go to the earliest parts, one each. A negative amount is split
 * like its positive mirror.
 * @param {Money} value
 * @param {number[]} weights
 * @returns {Money[]}
 */
export function allocate(value, weights) {
  if (weights.length === 0) {
    throw new RangeError('allocate needs at least one weight');
  }
  if (weights.some((w) => w < 0)) {
    throw new RangeError('Weights cannot be negative');
  }
  const total = weights.reduce((sum, w) => sum + w, 0);
  if (total <= 0) {
    throw new RangeError('Weights must add up to more than zero');
  }
  const sign = value.amount < 0 ? -1 : 1;
  const abs = Math.abs(value.amount);
  const parts = weights.map((w) => Math.floor((abs * w) / total));
  let remainder = abs - parts.reduce((sum, p) => sum + p, 0);
  for (let i = 0; remainder > 0; i = (i + 1) % parts.length) {
    parts[i] += 1;
    remainder -= 1;
  }
  return parts.map((p) => ({ amount: p === 0 ? 0 : sign * p, currency: value.currency }));
}
