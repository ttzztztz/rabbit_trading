use async_trait::async_trait;
use longbridge::{
    quote::{PushDepth, PushEvent, SubFlags},
    QuoteContext,
};
use std::{sync::Arc, time::SystemTime};
use tokio::sync::{
    mpsc::{Sender, UnboundedReceiver},
    Mutex,
};

use crate::{
    broker::{
        common::subscription::{SubscriptionController, SubscriptionWorker},
        longbridge::{broker::LongBridgeBroker, info::LongBridgeInfo},
    },
    model::{error::Error, quote::QuoteDepthInfo, symbol::Symbol},
};

pub struct LongBridgeQuoteDepthInfoSubscriptionWorker {
    symbol: Symbol,
    sys_sender: Sender<QuoteDepthInfo>,
    longbridge_context: Arc<Mutex<QuoteContext>>,
    longbridge_receiver: UnboundedReceiver<PushEvent>,
}

impl LongBridgeQuoteDepthInfoSubscriptionWorker {
    pub fn new(
        symbol: Symbol,
        sys_sender: Sender<QuoteDepthInfo>,
        longbridge_context: Arc<Mutex<QuoteContext>>,
        longbridge_receiver: UnboundedReceiver<PushEvent>,
    ) -> Self {
        LongBridgeQuoteDepthInfoSubscriptionWorker {
            symbol,
            sys_sender,
            longbridge_context,
            longbridge_receiver,
        }
    }

    pub(super) fn to_quote_depth_info(
        symbol: Symbol,
        longbridge_depth: PushDepth,
    ) -> QuoteDepthInfo {
        let current_timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        QuoteDepthInfo {
            symbol,
            sequence: current_timestamp,
            timestamp: current_timestamp,
            ask_list: longbridge_depth
                .asks
                .into_iter()
                .map(LongBridgeInfo::to_depth)
                .collect(),
            bid_list: longbridge_depth
                .bids
                .into_iter()
                .map(LongBridgeInfo::to_depth)
                .collect(),
        }
    }
}

#[async_trait]
impl SubscriptionWorker for LongBridgeQuoteDepthInfoSubscriptionWorker {
    async fn start(mut self) {
        let symbol_identifier = self.symbol.to_string();
        let sys_sender = self.sys_sender;
        let mut longbridge_receiver = self.longbridge_receiver;
        self.longbridge_context
            .lock()
            .await
            .subscribe([symbol_identifier], SubFlags::DEPTH, true)
            .await
            .unwrap();

        while let Some(event_detail) = longbridge_receiver.recv().await.map(|event| event.detail) {
            match event_detail {
                longbridge::quote::PushEventDetail::Depth(longbridge_depth) => {
                    let depth_info =
                        Self::to_quote_depth_info(self.symbol.clone(), longbridge_depth);
                    if let Err(send_result_err) = sys_sender.send(depth_info).await {
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

pub struct LongBridgeQuoteDepthInfoSubscriptionController {
    symbol: Symbol,
    longbridge_context: Arc<Mutex<QuoteContext>>,
}

impl LongBridgeQuoteDepthInfoSubscriptionController {
    pub fn new(symbol: Symbol, longbridge_context: Arc<Mutex<QuoteContext>>) -> Self {
        LongBridgeQuoteDepthInfoSubscriptionController {
            symbol,
            longbridge_context,
        }
    }
}

#[async_trait]
impl SubscriptionController for LongBridgeQuoteDepthInfoSubscriptionController {
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
