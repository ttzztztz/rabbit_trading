use std::sync::{atomic::AtomicBool, Arc};

use super::{
    heartbeat::HeartbeatTrait,
    info::{InfoInterceptorTrait, InfoTrait},
    subscription::{SubscriptionInterceptorTrait, SubscriptionTrait},
    transaction::{TransactionInterceptorTrait, TransactionTrait},
};
use crate::model::common::types::ConfigMap;

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

    fn create_info(&self) -> Box<dyn InfoTrait>;
    fn create_subscription(&self) -> Box<dyn SubscriptionTrait>;
    fn create_transaction(&self) -> Box<dyn TransactionTrait>;
    fn create_heartbeat(&self) -> Option<Box<dyn HeartbeatTrait>>;
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
