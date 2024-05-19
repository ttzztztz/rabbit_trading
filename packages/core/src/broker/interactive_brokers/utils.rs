use anyhow::{anyhow, Error};
use longbridge::Decimal;

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

    pub fn depth_size_to_volume(size_optional: Option<String>) -> Option<Decimal> {
        size_optional.map(|size| size.replace(",", "").parse().ok())?
    }
}
