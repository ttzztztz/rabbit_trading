use serde::{Deserialize, Serialize};

use super::metadata::PodMetadata;

#[derive(Deserialize)]
pub struct StopPodRequest {
    pod_id: String,
}

#[derive(Serialize)]
pub struct StopPodResponse {
    metadata: PodMetadata,
}
