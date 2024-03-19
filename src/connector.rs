use crate::currency::Currency;
use bigdecimal::BigDecimal;
use error_stack::{Context, Result};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ConnectorError {
    InvalidInput(String),
}

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
        write!(f, "{} {}", self.value, self.exchange_rate)
    }
}

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
        write!(f, "{}, {}", self.currency, self.exchange_rate)
    }
}

impl fmt::Display for ConnectorError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Error with connector")
    }
}

impl Context for ConnectorError {}

pub trait Connector {
    fn exchange(
        &self,
        source: &str,
        target: &str,
        amount: &BigDecimal,
    ) -> Result<ExchangeOutput, ConnectorError>;
    fn list_currencies(&self) -> Result<Vec<Currency>, ConnectorError>;

    fn latest(
        &self,
        base: &str,
        target: Option<Vec<String>>,
    ) -> Result<Vec<LatestOutput>, ConnectorError>;
}
