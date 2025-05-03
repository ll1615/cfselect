use crate::model::response;
use crate::model::response::Resp;

pub async fn health_check() -> anyhow::Result<Resp<()>, ()> {
    Ok(response::success2(None))
}

