use crate::{api::dns::sync, server::state::AppState};
use axum::routing::post;

pub fn setup(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router.route("/dns/sync", post(sync))
}
