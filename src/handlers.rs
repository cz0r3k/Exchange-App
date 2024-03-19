use crate::connector::Connector;
use bigdecimal::BigDecimal;
use clap::Args;

#[derive(Args, Debug)]
pub struct ExchangeArgs {
    /// Source currency
    #[arg(short, long)]
    source: String,
    /// Target currency
    #[arg(short, long)]
    target: String,
    /// Amount to be converted
    #[arg(value_parser = clap::value_parser!(BigDecimal))]
    amount: BigDecimal,
}
pub fn handle_exchange(args: &ExchangeArgs, connector: &impl Connector) {
    let value = connector
        .exchange(&args.source, &args.target, &args.amount)
        .unwrap();
    println!("{}", value);
}

pub fn handle_list_currencies(connector: &impl Connector) {
    let currencies = connector.list_currencies().unwrap();
    currencies.iter().for_each(|i| println!("{}", i));
}
