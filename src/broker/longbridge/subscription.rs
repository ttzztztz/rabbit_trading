use async_trait::async_trait;
use longbridge::{
    quote::{PushEvent, PushQuote, SubFlags},
    QuoteContext,
};
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Sender, UnboundedReceiver},
    Mutex,
};

use super::broker::LongBridgeBroker;
use crate::{
    broker::common::subscription::{
        SubscriptionController, SubscriptionData, SubscriptionTrait, SubscriptionWorker,
    },
    model::{
        error::Error,
        quote::{QueryInfoRequest, QuoteDepthInfo, QuoteRealTimeInfo},
        symbol::Symbol,
    },
};

// https://crates.io/crates/longbridge
pub(super) struct LongBridgeSubscription {}

impl LongBridgeSubscription {
    fn to_quote_real_time_info(symbol: Symbol, longbridge_quote: PushQuote) -> QuoteRealTimeInfo {
        let timestamp = longbridge_quote.timestamp.unix_timestamp() as u64;
        QuoteRealTimeInfo {
            symbol,
            sequence: timestamp,
            timestamp: timestamp as i64,
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
}

#[async_trait]
impl SubscriptionWorker for LongBridgeQuoteRealTimeInfoSubscriptionWorker {
    async fn start(mut self) {
        let symbol = self.symbol.clone();
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
                    let quote_info = LongBridgeSubscription::to_quote_real_time_info(
                        symbol.clone(),
                        longbridge_quote,
                    );
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

#[async_trait]
impl SubscriptionTrait for LongBridgeSubscription {
    async fn new() -> Self {
        LongBridgeSubscription {}
    }

    async fn quote_real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteRealTimeInfo>, Error> {
        let (longbridge_context, longbridge_receiver) =
            LongBridgeBroker::create_quote_context().await.unwrap();
        let (sys_sender, sys_receiver) = mpsc::channel(64);
        let longbridge_context_ref = Arc::new(Mutex::new(longbridge_context));

        let worker = LongBridgeQuoteRealTimeInfoSubscriptionWorker::new(
            request.symbol.clone(),
            sys_sender,
            longbridge_context_ref.clone(),
            longbridge_receiver,
        );
        let controller = LongBridgeQuoteRealTimeInfoSubscriptionController::new(
            request.symbol.clone(),
            longbridge_context_ref.clone(),
        );
        tokio::task::spawn(worker.start());
        Result::Ok((sys_receiver, Box::new(controller)))
    }

    async fn quote_depth_info(
        &self,
        _request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteDepthInfo>, Error> {
        todo!()
    }
}

#[cfg(test)]
mod test_longbridge_subscription {
    use log;
    use rust_decimal_macros::dec;
    use tokio::time::{sleep, Duration};

    use super::LongBridgeSubscription;
    use crate::{
        broker::common::subscription::SubscriptionTrait,
        model::{
            market::Market,
            quote::{QueryInfoRequest, QuoteKind},
            symbol::Symbol,
        },
    };

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_quote_real_time_info() {
        let longbridge_subscription = LongBridgeSubscription::new().await;
        let (mut receiver, _) = longbridge_subscription
            .quote_real_time_info(QueryInfoRequest {
                symbol: Symbol {
                    market: Market::HK,
                    identifier: "0700".to_owned(),
                },
                kind: QuoteKind::Stock,
            })
            .await
            .unwrap();
        tokio::select! {
            quote_info = receiver.recv() => {
                assert!(quote_info.is_some());
                let quote_info = quote_info.unwrap();
                log::warn!("quote_info: {quote_info:?}");
                assert_eq!("0700.HK", quote_info.symbol.to_string());
                assert!(quote_info.current_price > dec!(0.0));
                assert!(quote_info.volume > 0u64);
                assert!(quote_info.timestamp > 0i64);
            },
            _ = sleep(Duration::from_millis(3000))=> {
                panic!("loop not working!");
            },
        };
    }
}
