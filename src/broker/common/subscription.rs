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
    async fn quote_real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteRealTimeInfo>, Error>;
    async fn quote_depth_info(
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
    async fn before_quote_real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QueryInfoRequest, Error>;
    async fn after_quote_real_time_info(
        &self,
        result: SubscriptionData<QuoteRealTimeInfo>,
    ) -> Result<SubscriptionData<QuoteRealTimeInfo>, Error>;

    async fn before_quote_depth_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QueryInfoRequest, Error>;
    async fn after_quote_depth_info(
        &self,
        result: SubscriptionData<QuoteDepthInfo>,
    ) -> Result<SubscriptionData<QuoteDepthInfo>, Error>;
}

pub struct SubscriptionReflection {
    pub shadowed_subscription: Box<dyn SubscriptionTrait + Send + Sync>,
    pub interceptor: Box<dyn SubscriptionInterceptorTrait + Send + Sync>,
}

impl SubscriptionReflection {
    pub fn new(
        shadowed_subscription: Box<dyn SubscriptionTrait + Send + Sync>,
        interceptor: Option<Box<dyn SubscriptionInterceptorTrait + Send + Sync>>,
    ) -> Self {
        SubscriptionReflection {
            shadowed_subscription,
            interceptor: match interceptor {
                Some(interceptor) => interceptor,
                None => Box::new(NoOpSubscriptionInterceptor {}),
            },
        }
    }
}

pub struct NoOpSubscriptionInterceptor {}

#[async_trait]
impl SubscriptionInterceptorTrait for NoOpSubscriptionInterceptor {
    async fn before_quote_real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QueryInfoRequest, Error> {
        Result::Ok(request)
    }

    async fn after_quote_real_time_info(
        &self,
        result: SubscriptionData<QuoteRealTimeInfo>,
    ) -> Result<SubscriptionData<QuoteRealTimeInfo>, Error> {
        Result::Ok(result)
    }

    async fn before_quote_depth_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QueryInfoRequest, Error> {
        Result::Ok(request)
    }

    async fn after_quote_depth_info(
        &self,
        result: SubscriptionData<QuoteDepthInfo>,
    ) -> Result<SubscriptionData<QuoteDepthInfo>, Error> {
        Result::Ok(result)
    }
}
