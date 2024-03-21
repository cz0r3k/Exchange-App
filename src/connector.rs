pub use crate::connector_output::{ExchangeOutput, LatestOutput};
use crate::connectors::currencybeacon_connector::CurrencybeaconConnector;
use crate::connectors::mock_connector::MockConnector;
use crate::currency::Currency;
use bigdecimal::BigDecimal;
use clap::ValueEnum;
use error_stack::{Context, Result};
use std::fmt;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    ValueEnum,
    EnumIter,
    Display,
    strum_macros::EnumProperty,
    EnumString,
)]
pub enum Connectors {
    #[strum(props(API_KEY_ENV = ""))]
    Mock,
    #[strum(props(API_KEY_ENV = "EAPP_CURRENCYBEACON"))]
    Currencybeacon,
}

#[allow(clippy::module_name_repetitions)]
pub fn create_connector(connector: Connectors) -> Result<Box<dyn Connector>, ConnectorError> {
    match connector {
        Connectors::Currencybeacon => Ok(Box::new(CurrencybeaconConnector::new()?)),
        Connectors::Mock => Ok(Box::new(MockConnector::new())),
    }
}

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum ConnectorError {
    InvalidInput(String),
    ApiKeyRequirements,
    ApiError,
    SendingError,
    JsonParsingError,
    ParseError,
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
