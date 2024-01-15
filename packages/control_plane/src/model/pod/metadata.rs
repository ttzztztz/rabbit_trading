use rabbit_trading_core::model::config::pod::PodConfig;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PodMetadata {
    name: String,
    id: String,
    created_at: u64,
    config: PodConfig,
}
