use longbridge::{Config, QuoteContext, Result, TradeContext};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedReceiver;

use super::broker::LongBridgeBroker;
use crate::model::error::Error;

impl LongBridgeBroker {
    const OTHER_ERROR_CODE: &'static str = "other";

    pub async fn create_quote_context() -> Result<(
        QuoteContext,
        UnboundedReceiver<longbridge::quote::PushEvent>,
    )> {
        let config = Arc::new(Config::from_env().unwrap());
        QuoteContext::try_new(config.clone()).await
    }

    pub async fn create_trade_context() -> Result<(
        TradeContext,
        UnboundedReceiver<longbridge::trade::PushEvent>,
    )> {
        let config = Arc::new(Config::from_env().unwrap());
        TradeContext::try_new(config.clone()).await
    }

    pub fn to_rabbit_trading_err(err: longbridge::Error) -> Error {
        match err.into_simple_error() {
            longbridge::SimpleError::Response {
                code,
                message,
                trace_id,
            } => Error {
                code: format!("{}", code),
                message: format!("{}, trace_id={}", message, trace_id),
            },
            longbridge::SimpleError::Other(message) => Error {
                code: Self::OTHER_ERROR_CODE.to_owned(),
                message,
            },
        }
    }
}
