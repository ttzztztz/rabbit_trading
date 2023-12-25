use async_trait::async_trait;

use super::{
    info::LongBridgeInfo, subscription::LongBridgeSubscription, transaction::LongBridgeTransaction,
};
use crate::broker::common::{
    broker::BrokerTrait,
    info::{InfoContext, InfoTrait},
    subscription::SubscriptionTrait,
    transaction::{TransactionInterceptorTrait, TransactionReflection, TransactionTrait},
};

pub struct LongBridgeBroker {}

impl LongBridgeBroker {
    const IDENTIFIER: &'static str = "long_bridge";
}

#[async_trait]
impl BrokerTrait for LongBridgeBroker {
    fn get_broker_identifier(&self) -> String {
        return Self::IDENTIFIER.to_owned();
    }

    async fn create_info(&self, context: InfoContext) -> Box<dyn InfoTrait + Send + Sync> {
        Box::new(LongBridgeInfo::new(context).await)
    }

    async fn create_subscription(
        &self,
        context: InfoContext,
    ) -> Box<dyn SubscriptionTrait + Send + Sync> {
        Box::new(LongBridgeSubscription::new(context).await)
    }

    async fn create_transaction(
        &self,
        interceptor: Option<Box<dyn TransactionInterceptorTrait + Send + Sync>>,
    ) -> Box<dyn TransactionTrait + Send + Sync> {
        let long_bridge_transaction = Box::new(LongBridgeTransaction::new().await);
        Box::new(TransactionReflection::new(
            long_bridge_transaction,
            interceptor,
        ))
    }
}
