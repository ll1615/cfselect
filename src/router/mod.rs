use crate::server::state::AppState;
use axum::Router;

pub mod assets;
pub mod dns;
pub mod ip;
pub mod server;

pub fn setup(state: AppState) -> Router {
    let router = Router::new();
    let router = server::setup(router);
    let router = ip::setup(router);
    let router = dns::setup(router);
    let router = Router::new().nest("/api", router);

    let router = assets::setup(router);

    router.with_state(state)
}
