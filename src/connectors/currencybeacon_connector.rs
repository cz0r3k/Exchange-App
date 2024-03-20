use crate::connector::{Connector, ConnectorError, Connectors, ExchangeOutput, LatestOutput};
use crate::currency::Currency;
use bigdecimal::BigDecimal;
use error_stack::{Context, Report, Result, ResultExt};
use json::JsonValue;
use std::str::FromStr;
use std::{env, fmt};
use strum::EnumProperty;

const BASE_URL: &str = "https://api.currencybeacon.com/v1/";
pub struct CurrencybeaconConnector {
    http_client: reqwest::blocking::Client,
    api_key: String,
}

impl CurrencybeaconConnector {
    pub fn new() -> Result<Self, ConnectorError> {
        let connector = Connectors::Currencybeacon;
        match env::var(connector.get_str("API_KEY_ENV").unwrap()) {
            Ok(val) => Ok(CurrencybeaconConnector {
                http_client: reqwest::blocking::Client::new(),
                api_key: val,
            }),
            Err(_) => Err(Report::new(ConnectorError::ApiKeyRequirements)),
        }
    }
    fn make_request(&self, url: &str) -> Result<JsonValue, ApiError> {
        json::parse(&self.http_client.get(url).send().unwrap().text().unwrap())
            .change_context(ApiError)
    }
}

impl Connector for CurrencybeaconConnector {
    fn exchange(
        &self,
        source: &str,
        target: &str,
        amount: &BigDecimal,
    ) -> Result<ExchangeOutput, ConnectorError> {
        let currency = self
            .latest(source, Some(vec![target.to_string()]))?
            .first()
            .ok_or_else(|| Report::new(ConnectorError::ApiError))?
            .clone();
        let rate = currency.get_rate();
        Ok(ExchangeOutput::new(&(rate * amount), rate))
    }

    fn list_currencies(&self) -> Result<Vec<Currency>, ConnectorError> {
        let url = format!("{BASE_URL}currencies?api_key={}", self.api_key);
        let json = self
            .make_request(&url)
            .change_context(ConnectorError::ApiError)?;

        Ok(json["response"]
            .members()
            .map(|v| Currency::new(&v["short_code"].to_string(), Some(v["name"].to_string())))
            .collect::<Vec<Currency>>())
    }
    fn latest(
        &self,
        base: &str,
        target: Option<Vec<String>>,
    ) -> Result<Vec<LatestOutput>, ConnectorError> {
        let url = if target.is_some() {
            let target_str = target.unwrap().join(",");
            format!(
                "{BASE_URL}latest?api_key={}&base={base}&symbols={target_str}",
                self.api_key
            )
        } else {
            format!("{BASE_URL}latest?api_key={}&base={base}", self.api_key)
        };
        let json = &self
            .make_request(&url)
            .change_context(ConnectorError::ApiError)?;
        Ok(json["rates"]
            .entries()
            .map(|(k, v)| {
                LatestOutput::new(
                    Currency::new(k, None),
                    BigDecimal::from_str(&v.to_string()).unwrap(),
                )
            })
            .collect::<Vec<LatestOutput>>())
    }
}

#[derive(Debug)]
pub struct ApiError;
impl fmt::Display for ApiError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Error with api")
    }
}

impl Context for ApiError {}
