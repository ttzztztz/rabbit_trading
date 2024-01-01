use async_trait::async_trait;

use super::{
    info::LongBridgeInfo, subscription::LongBridgeSubscription, transaction::LongBridgeTransaction,
};
use crate::{
    broker::common::{
        broker::{BrokerInterceptorFactoryTrait, BrokerTrait},
        info::{InfoProxy, InfoTrait},
        subscription::{SubscriptionProxy, SubscriptionTrait},
        transaction::{TransactionProxy, TransactionTrait},
    },
    model::common::types::ConfigMap,
};

pub struct LongBridgeBroker {
    interceptor_factory: Box<dyn BrokerInterceptorFactoryTrait>,
}

#[async_trait]
impl BrokerTrait for LongBridgeBroker {
    fn new(
        interceptor_factory: Box<dyn BrokerInterceptorFactoryTrait>,
        _config_map: ConfigMap,
    ) -> Self {
        LongBridgeBroker {
            interceptor_factory,
        }
    }

    fn get_identifier() -> String {
        const IDENTIFIER: &'static str = "longbridge";
        return IDENTIFIER.to_owned();
    }

    async fn create_info(&self) -> Box<dyn InfoTrait> {
        let longbridge_info = Box::new(LongBridgeInfo::new().await);
        Box::new(InfoProxy::new(
            longbridge_info,
            self.interceptor_factory.create_info_interceptor().await,
        ))
    }

    async fn create_subscription(&self) -> Box<dyn SubscriptionTrait> {
        let longbridge_subscription = Box::new(LongBridgeSubscription::new().await);
        Box::new(SubscriptionProxy::new(
            longbridge_subscription,
            self.interceptor_factory
                .create_subscription_interceptor()
                .await,
        ))
    }

    async fn create_transaction(&self) -> Box<dyn TransactionTrait> {
        let longbridge_transaction = Box::new(LongBridgeTransaction::new().await);
        Box::new(TransactionProxy::new(
            longbridge_transaction,
            self.interceptor_factory
                .create_transaction_interceptor()
                .await,
        ))
    }
}
