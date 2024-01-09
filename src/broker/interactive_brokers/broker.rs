use async_trait::async_trait;

use super::{client_portal::client::IBClientPortal, heartbeat::InteractiveBrokersHeartbeat};
use crate::{
    broker::common::{
        broker::{BrokerInterceptorFactoryTrait, BrokerTrait},
        heartbeat::HeartbeatTrait,
        info::InfoTrait,
        subscription::SubscriptionTrait,
        transaction::TransactionTrait,
    },
    model::common::types::ConfigMap,
};

pub struct InteractiveBrokersBroker {
    config_map: ConfigMap,
    interceptor_factory: Box<dyn BrokerInterceptorFactoryTrait>,
}

impl InteractiveBrokersBroker {
    pub(super) fn create_ib_client_portal(config_map: ConfigMap) -> IBClientPortal {
        const CONFIG_KEY_HOST: &'static str = "ibkr.cp.host";

        let host = config_map.get(CONFIG_KEY_HOST).map(|val| val.clone());
        IBClientPortal::new(host)
    }
}

#[async_trait]
impl BrokerTrait for InteractiveBrokersBroker {
    fn new(
        interceptor_factory: Box<dyn BrokerInterceptorFactoryTrait>,
        config_map: ConfigMap,
    ) -> Self {
        InteractiveBrokersBroker {
            config_map,
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

    async fn create_heartbeat(&self) -> Option<Box<dyn HeartbeatTrait>> {
        Option::Some(Box::new(
            InteractiveBrokersHeartbeat::new(self.config_map.clone()).await,
        ))
    }
}
