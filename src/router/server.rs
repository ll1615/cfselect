//! 服务器健康检查路由模块
//!
//! 提供以下API端点:
//! - GET /server/health_check: 服务器健康状态检查

use crate::api::server::health_check;
use crate::server::state::AppState;
use axum::routing::get;

/// 配置服务器健康检查路由
pub fn setup(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router.route("/server/health_check", get(health_check))
}
