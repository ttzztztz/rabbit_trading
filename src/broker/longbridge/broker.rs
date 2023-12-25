use async_trait::async_trait;

use super::{
    info::LongBridgeInfo, subscription::LongBridgeSubscription, transaction::LongBridgeTransaction,
};
use crate::broker::common::{
    broker::BrokerTrait,
    info::InfoTrait,
    subscription::SubscriptionTrait,
    transaction::{TransactionInterceptorTrait, TransactionReflection, TransactionTrait},
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

    async fn create_info(&self) -> Box<dyn InfoTrait + Send + Sync> {
        Box::new(LongBridgeInfo::new().await)
    }

    async fn create_subscription(&self) -> Box<dyn SubscriptionTrait + Send + Sync> {
        Box::new(LongBridgeSubscription::new().await)
    }

    async fn create_transaction(
        &self,
        interceptor: Option<Box<dyn TransactionInterceptorTrait + Send + Sync>>,
    ) -> Box<dyn TransactionTrait + Send + Sync> {
        let longbridge_transaction = Box::new(LongBridgeTransaction::new().await);
        Box::new(TransactionReflection::new(
            longbridge_transaction,
            interceptor,
        ))
    }
}
