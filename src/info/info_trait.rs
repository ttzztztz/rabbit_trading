use std::iter::Map;
use std::sync::Arc;
use tokio::runtime::Runtime;

use crate::model::quote::{Quote, QuoteInfo};

pub struct InfoContext {
    pub quote: Quote,
    pub runtime: Arc<Runtime>,
    pub extra: Option<Map<String, String>>,
}

pub trait Info {
    fn new(context: InfoContext) -> Self;
    fn query_real_time(&self) -> QuoteInfo;
}
