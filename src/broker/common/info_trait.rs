use async_trait::async_trait;
use std::iter::Map;

use crate::model::{error::Error, quote::QuoteInfo, symbol::Symbol};

#[derive(Clone)]
pub struct InfoContext {
    pub symbol: Symbol,
    pub extra: Option<Map<String, String>>,
}

#[async_trait]
pub trait Info {
    async fn new(context: InfoContext) -> Self
    where
        Self: Sized;
    async fn query_real_time_info(&self) -> Result<QuoteInfo, Error>;
}
