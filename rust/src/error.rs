use std::fmt;

/// Every way a ledger-kit function can fail. Each variant matches a place
/// where the JavaScript library throws, with the same message.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// JS `RangeError: Cannot round <value>`.
    CannotRound(f64),
    /// The rounded value does not fit in an `i64` of minor units.
    AmountOutOfRange(f64),
    /// Adding amounts overflowed an `i64` of minor units.
    AmountOverflow,
    /// JS `RangeError: Unsupported currency: <code>`.
    UnsupportedCurrency(String),
    /// JS `TypeError: Currency mismatch: <a> vs <b>`.
    CurrencyMismatch { left: String, right: String },
    /// JS `RangeError: allocate needs at least one weight`.
    NoWeights,
    /// JS `RangeError: Weights cannot be negative`.
    NegativeWeight,
    /// JS `RangeError: Weights must add up to more than zero`.
    ZeroTotalWeight,
    /// JS `RangeError: Tax rate cannot be negative: <rate>`.
    NegativeTaxRate(f64),
    /// JS `RangeError: Expected YYYY-MM-DD, got "<text>"`.
    InvalidDateFormat(String),
    /// JS `RangeError: Not a real calendar date: "<text>"`.
    NotARealDate(String),
    /// A computed date falls outside years 0000-9999 and cannot be written as YYYY-MM-DD.
    DateOutOfRange,
    /// JS `RangeError: Unknown payment terms: "<terms>"`.
    UnknownPaymentTerms(String),
    /// JS `RangeError: Payment days must be a whole number >= 0, got <days>`.
    InvalidPaymentDays(i64),
}

/// Formats a float the way JavaScript's `String(number)` does for the
/// values that show up in error messages.
fn js_number(value: f64) -> String {
    if value.is_nan() {
        "NaN".to_string()
    } else if value.is_infinite() {
        if value > 0.0 { "Infinity" } else { "-Infinity" }.to_string()
    } else {
        value.to_string()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CannotRound(v) => write!(f, "Cannot round {}", js_number(*v)),
            Error::AmountOutOfRange(v) => {
                write!(
                    f,
                    "Amount out of range for i64 minor units: {}",
                    js_number(*v)
                )
            }
            Error::AmountOverflow => write!(f, "Amount overflowed i64 minor units"),
            Error::UnsupportedCurrency(code) => write!(f, "Unsupported currency: {code}"),
            Error::CurrencyMismatch { left, right } => {
                write!(f, "Currency mismatch: {left} vs {right}")
            }
            Error::NoWeights => write!(f, "allocate needs at least one weight"),
            Error::NegativeWeight => write!(f, "Weights cannot be negative"),
            Error::ZeroTotalWeight => write!(f, "Weights must add up to more than zero"),
            Error::NegativeTaxRate(r) => {
                write!(f, "Tax rate cannot be negative: {}", js_number(*r))
            }
            Error::InvalidDateFormat(t) => write!(f, "Expected YYYY-MM-DD, got \"{t}\""),
            Error::NotARealDate(t) => write!(f, "Not a real calendar date: \"{t}\""),
            Error::DateOutOfRange => write!(f, "Date is outside years 0000-9999"),
            Error::UnknownPaymentTerms(t) => write!(f, "Unknown payment terms: \"{t}\""),
            Error::InvalidPaymentDays(d) => {
                write!(f, "Payment days must be a whole number >= 0, got {d}")
            }
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
