use serde::{Deserialize, Serialize};

use super::tick_types::TickType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketDataRequest {
    pub conids: Vec<String>,
    pub since: Option<i64>,
    pub fields: Vec<TickType>,
}
