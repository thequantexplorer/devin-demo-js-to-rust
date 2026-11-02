//! Port of test/dates.test.js.

use ledger_kit::{add_months, due_date, DueDateOptions, Error};

// describe('addMonths')

#[test]
fn add_months_clamps_to_the_end_of_shorter_months() {
    assert_eq!(add_months("2026-01-31", 1).unwrap(), "2026-02-28");
    assert_eq!(add_months("2028-01-31", 1).unwrap(), "2028-02-29");
    assert_eq!(add_months("2026-05-31", 1).unwrap(), "2026-06-30");
}

#[test]
fn add_months_crosses_year_ends_in_both_directions() {
    assert_eq!(add_months("2026-11-15", 3).unwrap(), "2027-02-15");
    assert_eq!(add_months("2026-03-31", -1).unwrap(), "2026-02-28");
    assert_eq!(add_months("2026-01-15", -1).unwrap(), "2025-12-15");
}

#[test]
fn add_months_only_accepts_real_yyyy_mm_dd_dates() {
    let err = add_months("2026-02-30", 1).unwrap_err();
    assert!(err.to_string().contains("Not a real calendar date"));
    assert!(matches!(
        add_months("02/28/2026", 1),
        Err(Error::InvalidDateFormat(_))
    ));
    // addMonths('2026-01-15', 1.5) can't be written in Rust: `months` is an
    // i32, so a fractional month is a compile error. See the `compile_fail`
    // doctest on `add_months`, which `cargo test` runs.
}

// describe('dueDate')

#[test]
fn due_date_adds_a_number_of_days_and_rolls_off_weekends() {
    let defaults = DueDateOptions::default();
    assert_eq!(due_date("2026-01-15", 30, &defaults).unwrap(), "2026-02-16"); // Feb 14 is a Saturday
    let no_roll = DueDateOptions {
        roll_to_business_day: false,
        ..Default::default()
    };
    assert_eq!(due_date("2026-01-15", 30, &no_roll).unwrap(), "2026-02-14");
}

#[test]
fn due_date_reads_net_terms() {
    let defaults = DueDateOptions::default();
    assert_eq!(
        due_date("2026-03-02", "net30", &defaults).unwrap(),
        "2026-04-01"
    );
    assert_eq!(
        due_date("2026-03-02", "NET 45", &defaults).unwrap(),
        "2026-04-16"
    );
}

#[test]
fn due_date_is_due_the_same_day_for_due_on_receipt() {
    let defaults = DueDateOptions::default();
    assert_eq!(
        due_date("2026-03-10", "due_on_receipt", &defaults).unwrap(),
        "2026-03-10"
    );
}

#[test]
fn due_date_skips_holidays_when_rolling() {
    let options = DueDateOptions {
        holidays: vec!["2026-11-26".into()],
        ..Default::default()
    };
    assert_eq!(due_date("2026-11-26", 0, &options).unwrap(), "2026-11-27");
}

#[test]
fn due_date_rejects_unknown_or_negative_terms() {
    let defaults = DueDateOptions::default();
    let err = due_date("2026-01-15", "end of month", &defaults).unwrap_err();
    assert!(err.to_string().contains("Unknown payment terms"));
    assert!(matches!(
        due_date("2026-01-15", -5, &defaults),
        Err(Error::InvalidPaymentDays(-5))
    ));
}
