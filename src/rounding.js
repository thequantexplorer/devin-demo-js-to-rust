/**
 * Rounds to the nearest integer, sending exact halves to the nearest even
 * integer ("banker's rounding"). Used for all minor-unit math so that
 * totals don't drift over many line items.
 * @param {number} value
 * @returns {number}
 */
export function roundHalfEven(value) {
  if (!Number.isFinite(value)) {
    throw new RangeError(`Cannot round ${value}`);
  }
  const floor = Math.floor(value);
  const diff = value - floor;
  // Float math can leave 0.5 as 0.4999999999 (e.g. 1010 * 0.05).
  const EPSILON = 1e-9;
  if (Math.abs(diff - 0.5) < EPSILON) {
    // floor % 2 is -1 for negative odd floors, so compare against 0.
    return floor % 2 !== 0 ? floor + 1 : floor;
  }
  return Math.round(value);
}
