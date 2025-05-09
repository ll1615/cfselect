use axum::Json;
use axum::extract::State;
use serde::Deserialize;

use crate::client::namesilo;
use crate::model::response;
use crate::model::response::Resp;
use crate::server::state::AppState;

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
    let rr = records
        .iter()
        .find(|record| record.host == target_host)
        .ok_or("target host not found")?;

    // 更新 DNS 记录
    namesilo::dns_update(&state.config.namesilo, &req.ip, &rr.record_id).await?;

    Ok(response::success())
}

#[derive(Deserialize)]
pub struct SyncRequest {
    pub ip: String,
}
