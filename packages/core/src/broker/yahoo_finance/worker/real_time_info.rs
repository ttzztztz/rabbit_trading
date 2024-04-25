use anyhow::Error;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::{mpsc::Sender, Mutex};
use tokio::time::{sleep, Duration};

use crate::broker::common::info::InfoTrait;
use crate::broker::{
    common::subscription::{SubscriptionController, SubscriptionWorker},
    yahoo_finance::info::YahooFinanceInfo,
};
use crate::model::common::types::ConfigMap;
use crate::model::trading::quote::{QueryInfoRequest, QuoteRealTimeInfo};

pub struct YahooFinanceQuoteRealTimeInfoSubscriptionWorker {
    request: QueryInfoRequest,
    sender: Sender<QuoteRealTimeInfo>,

    working_flag: Arc<Mutex<bool>>,
}

impl YahooFinanceQuoteRealTimeInfoSubscriptionWorker {
    pub fn new(
        request: QueryInfoRequest,
        sender: Sender<QuoteRealTimeInfo>,
        working_flag: Arc<Mutex<bool>>,
    ) -> Self {
        YahooFinanceQuoteRealTimeInfoSubscriptionWorker {
            request,
            sender,
            working_flag,
        }
    }
}

#[async_trait]
impl SubscriptionWorker for YahooFinanceQuoteRealTimeInfoSubscriptionWorker {
    async fn start(self) -> Result<(), Error> {
        let info = YahooFinanceInfo::new(ConfigMap::new()).await;

        loop {
            if *self.working_flag.lock().await == false {
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
    working_flag: Arc<Mutex<bool>>,
}

impl YahooFinanceQuoteRealTimeInfoSubscriptionController {
    pub fn new(working_flag: Arc<Mutex<bool>>) -> Self {
        YahooFinanceQuoteRealTimeInfoSubscriptionController { working_flag }
    }
}

#[async_trait]
impl SubscriptionController for YahooFinanceQuoteRealTimeInfoSubscriptionController {
    async fn stop(self) -> Result<(), Error> {
        *self.working_flag.lock().await = false;
        Result::Ok(())
    }
}
