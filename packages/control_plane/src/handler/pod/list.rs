use axum::{extract::State, Json};

use crate::{
    handler::state::AppState,
    model::pod::list::{ListPodRequest, ListPodResponse},
};

pub(super) async fn list_handler(
    State(state): State<AppState>,
    Json(_): Json<ListPodRequest>,
) -> Json<ListPodResponse> {
    let readable_pod_store = state.pod_store.read().await;
    let pod_list = readable_pod_store
        .iter()
        .map(|(_, instance)| instance.metadata.clone())
        .collect();

    axum::Json(ListPodResponse { pod_list })
}
