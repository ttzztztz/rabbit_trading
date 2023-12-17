use async_trait::async_trait;
use std::collections::HashMap;

use crate::{
    broker::common::broker_trait::Broker, persistent_kv::persistent_kv_trait::PersistentKVStore,
};

pub struct StrategyContext<V: Send + Sync + Clone = String> {
    pub brokers: HashMap<String, Box<dyn Broker>>,
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
