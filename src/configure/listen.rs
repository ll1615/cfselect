//! 监听配置模块
//!
//! 定义服务器监听相关的配置项和工具方法

#![allow(dead_code)]

use serde::Deserialize;
use std::net::{AddrParseError, SocketAddr};

/// 监听配置结构体
///
/// # 字段
/// - `host`: 监听主机地址
/// - `port`: 监听端口号
#[derive(Debug, Deserialize, Clone)]
pub struct ListenConfig {
    pub host: String,
    pub port: u16,
}

impl ListenConfig {
    /// 获取完整地址(host:port格式)
    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// 获取HTTP协议地址(http://host:port格式)
    pub fn get_http_addr(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }

    /// 获取SocketAddr结构
    ///
    /// # 返回值
    /// - 成功: 返回SocketAddr
    /// - 失败: 返回AddrParseError
    pub fn get_socket_addr(&self) -> Result<SocketAddr, AddrParseError> {
        self.get_addr().parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试HTTP地址生成
    #[test]
    fn test_http_addr() {
        let config = ListenConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        };
        assert_eq!(config.get_http_addr(), "http://127.0.0.1:8080");
    }

    /// 测试Socket地址解析
    #[test]
    fn test_socket_addr() {
        let config = ListenConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        };
        assert_eq!(
            config.get_socket_addr().unwrap().to_string(),
            "127.0.0.1:8080"
        );
    }
}
