use rust_decimal::Decimal;
use std::{any::Any, collections::HashMap};

#[derive(Clone, Debug)]
pub enum QuoteKind {
    Stock,
    Option,
    Future,
}

impl std::string::ToString for QuoteKind {
    fn to_string(&self) -> String {
        match self {
            QuoteKind::Stock => String::from("Stock"),
            QuoteKind::Option => String::from("Option"),
            QuoteKind::Future => String::from("Future"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Quote {
    pub kind: QuoteKind,
    pub identifier: String,
}

impl std::string::ToString for Quote {
    fn to_string(&self) -> String {
        format!("{}:{}", self.kind.to_string(), self.identifier)
    }
}

// todo: prev_close, trading_session, is_trading
#[derive(Debug)]
pub struct QuoteInfo {
    pub quote: Quote,
    pub sequence: u64,
    pub timestamp: i64,
    pub current_price: Decimal,
    pub low_price: Option<Decimal>,
    pub high_price: Option<Decimal>,
    pub open_price: Option<Decimal>,
    pub prev_close: Option<Decimal>,
    pub volume: u64,
    pub turnover: Option<Decimal>,
    pub extra: Option<HashMap<String, Box<dyn Any + Send>>>,
}
