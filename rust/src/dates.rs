//! Dates are plain "YYYY-MM-DD" strings, read as UTC calendar days.

use crate::error::{Error, Result};

/// A UTC calendar day as a count of days since 1970-01-01.
type DayNumber = i64;

fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: i64, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ if is_leap_year(year) => 29,
        _ => 28,
    }
}

// Howard Hinnant's days_from_civil / civil_from_days (proleptic Gregorian).
fn days_from_civil(year: i64, month: u32, day: u32) -> DayNumber {
    let y = if month <= 2 { year - 1 } else { year };
    let era = y.div_euclid(400);
    let yoe = y.rem_euclid(400);
    let m = month as i64;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + day as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_from_days(days: DayNumber) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let year = yoe + era * 400 + if month <= 2 { 1 } else { 0 };
    (year, month, day)
}

fn parse_iso_date(text: &str) -> Result<(i64, u32, u32)> {
    let bytes = text.as_bytes();
    let well_formed = bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(i, b)| i == 4 || i == 7 || b.is_ascii_digit());
    if !well_formed {
        return Err(Error::InvalidDateFormat(text.to_string()));
    }
    let year: i64 = text[0..4].parse().expect("four ASCII digits");
    let month: u32 = text[5..7].parse().expect("two ASCII digits");
    let day: u32 = text[8..10].parse().expect("two ASCII digits");
    if !(1..=12).contains(&month) || day == 0 || day > days_in_month(year, month) {
        return Err(Error::NotARealDate(text.to_string()));
    }
    Ok((year, month, day))
}

fn to_iso_date(year: i64, month: u32, day: u32) -> Result<String> {
    if !(0..=9999).contains(&year) {
        return Err(Error::DateOutOfRange);
    }
    Ok(format!("{year:04}-{month:02}-{day:02}"))
}

/// 0 = Sunday ... 6 = Saturday, like JS `Date#getUTCDay`.
fn weekday(days: DayNumber) -> i64 {
    // 1970-01-01 was a Thursday.
    (days + 4).rem_euclid(7)
}

/// Adds calendar months. Negative values go back in time. If the target
/// month is shorter, the day is clamped to its last day:
/// 2026-01-31 + 1 month = 2026-02-28.
///
/// `months` is a whole number by type, so the JS "months must be a whole
/// number" error is a compile error here:
///
/// ```compile_fail
/// ledger_kit::add_months("2026-01-15", 1.5);
/// ```
pub fn add_months(date: &str, months: i32) -> Result<String> {
    let (year, month, day) = parse_iso_date(date)?;
    let index = year * 12 + (month as i64 - 1) + months as i64;
    let target_year = index.div_euclid(12);
    let target_month = (index.rem_euclid(12) + 1) as u32;
    let target_day = day.min(days_in_month(target_year, target_month));
    to_iso_date(target_year, target_month, target_day)
}

/// Payment terms: a number of days (`30`), a "net" string (`"net30"`,
/// `"NET 45"`), or `"due_on_receipt"`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaymentTerms {
    Days(i64),
    Text(String),
}

impl From<i64> for PaymentTerms {
    fn from(days: i64) -> Self {
        PaymentTerms::Days(days)
    }
}

impl From<i32> for PaymentTerms {
    fn from(days: i32) -> Self {
        PaymentTerms::Days(days as i64)
    }
}

impl From<&str> for PaymentTerms {
    fn from(text: &str) -> Self {
        PaymentTerms::Text(text.to_string())
    }
}

impl From<String> for PaymentTerms {
    fn from(text: String) -> Self {
        PaymentTerms::Text(text)
    }
}

/// Options for [`due_date`]. The default rolls to the next business day
/// and has no holidays.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DueDateOptions {
    /// YYYY-MM-DD dates to skip.
    pub holidays: Vec<String>,
    /// Move a weekend or holiday due date to the next business day.
    pub roll_to_business_day: bool,
}

impl Default for DueDateOptions {
    fn default() -> Self {
        DueDateOptions {
            holidays: Vec::new(),
            roll_to_business_day: true,
        }
    }
}

fn parse_net_terms(text: &str) -> Option<&str> {
    let lower = text.get(..3)?;
    if !lower.eq_ignore_ascii_case("net") {
        return None;
    }
    let digits = text[3..].trim_start();
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    Some(digits)
}

/// Works out when an invoice is due. A due date on a Saturday, Sunday or
/// holiday moves to the next business day, unless
/// `roll_to_business_day` is false.
pub fn due_date(
    issued: &str,
    terms: impl Into<PaymentTerms>,
    options: &DueDateOptions,
) -> Result<String> {
    let (year, month, day) = parse_iso_date(issued)?;
    let start = days_from_civil(year, month, day);

    let days = match terms.into() {
        PaymentTerms::Days(days) => days,
        PaymentTerms::Text(text) => {
            let trimmed = text.trim();
            if trimmed.to_lowercase() == "due_on_receipt" {
                0
            } else {
                let digits = parse_net_terms(trimmed)
                    .ok_or_else(|| Error::UnknownPaymentTerms(text.clone()))?;
                digits.parse::<i64>().map_err(|_| Error::DateOutOfRange)?
            }
        }
    };
    if days < 0 {
        return Err(Error::InvalidPaymentDays(days));
    }

    let mut due = start.checked_add(days).ok_or(Error::DateOutOfRange)?;
    // Days past 9999-12-31 can't be written as YYYY-MM-DD.
    if due > days_from_civil(9999, 12, 31) {
        return Err(Error::DateOutOfRange);
    }
    if options.roll_to_business_day {
        loop {
            let (y, m, d) = civil_from_days(due);
            let iso = to_iso_date(y, m, d)?;
            let wd = weekday(due);
            if wd != 0 && wd != 6 && !options.holidays.contains(&iso) {
                break;
            }
            due += 1;
        }
    }
    let (y, m, d) = civil_from_days(due);
    to_iso_date(y, m, d)
}
