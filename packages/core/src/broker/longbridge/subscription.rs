use anyhow::{Context, Error};
use async_trait::async_trait;
use std::sync::{atomic::AtomicBool, Arc};
use tokio::sync::mpsc;

use super::{
    broker::LongBridgeBroker,
    worker::{
        depth_info::{
            LongBridgeQuoteDepthInfoSubscriptionController,
            LongBridgeQuoteDepthInfoSubscriptionWorker,
        },
        real_time_info::{
            LongBridgeQuoteRealTimeInfoSubscriptionController,
            LongBridgeQuoteRealTimeInfoSubscriptionWorker,
        },
    },
};
use crate::{
    broker::common::subscription::{SubscriptionData, SubscriptionTrait, SubscriptionWorker},
    model::{
        common::types::ConfigMap,
        trading::quote::{QueryInfoRequest, QuoteDepthInfo, QuoteRealTimeInfo},
    },
};

// https://crates.io/crates/longbridge
pub struct LongBridgeSubscription {
    pub global_stopped_indicator: Arc<AtomicBool>,
}

impl LongBridgeSubscription {}

#[async_trait]
impl SubscriptionTrait for LongBridgeSubscription {
    fn new(_config_map: ConfigMap, global_stopped_indicator: Arc<AtomicBool>) -> Self {
        LongBridgeSubscription {
            global_stopped_indicator,
        }
    }

    async fn real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteRealTimeInfo>, Error> {
        let (longbridge_context, longbridge_receiver) = LongBridgeBroker::create_quote_context()
            .await
            .with_context(|| {
                format!(
                    "error when subscripting real_time_info request {:?}",
                    request
                )
            })?;
        let (sys_sender, sys_receiver) = mpsc::channel(64);

        let local_stopped_indicator = Arc::new(AtomicBool::new(false));
        let worker = LongBridgeQuoteRealTimeInfoSubscriptionWorker::new(
            request.symbol.clone(),
            sys_sender,
            longbridge_context,
            longbridge_receiver,
            local_stopped_indicator.clone(),
            self.global_stopped_indicator.clone(),
        );
        let controller =
            LongBridgeQuoteRealTimeInfoSubscriptionController::new(local_stopped_indicator);
        tokio::task::spawn(worker.start());
        Result::Ok((sys_receiver, Box::new(controller)))
    }

    async fn depth_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteDepthInfo>, Error> {
        let (longbridge_context, longbridge_receiver) = LongBridgeBroker::create_quote_context()
            .await
            .with_context(|| format!("error when subscripting depth_info request {:?}", request))?;
        let (sys_sender, sys_receiver) = mpsc::channel(64);

        let local_stopped_indicator = Arc::new(AtomicBool::new(false));
        let worker = LongBridgeQuoteDepthInfoSubscriptionWorker::new(
            request.symbol,
            sys_sender,
            longbridge_context,
            longbridge_receiver,
            local_stopped_indicator.clone(),
            self.global_stopped_indicator.clone(),
        );
        let controller =
            LongBridgeQuoteDepthInfoSubscriptionController::new(local_stopped_indicator.clone());
        tokio::task::spawn(worker.start());
        Result::Ok((sys_receiver, Box::new(controller)))
    }
}
