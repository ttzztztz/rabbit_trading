use super::{
    broker::BrokerConfig, metrics_registry::MetricsRegistryConfig,
    persistent_kv_store::PersistentKVStoreConfig, strategy::StrategyConfig,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PodConfig {
    pub pod_id: String,
    pub broker_list: Vec<BrokerConfig>,
    pub persistent_kv_store: PersistentKVStoreConfig,
    pub strategy: StrategyConfig,
    pub metrics_registry: MetricsRegistryConfig,
}