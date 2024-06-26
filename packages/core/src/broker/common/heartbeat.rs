use anyhow::Error;
use async_trait::async_trait;
use std::sync::{atomic::AtomicBool, Arc};

use crate::model::common::types::ConfigMap;

#[async_trait]
pub trait HeartbeatTrait: Send + Sync {
    fn new(config_map: ConfigMap, stopped_indicator: Arc<AtomicBool>) -> Self
    where
        Self: Sized;

    async fn start(&self) -> Result<(), Error>;
    async fn stop(&self) -> Result<(), Error>;
}
