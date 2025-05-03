use crate::configure::listen::ListenConfig;
use crate::configure::log::LogConfig;
use anyhow::Context;
use config::Environment;
use serde::Deserialize;
use std::io;
use tracing::*;
use tracing_appender::non_blocking::WorkerGuard;


pub mod listen;
pub mod log;


#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub listen: ListenConfig,
    pub log: LogConfig,
}

impl AppConfig {
    // 读取并解析配置文件
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

    pub fn init_tracing(&self) -> anyhow::Result<WorkerGuard> {
        // 设置日志输出时的格式，例如，是否包含日志级别、是否包含日志来源位置、设置日志的时间格式
        // 参考: https://docs.rs/tracing-subscriber/0.3.3/tracing_subscriber/fmt/struct.SubscriberBuilder.html#method.with_timer
        let format = tracing_subscriber::fmt::format()
            .with_level(true)
            .with_target(true);

        // let make_writer: Box<dyn Write + Send + 'static> = if self.log.file.enabled {
        //     Box::new(tracing_appender::rolling::daily(&self.log.file.dir, &self.log.file.name_prefix))
        // } else {
        //     Box::new(io::stdout())
        // };
        // let (non_blocking, _guard) = tracing_appender::non_blocking(writer);

        let (make_writer, _guard) = if self.log.file.enabled {
            let file_appender = tracing_appender::rolling::daily(&self.log.file.dir, &self.log.file.name_prefix);
            tracing_appender::non_blocking(file_appender)
        } else {
            tracing_appender::non_blocking(io::stdout())
        };

        let level: Level = self.log.level.parse()?;
        // 初始化并设置日志格式(定制和筛选日志)
        tracing_subscriber::fmt()
            .with_max_level(level)
            // .with_writer(io::stdout) // 写入标准输出
            .with_writer(make_writer) // 写入文件，将覆盖上面的标准输出
            .with_ansi(!self.log.file.enabled)  // 如果日志是写入文件，应将ansi的颜色输出功能关掉
            .event_format(format)
            .init();  // 初始化并将SubScriber设置为全局SubScriber

        Ok(_guard)
    }
}

