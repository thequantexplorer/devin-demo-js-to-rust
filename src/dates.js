// Dates are plain "YYYY-MM-DD" strings, read as UTC calendar days.

/**
 * @param {string} text
 * @returns {Date} Midnight UTC on that day.
 */
function parseISODate(text) {
  const match = /^(\d{4})-(\d{2})-(\d{2})$/.exec(text);
  if (!match) {
    throw new RangeError(`Expected YYYY-MM-DD, got "${text}"`);
  }
  const [, y, m, d] = match;
  const date = new Date(Date.UTC(Number(y), Number(m) - 1, Number(d)));
  if (toISODate(date) !== text) {
    throw new RangeError(`Not a real calendar date: "${text}"`);
  }
  return date;
}

/**
 * @param {Date} date
 * @returns {string}
 */
function toISODate(date) {
  return date.toISOString().slice(0, 10);
}

/**
 * @param {Date} date
 * @param {number} days
 * @returns {Date}
 */
function addDays(date, days) {
  const result = new Date(date.getTime());
  result.setUTCDate(result.getUTCDate() + days);
  return result;
}

/**
 * Adds calendar months. Negative values go back in time. If the target
 * month is shorter, the day is clamped to its last day:
 * 2026-01-31 + 1 month = 2026-02-28.
 * @param {string} date YYYY-MM-DD
 * @param {number} months Whole number.
 * @returns {string} YYYY-MM-DD
 */
export function addMonths(date, months) {
  if (!Number.isInteger(months)) {
    throw new RangeError(`months must be a whole number, got ${months}`);
  }
  const start = parseISODate(date);
  const year = start.getUTCFullYear();
  const month = start.getUTCMonth() + months;
  const lastDay = new Date(Date.UTC(year, month + 1, 0)).getUTCDate();
  const day = Math.min(start.getUTCDate(), lastDay);
  return toISODate(new Date(Date.UTC(year, month, day)));
}

/**
 * Payment terms: a number of days (30), a "net" string ("net30", "NET 45"),
 * or "due_on_receipt".
 * @typedef {number | string} PaymentTerms
 */

/**
 * Works out when an invoice is due. A due date on a Saturday, Sunday or
 * holiday moves to the next business day, unless rollToBusinessDay is false.
 * @param {string} issued Invoice date, YYYY-MM-DD.
 * @param {PaymentTerms} terms
 * @param {object} [options]
 * @param {string[]} [options.holidays] YYYY-MM-DD dates to skip.
 * @param {boolean} [options.rollToBusinessDay] Defaults to true.
 * @returns {string} YYYY-MM-DD
 */
export function dueDate(issued, terms, options = {}) {
  const { holidays = [], rollToBusinessDay = true } = options;
  const start = parseISODate(issued);

  let days;
  if (typeof terms === 'number') {
    days = terms;
  } else if (terms.trim().toLowerCase() === 'due_on_receipt') {
    days = 0;
  } else {
    const match = /^net\s*(\d+)$/i.exec(terms.trim());
    if (!match) {
      throw new RangeError(`Unknown payment terms: "${terms}"`);
    }
    days = Number(match[1]);
  }
  if (!Number.isInteger(days) || days < 0) {
    throw new RangeError(`Payment days must be a whole number >= 0, got ${days}`);
  }

  let due = addDays(start, days);
  if (rollToBusinessDay) {
    const isBusinessDay = (d) => d.getUTCDay() !== 0 && d.getUTCDay() !== 6 && !holidays.includes(toISODate(d));
    while (!isBusinessDay(due)) {
      due = addDays(due, 1);
    }
  }
  return toISODate(due);
}
