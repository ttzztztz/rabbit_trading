use axum::{extract::State, http::StatusCode, Json};

use crate::{
    handler::state::AppState,
    model::pod::inspect::{InspectPodRequest, InspectPodResponse},
};

pub(super) async fn inspect_handler(
    State(state): State<AppState>,
    Json(request): Json<InspectPodRequest>,
) -> Result<Json<InspectPodResponse>, StatusCode> {
    let readable_pod_store = state.pod_store.read().await;
    match readable_pod_store.get(&request.pod_id) {
        Some(pod_instance) => Result::Ok(axum::Json(InspectPodResponse {
            metadata: pod_instance.metadata.clone(),
        })),
        None => Result::Err(StatusCode::NOT_FOUND),
    }
}
