use async_trait::async_trait;

use super::{
    info::{InfoContext, InfoTrait},
    subscription::SubscriptionTrait,
    transaction::TransactionTrait,
};

#[async_trait]
pub trait BrokerTrait {
    async fn create_info(&self, context: InfoContext) -> Box<dyn InfoTrait + Send + Sync>;
    async fn create_subscription(
        &self,
        context: InfoContext,
    ) -> Box<dyn SubscriptionTrait + Send + Sync>;
    async fn create_transaction(&self) -> Box<dyn TransactionTrait + Send + Sync>;
}
