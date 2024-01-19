use axum::{extract::State, http::StatusCode, Json};

use crate::{
    handler::state::AppState,
    model::pod::stop::{StopPodRequest, StopPodResponse},
};

pub(super) async fn stop_handler(
    State(state): State<AppState>,
    Json(request): Json<StopPodRequest>,
) -> Result<Json<StopPodResponse>, StatusCode> {
    let readable_pod_store = state.pod_store.read().await;
    match readable_pod_store.get(&request.pod_id) {
        Some(pod_instance) => pod_instance
            .instance
            .stop()
            .await
            .map(|_| {
                axum::Json(StopPodResponse {
                    metadata: pod_instance.metadata.clone(),
                })
            })
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
        None => Result::Err(StatusCode::NOT_FOUND),
    }
}
