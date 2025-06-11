//! Cloudflare IP优选API模块
//!
//! 本模块提供Cloudflare IP优选相关功能，包括：
//! - 批量IP测试与优选(/select)
//! - 查询优选任务状态(/status)
//! - 获取优选结果IP列表(/selected)
//!
//! # 实现原理
//! 1. 使用CloudflareSpeedTest工具进行IP延迟测试
//! 2. 通过异步任务执行耗时操作
//! 3. 使用全局状态锁跟踪任务进度

use crate::model::response;
use crate::model::response::Resp;
use crate::model::select::Status;
use axum::Json;
use std::sync::{LazyLock, RwLock};
use tokio::fs;
use tokio::process::Command;
use tracing::*;

/// 全局IP优选任务状态锁
///
/// 使用`LazyLock`实现懒加载，`RwLock`保证线程安全
///
/// # 状态说明
/// - `Pending`: 初始状态，无进行中的优选任务
/// - `Processing`: 优选任务进行中
/// - `Success`: 优选任务成功完成
/// - `Failed`: 优选任务失败，包含错误信息
pub static STATUS: LazyLock<RwLock<Status>> = LazyLock::new(|| RwLock::new(Status::Pending));

/// 启动IP选择任务
///
/// # 参数
/// - `req`: 要测试的IP地址列表(JSON格式)
///
/// # 返回值
/// - 成功: 返回成功响应(立即返回，实际处理在后台进行)
/// - 失败: 返回错误响应
pub async fn select(
    Json(req): Json<Vec<String>>,
) -> anyhow::Result<Resp<Vec<Vec<String>>>, Resp<()>> {
    // 检查当前状态，避免重复执行优选任务
    if let Status::Processing = STATUS.read()?.clone() {
        return Ok(response::success());
    }

    // 获取写锁并更新状态为PROCESSING
    *STATUS.write()? = Status::Processing;

    // 在后台异步执行优选任务
    tokio::spawn(async move {
        let result = _select(req).await;
        let status = STATUS.write();
        if let Err(e) = status {
            error!("Failed to acquire lock: {}", e);
            return;
        }

        // 处理完成后更新状态
        if let Err(e) = result {
            *status.unwrap() = Status::Failed(e.to_string());
            return;
        }

        *status.unwrap() = Status::Success;
    });

    Ok(response::success())
}

/// 实际执行IP选择的核心逻辑
/// # 参数
/// - `ip_ranges`: 要测试的IP地址列表
/// # 返回值
/// - 成功: Ok(())
/// - 失败: 返回具体错误信息
async fn _select(ip_ranges: Vec<String>) -> anyhow::Result<()> {
    // 将IP列表写入临时文件ip.txt
    fs::write("ip.txt", ip_ranges.join("\n")).await?;

    // 调用CloudflareSpeedTest命令行工具，该工具会测试IP延迟并生成result.csv结果文件
    let mut cmd = Command::new("CloudflareSpeedTest");
    let output = cmd.output().await?;

    // 检查命令执行结果
    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("execute command failed: {}", err_msg));
    }

    Ok(())
}

/// 获取当前IP选择任务的状态
/// # 返回值
/// - 成功: 返回当前状态
/// - 失败: 如果状态为Failed，返回错误信息
pub async fn status() -> anyhow::Result<Resp<Status>, Resp<()>> {
    let status = STATUS.read()?.clone();
    if let Status::Failed(err) = status {
        return Err(err.into());
    }

    Ok(response::success_data(status))
}

/// 获取已选择的IP结果
/// # 返回值
/// - 成功: 返回筛选后的IP列表(只包含延迟>0的IP)
/// - 失败: 返回错误响应
pub async fn selected() -> anyhow::Result<Resp<Vec<Vec<String>>>, Resp<()>> {
    // 读取结果文件并过滤有效IP，格式为CSV，包含IP地址和延迟数据
    let result: Vec<Vec<String>> = tokio::fs::read_to_string("result.csv")
        .await?
        .lines()
        .skip(1) // 跳过CSV标题行
        .map(|line| line.split(',').map(|s| s.to_string()).collect())
        // 过滤出有效IP(延迟>0ms)
        .filter(|row: &Vec<String>| {
            row.last()
                .is_some_and(|n| n.parse::<f64>().is_ok_and(|n| n > 0.0))
        })
        .collect();

    Ok(response::success_data(result))
}
