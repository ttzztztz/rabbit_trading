use serde::{Deserialize, Serialize};

use crate::model::common::types::ConfigMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EventListenerConfig {
    pub identifier: String,
    pub config_map: ConfigMap,
}
