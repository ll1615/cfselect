//! DNS同步相关路由模块
//!
//! 提供以下API端点:
//! - POST /dns/sync: 同步DNS记录到Namesilo

use crate::{api::dns::sync, server::state::AppState};
use axum::routing::post;

/// 配置DNS同步路由
pub fn setup(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router.route("/dns/sync", post(sync))
}
