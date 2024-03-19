pub use crate::connector_output::{ExchangeOutput, LatestOutput};
use crate::currency::Currency;
use bigdecimal::BigDecimal;
use error_stack::{Context, Result};
use std::fmt;

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum ConnectorError {
    InvalidInput(String),
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
