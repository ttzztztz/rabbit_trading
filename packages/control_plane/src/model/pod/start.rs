use rabbit_trading_core::model::config::pod::PodConfig;
use serde::{Deserialize, Serialize};

use super::metadata::PodMetadata;

#[derive(Deserialize)]
pub struct StartPodRequest {
    config: PodConfig,
}

#[derive(Serialize)]
pub struct StartPodResponse {
    pod_id: String,
    metadata: PodMetadata,
}
