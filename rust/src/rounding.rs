use crate::error::{Error, Result};

/// Rounds to the nearest integer, sending exact halves to the nearest even
/// integer ("banker's rounding"). Used for all minor-unit math so that
/// totals don't drift over many line items.
pub fn round_half_even(value: f64) -> Result<i64> {
    if !value.is_finite() {
        return Err(Error::CannotRound(value));
    }
    let floor = value.floor();
    let diff = value - floor;
    // Float math can leave 0.5 as 0.4999999999 (e.g. 1010 * 0.05).
    const EPSILON: f64 = 1e-9;
    let rounded = if (diff - 0.5).abs() < EPSILON {
        // floor % 2.0 is -1.0 for negative odd floors, so compare against 0.
        if floor % 2.0 != 0.0 {
            floor + 1.0
        } else {
            floor
        }
    } else {
        value.round()
    };
    to_i64(rounded)
}

fn to_i64(value: f64) -> Result<i64> {
    // i64::MAX as f64 rounds up to 2^63, so the upper bound is exclusive.
    if value >= -(2f64.powi(63)) && value < 2f64.powi(63) {
        Ok(value as i64)
    } else {
        Err(Error::AmountOutOfRange(value))
    }
}
