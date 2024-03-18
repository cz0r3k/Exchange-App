use bigdecimal::BigDecimal;
use clap::{Args, Parser, Subcommand};

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

#[derive(Args, Debug)]
struct ExchangeArgs {
    /// source currency
    #[arg(short, long)]
    source: String,
    /// target currency
    #[arg(short, long)]
    target: String,
    /// Amount to be converted
    #[arg(value_parser = clap::value_parser!(BigDecimal))]
    amount: BigDecimal,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Exchange(args) => {
            println!(" {args:?}")
        }
        Commands::ListCurrencies => {
            println!("List");
        }
    }
}
