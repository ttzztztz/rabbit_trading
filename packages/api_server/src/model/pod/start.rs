use rabbit_trading_core::model::config::pod::PodConfig;
use serde::{Deserialize, Serialize};

use super::metadata::PodMetadata;

#[derive(Deserialize)]
pub struct StartPodRequest {
    pub config: PodConfig,
}

#[derive(Serialize)]
pub struct StartPodResponse {
    pub pod_id: String,
    pub metadata: PodMetadata,
}
