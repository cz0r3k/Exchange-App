use crate::connector::{Connector, ConnectorError};
use crate::connector_output::{ExchangeOutput, LatestOutput};
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
                Currency::new("GBP", Some("Pound Sterling".to_string())),
            ),
            (
                "PLN".to_string(),
                Currency::new("PLN", Some("Zloty".to_string())),
            ),
            (
                "USD".to_string(),
                Currency::new("USD", Some("US Dollar".to_string())),
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
            let msg = format!("Source currency code {source} not exist");
            Report::new(ConnectorError::InvalidInput(msg.clone())).attach_printable(msg)
        })?;
        let target_rate = self.rates.get(target).ok_or_else(|| {
            let msg = format!("Target currency code {target} not exist");
            Report::new(ConnectorError::InvalidInput(msg.clone())).attach_printable(msg)
        })?;
        Ok(target_rate / source_rate)
    }
    fn get_currency(&self, currency_str: &str) -> Result<Currency, ConnectorError> {
        let currency = self.currencies.get(currency_str);
        if let Some(currency) = currency {
            Ok(currency.clone())
        } else {
            let msg = format!("Currency code {currency_str} not exist");
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
        let exchange_rate = &self.rate(source, target)?;
        Ok(ExchangeOutput::new(
            &(amount * exchange_rate),
            exchange_rate,
        ))
    }

    fn list_currencies(&self) -> Result<Vec<Currency>, ConnectorError> {
        let mut currencies = self.currencies.values().cloned().collect::<Vec<_>>();
        currencies.sort_unstable();
        Ok(currencies)
    }

    fn latest(
        &self,
        base: &str,
        target: Option<Vec<String>>,
    ) -> Result<Vec<LatestOutput>, ConnectorError> {
        if !self.currencies.contains_key(base) {
            let msg = format!("Currency code {base} not exist");
            return Err(
                Report::new(ConnectorError::InvalidInput(msg.clone())).attach_printable(msg)
            );
        }
        let mut currencies = match target {
            Some(target) => target
                .iter()
                .map(|c| self.get_currency(c))
                .try_collect::<Vec<_>>()?,
            None => self.list_currencies().unwrap(),
        }
        .iter()
        .map(|c| LatestOutput::new(c.clone(), self.rate(base, c.get_short_code()).unwrap()))
        .collect::<Vec<_>>();
        currencies.sort_unstable();
        Ok(currencies)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;
    #[test]
    fn latest_base_not_exist() {
        let connector = MockConnector::new();
        if let Err(e) = connector.latest("PLM", Some(vec!["PLN".to_string()])) {
            if let ConnectorError::InvalidInput(msg) = e.current_context() {
                assert_eq!("Currency code PLM not exist", msg);
            }
        }
    }
    #[test]
    fn exchange_source_not_exist() {
        let connector = MockConnector::new();
        if let Err(e) = connector.exchange("PLM", "PLN", &BigDecimal::from(1)) {
            if let ConnectorError::InvalidInput(msg) = e.current_context() {
                assert_eq!("Source currency code PLM not exist", msg);
            }
        }
    }
    #[test]
    fn exchange_target_not_exist() {
        let connector = MockConnector::new();
        if let Err(e) = connector.exchange("PLN", "PLM", &BigDecimal::from(1)) {
            if let ConnectorError::InvalidInput(msg) = e.current_context() {
                assert_eq!("Target currency code PLM not exist", msg);
            }
        }
    }

    #[test]
    fn exchange_same_currency() {
        let connector = MockConnector::new();
        assert_eq!(
            ExchangeOutput::new(&BigDecimal::from(1), &BigDecimal::from(1)),
            connector
                .exchange("PLN", "PLN", &BigDecimal::from(1))
                .unwrap()
        );
    }

    #[test]
    fn exchange() {
        let connector = MockConnector::new();
        assert_eq!(
            ExchangeOutput::new(&BigDecimal::from(40), &BigDecimal::from(4)),
            connector
                .exchange("USD", "PLN", &BigDecimal::from(10))
                .unwrap()
        );
    }

    #[test]
    fn latest() {
        let connector = MockConnector::new();
        let currency = Currency::new("PLN", Some("Zloty".to_string()));
        assert_eq!(
            vec![LatestOutput::new(currency, BigDecimal::from(4))],
            connector
                .latest("USD", Some(vec!["PLN".to_string()]))
                .unwrap()
        );
    }
}
