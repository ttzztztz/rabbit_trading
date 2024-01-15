use axum::{routing::post, Router};

async fn default() -> &'static str {
    "Hello, World!"
}

pub fn initialize_pod_router(router: Router) -> Router {
    router
        .route("/pod/list", post(default))
        .route("/pod/inspect", post(default))
        .route("/pod/start", post(default))
        .route("/pod/stop", post(default))
}
