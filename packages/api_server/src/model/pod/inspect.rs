use serde::{Deserialize, Serialize};

use super::metadata::PodMetadata;

#[derive(Deserialize)]
pub struct InspectPodRequest {
    pub pod_id: String,
}

#[derive(Serialize)]
pub struct InspectPodResponse {
    pub metadata: PodMetadata,
}
