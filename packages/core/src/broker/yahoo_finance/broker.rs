use async_trait::async_trait;
use std::sync::{atomic::AtomicBool, Arc};

use super::{info::YahooFinanceInfo, subscription::YahooFinanceSubscription};
use crate::{
    broker::common::{
        broker::{BrokerInterceptorFactoryTrait, BrokerTrait},
        heartbeat::HeartbeatTrait,
        info::{InfoProxy, InfoTrait},
        subscription::{SubscriptionProxy, SubscriptionTrait},
        transaction::TransactionTrait,
    },
    model::common::types::ConfigMap,
};

pub struct YahooFinanceBroker {
    interceptor_factory: Box<dyn BrokerInterceptorFactoryTrait>,
    config_map: ConfigMap,
    stopped_indicator: Arc<AtomicBool>,
}

#[async_trait]
impl BrokerTrait for YahooFinanceBroker {
    fn new(
        interceptor_factory: Box<dyn BrokerInterceptorFactoryTrait>,
        config_map: ConfigMap,
        stopped_indicator: Arc<AtomicBool>,
    ) -> Self {
        YahooFinanceBroker {
            interceptor_factory,
            config_map,
            stopped_indicator,
        }
    }

    fn get_identifier() -> String {
        const IDENTIFIER: &'static str = "yahoo_finance";
        IDENTIFIER.to_owned()
    }

    fn create_info(&self) -> Box<dyn InfoTrait> {
        let yahoo_finance_info = Box::new(YahooFinanceInfo::new(self.config_map.clone()));
        Box::new(InfoProxy::new(
            yahoo_finance_info,
            self.interceptor_factory.create_info_interceptor(),
        ))
    }

    fn create_subscription(&self) -> Box<dyn SubscriptionTrait> {
        let yahoo_finance_subscription =
            Box::new(YahooFinanceSubscription::new(self.config_map.clone()));
        Box::new(SubscriptionProxy::new(
            yahoo_finance_subscription,
            self.interceptor_factory.create_subscription_interceptor(),
        ))
    }

    fn create_transaction(&self) -> Box<dyn TransactionTrait> {
        panic!("Yahoo Finance cannot be used for trading")
    }

    fn create_heartbeat(&self) -> Option<Box<dyn HeartbeatTrait>> {
        Option::None
    }
}
