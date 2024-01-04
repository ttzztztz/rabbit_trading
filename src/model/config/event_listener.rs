use crate::model::common::types::ConfigMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventListenerConfig {
    pub identifier: String,
    pub config_map: ConfigMap,
}
