use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    definition::{AssetClass, OptionRight},
    position::DisplayRule,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityDefinitions {
    pub secdef: Vec<Contract>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    #[serde(with = "unpack_exchanges", alias = "exchange")]
    pub all_exchanges: Vec<String>,
    pub asset_class: Option<AssetClass>,
    pub chinese_name: Option<String>,
    pub conid: i64,
    pub country_code: Option<String>,
    pub currency: Option<String>,
    pub display_rule: Option<DisplayRule>,
    pub expiry: Option<String>,
    pub full_name: Option<String>,
    pub group: Option<String>,
    pub has_options: Option<bool>,
    pub increment_rules: Option<Vec<IncrementRule>>,
    pub is_event_contract: Option<bool>,
    #[serde(rename = "isUS")]
    pub is_us: Option<bool>,
    pub last_trading_day: Option<String>,
    pub listing_exchange: Option<String>,
    pub multiplier: Option<Decimal>,
    pub name: Option<String>,
    pub page_size: Option<i64>,
    pub put_or_call: Option<OptionRight>,
    pub sector: Option<String>,
    pub sector_group: Option<String>,
    pub strike: Option<String>,
    pub ticker: Option<String>,
    pub time: Option<i64>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub und_conid: Option<i64>,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncrementRule {
    #[serde(with = "rust_decimal::serde::float")]
    pub increment: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub lower_edge: Decimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractDetail {
    pub r_t_h: bool,
    #[serde(rename = r#"con_id"#)]
    pub conid: i64,
    pub company_name: String,
    pub exchange: String,
    pub local_symbol: String,
    pub instrument_type: String,
    pub currency: String,
    pub category: Option<String>,
    pub industry: Option<String>,
    pub symbol: String,
    pub underlying_con_id: i64,
    pub cusip: Option<String>,
    pub expiry_full: Option<String>,
    pub maturity_date: Option<String>,
    pub multiplier: Option<String>,
    pub underlying_issuer: Option<String>,
    pub trading_class: Option<String>,
    #[serde(with = "unpack_exchanges")]
    pub valid_exchanges: Vec<String>,
    pub allow_sell_long: bool,
    pub is_zero_commission_security: bool,
    pub contract_clarification_type: Option<String>,
    pub contract_month: Option<String>,
    pub classifier: Option<String>,
}

pub struct GetContractDetailRequest {
    pub conid: i64,
}

pub type StockContracts = HashMap<String, Vec<StockContractInfo>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockContractInfo {
    pub asset_class: AssetClass,
    pub chinese_name: Option<String>,
    pub contracts: Vec<Contract>,
    pub name: String,
}

pub struct GetStocksBySymbolRequest {
    pub symbols: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchForSecurityRequest {
    /// symbol or name to be searched
    pub symbol: String,
    #[serde(rename = "name")]
    /// should be true if the search is to be performed by name. false by default.
    pub is_name: bool,
    /// If search is done by name, only the assets provided in this field will be returned. Currently, only STK is supported.
    #[serde(rename = "secType")]
    pub sec_type: AssetClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchForSecurityResponseSection {
    /// Asset Class
    #[serde(rename = "secType")]
    sec_type: Option<String>,
    /// List of expiration month(s) and year(s) in MMMYY format separated by semicolon
    #[serde(rename = "months")]
    months: Option<String>,
    /// Underlying symbol
    #[serde(rename = "symbol")]
    symbol: Option<String>,
    /// Listing Exchange
    #[serde(rename = "exchange")]
    exchange: Option<String>,
    /// For combo's defines the asset class for each leg
    #[serde(rename = "legSecType")]
    leg_sec_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchForSecurityResponse {
    /// Contract Identifier
    #[serde(rename = "conid")]
    pub conid: Option<i32>,
    /// Company Name - Exchange
    #[serde(rename = "companyHeader")]
    pub company_header: Option<String>,
    #[serde(rename = "companyName")]
    pub company_name: Option<String>,
    /// Underlying symbol
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// Exchange
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "restricted")]
    pub restricted: Option<String>,
    /// List of Future Option expirations in YYYMMDD format separated by semicolon
    #[serde(rename = "fop")]
    pub future_option_expirations: Option<String>,
    /// List of Option expirations in YYYYMMDD format separated by semicolon
    #[serde(rename = "opt")]
    pub option_expirations: Option<String>,
    /// List of Warrant expirations in YYYYMMDD format separated by semicolon
    #[serde(rename = "war")]
    pub warrant_expirations: Option<String>,
    #[serde(rename = "sections")]
    pub sections: Option<Vec<SearchForSecurityResponseSection>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSecurityDefinitionByContractIdRequest {
    conids: Vec<i64>,
}
