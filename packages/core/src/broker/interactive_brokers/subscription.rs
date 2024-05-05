use anyhow::Error;
use async_trait::async_trait;
use std::sync::{atomic::AtomicBool, Arc};
use tokio::sync::mpsc;

use super::worker::{
    depth_info::{IBQuoteDepthInfoSubscriptionController, IBQuoteDepthInfoSubscriptionWorker},
    real_time_info::{
        IBQuoteRealTimeInfoSubscriptionController, IBQuoteRealTimeInfoSubscriptionWorker,
    },
};
use crate::{
    broker::common::subscription::{SubscriptionData, SubscriptionTrait, SubscriptionWorker},
    model::{
        common::types::ConfigMap,
        trading::quote::{QueryInfoRequest, QuoteDepthInfo, QuoteRealTimeInfo},
    },
};

pub struct InteractiveBrokersSubscription {
    config_map: ConfigMap,
    global_stopped_indicator: Arc<AtomicBool>,
}

#[async_trait]
impl SubscriptionTrait for InteractiveBrokersSubscription {
    fn new(config_map: ConfigMap, global_stopped_indicator: Arc<AtomicBool>) -> Self {
        InteractiveBrokersSubscription {
            config_map,
            global_stopped_indicator,
        }
    }

    async fn real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteRealTimeInfo>, Error> {
        let (sys_sender, sys_receiver) = mpsc::channel(64);

        let local_stopped_indicator = Arc::new(AtomicBool::new(false));
        let worker = IBQuoteRealTimeInfoSubscriptionWorker::new(
            self.config_map.clone(),
            request.symbol.clone(),
            sys_sender,
            local_stopped_indicator.clone(),
            self.global_stopped_indicator.clone(),
        );
        let controller = IBQuoteRealTimeInfoSubscriptionController::new(local_stopped_indicator);
        tokio::task::spawn(worker.start());
        Result::Ok((sys_receiver, Box::new(controller)))
    }

    async fn depth_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteDepthInfo>, Error> {
        let (sys_sender, sys_receiver) = mpsc::channel(64);

        let local_stopped_indicator = Arc::new(AtomicBool::new(false));
        let worker = IBQuoteDepthInfoSubscriptionWorker::new(
            self.config_map.clone(),
            request.symbol.clone(),
            sys_sender,
            local_stopped_indicator.clone(),
            self.global_stopped_indicator.clone(),
        );
        let controller = IBQuoteDepthInfoSubscriptionController::new(local_stopped_indicator);
        tokio::task::spawn(worker.start());
        Result::Ok((sys_receiver, Box::new(controller)))
    }
}
