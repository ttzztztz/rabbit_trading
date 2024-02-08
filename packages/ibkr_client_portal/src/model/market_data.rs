use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use time::OffsetDateTime;

use super::definition::TickType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketDataRequest {
    pub conids: Vec<String>,
    pub since: Option<i64>,
    pub fields: Option<Vec<TickType>>,
}

pub type MarketDataResponse = Vec<HashMap<String, Value>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bar {
    #[serde(alias = "t")]
    pub timestamp: OffsetDateTime,
    #[serde(alias = "o")]
    pub open: rust_decimal::Decimal,
    #[serde(alias = "c")]
    pub close: rust_decimal::Decimal,
    #[serde(alias = "h")]
    pub high: rust_decimal::Decimal,
    #[serde(alias = "l")]
    pub low: rust_decimal::Decimal,
    #[serde(alias = "v")]
    pub volume: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketDataHistory {
    pub server_id: String,
    pub symbol: String,
    pub text: String,
    pub price_factor: i64,
    #[serde(with = "parse_datetime")]
    pub start_time: OffsetDateTime,
    pub high: Option<String>,
    pub low: Option<String>,
    pub time_period: String,
    pub bar_length: u32,
    pub md_availability: String,
    pub mkt_data_delay: i64,
    pub outside_rth: bool,
    pub trading_day_duration: Option<i64>,
    pub volume_factor: i64,
    pub price_display_rule: i64,
    pub price_display_value: String,
    pub negative_capable: bool,
    pub message_version: i64,
    pub data: Vec<Bar>,
    pub points: Option<u32>,
    pub travel_time: u32,
}

mod parse_datetime {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::{macros::format_description, OffsetDateTime};

    pub fn serialize<S>(date_time: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let format_description =
            format_description!("[year][month][day]-[offset_hour]:[offset_minute]:[offset_second]");
        let s = date_time
            .format(format_description)
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let format_description =
            format_description!("[year][month][day]-[offset_hour]:[offset_minute]:[offset_second]");
        let s = String::deserialize(deserializer)?.to_string();
        OffsetDateTime::parse(&s, format_description).map_err(serde::de::Error::custom)
    }
}

pub struct GetMarketDataHistoryRequest {
    pub conid: i64,
    pub exchange: Option<String>,
    pub period: String,
    pub bar: String,
    pub outside_rth: bool,
    pub start_time: Option<OffsetDateTime>,
}
