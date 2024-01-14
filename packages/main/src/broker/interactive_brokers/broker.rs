use async_trait::async_trait;
use ibkr_client_portal::client::IBClientPortal;

use super::heartbeat::InteractiveBrokersHeartbeat;
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
        const CONFIG_KEY_SSL: &'static str = "ibkr.cp.ssl";
        const CONFIG_KEY_HOST: &'static str = "ibkr.cp.host";
        const CONFIG_KEY_ACCOUNT: &'static str = "ibkr.cp.account";
        const CONFIG_DEFAULT_HOST: &'static str = "localhost:5000";

        let host = config_map
            .get(CONFIG_KEY_HOST)
            .map(|val| val.clone())
            .unwrap_or(CONFIG_DEFAULT_HOST.to_owned());
        let account = config_map.get(CONFIG_KEY_ACCOUNT).unwrap().clone();
        let listen_ssl = config_map
            .get(CONFIG_KEY_SSL)
            .map(|ssl| ssl == "true")
            .unwrap_or(true);
        IBClientPortal::new(account, host, listen_ssl)
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
