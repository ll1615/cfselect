//! 日志配置模块
//!
//! 定义日志相关的配置项，支持文件和终端两种输出方式

use serde::Deserialize;

/// 日志总配置
///
/// # 字段
/// - `file`: 文件日志配置
/// - `console`: 控制台日志配置
#[derive(Debug, Deserialize, Clone)]
pub struct LogConfig {
    pub file: FileConfig,
    pub console: ConsoleConfig,
}

/// 文件日志配置
///
/// # 字段
/// - `enabled`: 是否启用
/// - `level`: 日志级别
/// - `dir`: 日志目录
/// - `name_prefix`: 日志文件名前缀
#[derive(Debug, Deserialize, Clone)]
pub struct FileConfig {
    pub enabled: bool,
    pub level: String,
    pub dir: String,
    pub name_prefix: String,
}

/// 控制台日志配置
///
/// # 字段
/// - `enabled`: 是否启用
/// - `level`: 日志级别
#[derive(Debug, Deserialize, Clone)]
pub struct ConsoleConfig {
    pub enabled: bool,
    pub level: String,
}
