use serde::{Deserialize, Serialize};

use super::metadata::PodMetadata;

#[derive(Deserialize)]
pub struct InspectPodRequest {
    pod_id: String,
}

#[derive(Serialize)]
pub struct InspectPodResponse {
    metadata: PodMetadata,
}
