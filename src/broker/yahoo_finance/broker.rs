use async_trait::async_trait;

use super::{info::YahooFinanceInfo, subscription::YahooFinanceSubscription};
use crate::broker::common::{
    broker::BrokerTrait,
    info::{InfoContext, InfoTrait},
    subscription::SubscriptionTrait,
    transaction::TransactionTrait,
};

pub struct YahooFinanceBroker {}

#[async_trait]
impl BrokerTrait for YahooFinanceBroker {
    async fn create_info(context: InfoContext) -> Box<dyn InfoTrait> {
        Box::new(YahooFinanceInfo::new(context).await)
    }

    async fn create_subscription(context: InfoContext) -> Box<dyn SubscriptionTrait> {
        Box::new(YahooFinanceSubscription::new(context).await)
    }

    async fn create_transaction() -> Box<dyn TransactionTrait> {
        todo!("Yahoo Finance cannot be used for trading")
    }
}
