use anyhow::{Context, Error};
use rust_decimal::Decimal;

use super::broker::InteractiveBrokersBroker;
use crate::model::trading::currency::Currency;

impl InteractiveBrokersBroker {
    pub fn depth_size_to_volume(size_optional: Option<String>) -> Option<Decimal> {
        size_optional.map(|size| size.replace(",", "").parse().ok())?
    }

    pub fn parse_currency_from_optional_string(
        currency: Option<String>,
    ) -> Result<Currency, Error> {
        currency.clone()
            .with_context(|| format!("Error when retrieving currency from ibkr_client_portal::model::portfolio::Position"))?
            .parse::<Currency>()
            .with_context(|| format!("Error when parsing currency"))
    }
}
