# @acme/ledger-kit

Small money and date helpers used by Acme's invoicing services. It is plain JavaScript and needs Node 22 or later.

```sh
npm ci
npm test
```

## What it does

Eight public functions, all exported from `src/index.js`:

| Function | What it does |
| --- | --- |
| `roundHalfEven(value)` | Rounds to the nearest integer. Exact halves go to the even neighbour: `2.5 → 2`, `3.5 → 4`. |
| `money(major, currency)` | Builds `{ amount, currency }` in integer minor units: `money(19.99, 'USD')` is 1999 cents. Supports USD, EUR, GBP and JPY (no minor units). |
| `add(first, ...rest)` | Adds amounts of the same currency. Mixing currencies throws. |
| `multiply(value, factor)` | Multiplies by a quantity or rate and rounds half-even. |
| `allocate(value, weights)` | Splits an amount by weight without losing a cent: $1.00 three ways is 34, 33, 33 cents. |
| `applyTax(net, ratePercent)` | Returns `{ net, tax, gross }`. Tax is rounded half-even. |
| `addMonths(date, months)` | Adds calendar months to a `YYYY-MM-DD` date and clamps to the month end: `2026-01-31` + 1 is `2026-02-28`. |
| `dueDate(issued, terms, options)` | Works out a due date from `30`, `'net30'`, `'NET 45'` or `'due_on_receipt'`. A due date on a weekend or holiday moves to the next business day. |

## Rules

- Money is always an integer number of minor units (cents, yen). Never a float.
- Refunds and credit notes are negative amounts. Every money function treats a negative amount as the mirror of the positive one. Rounding `-2.5` gives `-2`, splitting -$1.00 three ways gives -34, -33, -33 cents, and the tax on a refund is the exact negative of the tax on the charge.
- Dates are `YYYY-MM-DD` strings for UTC calendar days. Anything else throws.

## Known problem

The billing service is being rewritten in Rust, and it needs these exact rules. Today it has to copy them by hand, because this repo has no Rust crate yet:

```text
$ cargo test --manifest-path rust/Cargo.toml
error: manifest path `rust/Cargo.toml` does not exist
```

The plan is to port the library to a Rust crate in `rust/` with the same functions and tests, and to run both test suites in CI so the two versions can't drift apart.

---

This repo is a starter for a Devin lab. See [DEVIN_DEMO.md](DEVIN_DEMO.md) for the steps and the prompt.
