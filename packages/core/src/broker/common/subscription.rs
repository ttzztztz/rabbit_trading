use async_trait::async_trait;
use std::time::{Duration, Instant};
use tokio::sync::mpsc::Receiver;

use crate::model::{
    common::{error::Error, types::ConfigMap},
    trading::quote::{QueryInfoRequest, QuoteDepthInfo, QuoteRealTimeInfo},
};

#[async_trait]
pub trait SubscriptionTrait: Send + Sync {
    async fn new(config_map: ConfigMap) -> Self
    where
        Self: Sized;
    async fn real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteRealTimeInfo>, Error>;
    async fn depth_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteDepthInfo>, Error>;
}

#[async_trait]
pub trait SubscriptionController: Send + Sync {
    async fn stop(self) -> Result<(), Error>;
}

#[async_trait]
pub trait SubscriptionWorker {
    async fn start(self) -> Result<(), Error>;
}

pub type SubscriptionData<T> = (Receiver<T>, Box<dyn SubscriptionController>);

#[async_trait]
pub trait SubscriptionInterceptorTrait: Send + Sync {
    async fn before_real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QueryInfoRequest, Error> {
        Result::Ok(request)
    }
    async fn after_real_time_info(
        &self,
        _request: QueryInfoRequest,
        result: Result<SubscriptionData<QuoteRealTimeInfo>, Error>,
        _duration: Duration,
    ) -> Result<SubscriptionData<QuoteRealTimeInfo>, Error> {
        result
    }

    async fn before_depth_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QueryInfoRequest, Error> {
        Result::Ok(request)
    }
    async fn after_depth_info(
        &self,
        _request: QueryInfoRequest,
        result: Result<SubscriptionData<QuoteDepthInfo>, Error>,
        _duration: Duration,
    ) -> Result<SubscriptionData<QuoteDepthInfo>, Error> {
        result
    }
}

pub struct SubscriptionProxy {
    pub shadowed_subscription: Box<dyn SubscriptionTrait>,
    pub interceptor: Box<dyn SubscriptionInterceptorTrait>,
}

impl SubscriptionProxy {
    pub fn new(
        shadowed_subscription: Box<dyn SubscriptionTrait>,
        interceptor_option: Option<Box<dyn SubscriptionInterceptorTrait>>,
    ) -> Self {
        SubscriptionProxy {
            shadowed_subscription,
            interceptor: match interceptor_option {
                Some(interceptor) => interceptor,
                None => Box::new(NoOpSubscriptionInterceptor {}),
            },
        }
    }
}

#[async_trait]
impl SubscriptionTrait for SubscriptionProxy {
    async fn new(_config_map: ConfigMap) -> Self {
        panic!("Cannot Call \"new\" on the proxy method!");
    }

    async fn real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteRealTimeInfo>, Error> {
        match self.interceptor.before_real_time_info(request).await {
            Ok(request) => {
                let instant = Instant::now();
                let result = self
                    .shadowed_subscription
                    .real_time_info(request.clone())
                    .await;
                let duration = instant.elapsed();
                self.interceptor
                    .after_real_time_info(request, result, duration)
                    .await
            }
            Err(err) => Result::Err(err),
        }
    }

    async fn depth_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteDepthInfo>, Error> {
        match self.interceptor.before_depth_info(request).await {
            Ok(request) => {
                let instant = Instant::now();
                let result = self.shadowed_subscription.depth_info(request.clone()).await;
                let duration = instant.elapsed();
                self.interceptor
                    .after_depth_info(request, result, duration)
                    .await
            }
            Err(err) => Result::Err(err),
        }
    }
}

pub struct NoOpSubscriptionInterceptor {}

impl SubscriptionInterceptorTrait for NoOpSubscriptionInterceptor {}
