use async_trait::async_trait;

use super::{
    info::{InfoTrait, InfoContext},
    subscription::SubscriptionTrait,
};

#[async_trait]
pub trait BrokerTrait {
    async fn create_info(context: InfoContext) -> Box<dyn InfoTrait>
    where
        Self: Sized;
    async fn create_subscription(context: InfoContext) -> Box<dyn SubscriptionTrait>
    where
        Self: Sized;
}
