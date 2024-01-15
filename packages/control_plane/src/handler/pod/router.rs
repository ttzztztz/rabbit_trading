use axum::{
    extract::State,
    routing::{get, post},
    Router,
};

use crate::handler::state::AppState;

async fn default(State(state): State<AppState>) -> &'static str {
    "Hello, World!"
}

pub fn initialize_pod_router(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/pod/list", post(default))
        .route("/pod", get(default).post(default).delete(default))
}
