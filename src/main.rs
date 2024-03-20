#![feature(iterator_try_collect)]
mod connector;
mod connector_output;
mod connectors;
mod currency;
mod handlers;
mod utility;

use crate::connector::{create_connector, Connectors};
use crate::utility::{get_connector, set_apikey, set_connector, show_connectors};
use clap::{Parser, Subcommand};
use handlers::{handle_exchange, handle_latest, handle_list_currencies, ExchangeArgs, LatestArgs};

#[derive(Parser)]
#[command( about, long_about = None)]
struct Cli {
    #[arg(long, exclusive = true)]
    show_connectors: bool,

    #[arg(long)]
    set_connector: Option<Connectors>,

    #[arg(long)]
    set_apikey: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
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

    if cli.show_connectors {
        show_connectors();
    }
    if cli.set_connector.is_some() {
        set_connector(cli.set_connector.unwrap());
    }
    let enum_connector = get_connector();
    if cli.set_apikey.is_some() {
        set_apikey(&cli.set_apikey.unwrap(), enum_connector);
    }

    let connector = match create_connector(enum_connector) {
        Ok(connector) => connector,
        Err(err) => {
            log::error!("\n{err:?}");
            std::process::exit(1);
        }
    };

    match &cli.command {
        Some(Commands::Exchange(args)) => match handle_exchange(args, &connector) {
            Ok(value) => println!("{value}"),
            Err(err) => log::error!("\n{err:?}"),
        },
        Some(Commands::ListCurrencies) => match handle_list_currencies(&connector) {
            Ok(currencies) => {
                for i in &currencies {
                    println!("{i}");
                }
            }
            Err(err) => log::error!("\n{err:?}"),
        },
        Some(Commands::Latest(args)) => match handle_latest(args, &connector) {
            Ok(currencies) => {
                for i in &currencies {
                    println!("{i}");
                }
            }
            Err(err) => log::error!("\n{err:?}"),
        },
        None => {}
    }
}
