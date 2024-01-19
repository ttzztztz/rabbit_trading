use rabbit_trading_core::model::config::pod::PodConfig;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct PodMetadata {
    pub id: String,
    pub created_at: u64,
    pub config: PodConfig,
}
