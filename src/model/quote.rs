use rust_decimal::Decimal;
use std::{any::Any, collections::HashMap};

#[derive(Clone, Debug)]
pub enum Region {
    CN,
    HK,
    US,
}

impl std::string::ToString for Region {
    fn to_string(&self) -> String {
        match self {
            Region::CN => String::from("CN"),
            Region::HK => String::from("HK"),
            Region::US => String::from("US"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub region: Region,
    pub identifier: String,
}

impl std::string::ToString for Symbol {
    fn to_string(&self) -> String {
        format!("{}.{}", self.identifier, self.region.to_string())
    }
}

// todo: prev_close, trading_session, is_trading
#[derive(Debug)]
pub struct QuoteInfo {
    pub symbol: Symbol,
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
