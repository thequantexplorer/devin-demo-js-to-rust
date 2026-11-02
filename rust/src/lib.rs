//! Rust port of `@acme/ledger-kit`: small money and date helpers used by
//! Acme's invoicing services.
//!
//! - Money is always an integer number of minor units (`i64`).
//! - Refunds and credit notes are negative amounts, treated as the mirror
//!   of the positive one.
//! - Dates are `YYYY-MM-DD` strings for UTC calendar days.

mod dates;
mod error;
mod money;
mod rounding;
mod tax;

pub use dates::{add_months, due_date, DueDateOptions, PaymentTerms};
pub use error::{Error, Result};
pub use money::{add, allocate, money, multiply, Currency, Money};
pub use rounding::round_half_even;
pub use tax::{apply_tax, TaxBreakdown};
