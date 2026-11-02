//! Port of test/tax.test.js.

use ledger_kit::{apply_tax, money, Currency, Error, Money, TaxBreakdown};

fn usd(amount: i64) -> Money {
    Money {
        amount,
        currency: Currency::Usd,
    }
}

#[test]
fn apply_tax_splits_net_tax_and_gross() {
    assert_eq!(
        apply_tax(money(100.0, "USD").unwrap(), 8.25).unwrap(),
        TaxBreakdown {
            net: usd(10000),
            tax: usd(825),
            gross: usd(10825)
        }
    );
}

#[test]
fn apply_tax_rounds_tax_half_even() {
    // 10.10 * 5% = 0.505 -> 0.50
    assert_eq!(
        apply_tax(money(10.1, "USD").unwrap(), 5.0)
            .unwrap()
            .tax
            .amount,
        50
    );
}

#[test]
fn apply_tax_rejects_negative_rates() {
    assert!(matches!(
        apply_tax(money(1.0, "USD").unwrap(), -1.0),
        Err(Error::NegativeTaxRate(_))
    ));
}
