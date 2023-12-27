use rust_decimal::Decimal;
use std::{any::Any, collections::HashMap};

use super::{currency::Currency, symbol::Symbol};

// todo: prev_close, trading_session, is_trading
#[derive(Debug)]
pub struct QuoteRealTimeInfo {
    pub symbol: Symbol,
    pub sequence: u64,
    pub timestamp: u64,
    pub current_price: Decimal,
    pub volume: u64,
    pub low_price: Option<Decimal>,
    pub high_price: Option<Decimal>,
    pub open_price: Option<Decimal>,
    pub prev_close: Option<Decimal>,
    pub turnover: Option<Decimal>,
    pub extra: Option<HashMap<String, Box<dyn Any + Send + Sync>>>,
}

#[derive(Debug)]
pub struct QuoteBasicInfo {
    pub symbol: Symbol,
    pub currency: Option<Currency>,
    pub lot_size: i32,
    pub total_shares: i64,
    pub circulating_shares: i64,
    pub eps: Decimal,
    pub eps_ttm: Decimal,
    pub bps: Decimal,
    pub dividend_yield: Decimal,
}

#[derive(Debug, Clone, Copy)]
pub struct Depth {
    pub position: i32,
    pub price: Decimal,
    pub volume: i64,
    pub order_count: i64,
}

#[derive(Debug)]
pub struct QuoteDepthInfo {
    pub symbol: Symbol,
    pub sequence: u64,
    pub timestamp: u64,
    pub ask_list: Vec<Depth>,
    pub bid_list: Vec<Depth>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum QuoteKind {
    Stock,
    Option,
    // todo: add more
}

#[derive(Debug, Clone)]
pub struct QueryInfoRequest {
    pub symbol: Symbol,
    pub kind: QuoteKind,
}
