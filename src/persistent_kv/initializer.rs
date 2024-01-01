use super::common::store::PersistentKVStoreTrait;
use crate::model::common::{error::Error, types::ConfigMap};

#[cfg(feature = "persistent__fs")]
use super::fs::store::FileSystemKVStore;
#[cfg(feature = "persistent__memory")]
use super::memory::store::MemoryKVStore;

pub async fn get_persistent_kv_instance(
    identifier: String,
    config_map: ConfigMap,
) -> Result<Box<dyn PersistentKVStoreTrait>, Error> {
    const IDENTIFIER_NOT_MATCHED_ERROR_CODE: &'static str = "IDENTIFIER_NOT_MATCHED";

    match identifier {
        #[cfg(feature = "persistent__fs")]
        identifier if identifier == FileSystemKVStore::get_identifier() => {
            Result::Ok(Box::new(FileSystemKVStore::new(config_map).await))
        }

        #[cfg(feature = "persistent__memory")]
        identifier if identifier == MemoryKVStore::get_identifier() => {
            Result::Ok(Box::new(MemoryKVStore::new(config_map).await))
        }

        _ => Result::Err(Error {
            code: IDENTIFIER_NOT_MATCHED_ERROR_CODE.to_owned(),
            message: format!("PersistentKV: {}", identifier),
        }),
    }
}
