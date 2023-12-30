use async_trait::async_trait;
use std::collections::HashMap;

use crate::model::common::error::Error;

#[derive(Clone)]
pub struct PersistentKVStoreParameters<V: Send + Sync + Clone> {
    pub configuration: HashMap<String, V>,
}

#[async_trait]
pub trait PersistentKVStore<V: Send + Sync + Clone> {
    async fn new(parameters: PersistentKVStoreParameters<V>) -> Self
    where
        Self: Sized;
    async fn read(&self, key: String) -> Result<V, Error>;
    async fn write(&self, key: String, value: V) -> Result<usize, Error>;
}
