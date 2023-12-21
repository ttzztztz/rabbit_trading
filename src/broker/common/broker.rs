use async_trait::async_trait;

use super::{
    info::{InfoContext, InfoTrait},
    subscription::SubscriptionTrait,
    transaction::TransactionTrait,
};

#[async_trait]
pub trait BrokerTrait {
    async fn create_info(context: InfoContext) -> Box<dyn InfoTrait>
    where
        Self: Sized;
    async fn create_subscription(context: InfoContext) -> Box<dyn SubscriptionTrait>
    where
        Self: Sized;
    async fn create_transaction() -> Box<dyn TransactionTrait>
    where
        Self: Sized;
}
