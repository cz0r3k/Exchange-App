use crate::connector::Connectors;
use globalenv::set_var;
use std::env;
use std::str::FromStr;
use strum::EnumProperty;
use strum::IntoEnumIterator;

const CONNECTOR_ENV: &str = "EAPP_CONNECTOR";

pub fn show_connectors() {
    for connector in Connectors::iter() {
        println!("{}", format!("{connector}").to_lowercase());
    }
}

pub fn set_connector(connector: Connectors) {
    let _ = set_var(CONNECTOR_ENV, &format!("{connector}"));
}

pub fn get_connector() -> Connectors {
    match env::var("EAPP_CONNECTOR") {
        Ok(connector_string) => match Connectors::from_str(&connector_string) {
            Ok(connector) => connector,
            Err(_) => Connectors::Mock,
        },
        Err(_) => Connectors::Mock,
    }
}

pub fn set_apikey(apikey: &str, connector: Connectors) {
    if connector.get_str("API_KEY_ENV").unwrap() == "" {
        eprintln!("set connector before setting api key");
    } else {
        let _ = set_var(connector.get_str("API_KEY_ENV").unwrap(), apikey);
    }
}
