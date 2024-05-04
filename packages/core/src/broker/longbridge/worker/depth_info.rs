use anyhow::{Context, Error};
use async_trait::async_trait;
use longbridge::{
    quote::{PushDepth, PushEvent, SubFlags},
    QuoteContext,
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::{
    mpsc::{Sender, UnboundedReceiver},
    Mutex,
};

use crate::{
    broker::{
        common::subscription::{SubscriptionController, SubscriptionWorker},
        longbridge::info::LongBridgeInfo,
    },
    model::trading::{quote::QuoteDepthInfo, symbol::Symbol},
    utils::time::get_now_unix_timestamp,
};

pub struct LongBridgeQuoteDepthInfoSubscriptionWorker {
    symbol: Symbol,
    sys_sender: Sender<QuoteDepthInfo>,
    longbridge_context: Arc<Mutex<QuoteContext>>,
    longbridge_receiver: UnboundedReceiver<PushEvent>,
    local_stopped_indicator: Arc<AtomicBool>,
    global_stopped_indicator: Arc<AtomicBool>,
}

impl LongBridgeQuoteDepthInfoSubscriptionWorker {
    pub fn new(
        symbol: Symbol,
        sys_sender: Sender<QuoteDepthInfo>,
        longbridge_context: Arc<Mutex<QuoteContext>>,
        longbridge_receiver: UnboundedReceiver<PushEvent>,
        local_stopped_indicator: Arc<AtomicBool>,
        global_stopped_indicator: Arc<AtomicBool>,
    ) -> Self {
        LongBridgeQuoteDepthInfoSubscriptionWorker {
            symbol,
            sys_sender,
            longbridge_context,
            longbridge_receiver,
            local_stopped_indicator,
            global_stopped_indicator,
        }
    }

    pub(super) fn to_quote_depth_info(
        symbol: Symbol,
        longbridge_depth: PushDepth,
    ) -> QuoteDepthInfo {
        let current_timestamp = get_now_unix_timestamp();

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
    async fn start(mut self) -> Result<(), Error> {
        let symbol_identifier = self.symbol.to_string();
        let sys_sender = self.sys_sender;
        let mut longbridge_receiver = self.longbridge_receiver;
        self.longbridge_context
            .lock()
            .await
            .subscribe([symbol_identifier], SubFlags::DEPTH, true)
            .await
            .with_context(|| {
                format!(
                    "Error when starting subscription, {:?}",
                    self.symbol.to_string()
                )
            })?;

        while let Some(event_detail) = longbridge_receiver.recv().await.map(|event| event.detail) {
            if self.global_stopped_indicator.load(Ordering::Relaxed)
                || self.local_stopped_indicator.load(Ordering::Relaxed)
            {
                return Result::Ok(());
            }

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

        Result::Ok(())
    }
}

pub struct LongBridgeQuoteDepthInfoSubscriptionController {
    symbol: Symbol,
    longbridge_context: Arc<Mutex<QuoteContext>>,
    local_stopped_indicator: Arc<AtomicBool>,
}

impl LongBridgeQuoteDepthInfoSubscriptionController {
    pub fn new(
        symbol: Symbol,
        longbridge_context: Arc<Mutex<QuoteContext>>,
        local_stopped_indicator: Arc<AtomicBool>,
    ) -> Self {
        LongBridgeQuoteDepthInfoSubscriptionController {
            symbol,
            longbridge_context,
            local_stopped_indicator,
        }
    }
}

#[async_trait]
impl SubscriptionController for LongBridgeQuoteDepthInfoSubscriptionController {
    async fn stop(self) -> Result<(), Error> {
        self.local_stopped_indicator.store(false, Ordering::Relaxed);

        let symbol_identifier = self.symbol.to_string();
        self.longbridge_context
            .lock()
            .await
            .unsubscribe([symbol_identifier], SubFlags::QUOTE)
            .await
            .with_context(|| {
                format!(
                    "Error when stopping the subscription {}",
                    self.symbol.to_string()
                )
            })
    }
}
