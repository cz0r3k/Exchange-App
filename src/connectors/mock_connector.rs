use crate::connector::Connector;
use crate::currency::Currency;
use bigdecimal::BigDecimal;
use std::ops::Mul;
use std::str::FromStr;

pub struct MockConnector {
    currencies: Vec<Currency>,
}

impl MockConnector {
    pub fn new() -> Self {
        let currencies = vec![
            Currency::new("Pound Sterling".to_string(), "GBP".to_string()),
            Currency::new("Zloty".to_string(), "PLN".to_string()),
            Currency::new("US Dollar".to_string(), "USD".to_string()),
        ];
        MockConnector { currencies }
    }
}

impl Connector for MockConnector {
    fn exchange(
        &self,
        source: &str,
        target: &str,
        amount: &BigDecimal,
    ) -> Result<BigDecimal, String> {
        match source {
            "GBP" => match target {
                "GBP" => Ok(amount.clone()),
                "PLN" => Ok(amount.clone() * 5),
                "USD" => Ok(amount.mul(BigDecimal::from_str("1.25").unwrap())),
                _ => todo!(),
            },
            "PLN" => match target {
                "GBP" => Ok(amount.mul(BigDecimal::from_str("0.2").unwrap())),
                "PLN" => Ok(amount.clone()),
                "USD" => Ok(amount.mul(BigDecimal::from_str("0.25").unwrap())),
                _ => todo!(),
            },
            "USD" => match target {
                "GBP" => Ok(amount.mul(BigDecimal::from_str("0.8").unwrap())),
                "PLN" => Ok(amount.clone() * 4),
                "USD" => Ok(amount.clone()),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    fn list_currencies(&self) -> Result<Vec<Currency>, String> {
        Ok(self.currencies.clone())
    }
}
