use anyhow::{Context, Error};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};

use super::broker::InteractiveBrokersBroker;
use crate::model::common::types::ConfigMap;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct IBConfig {
    pub symbol_to_conid: HashMap<String, i64>,
}

impl IBConfig {
    pub fn new(config_map: &ConfigMap) -> Result<Self, Error> {
        let path = config_map
            .get(InteractiveBrokersBroker::CONFIG_KEY_YAML_PATH)
            .map(|val| val.clone())
            .unwrap_or(InteractiveBrokersBroker::CONFIG_VALUE_DEFAULT_YAML_PATH.to_owned());
        let file = File::open(path.clone())?;
        serde_yaml_ng::from_reader(file)
            .with_context(|| format!("Error when reading config from path {}", path))
    }
}
