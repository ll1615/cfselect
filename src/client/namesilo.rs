//! Namesilo DNS API客户端模块
//!
//! 提供与Namesilo DNS API交互的功能，包括:
//! - DNS记录更新
//! - DNS记录列表查询

use crate::configure::namesilo::NamesiloConfig;
use anyhow::Ok;
use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::sync::LazyLock;

// 全局HTTP客户端，使用LazyLock确保线程安全初始化
static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .build()
        .expect("Failed to create reqwest client")
});

/// 更新DNS记录
///
/// # 参数
/// - `config`: Namesilo配置信息
/// - `ip`: 要更新的IP地址
/// - `rrid`: 记录ID
///
/// # 返回值
/// - 成功: Ok(())
/// - 失败: 返回错误信息
pub async fn dns_update(config: &NamesiloConfig, ip: &str, rrid: &str) -> anyhow::Result<()> {
    let mut url = reqwest::Url::parse(&config.url)?;
    url.set_path("/api/dnsUpdateRecord");
    url.query_pairs_mut().extend_pairs([
        ("version", "1"),
        ("type", "json"),
        ("key", config.key.as_str()),
        ("domain", config.domain.as_str()),
        ("rrid", rrid),
        ("rrhost", config.rrhost.as_str()),
        ("rrvalue", ip),
        ("rrttl", config.rrttl.as_str()),
    ]);

    let response = CLIENT.get(url).send().await?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to update DNS record: {}",
            response.status()
        ));
    }

    let nr: NamesiloResponse<DnsUpdateReply> = response.json().await?;
    if nr.reply.code != ResponseCode::Success {
        return Err(anyhow::anyhow!(
            "Failed to update DNS record: {}",
            nr.reply.detail
        ));
    }

    Ok(())
}

/// 获取DNS记录列表
///
/// # 参数
/// - `config`: Namesilo配置信息
///
/// # 返回值
/// - 成功: 返回资源记录列表
/// - 失败: 返回错误信息
pub async fn dns_list(config: &NamesiloConfig) -> anyhow::Result<Vec<ResourceRecord>> {
    let mut url = reqwest::Url::parse(&config.url)?;
    url.set_path("/api/dnsListRecords");
    url.query_pairs_mut().extend_pairs([
        ("version", "1"),
        ("type", "json"),
        ("key", config.key.as_str()),
        ("domain", config.domain.as_str()),
    ]);

    let response = CLIENT.get(url).send().await?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to list DNS record: {}",
            response.status()
        ));
    }

    let nr: NamesiloResponse<DnsListReply> = response.json().await?;
    if nr.reply.code != ResponseCode::Success {
        return Err(anyhow::anyhow!(
            "Failed to list DNS record: {}",
            nr.reply.detail
        ));
    }

    Ok(nr.reply.resource_record)
}

#[derive(Deserialize, Debug)]
pub struct NamesiloResponse<Reply> {
    pub reply: Reply,
}

#[derive(Deserialize, Debug)]
pub struct DnsUpdateReply {
    pub code: ResponseCode,
    pub detail: String,
}

#[derive(Deserialize, Debug)]
pub struct DnsListReply {
    pub code: ResponseCode,
    pub detail: String,
    pub resource_record: Vec<ResourceRecord>,
}

#[derive(Deserialize, Debug)]
pub struct ResourceRecord {
    pub record_id: String,
    pub host: String,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
#[repr(u16)]
pub enum ResponseCode {
    #[default]
    Success = 300,
}
