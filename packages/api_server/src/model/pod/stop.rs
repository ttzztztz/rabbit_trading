use serde::{Deserialize, Serialize};

use super::metadata::PodMetadata;

#[derive(Deserialize)]
pub struct StopPodRequest {
    pub pod_id: String,
}

#[derive(Serialize)]
pub struct StopPodResponse {
    pub metadata: PodMetadata,
}
