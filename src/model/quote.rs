use std::iter::Map;

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

#[derive(Debug)]
pub struct QuoteInfo {
    pub quote: Quote,
    pub timestamp: u64,
    pub price: f64,
    pub volume: u64,
    pub extra: Option<Map<String, String>>,
}
