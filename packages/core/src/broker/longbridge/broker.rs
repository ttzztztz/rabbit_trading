use async_trait::async_trait;
use std::sync::{atomic::AtomicBool, Arc};

use super::{
    info::LongBridgeInfo, subscription::LongBridgeSubscription, transaction::LongBridgeTransaction,
};
use crate::{
    broker::common::{
        broker::{BrokerInterceptorFactoryTrait, BrokerTrait},
        heartbeat::HeartbeatTrait,
        info::{InfoProxy, InfoTrait},
        subscription::{SubscriptionProxy, SubscriptionTrait},
        transaction::{TransactionProxy, TransactionTrait},
    },
    model::common::types::ConfigMap,
};

pub struct LongBridgeBroker {
    config_map: ConfigMap,
    interceptor_factory: Box<dyn BrokerInterceptorFactoryTrait>,
    stopped_indicator: Arc<AtomicBool>,
}

#[async_trait]
impl BrokerTrait for LongBridgeBroker {
    fn new(
        interceptor_factory: Box<dyn BrokerInterceptorFactoryTrait>,
        config_map: ConfigMap,
        stopped_indicator: Arc<AtomicBool>,
    ) -> Self {
        LongBridgeBroker {
            config_map,
            interceptor_factory,
            stopped_indicator,
        }
    }

    fn get_identifier() -> String {
        const IDENTIFIER: &'static str = "longbridge";
        IDENTIFIER.to_owned()
    }

    fn create_info(&self) -> Box<dyn InfoTrait> {
        let longbridge_info = Box::new(LongBridgeInfo::new(self.config_map.clone()));
        Box::new(InfoProxy::new(
            longbridge_info,
            self.interceptor_factory.create_info_interceptor(),
        ))
    }

    fn create_subscription(&self) -> Box<dyn SubscriptionTrait> {
        let longbridge_subscription =
            Box::new(LongBridgeSubscription::new(self.config_map.clone()));
        Box::new(SubscriptionProxy::new(
            longbridge_subscription,
            self.interceptor_factory.create_subscription_interceptor(),
        ))
    }

    fn create_transaction(&self) -> Box<dyn TransactionTrait> {
        let longbridge_transaction = Box::new(LongBridgeTransaction::new(self.config_map.clone()));
        Box::new(TransactionProxy::new(
            longbridge_transaction,
            self.interceptor_factory.create_transaction_interceptor(),
        ))
    }

    fn create_heartbeat(&self) -> Option<Box<dyn HeartbeatTrait>> {
        Option::None
    }
}
