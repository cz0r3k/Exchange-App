use crate::connector::Connectors;
use std::env;
use std::str::FromStr;
use strum::EnumProperty;
use strum::IntoEnumIterator;

const CONNECTOR_ENV: &str = "EAPP_CONNECTOR";
const API_KEY_ENV: &str = "API_KEY_ENV";

pub fn show_connectors() {
    let used_connector = get_connector();
    println!("Connectors:");
    for connector in Connectors::iter() {
        if connector == used_connector {
            print!("> ");
        } else {
            print!("  ");
        }
        println!("{}", format!("{connector}").to_lowercase());
    }
}

pub fn set_connector(connector: Connectors) {
    let connector = format!("{connector}");
    env::set_var(CONNECTOR_ENV, connector);
}

pub fn get_connector() -> Connectors {
    match env::var(CONNECTOR_ENV) {
        Ok(connector_string) => match Connectors::from_str(&connector_string) {
            Ok(connector) => connector,
            Err(_) => Connectors::Mock,
        },
        Err(_) => Connectors::Mock,
    }
}

pub fn set_apikey(apikey: &str, connector: Connectors) {
    if connector.get_str(API_KEY_ENV).unwrap() == "" {
        eprintln!("this connector not require api key");
    } else {
        env::set_var(connector.get_str(API_KEY_ENV).unwrap(), apikey);
    }
}
