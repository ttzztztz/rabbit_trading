use serde::{Deserialize, Serialize};

use super::contract::unpack_exchanges;

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
