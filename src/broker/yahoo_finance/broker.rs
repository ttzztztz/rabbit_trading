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
    async fn create_info(&self, context: InfoContext) -> Box<dyn InfoTrait + Send + Sync> {
        Box::new(YahooFinanceInfo::new(context).await)
    }

    async fn create_subscription(
        &self,
        context: InfoContext,
    ) -> Box<dyn SubscriptionTrait + Send + Sync> {
        Box::new(YahooFinanceSubscription::new(context).await)
    }

    async fn create_transaction(&self) -> Box<dyn TransactionTrait + Send + Sync> {
        todo!("Yahoo Finance cannot be used for trading")
    }
}
