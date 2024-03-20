#![allow(clippy::borrowed_box)]
use crate::connector::{Connector, ConnectorError, ExchangeOutput, LatestOutput};
use crate::currency::Currency;
use bigdecimal::BigDecimal;
use clap::Args;
use error_stack::Result;

#[derive(Args, Debug)]
pub struct ExchangeArgs {
    /// Source currency code
    #[arg(short, long)]
    source: String,
    /// Target currency code
    #[arg(short, long)]
    target: String,
    /// Amount to be converted
    #[arg(value_parser = clap::value_parser!(BigDecimal))]
    amount: BigDecimal,
}

#[derive(Args, Debug)]
pub struct LatestArgs {
    /// Base currency code
    #[arg(short, long)]
    base: String,
    /// Target currency code
    #[arg(short, long)]
    target: Option<Vec<String>>,
}
pub fn handle_exchange(
    args: &ExchangeArgs,
    connector: &Box<dyn Connector>,
) -> Result<ExchangeOutput, ConnectorError> {
    connector.exchange(&args.source, &args.target, &args.amount)
}

pub fn handle_list_currencies(
    connector: &Box<dyn Connector>,
) -> Result<Vec<Currency>, ConnectorError> {
    connector.list_currencies()
}

pub fn handle_latest(
    args: &LatestArgs,
    connector: &Box<dyn Connector>,
) -> Result<Vec<LatestOutput>, ConnectorError> {
    connector.latest(&args.base, args.target.clone())
}
