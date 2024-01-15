use serde::{Deserialize, Serialize};

use super::metadata::PodMetadata;

#[derive(Deserialize)]
pub struct ListPodRequest {}

#[derive(Serialize)]
pub struct ListPodResponse {
    pod_list: Vec<PodMetadata>,
}
