//! 静态资源路由模块
//!
//! 负责处理:
//! - 静态资源服务(/assets/*)
//! - 前端路由回退(/* -> index.html)

use crate::server::state::AppState;
use tower_http::services::{ServeDir, ServeFile};

/// 配置静态资源路由
pub fn setup(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
        .nest_service("/assets/", ServeDir::new("assets"))
        .fallback_service(ServeFile::new("assets/index.html"))
}
