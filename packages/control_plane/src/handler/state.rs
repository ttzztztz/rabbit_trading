use rabbit_trading_core::pod::Pod;
use std::{collections::BTreeMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub pod_store: Arc<RwLock<BTreeMap<String, Pod>>>,
}
