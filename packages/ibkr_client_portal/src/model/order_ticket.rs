use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderTicket {
    pub acct_id: String,
    pub conid: i64,
    pub conidex: String,
    pub sec_type: String,
    #[serde(rename = "cOID")]
    pub c_oid: String,
    pub parent_id: String,
    pub order_type: String,
    pub listing_exchange: String,
    pub is_single_group: bool,
    #[serde(rename = "outsideRTH")]
    pub outside_rth: bool,
    pub price: i64,
    pub aux_price: String,
    pub side: String,
    pub ticker: String,
    pub tif: String,
    pub trailing_amt: i64,
    pub trailing_type: String,
    pub referrer: String,
    pub quantity: i64,
    pub cash_qty: i64,
    pub fx_qty: i64,
    pub use_adaptive: bool,
    pub is_ccy_conv: bool,
    pub allocation_method: String,
    pub strategy: String,
    pub strategy_parameters: HashMap<String, Value>,
}
