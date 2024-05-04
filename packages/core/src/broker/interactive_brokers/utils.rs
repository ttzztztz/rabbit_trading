use anyhow::{anyhow, Error};

use super::broker::InteractiveBrokersBroker;
use crate::model::trading::{currency::Currency, symbol::Symbol};

impl InteractiveBrokersBroker {
    pub async fn get_conid_from_symbol(symbol: &Symbol) -> i64 {
        // TODO: implement this function
        265598 // Use AAPL as the placeholder
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
