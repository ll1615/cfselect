//! DNS记录同步模块，提供DNS记录同步功能
//!
//! 主要功能：
//! - 查询DNS记录
//! - 更新DNS记录

use axum::Json;
use axum::extract::State;
use serde::Deserialize;

use crate::client::namesilo;
use crate::model::response;
use crate::model::response::Resp;
use crate::server::state::AppState;

/// 同步DNS记录
///
/// # 参数
/// - `state`: 应用状态，包含配置信息
/// - `req`: 包含要更新的IP地址的请求
///
/// # 返回值
/// - 成功: 返回成功响应
/// - 失败: 返回错误响应
pub async fn sync(
    State(state): State<AppState>,
    Json(req): Json<SyncRequest>,
) -> anyhow::Result<Resp<()>, Resp<()>> {
    // 查找目标 DNS 记录
    let records = namesilo::dns_list(&state.config.namesilo).await?;
    let target_host = format!(
        "{}.{}",
        state.config.namesilo.rrhost, state.config.namesilo.domain
    );

    // 在记录中查找匹配的目标主机
    let rr = records
        .iter()
        .find(|record| record.host == target_host)
        .ok_or("target host not found")?;

    // 更新 DNS 记录到新的IP地址
    namesilo::dns_update(&state.config.namesilo, &req.ip, &rr.record_id).await?;

    Ok(response::success())
}

/// DNS同步请求结构体
///
/// # 字段
/// - `ip`: 要更新的IP地址
#[derive(Deserialize)]
pub struct SyncRequest {
    pub ip: String,
}
