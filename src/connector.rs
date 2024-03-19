use crate::currency::Currency;
use bigdecimal::BigDecimal;

pub trait Connector {
    fn exchange(
        &self,
        source: &str,
        target: &str,
        amount: &BigDecimal,
    ) -> Result<BigDecimal, String>;
    fn list_currencies(&self) -> Result<Vec<Currency>, String>;
}
