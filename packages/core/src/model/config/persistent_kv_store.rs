use crate::model::common::types::ConfigMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PersistentKVStoreConfig {
    pub identifier: String,
    pub config_map: ConfigMap,
}
