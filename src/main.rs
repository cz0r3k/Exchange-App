mod connector;
mod connectors;
mod currency;
mod handlers;

use crate::connectors::mock_connector::MockConnector;
use clap::{Parser, Subcommand};
use handlers::*;

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
}

fn main() {
    let cli = Cli::parse();
    let connector = MockConnector::new();

    match &cli.command {
        Commands::Exchange(args) => {
            handle_exchange(args, &connector);
        }
        Commands::ListCurrencies => {
            handle_list_currencies(&connector);
        }
    }
}
