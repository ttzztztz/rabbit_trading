use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tempfile::{tempdir, TempDir};
use tokio::fs;

use crate::{
    model::common::{error::Error, types::ConfigMap},
    persistent_kv::common::store::{BytesArray, PersistentKVStoreTrait},
};

enum KVStoreBackendPath {
    UserDefinedPath { base_path: String },
    FallbackTempDir { temp_dir: TempDir },
}

pub struct FileSystemKVStore {
    backend_path: KVStoreBackendPath,
}

impl FileSystemKVStore {
    fn to_rabbit_trading_err(io_error: std::io::Error) -> Error {
        const STD_IO_ERROR_CODE: &'static str = "STD::IO_ERROR";

        Error {
            code: STD_IO_ERROR_CODE.to_owned(),
            message: io_error.kind().to_string(),
        }
    }

    fn get_file_path_for_key(&self, key: String) -> PathBuf {
        match &self.backend_path {
            KVStoreBackendPath::UserDefinedPath { base_path } => Path::new(base_path).join(key),
            KVStoreBackendPath::FallbackTempDir { temp_dir } => temp_dir.path().join(key),
        }
    }
}

#[async_trait]
impl PersistentKVStoreTrait for FileSystemKVStore {
    fn get_identifier() -> String {
        const IDENTIFIER: &'static str = "FileSystemKVStore";
        IDENTIFIER.to_owned()
    }

    async fn new(config_map: ConfigMap) -> Self {
        const CONFIG_KEY_BASE_PATH: &'static str = "persistent.fs.base_path";

        let backend_path = match config_map.get(CONFIG_KEY_BASE_PATH) {
            Some(base_path) => KVStoreBackendPath::UserDefinedPath {
                base_path: base_path.clone(),
            },
            None => {
                let temp_dir = tempdir().unwrap();
                log::warn!(
                    "No persistent.fs.base_path was specified, using temp dir as fallback dir: {}",
                    temp_dir.path().display()
                );
                KVStoreBackendPath::FallbackTempDir { temp_dir }
            }
        };

        FileSystemKVStore { backend_path }
    }

    async fn read(&self, key: String) -> Result<BytesArray, Error> {
        let file_path = self.get_file_path_for_key(key);

        fs::read(file_path)
            .await
            .map_err(Self::to_rabbit_trading_err)
    }

    async fn write(&self, key: String, value: BytesArray) -> Result<usize, Error> {
        let value_len = value.len();
        let file_path = self.get_file_path_for_key(key);

        fs::write(file_path, value)
            .await
            .map(|_| value_len)
            .map_err(Self::to_rabbit_trading_err)
    }
}
