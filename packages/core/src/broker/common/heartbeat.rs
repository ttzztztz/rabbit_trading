use async_trait::async_trait;
use std::sync::{atomic::AtomicBool, Arc};

use crate::model::common::{error::Error, types::ConfigMap};

#[async_trait]
pub trait HeartbeatTrait: Send + Sync {
    async fn new(config_map: ConfigMap, stopped_indicator: Arc<AtomicBool>) -> Self
    where
        Self: Sized;

    async fn start(&self) -> Result<(), Error>;
}
