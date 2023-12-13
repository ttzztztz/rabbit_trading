use longbridge::{
    quote::{PushEvent, SubFlags},
    QuoteContext,
};
use std::cell::RefCell;
use tokio::sync::mpsc::{self, Sender};

use super::subscription_trait::Subscription;
use crate::{
    broker::long_bridge::LongBridgeBroker, info::info_trait::InfoContext, model::quote::QuoteInfo,
};

// https://crates.io/crates/longbridge
struct LongBridgeSubscription {
    context: InfoContext,
    longbridge_context: QuoteContext,
    longbridge_receiver: RefCell<mpsc::UnboundedReceiver<PushEvent>>,
}

impl LongBridgeSubscription {
    async fn start_loop(&self, sender: Sender<QuoteInfo>) {
        let identifier = self.get_identifier();
        self.longbridge_context
            .subscribe([identifier], SubFlags::QUOTE, true)
            .await
            .unwrap();

        while let Some(event_detail) = self
            .longbridge_receiver
            .borrow_mut()
            .recv()
            .await
            .map(|event| event.detail)
        {
            match event_detail {
                longbridge::quote::PushEventDetail::Quote(longbridge_quote) => {
                    let timestamp = longbridge_quote.timestamp.unix_timestamp() as u64;
                    let quote_info = QuoteInfo {
                        quote: self.context.quote.clone(),
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

    fn get_identifier(&self) -> String {
        self.context.quote.identifier.clone()
    }
}

impl Subscription for LongBridgeSubscription {
    fn new(context: InfoContext) -> Self {
        let (ctx, receiver) = context
            .runtime
            .block_on(LongBridgeBroker::create_context())
            .unwrap();

        LongBridgeSubscription {
            context,
            longbridge_context: ctx,
            longbridge_receiver: RefCell::new(receiver),
        }
    }

    fn subscribe(&self) -> Result<mpsc::Receiver<QuoteInfo>, crate::model::error::Error> {
        let (sender, receiver) = mpsc::channel(64);
        self.start_loop(sender); // todo: handle multi threading issue
        Result::Ok(receiver)
    }

    fn unsubscribe(&self) -> Result<(), crate::model::error::Error> {
        let identifier = self.get_identifier();
        self.context
            .runtime
            .block_on(
                self.longbridge_context
                    .unsubscribe([identifier], SubFlags::QUOTE),
            )
            .unwrap();

        Result::Ok(())
    }
}
