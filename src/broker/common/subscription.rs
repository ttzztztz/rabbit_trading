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
    async fn stop(&self) -> Result<(), Error>;
}

#[async_trait]
pub trait SubscriptionWorker {
    async fn start(self);
}

pub type SubscriptionData<T> = (Receiver<T>, Box<dyn SubscriptionController + Send + Sync>);
