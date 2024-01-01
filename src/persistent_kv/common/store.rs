use async_trait::async_trait;

use crate::model::common::{error::Error, types::ConfigMap};

pub type BytesArray = Vec<u8>;

#[async_trait]
pub trait PersistentKVStoreTrait: Send + Sync {
    fn get_identifier() -> String
    where
        Self: Sized;

    async fn new(config_map: ConfigMap) -> Self
    where
        Self: Sized;
    async fn read(&self, key: String) -> Result<BytesArray, Error>;
    async fn write(&self, key: String, value: BytesArray) -> Result<usize, Error>;
}
