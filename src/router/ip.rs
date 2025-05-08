use crate::api::ip::{select, selected, status};
use crate::server::state::AppState;
use axum::Router;
use axum::routing::*;

pub fn setup(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router.nest(
        "/ip",
        Router::new()
            .route("/select", post(select))
            .route("/select", get(selected))
            .route("/select/status", get(status)),
    )
}
