use longbridge::{Config, QuoteContext, TradeContext};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedReceiver;

use super::broker::LongBridgeBroker;

impl LongBridgeBroker {
    pub async fn create_quote_context() -> longbridge::Result<(
        QuoteContext,
        UnboundedReceiver<longbridge::quote::PushEvent>,
    )> {
        let config = Arc::new(Config::from_env().unwrap());
        QuoteContext::try_new(config.clone()).await
    }

    pub async fn create_trade_context() -> longbridge::Result<(
        TradeContext,
        UnboundedReceiver<longbridge::trade::PushEvent>,
    )> {
        let config = Arc::new(Config::from_env().unwrap());
        TradeContext::try_new(config.clone()).await
    }
}
