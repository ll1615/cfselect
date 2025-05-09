#![allow(dead_code)]

use serde::Deserialize;
use std::net::{AddrParseError, SocketAddr};

#[derive(Debug, Deserialize, Clone)]
pub struct ListenConfig {
    pub host: String,
    pub port: u32,
}

impl ListenConfig {
    // 获取地址
    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
    // 获取http地址
    pub fn get_http_addr(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
    // 获取socket地址
    pub fn get_socket_addr(&self) -> Result<SocketAddr, AddrParseError> {
        self.get_addr().parse()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    pub fn app_config_http_addr_test() {
        let config = ListenConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        };
        assert_eq!(config.get_http_addr(), "http://127.0.0.1:8080");
    }
    #[test]
    pub fn app_config_socket_addr_test() {
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
