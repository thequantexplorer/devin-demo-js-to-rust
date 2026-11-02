import { add, multiply } from './money.js';

/**
 * @typedef {import('./money.js').Money} Money
 */

/**
 * @typedef {object} TaxBreakdown
 * @property {Money} net
 * @property {Money} tax
 * @property {Money} gross
 */

/**
 * Applies a percentage tax to a net amount.
 * @param {Money} net
 * @param {number} ratePercent e.g. 8.25 for 8.25%.
 * @returns {TaxBreakdown}
 */
export function applyTax(net, ratePercent) {
  if (ratePercent < 0) {
    throw new RangeError(`Tax rate cannot be negative: ${ratePercent}`);
  }
  const tax = multiply(net, ratePercent / 100);
  return { net, tax, gross: add(net, tax) };
}
