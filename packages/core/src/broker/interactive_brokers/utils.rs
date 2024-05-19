use anyhow::{Context, Error};
use rust_decimal::Decimal;

use super::broker::InteractiveBrokersBroker;
use crate::model::trading::currency::Currency;

impl InteractiveBrokersBroker {
    pub fn depth_size_to_volume(size_optional: Option<String>) -> Result<Decimal, Error> {
        size_optional
            .clone()
            .with_context(|| {
                format!(
                    "Error when retrieving size from size_optional {:?}",
                    size_optional
                )
            })?
            .replace(",", "")
            .parse()
            .with_context(|| format!("Error when parsing string to decimal {:?}", size_optional))
    }

    pub fn parse_currency_from_optional_string(
        currency: Option<String>,
    ) -> Result<Currency, Error> {
        currency.clone()
            .with_context(|| format!("Error when retrieving currency from ibkr_client_portal::model::portfolio::Position"))?
            .parse::<Currency>()
            .with_context(|| format!("Error when parsing currency"))
    }

    pub fn parse_last_price(last_price_optional: Option<String>) -> Result<Decimal, Error> {
        last_price_optional
            .clone()
            .with_context(|| format!("Error last_price not exists"))
            .and_then(|last_price| {
                if last_price.starts_with("H") || last_price.starts_with("C") {
                    // Halt price, or prev closed price
                    &last_price.as_str()[1..]
                } else {
                    last_price.as_str()
                }
                .parse()
                .with_context(|| {
                    format!(
                        "Error when parsing {:?} into last_price",
                        last_price_optional
                    )
                })
            })
    }
}
