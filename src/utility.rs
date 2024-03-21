use crate::connector::ConnectorEnum;
use std::env;
use std::str::FromStr;
use strum::EnumProperty;
use strum::IntoEnumIterator;

const CONNECTOR_ENV: &str = "EAPP_CONNECTOR";
pub const API_KEY_ENV: &str = "API_KEY_ENV";

pub fn show_connectors() {
    let used_connector = get_connector();
    println!("Connectors:");
    for connector in ConnectorEnum::iter() {
        if connector == used_connector {
            print!("> ");
        } else {
            print!("  ");
        }
        println!("{}", format!("{connector}").to_lowercase());
    }
}

pub fn set_connector(connector: ConnectorEnum) {
    let connector = format!("{connector}");
    env::set_var(CONNECTOR_ENV, connector);
}

pub fn get_connector() -> ConnectorEnum {
    match env::var(CONNECTOR_ENV) {
        Ok(connector_string) => match ConnectorEnum::from_str(&connector_string) {
            Ok(connector) => connector,
            Err(_) => ConnectorEnum::Mock,
        },
        Err(_) => ConnectorEnum::Mock,
    }
}

pub fn set_apikey(apikey: &str, connector: ConnectorEnum) {
    if connector.get_str(API_KEY_ENV).unwrap() == "" {
        eprintln!("this connector not require api key");
    } else {
        env::set_var(connector.get_str(API_KEY_ENV).unwrap(), apikey);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_connector() {
        if env::var(CONNECTOR_ENV).is_err() {
            let connector = get_connector();
            assert_eq!(connector, ConnectorEnum::Mock);
        }
    }
}
