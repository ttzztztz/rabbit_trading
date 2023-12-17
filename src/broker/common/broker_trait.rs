use async_trait::async_trait;

use super::{
    info_trait::{Info, InfoContext},
    subscription_trait::Subscription,
};

#[async_trait]
pub trait Broker {
    async fn create_info(context: InfoContext) -> Box<dyn Info>
    where
        Self: Sized;
    async fn create_subscription(context: InfoContext) -> Box<dyn Subscription>
    where
        Self: Sized;
}
