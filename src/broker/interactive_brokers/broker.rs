use async_trait::async_trait;

use crate::{
    broker::common::{
        broker::{BrokerInterceptorFactoryTrait, BrokerTrait},
        info::InfoTrait,
        subscription::SubscriptionTrait,
        transaction::TransactionTrait,
    },
    model::common::types::ConfigMap,
};

pub struct InteractiveBrokersBroker {
    interceptor_factory: Box<dyn BrokerInterceptorFactoryTrait>,
}

#[async_trait]
impl BrokerTrait for InteractiveBrokersBroker {
    fn new(
        interceptor_factory: Box<dyn BrokerInterceptorFactoryTrait>,
        config_map: ConfigMap,
    ) -> Self {
        InteractiveBrokersBroker {
            interceptor_factory,
        }
    }

    fn get_identifier() -> String {
        const IDENTIFIER: &'static str = "interactive_brokers";
        return IDENTIFIER.to_owned();
    }

    async fn create_info(&self) -> Box<dyn InfoTrait> {
        todo!()
    }

    async fn create_subscription(&self) -> Box<dyn SubscriptionTrait> {
        todo!()
    }

    async fn create_transaction(&self) -> Box<dyn TransactionTrait> {
        todo!()
    }
}
