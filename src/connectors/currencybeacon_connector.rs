use crate::connector::{Connector, ConnectorEnum, ConnectorError, ExchangeOutput, LatestOutput};
use crate::currency::Currency;
use crate::utility::API_KEY_ENV;
use bigdecimal::BigDecimal;
use error_stack::{Context, Report, Result, ResultExt};
use json::JsonValue;
use reqwest::StatusCode;
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
        let connector = ConnectorEnum::Currencybeacon;
        match env::var(connector.get_str(API_KEY_ENV).unwrap()) {
            Ok(val) => Ok(CurrencybeaconConnector {
                http_client: reqwest::blocking::Client::new(),
                api_key: val,
            }),
            Err(_) => Err(Report::new(ConnectorError::ApiKeyRequirements)
                .attach_printable("This connector require api key")),
        }
    }
    fn make_request(&self, url: &str) -> Result<JsonValue, ConnectorError> {
        let response = self.http_client.get(url).send();
        match response {
            Ok(response) => {
                match response.status() {
                    StatusCode::OK => {
                        let text = response.text().change_context(ConnectorError::ParseError)?;
                        return json::parse(&text)
                            .change_context(ConnectorError::JsonParsingError)
                            .attach_printable(format!("Error during json parsing:\n{text}"));
                    }
                    StatusCode::UNAUTHORIZED => Err(Report::new(ApiError::AuthorizationError)
                        .attach_printable("Authorization error")),
                    StatusCode::INTERNAL_SERVER_ERROR => {
                        Err(Report::new(ApiError::ServerError).attach_printable("Server error"))
                    }
                    StatusCode::TOO_MANY_REQUESTS => Err(Report::new(ApiError::TooManyRequests)
                        .attach_printable("Too many requests")),
                    status_code => Err(Report::new(ApiError::SomethingElse)
                        .attach_printable(format!("Status code : {status_code:?}"))),
                }
                .change_context(ConnectorError::ApiError)
            }
            Err(err) => Err(err)
                .change_context(ConnectorError::SendingError)
                .attach_printable("Error with sending")?,
        }
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
        let json = self.make_request(&url)?;
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
        let url = match target {
            Some(target) => {
                let target_str = target.join(",");
                format!(
                    "{BASE_URL}latest?api_key={}&base={base}&symbols={target_str}",
                    self.api_key
                )
            }
            None => {
                format!("{BASE_URL}latest?api_key={}&base={base}", self.api_key)
            }
        };
        let json = &self.make_request(&url)?;
        Ok(json["rates"]
            .entries()
            .map(|(k, v)| {
                LatestOutput::new(
                    Currency::new(k, None),
                    BigDecimal::from_str(&v.to_string()).unwrap(),
                )
            })
            .collect::<Vec<_>>())
    }
}

#[derive(Debug)]
pub enum ApiError {
    ServerError,
    TooManyRequests,
    AuthorizationError,
    SomethingElse,
}
impl fmt::Display for ApiError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Error with api")
    }
}

impl Context for ApiError {}
