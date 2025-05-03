use crate::server::state::AppState;
use tower_http::services::{ServeDir, ServeFile};

pub fn setup(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
        .nest_service("/assets/", ServeDir::new("assets"))
        .fallback_service(ServeFile::new("assets/index.html"))
}
