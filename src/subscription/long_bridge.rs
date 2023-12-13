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

use super::subscription_trait::Subscription;
use crate::{
    broker::long_bridge::LongBridgeBroker, info::info_trait::InfoContext, model::quote::QuoteInfo,
};

// https://crates.io/crates/longbridge
struct LongBridgeSubscription {
    context: Arc<Mutex<InfoContext>>,
    longbridge_context: Arc<Mutex<QuoteContext>>,
    longbridge_receiver: Arc<Mutex<mpsc::UnboundedReceiver<PushEvent>>>,
}

impl LongBridgeSubscription {
    async fn start_loop(&self, sender: Sender<QuoteInfo>) {
        let identifier = self.get_identifier().await;
        let quote = self.context.lock().await.quote.clone();

        (*self.longbridge_context.lock().await)
            .subscribe([identifier], SubFlags::QUOTE, true)
            .await
            .unwrap();

        while let Some(event_detail) = (*self.longbridge_receiver.lock().await)
            .recv()
            .await
            .map(|event| event.detail)
        {
            match event_detail {
                longbridge::quote::PushEventDetail::Quote(longbridge_quote) => {
                    let timestamp = longbridge_quote.timestamp.unix_timestamp() as u64;
                    let quote_info = QuoteInfo {
                        quote: quote.clone(),
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

    async fn get_identifier(&self) -> String {
        self.context.lock().await.quote.identifier.clone()
    }
}

#[async_trait]
impl Subscription for LongBridgeSubscription {
    async fn new(context: InfoContext) -> Self {
        let (longbridge_context, receiver) = LongBridgeBroker::create_context().await.unwrap();

        LongBridgeSubscription {
            context: Arc::new(Mutex::new(context)),
            longbridge_context: Arc::new(Mutex::new(longbridge_context)),
            longbridge_receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    async fn subscribe(&self) -> Result<mpsc::Receiver<QuoteInfo>, crate::model::error::Error> {
        let (sender, receiver) = mpsc::channel(64);
        self.start_loop(sender); // todo: handle multi threading issue
        Result::Ok(receiver)
    }

    async fn unsubscribe(&self) -> Result<(), crate::model::error::Error> {
        let identifier = self.get_identifier().await;
        self.longbridge_context
            .lock()
            .await
            .unsubscribe([identifier], SubFlags::QUOTE)
            .await
            .unwrap();

        Result::Ok(())
    }
}
