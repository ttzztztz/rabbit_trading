use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use super::persistent_kv_trait::{PersistentKVStore, PersistentKVStoreInitializationContext};
use crate::model::error::Error;

pub struct MemoryKVStore<V: Send + Sync + Clone> {
    data: Arc<RwLock<HashMap<String, V>>>,
}

impl<V: Send + Sync + Clone> MemoryKVStore<V> {
    const KEY_NOT_EXISTS_MESSAGE: &'static str = "Key Not Exists";

    fn create_key_not_exists_err() -> Error {
        Error {
            code: Self::KEY_NOT_EXISTS_MESSAGE.to_owned(),
            message: Self::KEY_NOT_EXISTS_MESSAGE.to_owned(),
        }
    }
}

#[async_trait]
impl<V: Send + Sync + Clone> PersistentKVStore<V> for MemoryKVStore<V> {
    async fn new(_: PersistentKVStoreInitializationContext<V>) -> Self {
        MemoryKVStore::<V> {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn read(&self, key: String) -> Result<V, Error> {
        match self.data.read().await.get(key.as_str()) {
            Option::Some(val) => Result::Ok(val.clone()),
            Option::None => Result::Err(Self::create_key_not_exists_err()),
        }
    }

    async fn write(&self, key: String, value: V) -> Result<usize, Error> {
        self.data.write().await.insert(key, value);
        Result::Ok(1usize)
    }
}

#[cfg(test)]
mod test_memory_kv_store {
    use std::collections::HashMap;

    use super::{MemoryKVStore, PersistentKVStore, PersistentKVStoreInitializationContext};

    const MAP_KEY_1: &'static str = "key_1";
    const MAP_VALUE_1: &'static str = "114514";
    const MAP_KEY_2: &'static str = "key_2";

    #[tokio::test]
    async fn test_operations_on_memory_kv_store() {
        let kv_store: Box<dyn PersistentKVStore<String>> = Box::new(
            MemoryKVStore::new(PersistentKVStoreInitializationContext {
                configuration: HashMap::new(),
            })
            .await,
        );

        assert!(kv_store.read(MAP_KEY_1.to_owned()).await.is_err());
        assert!(kv_store.read(MAP_KEY_2.to_owned()).await.is_err());
        assert!(kv_store
            .write(MAP_KEY_1.to_owned(), MAP_VALUE_1.to_owned())
            .await
            .is_ok());
        assert!(kv_store.read(MAP_KEY_1.to_owned()).await.is_ok());
        assert!(kv_store.read(MAP_KEY_2.to_owned()).await.is_err());
    }
}
