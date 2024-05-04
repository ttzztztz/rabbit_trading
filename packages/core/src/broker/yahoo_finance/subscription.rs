use anyhow::Error;
use async_trait::async_trait;
use std::sync::{atomic::AtomicBool, Arc};
use tokio::sync::mpsc;

use super::worker::real_time_info::{
    YahooFinanceQuoteRealTimeInfoSubscriptionController,
    YahooFinanceQuoteRealTimeInfoSubscriptionWorker,
};
use crate::broker::common::{
    subscription::SubscriptionTrait,
    subscription::{SubscriptionData, SubscriptionWorker},
};
use crate::model::{
    common::types::ConfigMap,
    trading::quote::{QueryInfoRequest, QuoteDepthInfo, QuoteRealTimeInfo},
};

pub struct YahooFinanceSubscription {
    config_map: ConfigMap,
    global_stopped_indicator: Arc<AtomicBool>,
}

#[async_trait]
impl SubscriptionTrait for YahooFinanceSubscription {
    fn new(config_map: ConfigMap, global_stopped_indicator: Arc<AtomicBool>) -> Self {
        YahooFinanceSubscription {
            config_map,
            global_stopped_indicator,
        }
    }

    async fn real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteRealTimeInfo>, Error> {
        let (sender, receiver) = mpsc::channel(64);

        let local_stopped_indicator = Arc::new(AtomicBool::new(false));
        let worker = YahooFinanceQuoteRealTimeInfoSubscriptionWorker::new(
            request,
            sender,
            local_stopped_indicator.clone(),
            self.global_stopped_indicator.clone(),
        );
        let controller = YahooFinanceQuoteRealTimeInfoSubscriptionController::new(
            local_stopped_indicator.clone(),
        );

        tokio::task::spawn(worker.start());
        Result::Ok((receiver, Box::new(controller)))
    }

    async fn depth_info(
        &self,
        _request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteDepthInfo>, Error> {
        todo!()
    }
}
