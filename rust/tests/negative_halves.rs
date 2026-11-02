//! Regression tests: refunds must round like the mirror of the charge
//! (README "Rules"). round_half_even(-2.5) used to return -3 instead of -2.
//! test/negative-halves.test.js runs the same inputs through the JS.

use ledger_kit::{apply_tax, money, multiply, round_half_even, Currency, Money, TaxBreakdown};

fn usd(amount: i64) -> Money {
    Money {
        amount,
        currency: Currency::Usd,
    }
}

#[test]
fn rounds_negative_halves_to_the_even_neighbour() {
    assert_eq!(round_half_even(-2.5).unwrap(), -2);
    assert_eq!(round_half_even(-0.5).unwrap(), 0);
    assert_eq!(round_half_even(-4.5).unwrap(), -4);
}

#[test]
fn builds_half_cent_refunds_like_the_mirror_of_the_charge() {
    assert_eq!(money(-0.125, "USD").unwrap().amount, -12);
    assert_eq!(money(-0.005, "USD").unwrap().amount, 0);
}

#[test]
fn multiplies_a_refund_on_a_half_like_its_mirror() {
    assert_eq!(
        multiply(money(-0.05, "USD").unwrap(), 0.5).unwrap(),
        usd(-2)
    );
}

#[test]
fn taxes_a_refund_with_the_exact_negative_of_the_charge_tax() {
    // 10.10 * 5% = 0.505 -> 0.50, so -10.10 * 5% must be -0.50.
    assert_eq!(
        apply_tax(money(-10.1, "USD").unwrap(), 5.0).unwrap(),
        TaxBreakdown {
            net: usd(-1010),
            tax: usd(-50),
            gross: usd(-1060)
        }
    );
}
