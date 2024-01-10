use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    contract::{unpack_exchanges, IncrementRule},
    definition::{AssetClass, OptionRight},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Position {
    pub acct_id: String,
    #[serde(with = "unpack_exchanges")]
    pub all_exchanges: Vec<String>,
    pub asset_class: AssetClass,
    pub avg_cost: Decimal,
    pub avg_price: Decimal,
    pub base_avg_cost: Option<Decimal>,
    pub base_avg_price: Option<Decimal>,
    pub base_mkt_price: Option<Decimal>,
    pub base_mkt_value: Option<Decimal>,
    pub base_realized_pnl: Option<Decimal>,
    pub base_unrealized_pnl: Option<Decimal>,
    #[serde(skip)]
    pub chinese_name: String,
    pub con_exch_map: Vec<Value>,
    pub conid: i64,
    pub contract_desc: String,
    pub country_code: String,
    pub cross_currency: Option<bool>,
    pub currency: String,
    #[serde(skip)]
    pub display_rule: DisplayRule,
    pub exchs: Value,
    pub exercise_style: Value,
    pub expiry: Option<String>,
    pub full_name: String,
    pub group: String,
    pub has_options: bool,
    #[serde(skip)]
    pub increment_rules: Vec<IncrementRule>,
    pub is_event_contract: bool,
    #[serde(rename = "isUS")]
    pub is_us: bool,
    pub last_trading_day: Option<String>,
    pub listing_exchange: Option<String>,
    pub mkt_price: Decimal,
    pub mkt_value: Decimal,
    pub model: String,
    pub multiplier: Option<Decimal>,
    pub name: Option<String>,
    pub page_size: i64,
    pub position: Decimal,
    pub put_or_call: Option<OptionRight>,
    pub realized_pnl: Decimal,
    pub sector: String,
    pub sector_group: Option<String>,
    pub strike: Value,
    pub ticker: String,
    pub time: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub und_comp: Option<Value>,
    pub und_conid: i64,
    pub und_sym: Option<String>,
    pub unrealized_pnl: Decimal,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayRule {
    pub display_rule_step: Vec<DisplayRuleStep>,
    pub magnification: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayRuleStep {
    pub decimal_digits: i64,
    pub lower_edge: Decimal,
    pub whole_digits: i64,
}
