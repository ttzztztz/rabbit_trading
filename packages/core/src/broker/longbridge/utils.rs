use anyhow::{anyhow, Error};
use longbridge::{Config, QuoteContext, TradeContext};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedReceiver;

use super::broker::LongBridgeBroker;
use crate::model::trading::{currency::Currency, market::Market, symbol::Symbol};

impl LongBridgeBroker {
    pub async fn create_quote_context() -> longbridge::Result<(
        QuoteContext,
        UnboundedReceiver<longbridge::quote::PushEvent>,
    )> {
        let config = Arc::new(Config::from_env().unwrap());
        QuoteContext::try_new(config.clone()).await
    }

    pub async fn create_trade_context() -> longbridge::Result<(
        TradeContext,
        UnboundedReceiver<longbridge::trade::PushEvent>,
    )> {
        let config = Arc::new(Config::from_env().unwrap());
        TradeContext::try_new(config.clone()).await
    }

    pub fn to_currency(currency: &str) -> Result<Currency, Error> {
        match currency.to_uppercase().as_str() {
            "HKD" => Result::Ok(Currency::HKD),
            "USD" => Result::Ok(Currency::USD),
            "CNH" => Result::Ok(Currency::CNH),
            _ => Result::Err(anyhow!(
                "PARSING_ERROR Error when parsing currency {}",
                currency
            )),
        }
    }

    pub fn to_market(market: &str) -> Result<Market, Error> {
        match market.to_uppercase().as_str() {
            "US" => Result::Ok(Market::US),
            "HK" => Result::Ok(Market::HK),
            "CN" => Result::Ok(Market::CN),
            _ => Result::Err(anyhow!(
                "PARSING_ERROR Error when parsing market {}",
                market
            )),
        }
    }

    pub fn to_symbol(symbol: &str) -> Result<Symbol, Error> {
        let splitted_vec: Vec<&str> = symbol.split('.').collect();
        if splitted_vec.len() != 2 {
            return Result::Err(anyhow!(
                "PARSING_ERROR Error when parsing symbol {}",
                symbol
            ));
        }

        Result::Ok(Symbol {
            market: Self::to_market(splitted_vec[1])?,
            identifier: splitted_vec[0].to_owned(),
        })
    }
}
