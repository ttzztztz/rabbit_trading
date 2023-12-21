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
    async fn create_info(context: InfoContext) -> Box<dyn InfoTrait> {
        Box::new(LongBridgeInfo::new(context).await)
    }

    async fn create_subscription(context: InfoContext) -> Box<dyn SubscriptionTrait> {
        Box::new(LongBridgeSubscription::new(context).await)
    }

    async fn create_transaction() -> Box<dyn TransactionTrait> {
        Box::new(LongBridgeTransaction::new().await)
    }
}
