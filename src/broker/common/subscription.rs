use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

use crate::model::{
    error::Error,
    quote::{QueryInfoRequest, QuoteDepthInfo, QuoteRealTimeInfo},
};

#[async_trait]
pub trait SubscriptionTrait {
    async fn new() -> Self
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
pub trait SubscriptionController {
    async fn stop(self) -> Result<(), Error>;
}

#[async_trait]
pub trait SubscriptionWorker {
    async fn start(self);
}

pub type SubscriptionData<T> = (Receiver<T>, Box<dyn SubscriptionController + Send + Sync>);

#[async_trait]
pub trait SubscriptionInterceptorTrait {
    async fn before_real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QueryInfoRequest, Error> {
        Result::Ok(request)
    }
    async fn after_real_time_info(
        &self,
        result: Result<SubscriptionData<QuoteRealTimeInfo>, Error>,
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
        result: Result<SubscriptionData<QuoteDepthInfo>, Error>,
    ) -> Result<SubscriptionData<QuoteDepthInfo>, Error> {
        result
    }
}

pub struct SubscriptionProxy {
    pub shadowed_subscription: Box<dyn SubscriptionTrait + Send + Sync>,
    pub interceptor: Box<dyn SubscriptionInterceptorTrait + Send + Sync>,
}

impl SubscriptionProxy {
    pub fn new(
        shadowed_subscription: Box<dyn SubscriptionTrait + Send + Sync>,
        interceptor_option: Option<Box<dyn SubscriptionInterceptorTrait + Send + Sync>>,
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
    async fn new() -> Self {
        panic!("Cannot Call \"new\" on the proxy method!");
    }

    async fn real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteRealTimeInfo>, Error> {
        match self.interceptor.before_real_time_info(request).await {
            Ok(request) => {
                let result = self.shadowed_subscription.real_time_info(request).await;

                self.interceptor.after_real_time_info(result).await
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
                let result = self.shadowed_subscription.depth_info(request).await;

                self.interceptor.after_depth_info(result).await
            }
            Err(err) => Result::Err(err),
        }
    }
}

pub struct NoOpSubscriptionInterceptor {}

impl SubscriptionInterceptorTrait for NoOpSubscriptionInterceptor {}
