use serde::{Deserialize, Serialize};

use super::{
    broker::BrokerConfig, event_listener::EventListenerConfig,
    metrics_registry::MetricsRegistryConfig, persistent_kv_store::PersistentKVStoreConfig,
    strategy::StrategyConfig,
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PodConfig {
    pub pod_id: String,
    pub broker_list: Vec<BrokerConfig>,
    pub persistent_kv_store: PersistentKVStoreConfig,
    pub strategy: StrategyConfig,
    pub metrics_registry: MetricsRegistryConfig,
    pub event_listener_list: Vec<EventListenerConfig>,
}
