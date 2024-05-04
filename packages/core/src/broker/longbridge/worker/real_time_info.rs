use anyhow::{Context, Error};
use async_trait::async_trait;
use longbridge::{
    quote::{PushEvent, PushQuote, SubFlags},
    QuoteContext,
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    sync::mpsc::{Sender, UnboundedReceiver},
    time::timeout,
};

use crate::{
    broker::common::subscription::{SubscriptionController, SubscriptionWorker},
    model::trading::{quote::QuoteRealTimeInfo, symbol::Symbol},
};

pub struct LongBridgeQuoteRealTimeInfoSubscriptionWorker {
    symbol: Symbol,
    sys_sender: Sender<QuoteRealTimeInfo>,
    longbridge_context: QuoteContext,
    longbridge_receiver: UnboundedReceiver<PushEvent>,
    local_stopped_indicator: Arc<AtomicBool>,
    global_stopped_indicator: Arc<AtomicBool>,
}

impl LongBridgeQuoteRealTimeInfoSubscriptionWorker {
    pub fn new(
        symbol: Symbol,
        sys_sender: Sender<QuoteRealTimeInfo>,
        longbridge_context: QuoteContext,
        longbridge_receiver: UnboundedReceiver<PushEvent>,
        local_stopped_indicator: Arc<AtomicBool>,
        global_stopped_indicator: Arc<AtomicBool>,
    ) -> Self {
        LongBridgeQuoteRealTimeInfoSubscriptionWorker {
            symbol,
            sys_sender,
            longbridge_context,
            longbridge_receiver,
            local_stopped_indicator,
            global_stopped_indicator,
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
    async fn start(mut self) -> Result<(), Error> {
        let symbol_identifier = self.symbol.to_string();
        let sys_sender = self.sys_sender;
        let mut longbridge_receiver = self.longbridge_receiver;
        self.longbridge_context
            .subscribe([symbol_identifier.clone()], SubFlags::QUOTE, true)
            .await
            .with_context(|| {
                format!(
                    "failed to start subscription worker, symbol: {}",
                    self.symbol.to_string()
                )
            })?;

        loop {
            if self.global_stopped_indicator.load(Ordering::Relaxed)
                || self.local_stopped_indicator.load(Ordering::Relaxed)
            {
                self.longbridge_context
                    .unsubscribe([symbol_identifier], SubFlags::QUOTE)
                    .await
                    .with_context(|| {
                        format!(
                            "failed to stop subscription worker, symbol: {}",
                            self.symbol.to_string()
                        )
                    })?;
                return Result::Ok(());
            }

            match timeout(Duration::from_secs(3), longbridge_receiver.recv()).await {
                Err(_) => continue,
                Ok(push_event_optional) => {
                    if let Some(event_detail) = push_event_optional.map(|event| event.detail) {
                        match event_detail {
                            longbridge::quote::PushEventDetail::Quote(longbridge_quote) => {
                                let quote_info = Self::to_quote_real_time_info(
                                    self.symbol.clone(),
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
        }
    }
}

pub struct LongBridgeQuoteRealTimeInfoSubscriptionController {
    local_stopped_indicator: Arc<AtomicBool>,
}

impl LongBridgeQuoteRealTimeInfoSubscriptionController {
    pub fn new(local_stopped_indicator: Arc<AtomicBool>) -> Self {
        LongBridgeQuoteRealTimeInfoSubscriptionController {
            local_stopped_indicator,
        }
    }
}

#[async_trait]
impl SubscriptionController for LongBridgeQuoteRealTimeInfoSubscriptionController {
    async fn stop(self) -> Result<(), Error> {
        self.local_stopped_indicator.store(false, Ordering::Relaxed);
        Result::Ok(())
    }
}
