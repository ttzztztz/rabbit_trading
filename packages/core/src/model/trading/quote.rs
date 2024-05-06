use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{currency::Currency, symbol::Symbol};

// todo: prev_close, trading_session, is_trading
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct QuoteRealTimeInfo {
    pub symbol: Symbol,
    pub sequence: u64,
    pub timestamp: u64,
    pub current_price: Decimal,
    pub volume: Decimal,
    pub low_price: Option<Decimal>,
    pub high_price: Option<Decimal>,
    pub open_price: Option<Decimal>,
    pub prev_close: Option<Decimal>,
    pub turnover: Option<Decimal>,
    pub extra: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct QuoteBasicInfo {
    pub symbol: Symbol,
    pub currency: Option<Currency>,
    pub lot_size: i32,
    pub total_shares: Decimal,
    pub circulating_shares: Decimal,
    pub eps: Decimal,
    pub eps_ttm: Decimal,
    pub bps: Decimal,
    pub dividend_yield: Decimal,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Depth {
    pub position: Option<Decimal>,
    pub price: Decimal,
    pub volume: Decimal,
    pub order_count: Option<Decimal>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct QuoteDepthInfo {
    pub symbol: Symbol,
    pub sequence: u64,
    pub timestamp: u64,
    pub ask_list: Vec<Depth>,
    pub bid_list: Vec<Depth>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum QuoteKind {
    Stock,
    Option,
    // todo: add more
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct QueryInfoRequest {
    pub symbol: Symbol,
    pub kind: QuoteKind,
}
