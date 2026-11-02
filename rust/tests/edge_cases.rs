//! Edge cases from the README "Rules" section that the original JS tests
//! don't cover. test/edge-cases.test.js runs the same inputs through the JS.

use ledger_kit::{
    add, add_months, allocate, apply_tax, due_date, money, multiply, round_half_even, Currency,
    DueDateOptions, Error, Money, TaxBreakdown,
};

fn usd(amount: i64) -> Money {
    Money {
        amount,
        currency: Currency::Usd,
    }
}

fn m(major: f64) -> Money {
    money(major, "USD").unwrap()
}

fn amounts(parts: &[Money]) -> Vec<i64> {
    parts.iter().map(|p| p.amount).collect()
}

fn holidays(days: &[&str]) -> DueDateOptions {
    DueDateOptions {
        holidays: days.iter().map(|d| d.to_string()).collect(),
        ..Default::default()
    }
}

// describe('refunds mirror charges')

#[test]
fn builds_negative_money_like_the_positive_mirror() {
    assert_eq!(m(-19.99), usd(-1999));
    assert_eq!(
        money(-500.0, "JPY").unwrap(),
        Money {
            amount: -500,
            currency: Currency::Jpy
        }
    );
}

#[test]
fn adds_a_refund_back_to_zero() {
    assert_eq!(add(m(19.99), &[m(-19.99)]).unwrap(), usd(0));
}

#[test]
fn multiplies_a_refund_like_its_mirror() {
    assert_eq!(multiply(m(-19.99), 3.0).unwrap(), usd(-5997));
    assert_eq!(multiply(m(-0.07), 0.5).unwrap(), usd(-4));
}

#[test]
fn allocates_refunds_like_their_mirror() {
    assert_eq!(
        amounts(&allocate(m(-10.0), &[70.0, 30.0]).unwrap()),
        [-700, -300]
    );
    assert_eq!(
        amounts(&allocate(m(0.02), &[1.0, 1.0, 1.0]).unwrap()),
        [1, 1, 0]
    );
    assert_eq!(
        amounts(&allocate(m(-0.02), &[1.0, 1.0, 1.0]).unwrap()),
        [-1, -1, 0]
    );
    assert_eq!(amounts(&allocate(m(-0.05), &[1.0, 1.0]).unwrap()), [-3, -2]);
}

#[test]
fn taxes_a_refund_with_the_exact_negative_of_the_charge_tax() {
    assert_eq!(
        apply_tax(m(-100.0), 8.25).unwrap(),
        TaxBreakdown {
            net: usd(-10000),
            tax: usd(-825),
            gross: usd(-10825)
        }
    );
}

// describe('exact halves')

#[test]
fn rounds_positive_halves_to_even() {
    assert_eq!(round_half_even(4.5).unwrap(), 4);
    assert_eq!(round_half_even(5.5).unwrap(), 6);
    assert_eq!(round_half_even(-1.5).unwrap(), -2);
}

#[test]
fn rounds_half_cent_money_and_products_to_even() {
    assert_eq!(m(0.125).amount, 12);
    assert_eq!(m(0.005).amount, 0);
    assert_eq!(multiply(m(0.03), 0.5).unwrap().amount, 2);
}

// describe('month and year ends')

#[test]
fn adds_months_across_month_and_year_ends() {
    assert_eq!(add_months("2026-12-31", 1).unwrap(), "2027-01-31");
    assert_eq!(add_months("2027-01-31", -1).unwrap(), "2026-12-31");
    assert_eq!(add_months("2026-12-31", -12).unwrap(), "2025-12-31");
    assert_eq!(add_months("2026-01-01", -1).unwrap(), "2025-12-01");
    assert_eq!(add_months("2026-03-31", 1).unwrap(), "2026-04-30");
    assert_eq!(add_months("2026-10-31", -8).unwrap(), "2026-02-28");
}

#[test]
fn rolls_due_dates_across_year_ends_and_holidays() {
    let defaults = DueDateOptions::default();
    assert_eq!(
        due_date("2026-12-31", "net1", &defaults).unwrap(),
        "2027-01-01"
    );
    assert_eq!(
        due_date("2026-12-31", "net1", &holidays(&["2027-01-01"])).unwrap(),
        "2027-01-04"
    );
    assert_eq!(
        due_date("2026-12-24", 1, &holidays(&["2026-12-25"])).unwrap(),
        "2026-12-28"
    );
    assert_eq!(due_date("2026-02-27", 1, &defaults).unwrap(), "2026-03-02");
}

// describe('leap years')

#[test]
fn clamps_to_feb_29_only_in_leap_years() {
    assert_eq!(add_months("2024-01-31", 1).unwrap(), "2024-02-29");
    assert_eq!(add_months("2000-01-31", 1).unwrap(), "2000-02-29");
    assert_eq!(add_months("2100-01-31", 1).unwrap(), "2100-02-28");
    assert_eq!(add_months("2024-02-29", 12).unwrap(), "2025-02-28");
    assert_eq!(add_months("2024-02-29", -12).unwrap(), "2023-02-28");
    assert_eq!(add_months("2024-02-29", 48).unwrap(), "2028-02-29");
    assert_eq!(add_months("2024-02-29", 0).unwrap(), "2024-02-29");
    assert_eq!(
        due_date("2028-02-28", 1, &DueDateOptions::default()).unwrap(),
        "2028-02-29"
    );
}

#[test]
fn rejects_feb_29_outside_leap_years() {
    assert!(matches!(
        add_months("2025-02-29", 1),
        Err(Error::NotARealDate(_))
    ));
    assert!(matches!(
        add_months("2100-02-29", 1),
        Err(Error::NotARealDate(_))
    ));
}

// describe('date format')

#[test]
fn throws_on_anything_that_is_not_a_real_yyyy_mm_dd_day() {
    assert!(matches!(
        add_months("2026-04-31", 1),
        Err(Error::NotARealDate(_))
    ));
    assert!(matches!(
        add_months("2026-13-01", 1),
        Err(Error::NotARealDate(_))
    ));
    assert!(matches!(
        add_months("2026-00-10", 1),
        Err(Error::NotARealDate(_))
    ));
    assert!(matches!(
        add_months("2026-1-05", 1),
        Err(Error::InvalidDateFormat(_))
    ));
    assert!(matches!(
        add_months("2026-01-05T00:00:00Z", 1),
        Err(Error::InvalidDateFormat(_))
    ));
    assert!(matches!(
        due_date(" 2026-01-05", 30, &DueDateOptions::default()),
        Err(Error::InvalidDateFormat(_))
    ));
}
