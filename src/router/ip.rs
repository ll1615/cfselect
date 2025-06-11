//! IP优选相关路由模块
//!
//! 提供以下API端点:
//! - POST /ip/select: 启动IP优选任务
//! - GET /ip/select: 获取优选结果
//! - GET /ip/select/status: 查询优选任务状态

use crate::api::ip::{select, selected, status};
use crate::server::state::AppState;
use axum::Router;
use axum::routing::*;

/// 配置IP优选相关路由
pub fn setup(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router.nest(
        "/ip",
        Router::new()
            .route("/select", post(select))
            .route("/select", get(selected))
            .route("/select/status", get(status)),
    )
}
