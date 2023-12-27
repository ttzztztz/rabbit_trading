use async_trait::async_trait;
use longbridge::{
    quote::{PushEvent, PushQuote, SubFlags},
    QuoteContext,
};
use std::sync::Arc;
use tokio::sync::{
    mpsc::{Sender, UnboundedReceiver},
    Mutex,
};

use crate::{
    broker::{
        common::subscription::{SubscriptionController, SubscriptionWorker},
        longbridge::broker::LongBridgeBroker,
    },
    model::{error::Error, quote::QuoteRealTimeInfo, symbol::Symbol},
};

pub struct LongBridgeQuoteRealTimeInfoSubscriptionWorker {
    symbol: Symbol,
    sys_sender: Sender<QuoteRealTimeInfo>,
    longbridge_context: Arc<Mutex<QuoteContext>>,
    longbridge_receiver: UnboundedReceiver<PushEvent>,
}

impl LongBridgeQuoteRealTimeInfoSubscriptionWorker {
    pub fn new(
        symbol: Symbol,
        sys_sender: Sender<QuoteRealTimeInfo>,
        longbridge_context: Arc<Mutex<QuoteContext>>,
        longbridge_receiver: UnboundedReceiver<PushEvent>,
    ) -> Self {
        LongBridgeQuoteRealTimeInfoSubscriptionWorker {
            symbol,
            sys_sender,
            longbridge_context,
            longbridge_receiver,
        }
    }

    pub(super) fn to_quote_real_time_info(
        symbol: Symbol,
        longbridge_quote: PushQuote,
    ) -> QuoteRealTimeInfo {
        let timestamp = longbridge_quote.timestamp.unix_timestamp() as u64;
        QuoteRealTimeInfo {
            symbol,
            sequence: timestamp,
            timestamp,
            current_price: longbridge_quote.last_done,
            low_price: Option::Some(longbridge_quote.low),
            high_price: Option::Some(longbridge_quote.high),
            open_price: Option::Some(longbridge_quote.open),
            prev_close: Option::None,
            volume: longbridge_quote.volume as u64,
            turnover: Option::Some(longbridge_quote.turnover),
            extra: Option::None,
        }
    }
}

#[async_trait]
impl SubscriptionWorker for LongBridgeQuoteRealTimeInfoSubscriptionWorker {
    async fn start(mut self) {
        let symbol_identifier = self.symbol.to_string();
        let sys_sender = self.sys_sender;
        let mut longbridge_receiver = self.longbridge_receiver;
        self.longbridge_context
            .lock()
            .await
            .subscribe([symbol_identifier], SubFlags::QUOTE, true)
            .await
            .unwrap();

        while let Some(event_detail) = longbridge_receiver.recv().await.map(|event| event.detail) {
            match event_detail {
                longbridge::quote::PushEventDetail::Quote(longbridge_quote) => {
                    let quote_info =
                        Self::to_quote_real_time_info(self.symbol.clone(), longbridge_quote);
                    if let Err(send_result_err) = sys_sender.send(quote_info).await {
                        log::error!("error when sending into mpsc {}", send_result_err);
                    }
                }
                _ => {
                    log::error!("event not supported! {event_detail:?}");
                }
            }
        }
    }
}

pub struct LongBridgeQuoteRealTimeInfoSubscriptionController {
    symbol: Symbol,
    longbridge_context: Arc<Mutex<QuoteContext>>,
}

impl LongBridgeQuoteRealTimeInfoSubscriptionController {
    pub fn new(symbol: Symbol, longbridge_context: Arc<Mutex<QuoteContext>>) -> Self {
        LongBridgeQuoteRealTimeInfoSubscriptionController {
            symbol,
            longbridge_context,
        }
    }
}

#[async_trait]
impl SubscriptionController for LongBridgeQuoteRealTimeInfoSubscriptionController {
    async fn stop(self) -> Result<(), Error> {
        let symbol_identifier = self.symbol.to_string();
        self.longbridge_context
            .lock()
            .await
            .unsubscribe([symbol_identifier], SubFlags::QUOTE)
            .await
            .map_err(LongBridgeBroker::to_rabbit_trading_err)
    }
}
