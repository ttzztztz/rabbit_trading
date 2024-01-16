use rabbit_trading_core::pod::Pod;
use std::{collections::BTreeMap, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    model::pod::metadata::PodMetadata, utils::id_generator::common_trait::IdGeneratorTrait,
};

pub struct PodStoreInstance {
    pub metadata: PodMetadata,
    pub instance: Arc<Pod>,
}

#[derive(Clone)]
pub struct AppState {
    pub pod_store: Arc<RwLock<BTreeMap<String, PodStoreInstance>>>,
    pub id_generator: Arc<Box<dyn IdGeneratorTrait>>, // todo: add an abstraction layer for this
}

impl AppState {
    pub fn new(id_generator: Arc<Box<dyn IdGeneratorTrait>>) -> Self {
        AppState {
            pod_store: Arc::new(RwLock::new(BTreeMap::new())),
            id_generator,
        }
    }
}
