use crate::server::state::AppState;
use axum::Router;

pub mod server;

pub fn setup(state: AppState) -> Router {
    let router = Router::new();
    let router = server::setup(router);

    router.with_state(state)
}