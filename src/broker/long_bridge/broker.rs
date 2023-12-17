use async_trait::async_trait;

use super::{info::LongBridgeInfo, subscription::LongBridgeSubscription};
use crate::broker::common::{
    broker_trait::Broker,
    info_trait::{Info, InfoContext},
    subscription_trait::Subscription,
};

pub struct LongBridgeBroker {}

#[async_trait]
impl Broker for LongBridgeBroker {
    async fn create_info(context: InfoContext) -> Box<dyn Info> {
        Box::new(LongBridgeInfo::new(context).await)
    }

    async fn create_subscription(context: InfoContext) -> Box<dyn Subscription> {
        Box::new(LongBridgeSubscription::new(context).await)
    }
}
