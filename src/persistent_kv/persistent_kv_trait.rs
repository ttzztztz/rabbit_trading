use async_trait::async_trait;
use std::collections::HashMap;

use crate::model::error::Error;

#[derive(Clone)]
pub struct PersistentKVStoreInitializationContext<V: Send + Sync + Clone> {
    pub configuration: HashMap<String, V>,
}

#[async_trait]
pub trait PersistentKVStore<V: Send + Sync + Clone> {
    async fn new(context: PersistentKVStoreInitializationContext<V>) -> Self
    where
        Self: Sized;
    async fn read(&self, key: String) -> Result<V, Error>;
    async fn write(&self, key: String, value: V) -> Result<usize, Error>;
}
