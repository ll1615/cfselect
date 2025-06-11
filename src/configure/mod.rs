//! 应用程序配置模块
//!
//! 提供配置文件的读取、解析和初始化功能
//! 包含以下子模块:
//! - listen: 监听配置
//! - log: 日志配置 
//! - namesilo: Namesilo API配置

use crate::configure::listen::ListenConfig;
use crate::configure::log::LogConfig;
use anyhow::{Context, Ok};
use config::Environment;
use namesilo::NamesiloConfig;
use serde::Deserialize;
use std::io::BufWriter;
use tracing::{level_filters::LevelFilter, *};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

pub mod listen;
pub mod log;
pub mod namesilo;

/// 应用程序配置结构体
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    /// 监听配置
    pub listen: ListenConfig,
    /// 日志配置
    pub log: LogConfig,
    /// Namesilo API配置
    pub namesilo: NamesiloConfig,
}

impl AppConfig {
    /// 读取并解析配置文件
    ///
    /// # 返回值
    /// - 成功: 返回解析后的AppConfig
    /// - 失败: 返回错误信息
    pub fn read() -> anyhow::Result<AppConfig> {
        let conf = config::Config::builder()
            .add_source(config::File::with_name("config.toml"))
            // 添加环境变量来源，以APP为前缀，使用__作为分隔符
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()
            .context("读取配置文件失败")?;

        // 尝试将配置对象反序列化为AppConfig类型
        conf.try_deserialize().context("解析配置文件失败")
    }

    /// 初始化日志追踪系统
    ///
    /// # 返回值
    /// - 成功: 返回日志守卫(WorkerGuard)列表
    /// - 失败: 返回错误信息
    pub fn init_tracing(&self) -> anyhow::Result<Vec<WorkerGuard>> {
        let level_file: Level = self.log.file.level.parse()?;
        let level_console: Level = self.log.console.level.parse()?;
        let mut guards: Vec<WorkerGuard> = Vec::new();

        let file_registry = self.log.file.enabled.then(|| {
            let file_appender =
                tracing_appender::rolling::daily(&self.log.file.dir, &self.log.file.name_prefix);
            let buffered = BufWriter::new(file_appender);
            let (non_blocking, guard) = tracing_appender::non_blocking(buffered);
            guards.push(guard);

            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_target(true)
                .with_ansi(false)
                .with_filter(LevelFilter::from_level(level_file))
        });
        let console_registry = self.log.console.enabled.then(|| {
            let buffered = BufWriter::new(std::io::stdout());
            let (non_blocking, guard) = tracing_appender::non_blocking(buffered);
            guards.push(guard);

            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_target(true)
                .with_file(true)
                .with_line_number(true)
                .with_ansi(true)
                .with_filter(LevelFilter::from_level(level_console))
        });

        let registry = tracing_subscriber::registry()
            .with(file_registry)
            .with(console_registry);

        registry.try_init().context("设置全局默认Subscriber失败")?;

        Ok(guards)
    }
}
