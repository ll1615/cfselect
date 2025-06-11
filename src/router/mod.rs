//! 应用路由主模块
//!
//! 负责组织所有API路由和静态资源路由，主要功能：
//! - 初始化基础路由
//! - 集成各子模块路由
//! - 处理404未找到路由
//!
//! # 路由结构
//! - /api: 所有API路由的命名空间
//! - /assets: 静态资源路由
//! - /*: 前端路由

use crate::server::state::AppState;
use axum::Router;
use axum::http::StatusCode;

pub mod assets;
pub mod dns;
pub mod ip;
pub mod server;

/// 初始化并配置所有应用路由
pub fn setup(state: AppState) -> Router {
    // 1. 初始化基础路由，设置404处理
    let router = Router::new().fallback(|| async { StatusCode::NOT_FOUND });

    // 2-4. 按顺序集成各功能模块路由
    let router = server::setup(router);
    let router = ip::setup(router);
    let router = dns::setup(router);

    // 5. 将API路由统一挂载到/api路径下
    let router = Router::new().nest("/api", router);

    // 6. 添加静态资源路由
    let router = assets::setup(router);

    // 7. 注入共享状态
    router.with_state(state)
}
