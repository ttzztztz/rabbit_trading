use async_trait::async_trait;

use crate::{
    broker::common::{
        info::InfoTrait, subscription::SubscriptionTrait, transaction::TransactionTrait,
    },
    persistent_kv::common::persistent_kv::PersistentKVStore,
};

pub struct StrategyContext<V = String>
where
    V: Send + Sync + Clone,
{
    pub broker_info_list: Vec<Box<dyn InfoTrait + Send + Sync>>,
    pub broker_transaction_list: Vec<Box<dyn TransactionTrait + Send + Sync>>,
    pub broker_subscription_list: Vec<Box<dyn SubscriptionTrait + Send + Sync>>,
    pub persistent_kv_store: Box<dyn PersistentKVStore<V> + Send + Sync>,
}

#[async_trait]
pub trait StrategyTrait<V: Send + Sync + Clone = String> {
    async fn new(strategy_context: StrategyContext<V>) -> Self
    where
        Self: Sized;
    async fn start(&self);
    async fn stop(&self);
}
