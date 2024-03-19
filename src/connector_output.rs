use crate::currency::Currency;
use bigdecimal::BigDecimal;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;

pub struct ExchangeOutput {
    value: BigDecimal,
    exchange_rate: BigDecimal,
}

impl ExchangeOutput {
    pub fn new(value: BigDecimal, exchange_rate: BigDecimal) -> Self {
        ExchangeOutput {
            value,
            exchange_rate,
        }
    }
}
impl fmt::Display for ExchangeOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, exchange rate:{}", self.value, self.exchange_rate)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct LatestOutput {
    currency: Currency,
    exchange_rate: BigDecimal,
}

impl LatestOutput {
    pub fn new(currency: Currency, exchange_rate: BigDecimal) -> Self {
        LatestOutput {
            currency,
            exchange_rate,
        }
    }
}
impl fmt::Display for LatestOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, exchange rate:{}", self.currency, self.exchange_rate)
    }
}

impl Ord for LatestOutput {
    fn cmp(&self, other: &Self) -> Ordering {
        self.currency.cmp(&other.currency)
    }
}

impl PartialOrd for LatestOutput {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
