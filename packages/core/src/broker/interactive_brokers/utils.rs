use anyhow::{anyhow, Error};

use super::broker::InteractiveBrokersBroker;
use crate::model::trading::currency::Currency;

impl InteractiveBrokersBroker {
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
