use rust_decimal::Decimal;
use std::{any::Any, collections::HashMap};

use super::symbol::Symbol;

// todo: prev_close, trading_session, is_trading
#[derive(Debug)]
pub struct QuoteRealTimeInfo {
    pub symbol: Symbol,
    pub sequence: u64,
    pub timestamp: i64,
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
    pub sequence: u64,
    pub timestamp: i64,
}

#[derive(Debug)]
pub struct QuoteDepthInfo {
    pub symbol: Symbol,
    pub sequence: u64,
    pub timestamp: i64,
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
