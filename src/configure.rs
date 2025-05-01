use anyhow::Context;
use config::Environment;
use serde::Deserialize;

use server::ServerConfig;

pub mod server;


#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    // 读取并解析配置文件
    pub fn read() -> anyhow::Result<AppConfig> {
        // 创建配置文件构建器，用于加载和解析配置
        let conf = config::Config::builder()
            .add_source(config::File::with_name("config.toml"))
            // 添加环境变量来源，以APP为前缀，使用__作为分隔符
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()
            .context("读取配置文件失败")?;

        // 尝试将配置对象反序列化为AppConfig类型
        conf.try_deserialize().context("解析配置文件失败")
    }
}
