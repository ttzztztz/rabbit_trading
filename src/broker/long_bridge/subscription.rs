use async_trait::async_trait;
use longbridge::{
    quote::{PushEvent, SubFlags},
    QuoteContext,
};
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Sender},
    Mutex,
};

use super::broker::LongBridgeBroker;
use crate::{
    broker::common::{info::InfoContext, subscription::SubscriptionTrait},
    model::{error::Error, quote::QuoteInfo},
};

// https://crates.io/crates/longbridge
pub(super) struct LongBridgeSubscription {
    context: InfoContext,
    longbridge_context: Arc<Mutex<Option<QuoteContext>>>,
}

impl LongBridgeSubscription {
    async fn start_loop(
        info_context: InfoContext,
        longbridge_context: QuoteContext,
        sender: Sender<QuoteInfo>,
        mut long_bridge_receiver: mpsc::UnboundedReceiver<PushEvent>,
    ) {
        let symbol = info_context.symbol.clone();
        let symbol_identifier = info_context.symbol.to_string();
        longbridge_context
            .subscribe([symbol_identifier], SubFlags::QUOTE, true)
            .await
            .unwrap();

        while let Some(event_detail) = long_bridge_receiver.recv().await.map(|event| event.detail) {
            match event_detail {
                longbridge::quote::PushEventDetail::Quote(longbridge_quote) => {
                    let timestamp = longbridge_quote.timestamp.unix_timestamp() as u64;
                    let quote_info = QuoteInfo {
                        symbol: symbol.clone(),
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
                    };

                    if let Err(send_result_err) = sender.send(quote_info).await {
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

#[async_trait]
impl SubscriptionTrait for LongBridgeSubscription {
    async fn new(context: InfoContext) -> Self {
        LongBridgeSubscription {
            context,
            longbridge_context: Arc::new(Mutex::new(Option::None)),
        }
    }

    async fn subscribe(&self) -> Result<mpsc::Receiver<QuoteInfo>, Error> {
        let (sender, receiver) = mpsc::channel(64);
        let (longbridge_context, long_bridge_receiver) =
            LongBridgeBroker::create_quote_context().await.unwrap();
        let ctx = longbridge_context.clone();
        *self.longbridge_context.lock().await = Option::Some(longbridge_context);

        tokio::task::spawn(Self::start_loop(
            self.context.clone(),
            ctx.clone(),
            sender,
            long_bridge_receiver,
        ));
        Result::Ok(receiver)
    }

    async fn unsubscribe(&self) -> Result<(), Error> {
        let symbol_identifier = self.context.symbol.to_string();
        let longbridge_context_lock = self.longbridge_context.lock().await;
        if let Some(ctx) = longbridge_context_lock.as_ref() {
            ctx.unsubscribe([symbol_identifier], SubFlags::QUOTE)
                .await
                .unwrap();
        }
        Result::Ok(())
    }
}

#[cfg(test)]
mod test_long_bridge_subscription {
    use log;
    use rust_decimal_macros::dec;
    use tokio::time::{sleep, Duration};

    use super::LongBridgeSubscription;
    use crate::{
        broker::common::{info::InfoContext, subscription::SubscriptionTrait},
        model::{market::Market, symbol::Symbol},
    };

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_query_quote_info() {
        let long_bridge_subscription = LongBridgeSubscription::new(InfoContext {
            symbol: Symbol {
                identifier: "0700".to_owned(),
                market: Market::HK,
            },
            extra: Option::None,
        })
        .await;

        let mut receiver = long_bridge_subscription.subscribe().await.unwrap();
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
