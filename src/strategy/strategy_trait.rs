use async_trait::async_trait;

use crate::{
    info::info_trait::Info, persistent_kv::persistent_kv_trait::PersistentKVStore,
    subscription::subscription_trait::Subscription,
};

pub struct StrategyContext<V: Send + Sync + Clone = String> {
    pub info: Box<dyn Info>,
    // pub position: Position, # todo
    pub subscription: Box<dyn Subscription>,
    // pub tarnsaction: Transaction, # todo
    pub persistent_kv_store: Box<dyn PersistentKVStore<V>>,
}

#[async_trait]
pub trait Strategy<V: Send + Sync + Clone = String> {
    async fn new() -> Self
    where
        Self: Sized;
    async fn start(&self, context: StrategyContext<V>);
    async fn stop(&self);
}
