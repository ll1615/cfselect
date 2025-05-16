use crate::model::response;
use crate::model::response::Resp;
use crate::model::select::Status;
use axum::Json;
use std::sync::{LazyLock, RwLock};
use tokio::fs;
use tokio::process::Command;
use tracing::*;

pub static STATUS: LazyLock<RwLock<Status>> = LazyLock::new(|| RwLock::new(Status::Pending));

pub async fn select(
    Json(req): Json<Vec<String>>,
) -> anyhow::Result<Resp<Vec<Vec<String>>>, Resp<()>> {
    if let Status::Processing = STATUS.read()?.clone() {
        return Ok(response::success());
    }

    *STATUS.write()? = Status::Processing;

    tokio::spawn(async move {
        let result = _select(req).await;
        let status = STATUS.write();
        if let Err(e) = status {
            error!("Failed to acquire lock: {}", e);
            return;
        }

        if let Err(e) = result {
            *status.unwrap() = Status::Failed(e.to_string());
            return;
        }

        *status.unwrap() = Status::Success;
    });

    Ok(response::success())
}

async fn _select(ip_ranges: Vec<String>) -> anyhow::Result<()> {
    fs::write("ip.txt", ip_ranges.join("\n")).await?;

    let mut cmd = Command::new("CloudflareSpeedTest");
    let output = cmd.output().await?;

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("execute command failed: {}", err_msg));
    }

    Ok(())
}

pub async fn status() -> anyhow::Result<Resp<Status>, Resp<()>> {
    let status = STATUS.read()?.clone();
    if let Status::Failed(err) = status {
        return Err(err.into());
    }

    Ok(response::success_data(status))
}

pub async fn selected() -> anyhow::Result<Resp<Vec<Vec<String>>>, Resp<()>> {
    let result: Vec<Vec<String>> = tokio::fs::read_to_string("result.csv")
        .await?
        .lines()
        .skip(1)
        .map(|line| line.split(',').map(|s| s.to_string()).collect())
        .filter(|row: &Vec<String>| {
            row.last()
                .is_some_and(|n| n.parse::<f64>().is_ok_and(|n| n > 0.0))
        })
        .collect();

    Ok(response::success_data(result))
}
