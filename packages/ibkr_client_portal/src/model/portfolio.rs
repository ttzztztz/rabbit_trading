use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::{
    account::Allocation,
    contract::IncrementRule,
    definition::{AssetClass, OptionRight},
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    #[serde(rename = "acct_id")]
    pub account_id: String,
    pub all_exchanges: Option<String>,
    pub asset_class: AssetClass,
    pub avg_cost: Decimal,
    pub avg_price: Decimal,
    pub base_avg_cost: Option<Decimal>,
    pub base_avg_price: Option<Decimal>,
    pub base_market_price: Option<Decimal>,
    pub conid: Option<String>,
    pub currency: Option<String>,
    pub description: Option<String>,
    pub exercise_style: Option<String>,
    pub expiry: Option<String>,
    pub full_name: Option<String>,
    pub group: Option<String>,
    pub has_options: Option<bool>,
    #[serde(skip)]
    pub increment_rules: Option<IncrementRule>,
    pub is_event_contract: Option<bool>,
    #[serde(rename = "isUS")]
    pub is_us: Option<bool>,
    pub last_trading_day: Option<String>,
    pub listing_exchange: Option<String>,
    pub market_price: Decimal,
    pub market_value: Decimal,
    pub model: Option<String>,
    pub multiplier: Option<Decimal>,
    pub name: Option<String>,
    pub page_size: Option<i64>,
    pub position: Decimal,
    pub put_or_call: Option<OptionRight>,
    pub realized_pnl: Decimal,
    pub sec_type: Option<String>,
    pub sector: Option<String>,
    pub sector_group: Option<String>,
    pub strike: Option<Decimal>,
    pub ticker: Option<String>,
    pub timestamp: Option<i64>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    #[serde(rename = "undComp")]
    pub underlying_comp: Option<String>,
    #[serde(rename = "undConid")]
    pub underlying_conid: Vec<i64>,
    #[serde(rename = "undSym")]
    pub underlying_sym: Option<String>,
    pub unrealized_pnl: Decimal,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayRule {
    pub display_rule_step: Vec<DisplayRuleStep>,
    pub magnification: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayRuleStep {
    pub decimal_digits: i64,
    pub lower_edge: Decimal,
    pub whole_digits: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetPortfolioPositionsRequest {
    pub page: i32,
}

pub type GetPortfolioPositionsResponse = Vec<Position>;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetPortfolioAllocationRequest {
    #[serde(rename = "acctIds")]
    pub account_id_list: Vec<String>,
}

pub type GetPortfolioAllocationResponse = Allocation;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetPortfolioPositionByAccountAndConIdRequest {
    pub account_id: String,
    pub conid: i64,
}

pub type GetPortfolioPositionByAccountAndConIdResponse = Vec<Position>;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InvalidatePortfolioCacheRequest {
    pub account_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetPortfolioPositionByConIdRequest {
    pub conid: i64,
}

pub type GetPortfolioPositionByConIdResponse = HashMap<String, Vec<Position>>;
