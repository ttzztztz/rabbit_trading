use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::tick_types::TickType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketDataRequest {
    pub conids: Vec<String>,
    pub since: Option<i64>,
    pub fields: Vec<TickType>,
}

pub type MarketDataResponse = Vec<HashMap<String, Value>>;
