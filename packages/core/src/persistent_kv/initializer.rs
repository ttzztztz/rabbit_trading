use anyhow::{anyhow, Error};

use super::common::store::PersistentKVStoreTrait;
use crate::model::common::types::ConfigMap;

#[cfg(feature = "persistent__fs")]
use super::fs::store::FileSystemKVStore;
#[cfg(feature = "persistent__memory")]
use super::memory::store::MemoryKVStore;

pub async fn get_persistent_kv_instance(
    identifier: String,
    config_map: ConfigMap,
) -> Result<Box<dyn PersistentKVStoreTrait>, Error> {
    match identifier {
        #[cfg(feature = "persistent__fs")]
        identifier if identifier == FileSystemKVStore::get_identifier() => {
            Result::Ok(Box::new(FileSystemKVStore::new(config_map).await))
        }

        #[cfg(feature = "persistent__memory")]
        identifier if identifier == MemoryKVStore::get_identifier() => {
            Result::Ok(Box::new(MemoryKVStore::new(config_map).await))
        }

        _ => Result::Err(anyhow!(
            "IDENTIFIER_NOT_MATCHED PersistentKV: {}",
            identifier
        )),
    }
}
