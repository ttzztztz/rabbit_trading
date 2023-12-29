use async_trait::async_trait;

use super::{
    info::{InfoInterceptorTrait, InfoTrait},
    subscription::{SubscriptionInterceptorTrait, SubscriptionTrait},
    transaction::{TransactionInterceptorTrait, TransactionTrait},
};

#[async_trait]
pub trait BrokerTrait {
    fn get_broker_identifier(&self) -> String;

    async fn create_info(
        &self,
        interceptor: Option<Box<dyn InfoInterceptorTrait + Send + Sync>>,
    ) -> Box<dyn InfoTrait + Send + Sync>;
    async fn create_subscription(
        &self,
        interceptor: Option<Box<dyn SubscriptionInterceptorTrait + Send + Sync>>,
    ) -> Box<dyn SubscriptionTrait + Send + Sync>;
    async fn create_transaction(
        &self,
        interceptor: Option<Box<dyn TransactionInterceptorTrait + Send + Sync>>,
    ) -> Box<dyn TransactionTrait + Send + Sync>;
}
