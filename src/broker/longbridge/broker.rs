use async_trait::async_trait;

use super::{
    info::LongBridgeInfo, subscription::LongBridgeSubscription, transaction::LongBridgeTransaction,
};
use crate::broker::common::{
    broker::BrokerTrait,
    info::{InfoInterceptorTrait, InfoTrait},
    subscription::{SubscriptionInterceptorTrait, SubscriptionProxy, SubscriptionTrait},
    transaction::{TransactionInterceptorTrait, TransactionProxy, TransactionTrait},
};

pub struct LongBridgeBroker {}

impl LongBridgeBroker {
    const IDENTIFIER: &'static str = "longbridge";
}

#[async_trait]
impl BrokerTrait for LongBridgeBroker {
    fn get_broker_identifier(&self) -> String {
        return Self::IDENTIFIER.to_owned();
    }

    async fn create_info(
        &self,
        interceptor: Option<Box<dyn InfoInterceptorTrait + Send + Sync>>,
    ) -> Box<dyn InfoTrait + Send + Sync> {
        Box::new(LongBridgeInfo::new().await)
    }

    async fn create_subscription(
        &self,
        interceptor: Option<Box<dyn SubscriptionInterceptorTrait + Send + Sync>>,
    ) -> Box<dyn SubscriptionTrait + Send + Sync> {
        let longbridge_subscription = Box::new(LongBridgeSubscription::new().await);
        Box::new(SubscriptionProxy::new(longbridge_subscription, interceptor))
    }

    async fn create_transaction(
        &self,
        interceptor: Option<Box<dyn TransactionInterceptorTrait + Send + Sync>>,
    ) -> Box<dyn TransactionTrait + Send + Sync> {
        let longbridge_transaction = Box::new(LongBridgeTransaction::new().await);
        Box::new(TransactionProxy::new(longbridge_transaction, interceptor))
    }
}
