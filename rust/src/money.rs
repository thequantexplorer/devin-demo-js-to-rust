use std::fmt;

use crate::error::{Error, Result};
use crate::rounding::round_half_even;

/// The supported ISO 4217 currencies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    Usd,
    Eur,
    Gbp,
    Jpy,
}

impl Currency {
    /// Parses a case-insensitive ISO code such as `"usd"` or `"JPY"`.
    pub fn from_code(code: &str) -> Result<Currency> {
        match code.to_uppercase().as_str() {
            "USD" => Ok(Currency::Usd),
            "EUR" => Ok(Currency::Eur),
            "GBP" => Ok(Currency::Gbp),
            "JPY" => Ok(Currency::Jpy),
            _ => Err(Error::UnsupportedCurrency(code.to_string())),
        }
    }

    /// The upper-case ISO code, e.g. `"USD"`.
    pub fn code(self) -> &'static str {
        match self {
            Currency::Usd => "USD",
            Currency::Eur => "EUR",
            Currency::Gbp => "GBP",
            Currency::Jpy => "JPY",
        }
    }

    /// Number of minor-unit digits (2 for cents, 0 for yen).
    pub fn decimals(self) -> u32 {
        match self {
            Currency::Usd | Currency::Eur | Currency::Gbp => 2,
            Currency::Jpy => 0,
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

/// An amount of money in integer minor units (cents for USD, yen for JPY).
/// Negative amounts are refunds and credit notes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Money {
    pub amount: i64,
    pub currency: Currency,
}

fn assert_same_currency(a: &Money, b: &Money) -> Result<()> {
    if a.currency != b.currency {
        return Err(Error::CurrencyMismatch {
            left: a.currency.code().to_string(),
            right: b.currency.code().to_string(),
        });
    }
    Ok(())
}

/// Builds a Money value from a decimal amount, e.g. `money(19.99, "USD")`.
pub fn money(major: f64, currency: &str) -> Result<Money> {
    let currency = Currency::from_code(currency)?;
    let scale = 10f64.powi(currency.decimals() as i32);
    Ok(Money {
        amount: round_half_even(major * scale)?,
        currency,
    })
}

/// Adds one or more Money values of the same currency.
pub fn add(first: Money, rest: &[Money]) -> Result<Money> {
    let mut amount = first.amount;
    for m in rest {
        assert_same_currency(&first, m)?;
        amount = amount.checked_add(m.amount).ok_or(Error::AmountOverflow)?;
    }
    Ok(Money {
        amount,
        currency: first.currency,
    })
}

/// Multiplies by a plain number (quantity, rate) and rounds half-even.
pub fn multiply(value: Money, factor: f64) -> Result<Money> {
    Ok(Money {
        amount: round_half_even(value.amount as f64 * factor)?,
        currency: value.currency,
    })
}

/// Splits Money into parts by weight without losing a cent. Leftover minor
/// units go to the earliest parts, one each. A negative amount is split
/// like its positive mirror.
pub fn allocate(value: Money, weights: &[f64]) -> Result<Vec<Money>> {
    if weights.is_empty() {
        return Err(Error::NoWeights);
    }
    if weights.iter().any(|&w| w < 0.0) {
        return Err(Error::NegativeWeight);
    }
    let total: f64 = weights.iter().sum();
    if total.is_nan() || total <= 0.0 {
        return Err(Error::ZeroTotalWeight);
    }
    let sign: i64 = if value.amount < 0 { -1 } else { 1 };
    let abs = value.amount.unsigned_abs() as f64;
    let mut parts: Vec<i64> = weights
        .iter()
        .map(|&w| ((abs * w) / total).floor() as i64)
        .collect();
    let mut remainder = value.amount.unsigned_abs() as i64 - parts.iter().sum::<i64>();
    let mut i = 0;
    while remainder > 0 {
        parts[i] += 1;
        remainder -= 1;
        i = (i + 1) % parts.len();
    }
    Ok(parts
        .into_iter()
        .map(|p| Money {
            amount: sign * p,
            currency: value.currency,
        })
        .collect())
}
