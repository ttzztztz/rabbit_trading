use super::{
    broker_info::BrokerInfoConfig, broker_subscription::BrokerSubscriptionConfig,
    broker_transaction::BrokerTransactionConfig, persistent_kv_store::PersistentKVStoreConfig,
    strategy::StrategyConfig,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PodConfig {
    pub pod_id: String,
    pub broker_info_list: Vec<BrokerInfoConfig>,
    pub broker_transaction_list: Vec<BrokerTransactionConfig>,
    pub broker_subscription_list: Vec<BrokerSubscriptionConfig>,
    pub persistent_kv_store: PersistentKVStoreConfig,
    pub strategy: StrategyConfig,
}
