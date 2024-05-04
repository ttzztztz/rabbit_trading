use anyhow::Error;
use async_trait::async_trait;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::mpsc::Sender;

use crate::{
    broker::common::subscription::{SubscriptionController, SubscriptionWorker},
    model::trading::{quote::QuoteRealTimeInfo, symbol::Symbol},
};

pub struct IBQuoteRealTimeInfoSubscriptionWorker {
    symbol: Symbol,
    sys_sender: Sender<QuoteRealTimeInfo>,
    local_stopped_indicator: Arc<AtomicBool>,
    global_stopped_indicator: Arc<AtomicBool>,
}

impl IBQuoteRealTimeInfoSubscriptionWorker {
    pub fn new(
        symbol: Symbol,
        sys_sender: Sender<QuoteRealTimeInfo>,
        local_stopped_indicator: Arc<AtomicBool>,
        global_stopped_indicator: Arc<AtomicBool>,
    ) -> Self {
        IBQuoteRealTimeInfoSubscriptionWorker {
            symbol,
            sys_sender,
            local_stopped_indicator,
            global_stopped_indicator,
        }
    }
}

#[async_trait]
impl SubscriptionWorker for IBQuoteRealTimeInfoSubscriptionWorker {
    async fn start(mut self) -> Result<(), Error> {
        loop {
            if self.global_stopped_indicator.load(Ordering::Relaxed)
                || self.local_stopped_indicator.load(Ordering::Relaxed)
            {
                return Result::Ok(());
            }
        }
    }
}

pub struct IBQuoteRealTimeInfoSubscriptionController {
    local_stopped_indicator: Arc<AtomicBool>,
}

impl IBQuoteRealTimeInfoSubscriptionController {
    pub fn new(local_stopped_indicator: Arc<AtomicBool>) -> Self {
        IBQuoteRealTimeInfoSubscriptionController {
            local_stopped_indicator,
        }
    }
}

#[async_trait]
impl SubscriptionController for IBQuoteRealTimeInfoSubscriptionController {
    async fn stop(self) -> Result<(), Error> {
        self.local_stopped_indicator.store(false, Ordering::Relaxed);
        Result::Ok(())
    }
}
