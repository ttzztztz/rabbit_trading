use anyhow::Error;
use async_trait::async_trait;
use std::{collections::HashMap, time::Duration};

use crate::{
    broker::common::info::InfoInterceptorTrait,
    metrics::common::registry::MetricRegistryTrait,
    model::trading::quote::{QueryInfoRequest, QuoteBasicInfo, QuoteDepthInfo, QuoteRealTimeInfo},
    pod::event::event_bus::EventBus,
};

pub struct PodInfoInterceptor {
    event_bus: EventBus,
    metric_registry: Box<dyn MetricRegistryTrait>,
}

impl PodInfoInterceptor {
    pub fn new(event_bus: EventBus, metric_registry: Box<dyn MetricRegistryTrait>) -> Self {
        PodInfoInterceptor {
            event_bus,
            metric_registry,
        }
    }
}

#[async_trait]
impl InfoInterceptorTrait for PodInfoInterceptor {
    async fn after_query_basic_info(
        &self,
        _request: QueryInfoRequest,
        result: Result<QuoteBasicInfo, Error>,
        duration: Duration,
    ) -> Result<QuoteBasicInfo, Error> {
        self.metric_registry
            .timer(
                "system.pod.counter".to_owned(),
                HashMap::from([
                    ("component".to_owned(), "info".to_owned()),
                    ("method".to_owned(), "query_basic_info".to_owned()),
                    ("is_success".to_owned(), result.is_ok().to_string()),
                ]),
                duration,
            )
            .await;

        result
    }

    async fn after_query_real_time_info(
        &self,
        _request: QueryInfoRequest,
        result: Result<QuoteRealTimeInfo, Error>,
        duration: Duration,
    ) -> Result<QuoteRealTimeInfo, Error> {
        self.metric_registry
            .timer(
                "system.pod.counter".to_owned(),
                HashMap::from([
                    ("component".to_owned(), "info".to_owned()),
                    ("method".to_owned(), "query_real_time_info".to_owned()),
                    ("is_success".to_owned(), result.is_ok().to_string()),
                ]),
                duration,
            )
            .await;

        result
    }

    async fn after_query_depth(
        &self,
        _request: QueryInfoRequest,
        result: Result<QuoteDepthInfo, Error>,
        duration: Duration,
    ) -> Result<QuoteDepthInfo, Error> {
        self.metric_registry
            .timer(
                "system.pod.counter".to_owned(),
                HashMap::from([
                    ("component".to_owned(), "info".to_owned()),
                    ("method".to_owned(), "query_depth".to_owned()),
                    ("is_success".to_owned(), result.is_ok().to_string()),
                ]),
                duration,
            )
            .await;

        result
    }
}
