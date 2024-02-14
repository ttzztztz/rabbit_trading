use async_trait::async_trait;
use std::sync::{atomic::AtomicBool, Arc};

use super::{
    heartbeat::HeartbeatTrait,
    info::{InfoInterceptorTrait, InfoTrait},
    subscription::{SubscriptionInterceptorTrait, SubscriptionTrait},
    transaction::{TransactionInterceptorTrait, TransactionTrait},
};
use crate::model::common::types::ConfigMap;

#[async_trait]
pub trait BrokerTrait: Send + Sync {
    fn new(
        interceptor_factory: Box<dyn BrokerInterceptorFactoryTrait>,
        config_map: ConfigMap,
        stopped_indicator: Arc<AtomicBool>,
    ) -> Self
    where
        Self: Sized;
    fn get_identifier() -> String
    where
        Self: Sized;

    async fn create_info(&self) -> Box<dyn InfoTrait>;
    async fn create_subscription(&self) -> Box<dyn SubscriptionTrait>;
    async fn create_transaction(&self) -> Box<dyn TransactionTrait>;
    async fn create_heartbeat(&self) -> Option<Box<dyn HeartbeatTrait>>;
}

#[async_trait]
pub trait BrokerInterceptorFactoryTrait: Send + Sync {
    async fn create_info_interceptor(&self) -> Option<Box<dyn InfoInterceptorTrait>>;
    async fn create_subscription_interceptor(
        &self,
    ) -> Option<Box<dyn SubscriptionInterceptorTrait>>;
    async fn create_transaction_interceptor(&self) -> Option<Box<dyn TransactionInterceptorTrait>>;
}

pub struct EmptyBrokerInterceptorFactory {}

impl EmptyBrokerInterceptorFactory {
    pub fn new() -> Self {
        EmptyBrokerInterceptorFactory {}
    }
}

#[async_trait]
impl BrokerInterceptorFactoryTrait for EmptyBrokerInterceptorFactory {
    async fn create_info_interceptor(&self) -> Option<Box<dyn InfoInterceptorTrait>> {
        Option::None
    }

    async fn create_subscription_interceptor(
        &self,
    ) -> Option<Box<dyn SubscriptionInterceptorTrait>> {
        Option::None
    }

    async fn create_transaction_interceptor(&self) -> Option<Box<dyn TransactionInterceptorTrait>> {
        Option::None
    }
}
