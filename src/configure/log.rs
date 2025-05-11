use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LogConfig {
    pub file: FileConfig,
    pub console: ConsoleConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FileConfig {
    pub enabled: bool,
    pub level: String,
    pub dir: String,
    pub name_prefix: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConsoleConfig {
    pub enabled: bool,
    pub level: String,
}
