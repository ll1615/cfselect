//! 服务器状态管理模块
//!
//! 负责:
//! - 管理服务器共享状态
//! - 提供线程安全的状态访问
//! - 封装配置信息

use crate::configure::AppConfig;
use std::sync::Arc;

/// 应用共享状态容器
///
/// # 字段
/// - `config`: 应用配置的共享引用
///   - 类型: `Arc<AppConfig>`
///   - 线程安全: 通过Arc保证
///   - 生命周期: 与整个应用相同
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
}

impl AppState {
    /// 创建新的应用状态
    ///
    /// # 参数
    /// - `config`: 应用配置
    ///
    /// # 返回值
    /// - 成功: 返回初始化好的AppState
    /// - 失败: 返回错误信息
    pub async fn new(config: AppConfig) -> anyhow::Result<Self> {
        let config = Arc::new(config);
        Ok(Self { config })
    }
}
