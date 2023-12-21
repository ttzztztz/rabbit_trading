use async_trait::async_trait;

use crate::{
    broker::common::broker::BrokerTrait, persistent_kv::persistent_kv_trait::PersistentKVStore,
};

pub struct StrategyContext<V = String>
where
    V: Send + Sync + Clone,
{
    pub broker_list: Vec<Box<dyn BrokerTrait + Send + Sync>>,
    pub persistent_kv_store: Box<dyn PersistentKVStore<V> + Send + Sync>,
}

#[async_trait]
pub trait StrategyTrait<V: Send + Sync + Clone = String> {
    async fn new(context: StrategyContext<V>) -> Self
    where
        Self: Sized;
    async fn start(&self);
    async fn stop(&self);
}
