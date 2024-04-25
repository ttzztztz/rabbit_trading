use anyhow::{Context, Error};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

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
pub struct LongBridgeSubscription {}

impl LongBridgeSubscription {}

#[async_trait]
impl SubscriptionTrait for LongBridgeSubscription {
    async fn new(_config_map: ConfigMap) -> Self {
        LongBridgeSubscription {}
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
        let longbridge_context_ref = Arc::new(Mutex::new(longbridge_context));

        let worker = LongBridgeQuoteRealTimeInfoSubscriptionWorker::new(
            request.symbol.clone(),
            sys_sender,
            longbridge_context_ref.clone(),
            longbridge_receiver,
        );
        let controller = LongBridgeQuoteRealTimeInfoSubscriptionController::new(
            request.symbol.clone(),
            longbridge_context_ref.clone(),
        );
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
        let longbridge_context_ref = Arc::new(Mutex::new(longbridge_context));

        let worker = LongBridgeQuoteDepthInfoSubscriptionWorker::new(
            request.symbol.clone(),
            sys_sender,
            longbridge_context_ref.clone(),
            longbridge_receiver,
        );
        let controller = LongBridgeQuoteDepthInfoSubscriptionController::new(
            request.symbol.clone(),
            longbridge_context_ref.clone(),
        );
        tokio::task::spawn(worker.start());
        Result::Ok((sys_receiver, Box::new(controller)))
    }
}
