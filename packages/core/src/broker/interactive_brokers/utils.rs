use anyhow::{anyhow, Error};

use super::broker::InteractiveBrokersBroker;
use crate::model::trading::{currency::Currency, market::Market, symbol::Symbol};

impl InteractiveBrokersBroker {
    pub async fn get_conid_from_symbol(symbol: &Symbol) -> i64 {
        // TODO: implement this function
        265598 // Use AAPL as the placeholder
    }

    pub async fn get_symbol_from_conid(conid: i64) -> Symbol {
        // TODO: implement this function
        Symbol {
            market: Market::US,
            identifier: "AAPL".to_owned(),
        } // Use AAPL as the placeholder
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
}
