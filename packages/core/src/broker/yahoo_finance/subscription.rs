use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use super::worker::real_time_info::{
    YahooFinanceQuoteRealTimeInfoSubscriptionController,
    YahooFinanceQuoteRealTimeInfoSubscriptionWorker,
};
use crate::broker::common::{
    subscription::SubscriptionTrait,
    subscription::{SubscriptionData, SubscriptionWorker},
};
use crate::model::{
    common::{error::Error, types::ConfigMap},
    trading::quote::{QueryInfoRequest, QuoteDepthInfo, QuoteRealTimeInfo},
};

pub struct YahooFinanceSubscription {
    config_map: ConfigMap,
}

#[async_trait]
impl SubscriptionTrait for YahooFinanceSubscription {
    async fn new(config_map: ConfigMap) -> Self {
        YahooFinanceSubscription { config_map }
    }

    async fn real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteRealTimeInfo>, Error> {
        let (sender, receiver) = mpsc::channel(64);

        let working_flag = Arc::new(Mutex::new(true));
        let worker = YahooFinanceQuoteRealTimeInfoSubscriptionWorker::new(
            request,
            sender,
            working_flag.clone(),
        );
        let controller =
            YahooFinanceQuoteRealTimeInfoSubscriptionController::new(working_flag.clone());

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
