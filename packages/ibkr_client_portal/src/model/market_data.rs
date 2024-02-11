use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use time::OffsetDateTime;

use super::definition::TickType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMarketDataRequest {
    pub conids: Vec<i64>,
    pub since: Option<i64>,
    pub fields: Option<Vec<TickType>>,
}

pub type GetMarketDataResponse = Vec<HashMap<String, Value>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketDataHistoryBar {
    #[serde(alias = "t")]
    pub timestamp: OffsetDateTime,
    #[serde(alias = "o")]
    pub open: Decimal,
    #[serde(alias = "c")]
    pub close: Decimal,
    #[serde(alias = "h")]
    pub high: Decimal,
    #[serde(alias = "l")]
    pub low: Decimal,
    #[serde(alias = "v")]
    pub volume: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketDataHistory {
    /// Underlying symbol
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// companyName
    #[serde(rename = "text")]
    pub text: Option<String>,
    /// priceFactor is price increment obtained from display rule
    #[serde(rename = "priceFactor")]
    pub price_factor: Option<i32>,
    /// start date time in the format YYYYMMDD-HH:mm:ss
    #[serde(rename = "startTime")]
    pub start_time: Option<OffsetDateTime>,
    /// High value during this time series with format %h/%v/%t. %h is the high price (scaled by priceFactor), %v is volume (volume factor will always be 100 (reported volume = actual volume/100)) and %t is minutes from start time of the chart
    #[serde(rename = "high")]
    pub high: Option<String>,
    /// Low value during this time series with format %l/%v/%t. %l is the low price (scaled by priceFactor), %v is volume (volume factor will always be 100 (reported volume = actual volume/100)) and %t is minutes from start time of the chart
    #[serde(rename = "low")]
    pub low: Option<String>,
    /// The duration for the historical data request
    #[serde(rename = "timePeriod")]
    pub time_period: Option<String>,
    /// The number of seconds in a bar
    #[serde(rename = "barLength")]
    pub bar_length: Option<i32>,
    /// Market Data Availability. The field may contain two chars. The first char is the primary code: S = Streaming, R = Realtime, D = Delayed, Z = Frozen, Y = Frozen Delayed. The second char is the secondary code: P = Snapshot Available, p = Consolidated.
    #[serde(rename = "mdAvailability")]
    pub md_availability: Option<String>,
    /// The time it takes, in milliseconds, to process the historical data request
    #[serde(rename = "mktDataDelay")]
    pub mkt_data_delay: Option<i32>,
    /// The historical data returned includes outside of regular trading hours
    #[serde(rename = "outsideRth")]
    pub outside_rth: Option<bool>,
    /// The number of seconds in the trading day
    #[serde(rename = "tradingDayDuration")]
    pub trading_day_duration: Option<i32>,
    #[serde(rename = "volumeFactor")]
    pub volume_factor: Option<i32>,
    #[serde(rename = "priceDisplayRule")]
    pub price_display_rule: Option<i32>,
    #[serde(rename = "priceDisplayValue")]
    pub price_display_value: Option<String>,
    #[serde(rename = "negativeCapable")]
    pub negative_capable: Option<bool>,
    #[serde(rename = "messageVersion")]
    pub message_version: Option<i32>,
    #[serde(rename = "data")]
    pub data: Option<Vec<MarketDataHistoryBar>>,
    /// total number of points
    #[serde(rename = "points")]
    pub points: Option<i32>,
    #[serde(rename = "travelTime")]
    pub travel_time: Option<i32>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UnsubscribeAllMarketDataResponse {
    /// true means market data is cancelled, false means it is not.
    #[serde(rename = "confirmed")]
    pub confirmed: Option<bool>,
}

pub struct UnsubscribeMarketDataRequest {
    pub conid: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnsubscribeMarketDataResponse {
    /// true means market data is cancelled, false means it is not.
    #[serde(rename = "confirmed")]
    pub confirmed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketHistoryDataBarsData {
    /// Time - Formatted in unix time in ms.
    #[serde(rename = "t")]
    pub time: Option<String>,
    /// Open - First price returned for bar value.
    #[serde(rename = "o")]
    pub open: Option<Decimal>,
    /// Close - Last price returned for bar value.
    #[serde(rename = "c")]
    pub close: Option<Decimal>,
    /// High - High price returned for bar value.
    #[serde(rename = "h")]
    pub high: Option<Decimal>,
    /// Low - Last price returned for bar value.
    #[serde(rename = "l")]
    pub low: Option<Decimal>,
    /// Volume - Traded volume for bar value.
    #[serde(rename = "v")]
    pub volume: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketHistoryDataBars {
    /// First price returned for bar value.
    #[serde(rename = "open")]
    pub open: Option<f32>,
    /// Start Time in the format YYYYMMDD.
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    /// Start Time Value - Formatted in unix time in ms.
    #[serde(rename = "startTimeVal")]
    pub start_time_val: Option<i32>,
    /// End Time in the format YYYYMMDD.
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    /// End Time Value - Formatted in unix time in ms.
    #[serde(rename = "endTimeVal")]
    pub end_time_val: Option<i32>,
    /// total number of data points.
    #[serde(rename = "points")]
    pub points: Option<i32>,
    #[serde(rename = "data")]
    pub data: Option<Vec<MarketHistoryDataBarsData>>,
    /// If 0 then data is returned in real time. Otherwise will return the number of seconds history data is delayed.
    #[serde(rename = "mktDataDelay")]
    pub mkt_data_delay: Option<i32>,
}

pub struct GetMarketDataHistoryBetaRequest {
    /// contract id
    pub conid: i64,
    pub period: String,
    /// Enum: "min" "h" "d" "w" "m" "y"
    /// Time period for history request.
    ///
    /// min: Minutes
    /// h: Hours
    /// d: Days
    /// w: Weeks
    /// m: Months
    /// y: Years
    pub bar: Option<String>,
    /// For contracts that support it, will determine if history data includes outside of regular trading hours.
    pub outside_regular_trading_hours: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMarketDataHistoryBetaResponse {
    #[serde(rename = "bars")]
    pub bars: MarketHistoryDataBars,
}
