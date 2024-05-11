use ibkr_client_portal::{client::IBClientPortal, retry_policies::ExponentialBackoff};
use std::sync::{atomic::AtomicBool, Arc};

use super::{
    heartbeat::InteractiveBrokersHeartbeat, info::InteractiveBrokersInfo,
    subscription::InteractiveBrokersSubscription, transaction::InteractiveBrokersTransaction,
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

pub struct InteractiveBrokersBroker {
    config_map: ConfigMap,
    interceptor_factory: Box<dyn BrokerInterceptorFactoryTrait>,
    stopped_indicator: Arc<AtomicBool>,
}

impl InteractiveBrokersBroker {
    const CONFIG_KEY_ACCOUNT: &'static str = "ibkr.cp.account";
    const CONFIG_MAX_REPLY_COUNT: &'static str = "ibkr.cp.max.retry.count";

    pub(super) fn create_ib_client_portal(config_map: ConfigMap) -> IBClientPortal {
        const CONFIG_KEY_SSL: &'static str = "ibkr.cp.ssl";
        const CONFIG_KEY_HOST: &'static str = "ibkr.cp.host";
        const CONFIG_DEFAULT_HOST: &'static str = "localhost:5000";

        let host = config_map
            .get(CONFIG_KEY_HOST)
            .map(|val| val.clone())
            .unwrap_or(CONFIG_DEFAULT_HOST.to_owned());
        let account = config_map.get(Self::CONFIG_KEY_ACCOUNT).unwrap().clone();
        let listen_ssl = config_map
            .get(CONFIG_KEY_SSL)
            .map(|ssl| ssl == "true")
            .unwrap_or(true);
        IBClientPortal::new(
            account,
            host,
            listen_ssl,
            ExponentialBackoff::builder().build_with_max_retries(3),
        )
    }

    pub(super) fn get_account_id(config_map: &ConfigMap) -> String {
        config_map.get(Self::CONFIG_KEY_ACCOUNT).unwrap().to_owned()
    }

    pub(super) fn get_place_order_max_reply_count(config_map: &ConfigMap) -> i32 {
        config_map
            .get(Self::CONFIG_MAX_REPLY_COUNT)
            .unwrap()
            .parse()
            .unwrap()
    }
}

impl BrokerTrait for InteractiveBrokersBroker {
    fn new(
        interceptor_factory: Box<dyn BrokerInterceptorFactoryTrait>,
        config_map: ConfigMap,
        stopped_indicator: Arc<AtomicBool>,
    ) -> Self {
        InteractiveBrokersBroker {
            config_map,
            interceptor_factory,
            stopped_indicator,
        }
    }

    fn get_identifier() -> String {
        const IDENTIFIER: &'static str = "interactive_brokers";
        IDENTIFIER.to_owned()
    }

    fn create_info(&self) -> Box<dyn InfoTrait> {
        let interactive_brokers_info =
            Box::new(InteractiveBrokersInfo::new(self.config_map.clone()));
        Box::new(InfoProxy::new(
            interactive_brokers_info,
            self.interceptor_factory.create_info_interceptor(),
        ))
    }

    fn create_subscription(&self) -> Box<dyn SubscriptionTrait> {
        let interactive_brokers_subscription = Box::new(InteractiveBrokersSubscription::new(
            self.config_map.clone(),
            self.stopped_indicator.clone(),
        ));
        Box::new(SubscriptionProxy::new(
            interactive_brokers_subscription,
            self.interceptor_factory.create_subscription_interceptor(),
        ))
    }

    fn create_transaction(&self) -> Box<dyn TransactionTrait> {
        let interactive_brokers_transaction =
            Box::new(InteractiveBrokersTransaction::new(self.config_map.clone()));
        Box::new(TransactionProxy::new(
            interactive_brokers_transaction,
            self.interceptor_factory.create_transaction_interceptor(),
        ))
    }

    fn create_heartbeat(&self) -> Option<Box<dyn HeartbeatTrait>> {
        Option::Some(Box::new(InteractiveBrokersHeartbeat::new(
            self.config_map.clone(),
            self.stopped_indicator.clone(),
        )))
    }
}
