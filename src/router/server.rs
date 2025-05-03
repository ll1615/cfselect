use crate::api::server::health_check;
use crate::server::state::AppState;
use axum::routing::get;

pub fn setup(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router.route("/api/server/health_check", get(health_check))
}
