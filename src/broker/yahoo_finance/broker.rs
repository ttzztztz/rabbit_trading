use async_trait::async_trait;

use super::{info::YahooFinanceInfo, subscription::YahooFinanceSubscription};
use crate::broker::common::{
    broker::BrokerTrait,
    info::InfoTrait,
    subscription::SubscriptionTrait,
    transaction::{TransactionInterceptorTrait, TransactionTrait},
};

pub struct YahooFinanceBroker {}

impl YahooFinanceBroker {
    const IDENTIFIER: &'static str = "yahoo_finance";
}

#[async_trait]
impl BrokerTrait for YahooFinanceBroker {
    fn get_broker_identifier(&self) -> String {
        return Self::IDENTIFIER.to_owned();
    }

    async fn create_info(&self) -> Box<dyn InfoTrait + Send + Sync> {
        Box::new(YahooFinanceInfo::new().await)
    }

    async fn create_subscription(&self) -> Box<dyn SubscriptionTrait + Send + Sync> {
        Box::new(YahooFinanceSubscription::new().await)
    }

    async fn create_transaction(
        &self,
        _interceptor: Option<Box<dyn TransactionInterceptorTrait + Send + Sync>>,
    ) -> Box<dyn TransactionTrait + Send + Sync> {
        todo!("Yahoo Finance cannot be used for trading")
    }
}
