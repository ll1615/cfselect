//! Namesilo API配置模块
//!
//! 定义与Namesilo DNS服务交互所需的配置项

use serde::Deserialize;

/// Namesilo API配置
///
/// # 字段
/// - `url`: Namesilo API基础URL
/// - `key`: API密钥
/// - `domain`: 要管理的域名
/// - `rrhost`: 记录主机名(如"@", "www"等)
/// - `rrttl`: DNS记录TTL值
#[derive(Debug, Deserialize, Clone)]
pub struct NamesiloConfig {
    pub url: String,
    pub key: String,
    pub domain: String,
    pub rrhost: String,
    pub rrttl: String,
}
