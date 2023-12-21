use async_trait::async_trait;

use super::{
    info::LongBridgeInfo, subscription::LongBridgeSubscription, transaction::LongBridgeTransaction,
};
use crate::broker::common::{
    broker::BrokerTrait,
    info::{InfoContext, InfoTrait},
    subscription::SubscriptionTrait,
    transaction::TransactionTrait,
};

pub struct LongBridgeBroker {}

#[async_trait]
impl BrokerTrait for LongBridgeBroker {
    async fn create_info(&self, context: InfoContext) -> Box<dyn InfoTrait + Send + Sync> {
        Box::new(LongBridgeInfo::new(context).await)
    }

    async fn create_subscription(
        &self,
        context: InfoContext,
    ) -> Box<dyn SubscriptionTrait + Send + Sync> {
        Box::new(LongBridgeSubscription::new(context).await)
    }

    async fn create_transaction(&self) -> Box<dyn TransactionTrait + Send + Sync> {
        Box::new(LongBridgeTransaction::new().await)
    }
}
