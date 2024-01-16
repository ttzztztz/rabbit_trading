use axum::{extract::State, Json};
use rabbit_trading_core::pod::Pod;
use std::sync::Arc;

use crate::{
    handler::state::{AppState, PodStoreInstance},
    model::pod::start::{StartPodRequest, StartPodResponse},
    utils::metadata::generate_pod_metadata,
};

pub(super) async fn start_handler(
    State(state): State<AppState>,
    Json(request): Json<StartPodRequest>,
) -> Json<StartPodResponse> {
    let pod_config = request.config;
    let pod_metadata = generate_pod_metadata(state.id_generator.clone(), pod_config.clone());
    let pod_id = pod_metadata.id.clone();

    let pod = Arc::new(Pod::new(pod_config, pod_id.clone()));
    let pod_store_instance = PodStoreInstance {
        metadata: pod_metadata.clone(),
        instance: pod.clone(),
    };
    let mut writable_pod_store = state.pod_store.write().await;
    writable_pod_store.insert(pod_id.clone(), pod_store_instance);

    tokio::task::spawn(async move { pod.clone().start().await });
    axum::Json(StartPodResponse {
        pod_id,
        metadata: pod_metadata,
    })
}
