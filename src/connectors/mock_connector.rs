use crate::connector::{Connector, ConnectorError, ExchangeOutput, LatestOutput};
use crate::currency::Currency;
use bigdecimal::BigDecimal;
use error_stack::{Report, Result};
use std::collections::HashMap;
use std::str::FromStr;

pub struct MockConnector {
    currencies: HashMap<String, Currency>,
    rates: HashMap<String, BigDecimal>,
}

impl MockConnector {
    pub fn new() -> Self {
        let currencies = HashMap::from([
            (
                "GBP".to_string(),
                Currency::new("Pound Sterling".to_string(), "GBP".to_string()),
            ),
            (
                "PLN".to_string(),
                Currency::new("Zloty".to_string(), "PLN".to_string()),
            ),
            (
                "USD".to_string(),
                Currency::new("US Dollar".to_string(), "USD".to_string()),
            ),
        ]);
        let rates = HashMap::from([
            ("GBP".to_string(), BigDecimal::from_str("0.8").unwrap()),
            ("USD".to_string(), BigDecimal::from_str("1").unwrap()),
            ("PLN".to_string(), BigDecimal::from_str("4").unwrap()),
        ]);
        MockConnector { currencies, rates }
    }
    fn rate(&self, source: &str, target: &str) -> Result<BigDecimal, ConnectorError> {
        let source_rate = self.rates.get(source).ok_or_else(|| {
            let msg = format!("Source currency code {} not exist", source);
            Report::new(ConnectorError::InvalidInput(msg.clone())).attach_printable(msg)
        })?;
        let target_rate = self.rates.get(target).ok_or_else(|| {
            let msg = format!("Target currency code {} not exist", target);
            Report::new(ConnectorError::InvalidInput(msg.clone())).attach_printable(msg)
        })?;
        Ok(target_rate / source_rate)
    }
    fn get_currency(&self, currency_str: &str) -> Result<Currency, ConnectorError> {
        let currency = self.currencies.get(currency_str);
        if let Some(currency) = currency {
            Ok(currency.clone())
        } else {
            let msg = format!("Currency code {} not exist", currency_str);
            Err(Report::new(ConnectorError::InvalidInput(msg.clone())).attach_printable(msg))
        }
    }
}

impl Connector for MockConnector {
    fn exchange(
        &self,
        source: &str,
        target: &str,
        amount: &BigDecimal,
    ) -> Result<ExchangeOutput, ConnectorError> {
        let exchange_rate = self.rate(source, target)?;
        let value = amount * exchange_rate.clone();
        Ok(ExchangeOutput::new(value, exchange_rate))
    }

    fn list_currencies(&self) -> Result<Vec<Currency>, ConnectorError> {
        Ok(self.currencies.values().cloned().collect())
    }

    fn latest(
        &self,
        base: &str,
        target: Option<Vec<String>>,
    ) -> Result<Vec<LatestOutput>, ConnectorError> {
        if !self.currencies.contains_key(base) {
            let msg = format!("Currency code {} not exist", base);
            return Err(
                Report::new(ConnectorError::InvalidInput(msg.clone())).attach_printable(msg)
            );
        }
        Ok(if target.is_some() {
            target
                .unwrap()
                .iter()
                .map(|c| self.get_currency(c))
                .try_collect::<Vec<_>>()?
        } else {
            self.list_currencies().unwrap()
        }
        .iter()
        .map(|c| LatestOutput::new(c.clone(), self.rate(base, c.get_short_code()).unwrap()))
        .collect::<Vec<_>>())
    }
}
