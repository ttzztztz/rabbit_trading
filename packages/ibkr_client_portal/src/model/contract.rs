use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    definition::{AssetClass, OptionRight},
    position::DisplayRule,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityDefinitions {
    pub secdef: Vec<Contract>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Contract {
    #[serde(with = "unpack_exchanges", alias = "exchange")]
    pub all_exchanges: Vec<String>,
    pub asset_class: AssetClass,
    pub chinese_name: String,
    pub conid: i64,
    pub country_code: String,
    pub currency: String,
    pub display_rule: DisplayRule,
    pub expiry: Option<String>,
    pub full_name: String,
    pub group: Option<String>,
    pub has_options: bool,
    pub increment_rules: Vec<IncrementRule>,
    pub is_event_contract: bool,
    #[serde(rename = "isUS")]
    pub is_us: Option<bool>,
    pub last_trading_day: Option<String>,
    pub listing_exchange: String,
    pub multiplier: Decimal,
    pub name: Option<String>,
    pub page_size: Option<i64>,
    pub put_or_call: Option<OptionRight>,
    pub sector: Option<String>,
    pub sector_group: Option<String>,
    pub strike: String,
    pub ticker: String,
    pub time: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub und_conid: i64,
    pub cross_currency: Option<bool>,
    pub und_comp: Option<Value>,
    pub und_sym: Option<String>,
}

pub mod unpack_exchanges {
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(exchanges: &[String], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = exchanges
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(",");
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let exchanges: Vec<String> = s.split(',').map(|e| e.trim().to_string()).collect();

        Ok(exchanges)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncrementRule {
    #[serde(with = "rust_decimal::serde::float")]
    pub increment: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub lower_edge: Decimal,
}
