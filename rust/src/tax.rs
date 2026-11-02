use crate::error::{Error, Result};
use crate::money::{add, multiply, Money};

/// The result of [`apply_tax`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaxBreakdown {
    pub net: Money,
    pub tax: Money,
    pub gross: Money,
}

/// Applies a percentage tax to a net amount. `rate_percent` is e.g. 8.25 for 8.25%.
pub fn apply_tax(net: Money, rate_percent: f64) -> Result<TaxBreakdown> {
    if rate_percent < 0.0 {
        return Err(Error::NegativeTaxRate(rate_percent));
    }
    let tax = multiply(net, rate_percent / 100.0)?;
    Ok(TaxBreakdown {
        net,
        tax,
        gross: add(net, &[tax])?,
    })
}
