//! API模块
//!
//! 包含应用的所有API端点处理逻辑：
//! - `dns`: DNS记录同步相关API
//! - `ip`: IP优选相关API
//! - `server`: 服务器健康检查API

pub mod dns;
pub mod ip;
pub mod server;
