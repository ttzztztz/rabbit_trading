use anyhow::Error;
use async_trait::async_trait;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::time::{sleep, Duration};

use crate::broker::{
    common::{
        info::InfoTrait,
        subscription::{SubscriptionController, SubscriptionWorker},
    },
    yahoo_finance::info::YahooFinanceInfo,
};
use crate::model::{
    common::types::ConfigMap,
    trading::quote::{QueryInfoRequest, QuoteRealTimeInfo},
};

pub struct YahooFinanceQuoteRealTimeInfoSubscriptionWorker {
    request: QueryInfoRequest,
    sender: Sender<QuoteRealTimeInfo>,

    local_stopped_indicator: Arc<AtomicBool>,
    global_stopped_indicator: Arc<AtomicBool>,
}

impl YahooFinanceQuoteRealTimeInfoSubscriptionWorker {
    pub fn new(
        request: QueryInfoRequest,
        sender: Sender<QuoteRealTimeInfo>,
        local_stopped_indicator: Arc<AtomicBool>,
        global_stopped_indicator: Arc<AtomicBool>,
    ) -> Self {
        YahooFinanceQuoteRealTimeInfoSubscriptionWorker {
            request,
            sender,
            local_stopped_indicator,
            global_stopped_indicator,
        }
    }
}

#[async_trait]
impl SubscriptionWorker for YahooFinanceQuoteRealTimeInfoSubscriptionWorker {
    async fn start(self) -> Result<(), Error> {
        let info = YahooFinanceInfo::new(ConfigMap::new());

        loop {
            if self.local_stopped_indicator.load(Ordering::Relaxed)
                || self.global_stopped_indicator.load(Ordering::Relaxed)
            {
                return Result::Ok(());
            }

            let real_time_info_result = info.query_real_time_info(self.request.clone()).await;
            if let Result::Ok(quote_info) = real_time_info_result {
                if let Err(send_result_err) = self.sender.send(quote_info).await {
                    log::error!("error when sending into mpsc {}", send_result_err);
                }
            }
            sleep(Duration::from_millis(1000)).await;
        }
    }
}

pub struct YahooFinanceQuoteRealTimeInfoSubscriptionController {
    local_stopped_indicator: Arc<AtomicBool>,
}

impl YahooFinanceQuoteRealTimeInfoSubscriptionController {
    pub fn new(local_stopped_indicator: Arc<AtomicBool>) -> Self {
        YahooFinanceQuoteRealTimeInfoSubscriptionController {
            local_stopped_indicator,
        }
    }
}

#[async_trait]
impl SubscriptionController for YahooFinanceQuoteRealTimeInfoSubscriptionController {
    async fn stop(self) -> Result<(), Error> {
        self.local_stopped_indicator.store(false, Ordering::Relaxed);
        Result::Ok(())
    }
}
