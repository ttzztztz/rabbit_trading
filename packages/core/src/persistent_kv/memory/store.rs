use anyhow::{anyhow, Error};
use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    model::common::types::ConfigMap,
    persistent_kv::common::store::{BytesArray, PersistentKVStoreTrait},
};

pub struct MemoryKVStore {
    data: Arc<RwLock<HashMap<String, BytesArray>>>,
}

#[async_trait]
impl PersistentKVStoreTrait for MemoryKVStore {
    fn get_identifier() -> String {
        const IDENTIFIER: &'static str = "MemoryKVStore";
        IDENTIFIER.to_owned()
    }

    async fn new(_: ConfigMap) -> Self {
        MemoryKVStore {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn read(&self, key: String) -> Result<BytesArray, Error> {
        match self.data.read().await.get(key.as_str()) {
            Option::Some(val) => Result::Ok(val.clone()),
            Option::None => Result::Err(anyhow!("MEM_KEY_NOT_EXISTS, key: {}", key)),
        }
    }

    async fn write(&self, key: String, value: BytesArray) -> Result<usize, Error> {
        self.data.write().await.insert(key, value);
        Result::Ok(1usize)
    }
}
