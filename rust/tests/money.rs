//! Port of test/money.test.js.

use ledger_kit::{add, allocate, money, multiply, round_half_even, Currency, Error, Money};

fn amounts(parts: &[Money]) -> Vec<i64> {
    parts.iter().map(|p| p.amount).collect()
}

// describe('roundHalfEven')

#[test]
fn round_half_even_sends_exact_halves_to_the_even_neighbour() {
    assert_eq!(round_half_even(0.5).unwrap(), 0);
    assert_eq!(round_half_even(1.5).unwrap(), 2);
    assert_eq!(round_half_even(2.5).unwrap(), 2);
    assert_eq!(round_half_even(3.5).unwrap(), 4);
}

#[test]
fn round_half_even_rounds_everything_else_to_the_nearest_integer() {
    assert_eq!(round_half_even(1.2).unwrap(), 1);
    assert_eq!(round_half_even(1.7).unwrap(), 2);
    assert_eq!(round_half_even(-1.2).unwrap(), -1);
    assert_eq!(round_half_even(-3.5).unwrap(), -4);
}

#[test]
fn round_half_even_rejects_nan_and_infinity() {
    assert!(matches!(
        round_half_even(f64::NAN),
        Err(Error::CannotRound(_))
    ));
    let err = round_half_even(f64::INFINITY).unwrap_err();
    assert!(err.to_string().contains("Cannot round Infinity"));
}

// describe('money')

#[test]
fn money_stores_amounts_as_integer_minor_units() {
    assert_eq!(
        money(19.99, "USD").unwrap(),
        Money {
            amount: 1999,
            currency: Currency::Usd
        }
    );
    assert_eq!(
        money(500.0, "jpy").unwrap(),
        Money {
            amount: 500,
            currency: Currency::Jpy
        }
    );
    assert_eq!(
        money(-10.1, "EUR").unwrap(),
        Money {
            amount: -1010,
            currency: Currency::Eur
        }
    );
}

#[test]
fn money_rejects_unsupported_currencies() {
    assert!(matches!(
        money(1.0, "XYZ"),
        Err(Error::UnsupportedCurrency(_))
    ));
    let err = money(1.0, "BTC").unwrap_err();
    assert!(err.to_string().contains("Unsupported currency: BTC"));
}

// describe('add')

#[test]
fn add_adds_one_or_more_amounts_of_the_same_currency() {
    let a = money(10.0, "USD").unwrap();
    let b = money(2.5, "USD").unwrap();
    assert_eq!(
        add(a, &[b, b]).unwrap(),
        Money {
            amount: 1500,
            currency: Currency::Usd
        }
    );
    assert_eq!(
        add(a, &[money(-12.5, "USD").unwrap()]).unwrap(),
        Money {
            amount: -250,
            currency: Currency::Usd
        }
    );
}

#[test]
fn add_refuses_to_mix_currencies() {
    let err = add(money(10.0, "USD").unwrap(), &[money(1.0, "EUR").unwrap()]).unwrap_err();
    assert!(err.to_string().contains("Currency mismatch: USD vs EUR"));
}

// describe('multiply')

#[test]
fn multiply_multiplies_with_half_even_rounding() {
    assert_eq!(
        multiply(money(0.05, "USD").unwrap(), 0.5).unwrap(),
        Money {
            amount: 2,
            currency: Currency::Usd
        }
    );
    assert_eq!(
        multiply(money(0.07, "USD").unwrap(), 0.5).unwrap(),
        Money {
            amount: 4,
            currency: Currency::Usd
        }
    );
    assert_eq!(
        multiply(money(19.99, "USD").unwrap(), 3.0).unwrap(),
        Money {
            amount: 5997,
            currency: Currency::Usd
        }
    );
}

// describe('allocate')

#[test]
fn allocate_splits_without_losing_a_cent() {
    let one = money(1.0, "USD").unwrap();
    assert_eq!(
        amounts(&allocate(one, &[1.0, 1.0, 1.0]).unwrap()),
        [34, 33, 33]
    );
    let ten = money(10.0, "USD").unwrap();
    assert_eq!(amounts(&allocate(ten, &[70.0, 30.0]).unwrap()), [700, 300]);
}

#[test]
fn allocate_splits_a_refund_like_its_positive_mirror() {
    let refund = money(-1.0, "USD").unwrap();
    assert_eq!(
        amounts(&allocate(refund, &[1.0, 1.0, 1.0]).unwrap()),
        [-34, -33, -33]
    );
}

#[test]
fn allocate_rejects_bad_weights() {
    let one = money(1.0, "USD").unwrap();
    assert!(matches!(allocate(one, &[]), Err(Error::NoWeights)));
    let err = allocate(one, &[0.0, 0.0]).unwrap_err();
    assert!(err
        .to_string()
        .contains("Weights must add up to more than zero"));
    let err = allocate(one, &[2.0, -1.0]).unwrap_err();
    assert!(err.to_string().contains("Weights cannot be negative"));
}
