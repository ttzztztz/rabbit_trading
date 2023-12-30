use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PersistentKVStoreConfig {
    pub identifier: String,
    pub config_map: HashMap<String, String>,
}
