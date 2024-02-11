use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    account::Allocation,
    contract::{unpack_exchanges, IncrementRule},
    definition::{AssetClass, OptionRight},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    pub conid: Option<i32>,
    pub exercise_style: Option<String>,
    pub expiry: Option<String>,
    pub full_name: Vec<String>,
    pub group: Vec<String>,
    pub has_options: Vec<bool>,
    #[serde(skip)]
    pub increment_rules: Vec<IncrementRule>,
    pub is_event_contract: Vec<bool>,
    #[serde(rename = "isUS")]
    pub is_us: Vec<bool>,
    pub last_trading_day: Option<String>,
    pub listing_exchange: Option<String>,
    pub mkt_price: Decimal,
    pub mkt_value: Decimal,
    pub model: Vec<String>,
    pub multiplier: Option<Decimal>,
    pub name: Option<String>,
    pub page_size: Vec<i64>,
    pub position: Decimal,
    pub put_or_call: Option<OptionRight>,
    pub realized_pnl: Decimal,
    pub sector: Vec<String>,
    pub sector_group: Option<String>,
    pub strike: Vec<Value>,
    pub ticker: Vec<String>,
    pub time: Vec<i64>,
    #[serde(rename = "type")]
    pub type_field: Vec<String>,
    pub und_comp: Option<Value>,
    pub und_conid: Vec<i64>,
    pub und_sym: Option<String>,
    pub unrealized_pnl: Decimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayRule {
    pub display_rule_step: Vec<DisplayRuleStep>,
    pub magnification: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayRuleStep {
    pub decimal_digits: i64,
    pub lower_edge: Decimal,
    pub whole_digits: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPortfolioPositionsRequest {
    pub page: i32,
}

pub type GetPortfolioPositionsResponse = Vec<Position>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPortfolioAllocationRequest {
    #[serde(rename = "acctIds")]
    pub account_id_list: Option<Vec<String>>,
}

pub type GetPortfolioAllocationResponse = Vec<Allocation>;

pub struct GetPortfolioPositionByAccountAndConIdRequest {
    pub account_id: String,
    pub conid: i64,
}

pub type GetPortfolioPositionByAccountAndConIdResponse = Vec<Position>;

pub struct InvalidatePortfolioCacheRequest {
    pub account_id: String,
}

pub struct GetPortfolioPositionByConIdRequest {
    pub conid: i64,
}

pub type GetPortfolioPositionByConIdResponse = HashMap<String, Position>;
