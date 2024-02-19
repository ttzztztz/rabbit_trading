use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    model::common::{error::Error, types::ConfigMap},
    persistent_kv::common::store::{BytesArray, PersistentKVStoreTrait},
};

pub struct MemoryKVStore {
    data: Arc<RwLock<HashMap<String, BytesArray>>>,
}

impl MemoryKVStore {
    fn create_key_not_exists_err() -> Error {
        const KEY_NOT_EXISTS_ERROR_CODE: &'static str = "MEM_KEY_NOT_EXISTS";
        const KEY_NOT_EXISTS_MESSAGE: &'static str = "Key Not Exists";

        Error {
            code: KEY_NOT_EXISTS_ERROR_CODE.to_owned(),
            message: KEY_NOT_EXISTS_MESSAGE.to_owned(),
        }
    }
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
            Option::None => Result::Err(Self::create_key_not_exists_err()),
        }
    }

    async fn write(&self, key: String, value: BytesArray) -> Result<usize, Error> {
        self.data.write().await.insert(key, value);
        Result::Ok(1usize)
    }
}
