use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::Date;

pub type FuturesContracts = HashMap<String, Vec<FuturesContract>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesContract {
    pub conid: i64,
    #[serde(with = "parse_date")]
    pub expiration_date: Date,
    #[serde(with = "parse_date", rename = "ltd")]
    pub last_trading_day: Date,
    pub symbol: String,
    pub underlying_conid: i64,
}

mod parse_date {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::{macros::format_description, Date};

    pub fn serialize<S>(date: &Date, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let format_description = format_description!("[year][month][day]");
        let s = date
            .format(format_description)
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Date, D::Error>
    where
        D: Deserializer<'de>,
    {
        let format_description = format_description!("[year][month][day]");
        let s = String::deserialize(deserializer)?.to_string();
        Date::parse(&s, format_description).map_err(serde::de::Error::custom)
    }
}

pub struct GetFuturesBySymbolRequest {
    pub symbols: Vec<String>,
}
