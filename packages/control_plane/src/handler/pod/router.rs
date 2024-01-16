use axum::{routing::post, Router};

use super::{
    inspect::inspect_handler, list::list_handler, start::start_handler, stop::stop_handler,
};
use crate::handler::state::AppState;

pub fn initialize_pod_router(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/pod/list", post(list_handler))
        .route("/pod/inspect", post(inspect_handler))
        .route("/pod", post(start_handler).delete(stop_handler))
}
