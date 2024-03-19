#![feature(iterator_try_collect)]

mod connector;
mod connector_output;
mod connectors;
mod currency;
mod handlers;

use crate::connectors::mock_connector::MockConnector;
use clap::{Parser, Subcommand};
use handlers::{handle_exchange, handle_latest, handle_list_currencies, ExchangeArgs, LatestArgs};

#[derive(Parser)]
#[command( about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Exchange currency to another
    Exchange(ExchangeArgs),
    /// List all available currencies
    ListCurrencies,
    /// List currencies with exchange rate
    Latest(LatestArgs),
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    let connector = MockConnector::new();

    match &cli.command {
        Commands::Exchange(args) => match handle_exchange(args, &connector) {
            Ok(value) => println!("{value}"),
            Err(err) => log::error!("\n{err:?}"),
        },
        Commands::ListCurrencies => match handle_list_currencies(&connector) {
            Ok(currencies) => {
                for i in &currencies {
                    println!("{i}");
                }
            }
            Err(err) => log::error!("\n{err:?}"),
        },
        Commands::Latest(args) => match handle_latest(args, &connector) {
            Ok(currencies) => {
                for i in &currencies {
                    println!("{i}");
                }
            }
            Err(err) => log::error!("\n{err:?}"),
        },
    }
}
