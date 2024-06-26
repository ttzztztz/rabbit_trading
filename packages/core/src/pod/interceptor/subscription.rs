use anyhow::Error;
use async_trait::async_trait;
use std::{collections::HashMap, time::Duration};

use crate::{
    broker::common::subscription::{SubscriptionData, SubscriptionInterceptorTrait},
    metrics::common::registry::MetricRegistryTrait,
    model::trading::quote::{QueryInfoRequest, QuoteDepthInfo, QuoteRealTimeInfo},
};

pub struct PodSubscriptionInterceptor {
    metric_registry: Box<dyn MetricRegistryTrait>,
}

impl PodSubscriptionInterceptor {
    pub fn new(metric_registry: Box<dyn MetricRegistryTrait>) -> Self {
        PodSubscriptionInterceptor { metric_registry }
    }
}

#[async_trait]
impl SubscriptionInterceptorTrait for PodSubscriptionInterceptor {
    async fn after_real_time_info(
        &self,
        _request: QueryInfoRequest,
        result: Result<SubscriptionData<QuoteRealTimeInfo>, Error>,
        duration: Duration,
    ) -> Result<SubscriptionData<QuoteRealTimeInfo>, Error> {
        self.metric_registry
            .timer(
                "system.pod.counter".to_owned(),
                HashMap::from([
                    ("component".to_owned(), "subscription".to_owned()),
                    ("method".to_owned(), "real_time_info".to_owned()),
                    ("is_success".to_owned(), result.is_ok().to_string()),
                ]),
                duration,
            )
            .await;

        result
    }

    async fn after_depth_info(
        &self,
        _request: QueryInfoRequest,
        result: Result<SubscriptionData<QuoteDepthInfo>, Error>,
        duration: Duration,
    ) -> Result<SubscriptionData<QuoteDepthInfo>, Error> {
        self.metric_registry
            .timer(
                "system.pod.counter".to_owned(),
                HashMap::from([
                    ("component".to_owned(), "subscription".to_owned()),
                    ("method".to_owned(), "depth_info".to_owned()),
                    ("is_success".to_owned(), result.is_ok().to_string()),
                ]),
                duration,
            )
            .await;

        result
    }
}
