use async_trait::async_trait;

use crate::model::common::types::ConfigMap;

use super::{
    info::{InfoInterceptorTrait, InfoTrait},
    subscription::{SubscriptionInterceptorTrait, SubscriptionTrait},
    transaction::{TransactionInterceptorTrait, TransactionTrait},
};

#[async_trait]
pub trait BrokerTrait: Send + Sync {
    fn new(
        interceptor_factory: Box<dyn BrokerInterceptorFactoryTrait>,
        config_map: ConfigMap,
    ) -> Self
    where
        Self: Sized;
    fn get_identifier() -> String
    where
        Self: Sized;

    async fn create_info(&self) -> Box<dyn InfoTrait>;
    async fn create_subscription(&self) -> Box<dyn SubscriptionTrait>;
    async fn create_transaction(&self) -> Box<dyn TransactionTrait>;
}

pub trait BrokerInterceptorFactoryTrait: Send + Sync {
    fn create_info_interceptor(&self) -> Option<Box<dyn InfoInterceptorTrait>>;
    fn create_subscription_interceptor(&self) -> Option<Box<dyn SubscriptionInterceptorTrait>>;
    fn create_transaction_interceptor(&self) -> Option<Box<dyn TransactionInterceptorTrait>>;
}

pub struct EmptyBrokerInterceptorFactory {}

impl EmptyBrokerInterceptorFactory {
    pub fn new() -> Self {
        EmptyBrokerInterceptorFactory {}
    }
}

impl BrokerInterceptorFactoryTrait for EmptyBrokerInterceptorFactory {
    fn create_info_interceptor(&self) -> Option<Box<dyn InfoInterceptorTrait>> {
        Option::None
    }

    fn create_subscription_interceptor(&self) -> Option<Box<dyn SubscriptionInterceptorTrait>> {
        Option::None
    }

    fn create_transaction_interceptor(&self) -> Option<Box<dyn TransactionInterceptorTrait>> {
        Option::None
    }
}
