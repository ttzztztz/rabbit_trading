use async_trait::async_trait;

use super::{info::YahooFinanceInfo, subscription::YahooFinanceSubscription};
use crate::broker::common::{
    broker_trait::Broker,
    info_trait::{Info, InfoContext},
    subscription_trait::Subscription,
};

pub struct YahooFinanceBroker {}

#[async_trait]
impl Broker for YahooFinanceBroker {
    async fn create_info(context: InfoContext) -> Box<dyn Info> {
        Box::new(YahooFinanceInfo::new(context).await)
    }

    async fn create_subscription(context: InfoContext) -> Box<dyn Subscription> {
        Box::new(YahooFinanceSubscription::new(context).await)
    }
}
