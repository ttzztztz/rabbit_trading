use anyhow::Error;
use async_trait::async_trait;
use std::sync::{atomic::AtomicBool, Arc};

use crate::{
    broker::common::broker::BrokerTrait, model::common::types::ConfigMap,
    persistent_kv::common::store::PersistentKVStoreTrait,
};

pub struct StrategyContext {
    pub broker_list: Vec<Box<dyn BrokerTrait>>,
    pub persistent_kv_store: Box<dyn PersistentKVStoreTrait>,
    pub config_map: ConfigMap,
    pub stopped_indicator: Arc<AtomicBool>,
}

#[async_trait]
pub trait StrategyTrait: Send + Sync {
    fn new(strategy_context: StrategyContext) -> Self
    where
        Self: Sized;
    fn get_identifier() -> String
    where
        Self: Sized;

    async fn start(&self) -> Result<(), Error>;
    async fn stop(&self) -> Result<(), Error>;
}
